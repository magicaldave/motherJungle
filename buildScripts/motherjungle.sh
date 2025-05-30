#!/usr/bin/env bash
# Master build script for Starwind DE

MASTER_ARCHIVES=(
# Main
"Starwind Version 3.1-48909-3-1-1679098521.zip"
# Billy
"Lightning in Starwind-50484-1-1-1662683147.7z"
"Starwind Better Bodies with Underwear-50484-1-3-1682303565.7z"
"Starwind Enhanced-50484-2-51-1682347115.7z"
"Planet Replacer-50484-1-2-1680209481.7z"
"Starwind Hut Home-50484-1-1-1662662593.7z"
"Starwind Manor Home-50484-1-1679685795.7z"
"Champion of Taris-52047-1-2-1674843827.7z"
"Starwind Pazaak Champion-51812-1-1-1671367888.7z"
"Dark Apprentice-51881-1-2-1674433843.7z"
# Jawohl
"SWC Starwind music replacer-52370-1-0-1676635695.rar"
"Starwind - Death Troopers V0.9-52709-0-9-1682080991.rar"
"TriOpArmor.zip"
# Henry
"Ahtar Companion - A Starwind Mod V1.1 - Compatibility Update-51036-1-1-1663474731.zip"
"HEAU Companion - A Starwind Mod V.1.0-52703-1-0-1681948667.zip"
"Ignatious the Mad Companion - A Starwind Mod V.1.0 - The Release-50960-1-0-1649379538.zip"
"Jiaza Companion - A Starwind Mod V.1.0 - The Release-51869-1-0-1664814884.zip"
"Jiub Onasi Companion - A Starwind Mod V.1.3-50554-1-3-1682109165.zip"
"Mac Vuart Companion - A Starwind Mod V.1.0-52659-1-0-1680892886.zip"
"Snaesk Zyeq Companion - A Starwind Mod V.1.0-52627-1-0-1680808409(1).zip"
"Cunnov Dell Companion - A Starwind Mod V.1.0-52671-1-0-1681251145.zip"
"Eseh'vehu Companion - A Starwind Mod V1.1 - Compatibility Update-51100-1-1-1663474762.zip"
"The Siddah Ca Way - Official Starwind Expansion Pack V.1.0-52530-1-0-1679455920.zip"
"Defend Sandriver - A Starwind Mod-52366-1-0-1676576156.zip"
# Tom
"Starwind Sabers Plus-52179-1-982-1678327675.zip"
# TubTubs
"Starwind - Improved Kolto Tanks-50946-1-0-1649004863.zip")

DATA_FOLDERS=("Icons"
"Meshes"
"Music"
"Sound"
"Splash"
"Textures"
"Video"
)

# Jawohl's folders are in lowercase already, but that's a bad time
JDAWG_DIRS=("textures"
"icons"
"meshes"
)

MUSIC_DIRS=("battle"
"explore"
)

SUB_MODS=("Starwind Sabers Plus/"
"StarwindImprovedKoltoTanks/"
"SWC music mod/Data Files/"
)

OLD_FOLDERS=(
    "Data Files"
    "mergeDir"
    "StarwindGFX"
    "StarwindSFX"
)

OLD_FILES=(
    "StarwindGFX.bsa"
    "StarwindDE.esp"
)

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

# Stop on errors
set -e

# Destroy the old data if already present
for oldDir in "${OLD_FOLDERS[@]}"; do [ -d "$oldDir" ] && rm -rf $oldDir; done
for oldFile in "${OLD_FILES[@]}"; do [ -f "$oldFile" ] && rm -rf $oldFile; done

#Extract and remove everything first
for archive in "${MASTER_ARCHIVES[@]}"; do 7z x -y ../plugins/"$archive"; done

# Lowername everything for linux Compatibility before attempting any overwrites
./lowername.py

# Clean up the mess we've made!
mv "Starwind3.1/Data Files/" "Data Files/"
rm -rf Starwind3.1/

for sub_mod in "${SUB_MODS[@]}"; do rsync -av  "$sub_mod" . ; rm -r "$sub_mod"; done

# We ain't here for readmes, damnit
rm -r "SWC music mod"
rm *.txt

for dir in "${MUSIC_DIRS[@]}"; do mv Music/$dir Music/$(python -c "print(\"$dir\".capitalize())"); done

for dir in "${JDAWG_DIRS[@]}"; do rsync -av $dir/* ${dir^} ; rm -rf $dir; done

# Do the overwrites
for folder in "${DATA_FOLDERS[@]}"; do rsync -av "$folder" "Data Files"; rm -rf "$folder"; done

# Apply the manual patches
mv "Data Files"/*.esm .
./manual_starwind_clean.sh
./make_exterior_cells

# Remove tes3cmd backups
rm -rf backups


./mergeplugins.sh

wine ./BSArch.exe pack "Z:\home\s3kshun-8\GitHub\motherJungle\buildScripts\Data Files" "Z:\home\s3kshun-8\GitHub\motherJungle\buildScripts\Data Files\Starwind.bsa" -sse -z

mv "Data Files"/Starwind.bsa .
rm -rf "Data Files"/*

mv Starwind.bsa "Data Files"
mv StarwindDE.esp "Data Files"
