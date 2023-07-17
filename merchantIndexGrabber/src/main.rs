// Well, we need to work on plugins after all
use tes3::esp::*;
// Store the data in a hashset
use std::collections::HashSet;
// List directories to write all local plugins
use std::fs;
// read arguments in case a specific plugin is requested for dumping
use std::env;
// Er, also write data to a file
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let plugin_list;

    let mut merchant_data = String::new();

    merchant_data.push_str("{\n");

    let args: Vec<_> = env::args().collect();

    match args.len() {
	      1 => plugin_list = return_plugin_list(vec![]),
	      2 => plugin_list = vec![(Plugin::from_path(&args[1])?, args[1].clone())],
	      _ => todo!()
    };

    for (plugin, _plugin_name) in &plugin_list {

        // let mut instantiated_count = 0;

        let restocking_count = count_restocking_merchants(&plugin);

        if restocking_count == 0 {continue;}

        // println!("{} restock count: {}", plugin_name, count_restocking_merchants(&plugin));

        let merchant_ids = collect_merchant_ids(&plugin);

        for cell in plugin.objects_of_type::<Cell>() {
            // We'll store the entire table for the cell as a single string
            // just to make this harder on ourselves

            for (_, reference) in &cell.references {
                let ref_name = &reference.id.to_ascii_lowercase();

                if !merchant_ids.contains(ref_name) {continue;}

                // println!("Ref: {ref_name}, Plugin: {plugin_name}");

                let mut merchant = get_merchant(&plugin, ref_name);

                // println!("{ref_name}");

                merchant = get_composite_inventory(&plugin_list, merchant);

                merchant_data.push_str(&build_ref_initializer(&reference));

                merchant_data.push_str(&write_merchant_inventory(merchant.clone()));

                // instantiated_count += 1;
            }
        }
        // if instantiated_count > 0 {
        //     println!("Wrote {instantiated_count} restocking NPCs to disk from {plugin_name}");
        // }
    }
    // Lazily remove the extra comma at the very end of the data to make valid JSON.
    merchant_data.truncate(merchant_data.len() - 2);

    merchant_data.push_str("\n}\n");

    fs::write("merchantIndexDatabase.json", merchant_data).expect("Merchant data failed to write");

    Ok(())
}

fn build_ref_initializer(refr: &Reference) -> String {
    // format!("    [\"{0}-0\"] = {{ \n        \"name\": \"{1}\"\n", refr.refr_index.to_string(), refr.id.to_ascii_lowercase())
    format!("\"{0}\":{{\n    \"uniqueIndex\":\"{1}-0\",\n    \"refId\":\"{0}\",\n    \"items\":{{", refr.id.to_ascii_lowercase(), refr.refr_index.to_string())
}

fn collect_merchant_ids(plugin: &Plugin) -> HashSet<String> {
    let mut results = HashSet::new();
    for npc in plugin.objects_of_type::<Npc>() {
	      let npc_name = npc.editor_id().to_ascii_lowercase();

	      if is_merchant(npc) && merchant_restocks(npc) {results.insert(npc_name);}
    }
    results
}

fn get_merchant (plugin: &Plugin, npc_name: &String) -> Npc {

    for npc in plugin.objects_of_type::<Npc>() {

        // println!("comparing {} and {}", &npc.editor_id().to_ascii_lowercase(), npc_name);

        if &npc.editor_id().to_ascii_lowercase() != npc_name { continue; }

        return npc.clone();
    }
    panic!()
}

fn write_merchant_inventory(mut npc: Npc) -> String {
    npc.inventory.retain(|item| item.0 < 0);

    let mut merchant_entry = String::new();

    // println!("{:?}", npc.inventory);

    let inv_len = npc.inventory.len();

    for (item_no, (count, item_id)) in npc.inventory.iter().enumerate() {

        let item_name = &item_id.as_str().to_ascii_lowercase();
        let item_count = &(-count);
        // merchant_entry.push_str(format!("{{\n        \"refId\":\"{item_name}\",\n        \"count\":{item_count}\n    }}{0}",
        //                                 if (item_no + 1) != inv_len {","} else {""} ).as_str());
        merchant_entry.push_str(format!("\n        \"{item_name}\":{item_count}{0}",
                                        if (item_no + 1) != inv_len {","} else {""} ).as_str());

        // println!("{:?}", npc.inventory);

        // println!("{0} {inv_len}, {item_no}", npc.name);
    }
    merchant_entry.push_str("\n    }\n},\n");
    merchant_entry
}

fn merchant_restocks(npc: &Npc) -> bool {
    for (count, _) in &npc.inventory { if *count < 0 { return true; } }
    return false
}

fn is_merchant(npc: &Npc) -> bool {
    // Ignore the last four bits as they don't represent anything that needs to restock
    if (npc.ai_data.services & 16383) == 0 { return false }

    else { return true }
}

fn count_restocking_merchants(plugin: &Plugin) -> u32 {
    let mut instantiated_count = 0;

    for npc in plugin.objects_of_type::<Npc>() {

        if !is_merchant(npc) {continue;}

        for item in npc.inventory.iter() {
            if item.0 < 0 {
                // println!("{0} has restocking item {1} with count {2:?}", npc.editor_id(), item.0, item.1);
                instantiated_count += 1;
                break;
            }
        }
    }
    instantiated_count
}

fn return_plugin_list(mut extensions: Vec<&str>) -> Vec<(Plugin, String)> {
    let mut installed_plugins = vec![];
    let cwd = fs::read_dir(".");

    if extensions.len() == 0 { extensions = vec!["esp", "esm", "omwaddon", "omwgame"]; }

    // Everything is an option!
    for file in cwd.unwrap() {

        // Get the path object for the associated file
        let filename = file.expect("Failed to open file").path();

        // Ignore directories
        if filename.is_dir() || filename.extension().is_none() || !is_tes3(&filename) {continue;}

        // Grab the extension (Unwrap everything! AND THEN DO IT AGAIN!)
        let file_string = filename.extension().unwrap().to_str().unwrap().to_ascii_lowercase();

        // Push installed plugin to the list, HOW DARE YOU THINK YOU'RE DONE UNWRAPPING
        // UNWRAP IT ONE LAST TIME YOU SORRY MAGGOT
        if extensions.contains(&file_string.as_str()) {

            let real_string = filename.to_str().unwrap().to_string();
            let plugin = Plugin::from_path(&real_string);

            match plugin {
                Ok(ref plugin) => plugin,
                Err(ref _plugin) => {
                    fs::write("FailLog.txt", format!("{} could not be loaded by tes3", real_string))
                        .expect("No, I don't think i will handle errors with the error handling inside the error handling, Ferris");
                    continue;
                }
            };

            installed_plugins.push((plugin.unwrap(),
                                    real_string));
        }
    }
    installed_plugins
}

// New function that takes the desired ID and plugin list as argument
// Iterate through every plugin
// Get the desired NPC ID out of the first plugin and create a new NPC object with the summarized inventory
// Make the subfunction write out each object
fn get_composite_inventory(plugin_list: &Vec<(Plugin, String)>, mut merchant: Npc) -> Npc {

    for (plugin, _plugin_name) in plugin_list {

        for npc in plugin.objects_of_type::<Npc>() {
            if npc.editor_id_ascii_lowercase() != merchant.editor_id_ascii_lowercase() { continue; }

            for item in &npc.inventory {
                if !merchant.inventory.contains(item) {
                    let mut mod_item = item.clone();
                    mod_item.1 = FixedString(item.1.to_ascii_lowercase().to_string());
                    // println!("{} had {} added by mod {}", npc.editor_id(), mod_item.1.to_string(), _plugin_name);
                    merchant.inventory.push(mod_item);
                }
            }
        }
    }
    merchant.clone()
}

fn is_tes3(filename: &PathBuf) -> bool {
    let file_to_open = filename;
    let file = match fs::File::open(file_to_open) {
        Ok(file) => file,
        Err(_err) => { return false; }
    };
	  let mut reader = BufReader::new(file);
	  let mut buf = [0; 4];
	  match reader.read_exact(&mut buf) {
	      Ok(n) => n,
	      Err(_err) => { return false; }
	  };

    // println!("{:?} from {:?}", buf, filename);
	  if &buf == b"TES3" { return true };
    false
}
