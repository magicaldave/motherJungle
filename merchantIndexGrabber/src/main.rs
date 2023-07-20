use std::collections::HashMap;
use std::path::PathBuf;

use rayon::prelude::*;
use serde::Serialize;

use tes3::esp::*;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[derive(Default, Serialize)]
struct MerchantData {
    restocks_gold: bool,
    restocks_items: bool,
    restocks_containers: bool,
    gold_pool: u32,
    items: HashMap<String, i32>,
}

fn main() -> std::io::Result<()> {
    let args: Vec<_> = std::env::args().collect();

    let plugins = match args.len() {
        1 => collect_plugins(),
        _ => todo!(),
    };

    // Mapping of { MerchantId => MerchantData }
    let mut merchants = HashMap::<String, MerchantData>::new();

    for (restocking_inventories, _path) in plugins {
        for restocking_inventory in restocking_inventories {
            let merchent_data = merchants.entry(restocking_inventory.id).or_default();

            merchent_data.restocks_items = true;
            merchent_data.restocks_gold = true;
            merchent_data.restocks_containers = true;
            merchent_data.gold_pool = restocking_inventory.gold_pool;

            merchent_data.items.extend(restocking_inventory.items);
        }
    }

    let serialized = serde_json::to_string_pretty(&merchants) //
        .expect("Failed to serialize merchant data to JSON");

    std::fs::write("merchantIndexDatabase.json", serialized) //
        .expect("Merchant data failed to write");

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

            Some(RestockingInventory {
                id,
                gold_pool,
                items,
            })
        })
        .collect()
}

fn collect_plugins() -> Vec<(Vec<RestockingInventory>, PathBuf)> {
    std::fs::read_dir(".")
        .expect("Failed to read directory")
        .par_bridge()
        .filter_map(|entry| {
            let path = entry.ok()?.path();
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

            let restocking_inventories = collect_restocking_inventories(plugin);
            if restocking_inventories.is_empty() {
                return None;
            }

            Some((restocking_inventories, path))
        })
        .collect()
}
