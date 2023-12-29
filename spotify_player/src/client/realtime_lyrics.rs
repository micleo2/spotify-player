use crate::state::*;
use anyhow::{Context, Result};
use rspotify::http::Query;
use serde_json::Value;
use std::fs;

const GET_TOKEN_URL: &str = "https://open.spotify.com/get_access_token";

pub struct RealtimeLyricsClient {
    sp_dc_cookie: String,
    access_token: LyricToken,
}

pub struct LyricToken {
    value: String,
    expiration_time_ms: u64,
}

impl RealtimeLyricsClient {
    pub async fn new(http: &reqwest::Client, sp_dc_cookie: String) -> Result<Self> {
        let response = http
            .get(GET_TOKEN_URL)
            .header("User-Agent", "reqwest")
            .header("app-platform", "WebPlayer")
            .header("Cookie", format!("sp_dc={sp_dc_cookie}"))
            .send()
            .await?
            .text()
            .await?;
        let parsed_response: Value = serde_json::from_str(&response)?;
        let access_token_val = parsed_response["accessToken"]
            .as_str()
            .context("missing accessToken")?
            .to_string();
        let expiration_time_ms = parsed_response["accessTokenExpirationTimestampMs"]
            .as_i64()
            .context("missing accessTokenExpirationTimestampMs")?
            as u64;
        Ok(RealtimeLyricsClient {
            sp_dc_cookie,
            access_token: LyricToken {
                value: access_token_val,
                expiration_time_ms,
            },
        })
    }

    pub async fn get_lyrics(
        &mut self,
        http: &reqwest::Client,
        track_id_str: String,
    ) -> Result<RealtimeLyrics> {
        self.ensure_valid_token().await?;

        let url = format!("https://spclient.wg.spotify.com/color-lyrics/v2/track/{track_id_str}");
        let mut payload = Query::with_capacity(3);
        payload.insert("market", "from_token");
        payload.insert("format", "json");
        payload.insert("vocalRemoval", "false");
        let token_str = &self.access_token.value;
        let raw_response_str = http
            .get(url.clone())
            .header(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {token_str}"),
            )
            .header("app-platform", "WebPlayer")
            .query(&payload)
            .send()
            .await?
            .text()
            .await?
            .to_string();

        let v: Value = serde_json::from_str(&raw_response_str)?;
        let synced_lyrics = &v["lyrics"]["lines"];
        let mut typed_res = RealtimeLyrics { lyrics: Vec::new() };
        if synced_lyrics.is_array() {
            let lyrics_arr = synced_lyrics.as_array().context("no lyrics array")?;
            for elm in lyrics_arr {
                let cur_word = elm["words"].as_str().context("no words property")?;
                typed_res.lyrics.push(RealtimeLyric {
                    words: if cur_word.is_empty() {
                        "♪".to_string()
                    } else {
                        cur_word.to_owned()
                    },
                    start_time_ms: elm["startTimeMs"]
                        .as_str()
                        .context("no startTimeMs property")?
                        .parse()
                        .unwrap(),
                });
            }
        }
        // If the first lyric doesn't start until 2 seconds into the song, insert a placeholder
        // lyric.
        if typed_res.lyrics[0].start_time_ms > 2000 {
            typed_res.lyrics.insert(
                0,
                RealtimeLyric {
                    words: "♪".to_string(),
                    start_time_ms: 0,
                },
            )
        }
        Ok(typed_res)
    }

    async fn ensure_valid_token(&mut self) -> Result<()> {
        Ok(())
    }
}
