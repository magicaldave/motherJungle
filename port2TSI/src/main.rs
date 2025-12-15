use tes3::esp::*;

use std::env;

fn main() -> std::io::Result<()> {
    let plugin_name;

    let args: Vec<_> = env::args().collect();

    match args.len() {
        1 => plugin_name = &args[1],
        _ => todo!(),
    };

    let mut plugin_to_patch = Plugin::from_path(plugin_name)?;

    let mut deleted = String::new();

    plugin_to_patch
        .objects_of_type_mut::<Cell>()
        .for_each(|cell| {
            cell.references.retain(|(mast_idx, _), reference| {
                deleted = format!("{deleted}\n{}-{}", reference.id, reference.refr_index);
                *mast_idx == 0
            })
        });

    println!("{deleted}");

    let _ = plugin_to_patch.save_path(format!("ported_{}", plugin_name));

    Ok(())
}
