#!/usr/bin/env python

import os
import sys
import json
from syrics.api import Spotify

sp = Spotify(os.environ["SP_DC"])

results = sp.get_lyrics(sys.argv[1])

print(json.dumps(results))
