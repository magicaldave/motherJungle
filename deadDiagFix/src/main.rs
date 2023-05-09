use tes3::esp::*;
use std::env;
use std::process;

fn main() -> std::io::Result<()> {

    let plugin_name;

    let args: Vec<_> = env::args().collect();
    match args.len() {
	1 => plugin_name = "Starwind.esp",
	2 => plugin_name = &args[1],
	_ => todo!()
    };



    for disallowed_plugin in ["Bloodmoon.esm", "Morrowind.esm", "Tribunal.esm"] {
	if plugin_name.eq(disallowed_plugin) {
	    println!("Please don't use this on the Vanilla ESM files!");
	    process::exit(0x0100);
	}
    }

    println!("Patching plugin {}", plugin_name);

    let mut plugin = Plugin::from_path(plugin_name)?;

    for info in plugin.objects_of_type_mut::<DialogueInfo>() {
	for filter in info.filters.iter_mut() {
	    if matches!(filter.function, FilterFunction::DeadType)
		&& matches!(filter.comparison, FilterComparison::Equal)
		&& filter.value != tes3::esp::FilterValue::Integer(0) {
		    filter.comparison = FilterComparison::GreaterEqual;
		    println!("Patching: {:?}, {:?}, {:?}, {:?}, {:?}", info.id, filter.id, filter.function, filter.comparison, filter.value);
	    }
	}
    }

    // Save the updated plugin.
    plugin.save_path(plugin_name)?;

    Ok(())
}
