use crate::{command, state::model::*};
use tui::widgets::ListState;

#[derive(Debug)]
pub enum PopupState {
    CommandHelp { scroll_offset: usize },
    Search { query: String },
    UserPlaylistList(PlaylistPopupAction, ListState),
    UserFollowedArtistList(ListState),
    UserSavedAlbumList(ListState),
    DeviceList(ListState),
    ArtistList(ArtistPopupAction, Vec<Artist>, ListState),
    ThemeList(Vec<crate::config::Theme>, ListState),
    ActionList(ActionListItem, ListState),
    Queue { scroll_offset: usize },
}

#[derive(Debug, Clone)]
pub enum ActionListItem {
    Track(Track, Vec<command::TrackAction>),
    Artist(Artist, Vec<command::ArtistAction>),
    Album(Album, Vec<command::AlbumAction>),
    Playlist(Playlist, Vec<command::PlaylistAction>),
}

/// An action on an item in a playlist popup list
#[derive(Debug)]
pub enum PlaylistPopupAction {
    Browse,
    AddTrack(TrackId<'static>),
}

/// An action on an item in an artist popup list
#[derive(Copy, Clone, Debug)]
pub enum ArtistPopupAction {
    Browse,
    ShowActions,
}

impl PopupState {
    /// gets the (immutable) list state of a (list) popup
    pub fn list_state(&self) -> Option<&ListState> {
        match self {
            Self::DeviceList(list_state) => Some(list_state),
            Self::UserPlaylistList(.., list_state) => Some(list_state),
            Self::UserFollowedArtistList(list_state) => Some(list_state),
            Self::UserSavedAlbumList(list_state) => Some(list_state),
            Self::ArtistList(.., list_state) => Some(list_state),
            Self::ThemeList(.., list_state) => Some(list_state),
            Self::ActionList(.., list_state) => Some(list_state),
            Self::CommandHelp { .. } | Self::Search { .. } | Self::Queue { .. } => None,
        }
    }

    /// gets the (mutable) list state of a (list) popup
    pub fn list_state_mut(&mut self) -> Option<&mut ListState> {
        match self {
            Self::DeviceList(list_state) => Some(list_state),
            Self::UserPlaylistList(.., list_state) => Some(list_state),
            Self::UserFollowedArtistList(list_state) => Some(list_state),
            Self::UserSavedAlbumList(list_state) => Some(list_state),
            Self::ArtistList(.., list_state) => Some(list_state),
            Self::ThemeList(.., list_state) => Some(list_state),
            Self::ActionList(.., list_state) => Some(list_state),
            Self::CommandHelp { .. } | Self::Search { .. } | Self::Queue { .. } => None,
        }
    }

    /// gets the selected position of a (list) popup
    pub fn list_selected(&self) -> Option<usize> {
        match self.list_state() {
            None => None,
            Some(state) => state.selected(),
        }
    }

    /// selects a position in a (list) popup
    pub fn list_select(&mut self, id: Option<usize>) {
        match self.list_state_mut() {
            None => {}
            Some(state) => state.select(id),
        }
    }
}

impl ActionListItem {
    pub fn n_actions(&self) -> usize {
        match self {
            ActionListItem::Track(.., actions) => actions.len(),
            ActionListItem::Artist(.., actions) => actions.len(),
            ActionListItem::Album(.., actions) => actions.len(),
            ActionListItem::Playlist(.., actions) => actions.len(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            ActionListItem::Track(track, ..) => &track.name,
            ActionListItem::Artist(artist, ..) => &artist.name,
            ActionListItem::Album(album, ..) => &album.name,
            ActionListItem::Playlist(playlist, ..) => &playlist.name,
        }
    }

    pub fn actions_desc(&self) -> Vec<String> {
        match self {
            ActionListItem::Track(.., actions) => {
                actions.iter().map(|a| format!("{a:?}")).collect::<Vec<_>>()
            }
            ActionListItem::Artist(.., actions) => {
                actions.iter().map(|a| format!("{a:?}")).collect::<Vec<_>>()
            }
            ActionListItem::Album(.., actions) => {
                actions.iter().map(|a| format!("{a:?}")).collect::<Vec<_>>()
            }
            ActionListItem::Playlist(.., actions) => {
                actions.iter().map(|a| format!("{a:?}")).collect::<Vec<_>>()
            }
        }
    }
}
