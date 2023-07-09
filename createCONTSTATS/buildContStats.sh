#!/usr/bin/env bash

# .*boulder.* ???
# Vvardenfell Statics
tes3cmd dump --type STAT --match "flora.*tree.*" --raw-with-header statCONTS.esp Morrowind.esm
tes3cmd dump --type STAT --match "parasol" --raw statCONTS.esp Morrowind.esm
tes3cmd dump --type STAT --match "ash_log" --raw statCONTS.esp Morrowind.esm
tes3cmd dump --type STAT --match "terrain_boulder.*" --raw statCONTS.esp Morrowind.esm
tes3cmd dump --type STAT --match ".*grass.*" --raw statCONTS.esp Morrowind.esm
tes3cmd dump --type STAT --match "in.*rock.*" --no-match "(mud|bone|lava|unique)" --raw statCONTS.esp Morrowind.esm
tes3cmd dump --type LIGH --match "(torch|lantern|candle|streetlight|sconce|chandelier|brazier)"  --no-match "(sotha|unique)" --raw statCONTS.esp Morrowind.esm

# Tribunal Statics
tes3cmd dump --type STAT --match "flora.*tree.*" --raw statCONTS.esp Tribunal.esm
tes3cmd dump --type STAT --match "in.*rock.*" --no-match "(mud|bone|lava|unique|stairs)" --raw statCONTS.esp Tribunal.esm
tes3cmd dump --type STAT --match ".*grass.*" --raw statCONTS.esp Tribunal.esm
tes3cmd dump --type LIGH --match "(torch|lantern|candle|streetlight|sconce|chandelier|brazier)" --no-match "(sotha|unique)" --raw statCONTS.esp Tribunal.esm

# Bloodmoon Statics
tes3cmd dump --type STAT --match "flora.*tree.*" --no-match "branch" --raw statCONTS.esp Bloodmoon.esm
tes3cmd dump --type STAT --match ".*grass.*" --raw statCONTS.esp Bloodmoon.esm
tes3cmd dump --type STAT --match "terrain.*rock.*" --no-match "(mud|bone|lava|unique|hunroor|erlendr|nikulas|ulfgar)" --raw statCONTS.esp Bloodmoon.esm
tes3cmd dump --type LIGH --match "(torch|lantern|candle|streetlight|sconce|chandelier|brazier)" --no-match "(sotha|unique)" --raw statCONTS.esp Bloodmoon.esm
