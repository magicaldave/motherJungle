#!/usr/bin/env bash
# Before running this script, make sure to do the necessary script edits by hand.
# Also, open the plugin and create new regions for each planet.

JUNK_CELL=("falensarano, propylon chamber"
"berandas, propylon chamber"
"moonmoth legion fort, prison towers"
"gnisis, madach tradehouse"
"gnisis, arvs-drelen"
"hairat-vassamsi egg mine, queen's lair"
"cavern of the incarnate"
"pelagiad, south wall"
"tukushapal"
"solstheim, legge"
"solstheim, gyldenhul barrow"
"yakin"
"seyda neen, census and excise office"
"mournhold, plaza brindisi dorom"
"surirulk"
"kogoruhn, vault of aerode"
"kogoruhn, hall of maki"
"koal cave"
"toddtest"
"testcell"
"mournhold, great bazaar"
"mournhold, royal palace: helseth's chambers"
"mournhold, royal palace: basement"
"nerano ancestral tomb"
"kashyyk"
"baram ancestral tomb"
"balmora, hecerinde's house"
"balmora, drarayne thelas' storage"
"ashinabi, smuggler den"
"vivec, palace of vivec")


JUNK_DIAL=("ashlander challenges"
"ashlander customs"
"ashlander challenges"
"almalexia")


REMOVED_DIALOG=("174442464547243162"
"17904171831116210117"
"2317424906205619488"
"25001915518026113"
"285063104121489846"
"298369343260528928"
"31123159503217631501"
"3214713032489529377"
"3270014189293947506"
"4877232371438411428")

DELETED_SCRIPTS=("db_assassinscript"
"dbattackscript"
"dbattackscriptold"
"dbcontractscript"
"dbdartscript"
"dbfixscript"
"dbhideoutscript")


# Delete Vanilla dialogs
for dial in "${JUNK_DIAL[@]}"; do tes3cmd delete --hide-backups --type DIAL --exact-id "$dial" StarwindRemasteredV1.15.esm StarwindRemasteredPatch.esm; done

# Delete junk cells added by the CS bug
for cell in "${JUNK_CELL[@]}"; do tes3cmd delete --hide-backups --exact-id "$cell" StarwindRemasteredV1.15.esm StarwindRemasteredPatch.esm; done

# Delete disabled dialog present in both plugins
# some deleted droid dialog and replaced text for the ship purchase
for info in "${QUESTIONABLE_DIALOG[@]}"; do tes3cmd delete --type INFO --exact-id "$info" StarwindRemasteredV1.15.esm StarwindRemasteredPatch.esm; done

# No cells are exteriors, so delete all those fuckers too
tes3cmd delete --exterior StarwindRemasteredV1.15.esm StarwindRemasteredPatch.esm

#This script is actually SW_traveltokashyyk but I figure if anybody was actually using this script the typo would have been noticed a long time ago
# When I ran a global search against it and tried to dump instances of it out of the plugin it didn't appear to have any references
tes3cmd delete --type SCPT --exact-id sw_ StarwindRemasteredV1.15.esm


# Make custom regions for every planet
# Update the tatooine cells to be exteriors

# Delete female ewok head ref as they're the same (Endor uses the male mesh for both??)
tes3cmd delete --type BODY --exact-id "sw_ewokheadf" StarwindRemasteredPatch.esm StarwindRemasteredV1.15.esm

#Everything below this point needs a corresponding fix in the CS, most of these should be disabled for upstream compatibility

#for script in "${DELETED_SCRIPTS[@]}"; do tes3cmd delete --type SCPT --exact-id "$script" StarwindRemasteredV1.15.esm StarwindRemasteredPatch.esm; done


# I think the pathgrids in Starwind are too complex for this, but our server's going fine without them so I'll keep this part
#tes3cmd delete --type PGRD StarwindRemasteredPatch.esm StarwindRemasteredV1.15.esm

# Modifying the original regions caused problems, make new ones instead
# Also, flag the correct cells as exteriors
#tes3cmd delete --type REGN StarwindRemasteredPatch.esm

