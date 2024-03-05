use tes3::esp::*;

use std::env;
use std::fs;

fn main() -> std::io::Result<()> {
    let plugin_name;

    let args: Vec<_> = env::args().collect();

    match args.len() {
        1 => todo!(),
        2 => plugin_name = &args[1],
        _ => todo!(),
    };

    let mut plugin_to_patch = Plugin::from_path(plugin_name)?;

    // // Create the plugin header
    // let mut header = Header {
    //     version: 1.3,
    //     ..Default::default()
    // };

    // let base_plugins = vec!["Morrowind.esm",
    // 	                        "Tribunal.esm",
    // 	                        "Bloodmoon.esm"];

    // for plugin_name in &base_plugins {
    //       // Get the plugin size for an accurate header
    //       let plugin_size = fs::metadata(plugin_name)?.len();

    //       // Push the name and size to the header
    //       header.masters.push((plugin_name.to_string(), plugin_size));
    // }

    // plugin_to_patch.objects.push(tes3::esp::TES3Object::Header(header));

    let mut refcount = 0;

    for cell in plugin_to_patch.objects_of_type_mut::<Cell>() {
        let mut cellrefs: Vec<((u32, u32), Reference)> = vec![];
        // let mut cellrefs = HashSet::new();
        for mut reference in cell.references.clone() {
            if reference.0 .0 == 0 {
                reference.1.refr_index = refcount;
                reference.0 .1 = refcount;
                cellrefs.push(reference);
                refcount += 1;
                println!("{refcount}");
            }
        }
        cell.references.clear();
        cell.references.extend(cellrefs);
    }

    let _ = plugin_to_patch.save_path(format!("ported_{}", plugin_name));

    Ok(())
}
