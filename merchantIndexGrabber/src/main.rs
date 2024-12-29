use std::collections::HashMap;
use std::path::PathBuf;

use rayon::prelude::*;
use serde::Serialize;

use tes3::esp::*;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum MerchantsOrLists {
    Merchants(MerchantData),
    LeveledLists(LeveledData),
}

#[derive(Debug, Default, Serialize)]
struct MerchantData {
    restock_minutes: u32,
    restocks_gold: bool,
    restocks_items: bool,
    restocks_containers: bool,
    gold_pool: u32,
    items: HashMap<String, i32>,
}

#[derive(Debug, Default, Serialize)]
struct LeveledData {
    is_list: bool,
    list_flags: u32,
    chance_none: u8,
    items: Vec<(String, u16)>,
}

fn main() -> std::io::Result<()> {
    let args: Vec<_> = std::env::args().collect();

    let plugins = match args.len() {
        1 => collect_plugins(),
        _ => todo!(),
    };

    // Mapping of { MerchantId => MerchantData }
    let mut data = HashMap::<String, MerchantsOrLists>::new();

    for (restocking_inventories, leveled_items, _path) in plugins {
        for restocking_inventory in restocking_inventories {
            let merchant_data = crate::MerchantData {
                restock_minutes: 0,
                restocks_items: true,
                restocks_gold: true,
                restocks_containers: true,
                gold_pool: restocking_inventory.gold_pool,
                items: convert_to_hashmap_npc(restocking_inventory.items),
            };
            data.insert(
                restocking_inventory.id,
                MerchantsOrLists::Merchants(merchant_data),
            );
        }

        for leveled_item in leveled_items {
            let entry = crate::LeveledData {
                is_list: true,
                list_flags: leveled_item.leveled_item_flags.bits(),
                chance_none: leveled_item.chance_none,
                items: leveled_item.items,
            };
            data.insert(leveled_item.id, MerchantsOrLists::LeveledLists(entry));
        }
    }

    let serialized_items = serde_json::to_string_pretty(&data) //
        .expect("Failed to serialize merchant data to JSON");

    std::fs::write("merchantIndexDatabase.json", serialized_items) //
        .expect("LEVI data failed to write");

    Ok(())
}

struct RestockingInventory {
    id: String,
    gold_pool: u32,
    items: Vec<(String, i32)>,
}

fn collect_restocking_inventories(plugin: Plugin) -> Vec<RestockingInventory> {
    plugin
        .objects
        .into_iter()
        .filter_map(|object| {
            // Extract only the information we care about.
            let (mut id, inventory, gold_pool) = match object {
                TES3Object::Npc(obj) => (obj.id, obj.inventory, obj.data.gold),
                TES3Object::Creature(obj) => (obj.id, obj.inventory, obj.data.gold),
                TES3Object::Container(obj) => (obj.id, obj.inventory, 0),
                _ => {
                    return None;
                }
            };

            // Filter out non-restocking content.
            let items: Vec<_> = inventory
                .into_iter()
                .filter_map(|(count, mut id)| {
                    if count < 0 {
                        id.make_ascii_lowercase();
                        Some((id.into(), count.abs()))
                    } else {
                        None
                    }
                })
                .collect();

            // If there are no restocking items, then this is not a restocking merchant.
            if items.is_empty() {
                return None;
            }

            // Lowercase the ID to make it easier to work with.
            id.make_ascii_lowercase();

            println!("MIG: {}", id);

            Some(RestockingInventory {
                id,
                gold_pool,
                items,
            })
        })
        .collect()
}

fn collect_levelled_items(plugin: Plugin) -> Vec<LeveledItem> {
    let mut items = vec![];

    for levi in plugin.objects_of_type::<LeveledItem>() {
        let mut levi_lower = levi.clone();
        levi_lower.id.make_ascii_lowercase();
        for item in levi_lower.items.iter_mut() {
            item.0.make_ascii_lowercase();
        }
        items.push(levi_lower);
    }
    items
}

fn convert_to_hashmap_npc(items: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut item_map = HashMap::<String, i32>::new();

    for (item, count) in items {
        item_map.insert(item, count);
    }
    item_map
}

fn collect_plugins() -> Vec<(Vec<RestockingInventory>, Vec<LeveledItem>, PathBuf)> {
    let config = openmw_cfg::get_config().expect("Failed to read openmw.cfg file!");

    openmw_cfg::get_plugins(&config)
        .expect("Failed to read directory")
        .into_iter()
        .par_bridge()
        .filter_map(|path| {
            let extension = path.extension()?.to_ascii_lowercase();
            let ("esp" | "esm" | "omwaddon" | "omwgame") = extension.to_str()? else {
                return None;
            };

            let mut plugin = Plugin::new();
            if plugin
                .load_path_filtered(&path, |tag| {
                    matches!(&tag, Npc::TAG | Creature::TAG | Container::TAG)
                })
                .is_err()
            {
                println!("Failed to load {}", path.display());
                return None;
            }

            let mut levis = Plugin::new();
            if levis
                .load_path_filtered(&path, |tag| matches!(&tag, LeveledItem::TAG))
                .is_err()
            {
                println!("Failed to load {}", path.display());
                return None;
            }

            let leveled_items = collect_levelled_items(levis);

            let restocking_inventories = collect_restocking_inventories(plugin);
            if restocking_inventories.is_empty() {
                return None;
            }

            Some((restocking_inventories, leveled_items, path))
        })
        .collect()
}
