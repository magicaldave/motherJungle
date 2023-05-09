use tes3::esp::*;

fn main() -> std::io::Result<()> {
    let plugin_names = vec![
	"StarwindRemasteredV1.15.esm",
	"StarwindRemasteredPatch.esm"
    ];

    let outdoor_areas = vec![
	"Tatooine",
	"Tatooine, Sandriver",
	"Tatooine, Sand Hole",
	"Tatooine, Dune Sea",
	"Tatooine, Deep Sea",
	"Tatooine, Arena",
	"Tatooine, Rodian District",
	"Tatooine, The Hidden Stash",
	"Tatooine, Swoop Racetrack",
	"Tatooine, Death Canyon",
	"Tatooine, Expanse"
    ];

    // Dump the cells from a list
    // Replace the regions too
    for plugin_name in plugin_names {
	let mut plugin = Plugin::from_path(plugin_name)?;

	for cell in plugin.objects_of_type_mut::<Cell>() {
	    if outdoor_areas.contains(&&*cell.name) {
		cell.data.flags.set(CellFlags::BEHAVES_LIKE_EXTERIOR, true);
		println!("Setting exterior flag on: {0}", cell.name);
            }
	}
	let _ = plugin.save_path(plugin_names[index]);
    }
    Ok(())
}
