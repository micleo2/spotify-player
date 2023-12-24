#!/usr/bin/env python

import sys
import json
from syrics.api import Spotify

sp_dc = "";
sp = Spotify(sp_dc)

results = sp.get_lyrics(sys.argv[1])

print(json.dumps(results))
