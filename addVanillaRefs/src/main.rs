use tes3::esp::*;

use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let mut plugin = Plugin::from_path("Starwind.esp")?;

    let mut defined_ids = collect_defined_ids(&plugin);
    let required_ids = collect_required_ids(&plugin);

    // Copy over any objects that are in `required_ids` but not in `defined_ids`.
    for plugin_name in ["Bloodmoon.esm", "Tribunal.esm", "Morrowind.esm"] {
        let base_plugin = Plugin::from_path(plugin_name)?;

        for object in &base_plugin.objects {
	    if never_copy(object) { continue; }


            let id = object.editor_id().to_ascii_lowercase();
            if required_ids.contains(&id)
                && defined_ids.insert(id)
            {
                println!("Copying '{}' ({}) to 'Starwind.esp' from '{}'", object.editor_id(), object.tag_str(), plugin_name);
                plugin.objects.push(object.clone());
            }
        }

        // Also copy over any GMSTs that are missing.
        for gmst in base_plugin.objects_of_type::<GameSetting>() {
            let id = gmst.id.to_ascii_lowercase();
            if !defined_ids.contains(&id) {
                defined_ids.insert(id);
                plugin.objects.push(gmst.clone().into());
            }
        }
        // Also copy over any MGEFs that are missing.
	let defined_effects: HashSet<_> = plugin.objects_of_type::<MagicEffect>()
	    .map(|effect| effect.effect_id)
	    .collect();
	for effect in base_plugin.objects_of_type::<MagicEffect>() {
	    if !defined_effects.contains(&effect.effect_id) {
		plugin.objects.push(effect.clone().into());
	    }
	}
        // Also copy over any RACEs that are missing.
        for race in base_plugin.objects_of_type::<Race>() {
            let id = race.id.to_ascii_lowercase();
            if !defined_ids.contains(&id) {
                defined_ids.insert(id);
                plugin.objects.push(race.clone().into());
            }
        }
        // Also copy over any CLASses that are missing.
        for class in base_plugin.objects_of_type::<Class>() {
            let id = class.id.to_ascii_lowercase();
            if !defined_ids.contains(&id) {
                defined_ids.insert(id);
                plugin.objects.push(class.clone().into());
            }
        }
    }
    for info in plugin.objects_of_type_mut::<DialogueInfo>() {
	let cell = info.speaker_cell.to_ascii_lowercase();
	if !defined_ids.contains(&cell) {
	    info.speaker_cell = "igtestcell".to_string();
	}
	info.filters.retain(|filter| {
            if filter.kind == FilterType::NotCell {
		let cell = filter.id.to_ascii_lowercase();
		defined_ids.contains(&cell)
            } else {
		true
            }
	});
    }

    for object in &mut plugin.objects {
	let ai_packages = match object {
            TES3Object::Creature(creature) => &mut creature.ai_packages,
            TES3Object::Npc(npc) => &mut npc.ai_packages,
            _ => continue,
	};
	for ai_package in ai_packages {
            let cell = match ai_package {
		AiPackage::Escort(pkg) => &mut pkg.cell,
		AiPackage::Follow(pkg) => &mut pkg.cell,
		_ => continue,
            };
            if !defined_ids.contains(&cell.to_ascii_lowercase()) {
		*cell = "igtestcell".to_string();
            }
	}
    }


    // Remove masters from the plugin header.
    let header = plugin.header_mut().unwrap();
    header.masters.clear();
    header.version = 1.3;

    // Save the updated plugin.
    plugin.save_path("Starwind.esp")?;

    Ok(())
}

fn collect_defined_ids(plugin: &Plugin) -> HashSet<String> {
    let mut results = HashSet::new();
    for object in &plugin.objects {
        if !never_copy(object) {
            results.insert(object.editor_id().to_ascii_lowercase());
        }
    }
    results
}

fn collect_required_ids(plugin: &Plugin) -> HashSet<String> {
    let mut results = HashSet::new();
    for object in &plugin.objects {
        // Save the ids of any objects required by the current object.
        match object {
            TES3Object::Race(race) => {
                for spell in &race.spells {
                    results.insert(spell.to_ascii_lowercase());
                }
            },
            TES3Object::SoundGen(soundgen) => {
                results.insert(soundgen.creature.to_ascii_lowercase());
                results.insert(soundgen.sound.to_ascii_lowercase());
            },
            TES3Object::MagicEffect(magic_effect) => {
                results.insert(magic_effect.bolt_sound.to_ascii_lowercase());
                results.insert(magic_effect.cast_sound.to_ascii_lowercase());
                results.insert(magic_effect.hit_sound.to_ascii_lowercase());
                results.insert(magic_effect.area_sound.to_ascii_lowercase());
                results.insert(magic_effect.cast_visual.to_ascii_lowercase());
                results.insert(magic_effect.bolt_visual.to_ascii_lowercase());
                results.insert(magic_effect.hit_visual.to_ascii_lowercase());
                results.insert(magic_effect.area_visual.to_ascii_lowercase());
            },
            TES3Object::Region(region) => {
		results.insert(region.id.clone().to_ascii_lowercase());
                results.insert(region.sleep_creature.to_ascii_lowercase());
                for (sound, _) in &region.sounds {
                    results.insert(sound.to_ascii_lowercase());
                }
            },
            TES3Object::Birthsign(birthsign) => {
                for spell in &birthsign.spells {
                    results.insert(spell.to_ascii_lowercase());
                }
            },
            TES3Object::Door(door) => {
                results.insert(door.script.to_ascii_lowercase());
                results.insert(door.open_sound.to_ascii_lowercase());
                results.insert(door.close_sound.to_ascii_lowercase());
            },
            TES3Object::MiscItem(misc_item) => {
                results.insert(misc_item.script.to_ascii_lowercase());
            },
            TES3Object::Weapon(weapon) => {
                results.insert(weapon.script.to_ascii_lowercase());
                results.insert(weapon.enchanting.to_ascii_lowercase());
            },
            TES3Object::Container(container) => {
                results.insert(container.script.to_ascii_lowercase());
		for item in &container.inventory {
		    results.insert(item.1.to_ascii_lowercase());
		}
            },
            TES3Object::Creature(creature) => {
                results.insert(creature.script.to_ascii_lowercase());
                for (_, item) in &creature.inventory {
                    results.insert(item.to_ascii_lowercase());
                }
                for spell in &creature.spells {
                    results.insert(spell.to_ascii_lowercase());
                }
                for package in &creature.ai_packages {
                    if let AiPackage::Activate(activate) = package {
                        results.insert(activate.target.to_ascii_lowercase());
                    }
                }
                results.insert(creature.sound.to_ascii_lowercase());
            },
            TES3Object::Bodypart(bodypart) => {
                results.insert(bodypart.name.to_ascii_lowercase()); // should be named `.race`
            },
            TES3Object::Light(light) => {
                results.insert(light.script.to_ascii_lowercase());
		results.insert(light.sound.to_ascii_lowercase());
            },
            TES3Object::Npc(npc) => {
                results.insert(npc.script.to_ascii_lowercase());
                for (_, item) in &npc.inventory {
                    results.insert(item.to_ascii_lowercase());
                }
                for spell in &npc.spells {
                    results.insert(spell.to_ascii_lowercase());
                }
                for package in &npc.ai_packages {
                    if let AiPackage::Activate(activate) = package {
                        results.insert(activate.target.to_ascii_lowercase());
			println!("{} added as an activation target", activate.target.to_ascii_lowercase());
                    }
                }
                results.insert(npc.race.to_ascii_lowercase());
                results.insert(npc.class.to_ascii_lowercase());
                results.insert(npc.faction.to_ascii_lowercase());
                results.insert(npc.head.to_ascii_lowercase());
                results.insert(npc.hair.to_ascii_lowercase());
            },
            TES3Object::Armor(armor) => {
                results.insert(armor.script.to_ascii_lowercase());
                results.insert(armor.enchanting.to_ascii_lowercase());
                for biped_object in &armor.biped_objects {
                    results.insert(biped_object.male_bodypart.to_ascii_lowercase());
                    results.insert(biped_object.female_bodypart.to_ascii_lowercase());
                }
            },
            TES3Object::Clothing(clothing) => {
                results.insert(clothing.script.to_ascii_lowercase());
                results.insert(clothing.enchanting.to_ascii_lowercase());
                for biped_object in &clothing.biped_objects {
                    results.insert(biped_object.male_bodypart.to_ascii_lowercase());
                    results.insert(biped_object.female_bodypart.to_ascii_lowercase());
                }
            },
            TES3Object::RepairItem(repair_item) => {
                results.insert(repair_item.script.to_ascii_lowercase());
            },
            TES3Object::Activator(activator) => {
                results.insert(activator.script.to_ascii_lowercase());
            },
            TES3Object::Apparatus(apparatus) => {
                results.insert(apparatus.script.to_ascii_lowercase());
            },
            TES3Object::Lockpick(lockpick) => {
                results.insert(lockpick.script.to_ascii_lowercase());
            },
            TES3Object::Probe(probe) => {
                results.insert(probe.script.to_ascii_lowercase());
            },
            TES3Object::Ingredient(ingredient) => {
                results.insert(ingredient.script.to_ascii_lowercase());
            },
            TES3Object::Book(book) => {
                results.insert(book.script.to_ascii_lowercase());
                results.insert(book.enchanting.to_ascii_lowercase());
            },
            TES3Object::Alchemy(alchemy) => {
                results.insert(alchemy.script.to_ascii_lowercase());
            },
            TES3Object::LeveledItem(leveled_item) => {
                for (item, _) in &leveled_item.items {
                    results.insert(item.to_ascii_lowercase());
                }
            },
            TES3Object::LeveledCreature(leveled_creature) => {
                for (creature, _) in &leveled_creature.creatures {
                    results.insert(creature.to_ascii_lowercase());
                }
            },
            TES3Object::Cell(cell) => {
                if let Some(region) = &cell.region {
                    results.insert(region.to_ascii_lowercase());
                }
                for reference in cell.references.values() {
                    results.insert(reference.id.to_ascii_lowercase());
		    println!("{} added to Starwind.esp as a cell reference in {}", reference.id, cell.name);
                    if let Some(owner) = &reference.owner {
                        results.insert(owner.to_ascii_lowercase());
			println!("{} added to Starwind.esp as an owner reference", owner.to_ascii_lowercase());
                    }
                    if let Some(owner_global) = &reference.owner_global {
                        results.insert(owner_global.to_ascii_lowercase());
			println!("{} added to Starwind.esp as an owner reference", owner_global.to_ascii_lowercase());
                    }
                    if let Some(owner_faction) = &reference.owner_faction {
                        results.insert(owner_faction.to_ascii_lowercase());
			println!("{} added to Starwind.esp as an owner reference", owner_faction.to_ascii_lowercase());
                    }
                    if let Some(key) = &reference.key {
                        results.insert(key.to_ascii_lowercase());
                    }
                    if let Some(trap) = &reference.trap {
                        results.insert(trap.to_ascii_lowercase());
                    }
                    if let Some(soul) = &reference.soul {
                        results.insert(soul.to_ascii_lowercase());
                    }
                }
            },
            TES3Object::DialogueInfo(dialogue_info) => {
                results.insert(dialogue_info.speaker_id.to_ascii_lowercase());
		println!("{} imported as a line spoken by {}", dialogue_info.id, dialogue_info.speaker_id.to_ascii_lowercase());
                results.insert(dialogue_info.speaker_race.to_ascii_lowercase());
                results.insert(dialogue_info.speaker_class.to_ascii_lowercase());
                results.insert(dialogue_info.speaker_faction.to_ascii_lowercase());
                results.insert(dialogue_info.player_faction.to_ascii_lowercase());
                results.insert(dialogue_info.sound_path.to_ascii_lowercase());
                for filter in &dialogue_info.filters {
                    results.insert(filter.id.to_ascii_lowercase());
                }
            },
            // TES3Object::Header(_) => {},
            // TES3Object::GameSetting(_) => {},
            // TES3Object::GlobalVariable(_) => {},
            // TES3Object::Class(_) => {},
            TES3Object::Faction(faction) => {
		results.insert(faction.id.to_ascii_lowercase());
		for reaction in &faction.reactions {
		    results.insert(reaction.faction.clone().to_ascii_lowercase());
		}
	    },
            TES3Object::Sound(sound) => {
		results.insert(sound.id.to_ascii_lowercase());
	    },
            // TES3Object::Skill(_) => {},
            // TES3Object::Script(_) => {},
            // TES3Object::StartScript(_) => {},
            // TES3Object::LandscapeTexture(_) => {},
            // TES3Object::Spell(_) => {},
            // TES3Object::Static(_) => {},
            // TES3Object::Enchanting(_) => {},
            // TES3Object::Landscape(_) => {},
            // TES3Object::PathGrid(_) => {},
            // TES3Object::Dialogue(_) => {},
            _ => {}
        }
    }
    results
}

fn never_copy(object: &TES3Object) -> bool {
    matches!(object,
        TES3Object::Header(_)
        | TES3Object::Skill(_)
        | TES3Object::StartScript(_)
        | TES3Object::LandscapeTexture(_)
        | TES3Object::Landscape(_)
        | TES3Object::PathGrid(_)
        | TES3Object::Dialogue(_)
        | TES3Object::DialogueInfo(_)
        | TES3Object::Cell(_)
    )
}
