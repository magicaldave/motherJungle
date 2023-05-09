use tes3::esp::*;
use std::env;


fn main() -> std::io::Result<()> {
    let mut plugin;

    if env::args().nth(1).is_some() {
	let plugin_name = &env::args().nth(1).unwrap().to_string();
	plugin = Plugin::from_path(plugin_name)?;
	println!("Patching plugin {}", plugin_name);
    }
    else {
	plugin = Plugin::from_path("Starwind.esp")?;
    }

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
    plugin.save_path("Starwind.esp")?;

    Ok(())
}
