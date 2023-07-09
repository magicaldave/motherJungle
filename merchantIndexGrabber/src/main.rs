use tes3::esp::*;

use std::collections::HashSet;
use std::fs::File;
use std::env;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let plugin_name;

    let args: Vec<_> = env::args().collect();
    match args.len() {
	      1 => plugin_name = "Morrowind.esm",
	      2 => plugin_name = &args[1],
	      _ => todo!()
    };

    let plugin = Plugin::from_path(plugin_name)?;

    let merchant_ids = collect_merchant_ids(&plugin);

    let mut index_list = File::create(plugin_name.to_owned() + ".txt")?;

    let mut restock_count = 0;

    for cell in plugin.objects_of_type::<Cell>() {
        for reference in &cell.references {
            let ref_name = &reference.1.id.to_ascii_lowercase();
            if merchant_ids.contains(ref_name)
                && check_merchant_inventory(&plugin, ref_name.to_string()) {

                let obj_idx = reference.1.refr_index.to_string();

                let merchant_data = format!("    [\"{obj_idx}-0\"] = {{ -- {ref_name}\n").to_string();

                index_list.write(merchant_data.as_bytes()).expect("Failed to write");

                write_merchant_inventory(&plugin, &index_list, ref_name.to_string());

                index_list.write("    },\n".as_bytes()).expect("Failed to write");

                restock_count += 1;
            }
        }
    }

    println!("Wrote {restock_count} restocking NPCs to file.");

    Ok(())
}

fn collect_merchant_ids(plugin: &Plugin) -> HashSet<String> {
    let mut results = HashSet::new();
    for npc in plugin.objects_of_type::<Npc>() {
	      let npc_name = npc.editor_id().to_ascii_lowercase();
        let services = npc.ai_data.services;
        // Ignore the last four bits as they don't represent anything that needs to restock
	      if (services & 16383) != 0 {
		        results.insert(npc_name);
	      }
    }
    results
}

fn write_merchant_inventory(plugin: &Plugin, mut data_base: &File, npc_name: String) {

    for npc in plugin.objects_of_type::<Npc>() {
        if npc.editor_id().to_ascii_lowercase() == npc_name {
            for item in npc.inventory.iter() {
                if item.0 < 0 {
                    let item_name = &item.1.as_str();
                    let item_count = &(-item.0);
                    let entry = format!("        {{\n            refId = \"{item_name}\",\n            count = {item_count},\n        }},\n");
                    data_base.write(entry.as_bytes()).expect("Failed to write");
                }
            }
        }
    }
}

fn check_merchant_inventory(plugin: &Plugin, npc_name: String) -> bool {
    let mut has_restocking_items = false;

    for npc in plugin.objects_of_type::<Npc>() {
        if npc.editor_id().to_ascii_lowercase() == npc_name {
            for item in npc.inventory.iter() {
                if item.0 < 0 {
                    has_restocking_items = true;
                    break;
                }
            }
        }
    }
    has_restocking_items
}
