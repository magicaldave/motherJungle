#!/usr/bin/env bash

BASE_PLUGINS=(
# Henry
"ahtar companion mod - starwind.esp"
"eseh'vehu companion mod - starwind.esp"
"heau companion - a starwind mod.esp"
"ignatious the mad companion mod - starwind.esp"
"jiaza companion mod - starwind.esp"
"mac vuart companion - a starwind mod.esp"
"snaesk zyeq companion - a starwind mod.esp"
"defend sandriver mod - starwind.esp"
"the siddah ca way - official starwind expansion pack.esp"
# Billy
"starwind hut home.esp"
"pazaak champion.esp"
# Tubtubs
"starwindimprovedkoltotanks.esp"
)

ENHANCED_PLUGINS=(
# Billy
"starwind better bodies.esp"
"playable lightning.esp"
"starwind manor home.esp"
# Jawohl
"starwind - death troopers v0.9a.esp"
# Henry
"cunnov dell companion - a starwind mod.esp"
# Billy
"champion of taris.esp"
"dark apprentice.esp"
)


# Prepare to MERGE
[ ! -d "mergeDir/" ] && mkdir mergeDir

mv *.esm mergeDir
mv *.esp mergeDir
mv *.omwaddon mergeDir

cd mergeDir
rm *\~* # delete loose backups

for plugin in "${BASE_PLUGINS[@]}"; do ../merge_to_master "$plugin" StarwindRemasteredPatch.esm; done

for plugin in "${ENHANCED_PLUGINS[@]}"; do ../merge_to_master "$plugin" "Starwind Enhanced.esm"; done

../merge_to_master "Starwind Enhanced.esm" StarwindRemasteredPatch.esm

../merge_to_master StarwindRemasteredPatch.esm StarwindRemasteredV1.15.esm

tes3cmd esp StarwindRemasteredV1.15.esm

mv StarwindRemasteredV1.15.esp ../StarwindDE.esp

# Kill the merge folder
cd ..
rm -rf mergeDir
