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

    let mut merchant_indices: HashSet<String> = HashSet::new();

    let mut index_list = File::create(plugin_name.to_owned() + ".txt")?;

    for cell in plugin.objects_of_type::<Cell>() {
        for reference in &cell.references {
            let ref_name = &reference.1.id.to_ascii_lowercase();
            if merchant_ids.contains(ref_name) {
                let obj_idx = reference.1.refr_index;
                let merchant_data = format!("    [\"{obj_idx}-0\"] = {{ -- {ref_name}\n").to_string();
                index_list.write(merchant_data.as_bytes()).expect("Failed to write");
                write_merchant_inventory(&plugin, &index_list, ref_name.to_string());
                index_list.write("    },\n".as_bytes()).expect("Failed to write");
                // merchant_indices.insert(merchant_data.to_string());
            }
        }
    }


    Ok(())
}

fn collect_merchant_ids(plugin: &Plugin) -> HashSet<String> {
    let mut results = HashSet::new();
    for npc in plugin.objects_of_type::<Npc>() {
	      let npc_name = npc.editor_id().to_ascii_lowercase();
        let services = npc.ai_data.services;
        // Ignore the last four bits as they don't represent anything that needs to restock
	      if (npc.ai_data.services & 16383) != 0 {
		        results.insert(npc.editor_id().to_ascii_lowercase());
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
