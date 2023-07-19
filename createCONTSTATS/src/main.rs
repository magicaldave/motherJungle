use tes3::esp::*;


fn main() -> std::io::Result<()> {
    let mut plugin = Plugin::from_path("statCONTS.esp")?;

    let mut stack = Vec::new();

    // collect light names from the base game
    for plugin_name in ["Morrowind.esm", "Tribunal.esm", "Bloodmoon.esm"] {

	      // Load the actual plugin now that clerical stuff is done
        let base_plugin = Plugin::from_path(plugin_name)?;

	      for container in base_plugin.objects_of_type::<Container>() {
	          if container.id.contains("flora") && !container.id.contains("unique") {

		            // Make a dupe for the new plugin
		            let mut new_container = container.clone();

                new_container.inventory.clear();

		            new_container.inventory.push((5, FixedString("JWS_Plant Drops".to_string())));

		            plugin.objects.push(tes3::esp::TES3Object::Container(new_container));
	          }
	      }
    }



     for stat in plugin.objects_of_type_mut::<Static>() {
	 // Materials harvested from containers; default to trees, override for rocks
	 let mut mat1_name = "JWS_Branch Drops";
	 let mut mat2_name = "JWS_Tree_Resin Drops";
	 let mut rare_count = 1;
	 // Region, then container names
	 let pre_name;
	 let cont_name;

	 match &stat.id.clone().to_ascii_lowercase() {
	     id if id.contains("wg") => {
		 pre_name = "West Gash ";
	     },
	     id if id.contains("ashland") => {
		 pre_name = "Ashlands ";
	     },
	     id if id.contains("rm") => {
		 pre_name = "Red Mountain ";
	     },
	     id if id.contains("_ai_") => {
		 pre_name = "Ascadian Isles ";
	     },
	     id if id.contains("gl") => {
		 pre_name = "Grasslands ";
	     },
	     id if id.contains("ac") => {
		 pre_name = "Azura's Coast ";
	     },
	     id if id.contains("ma") => {
		 pre_name = "Molag Amur ";
	     },
	     id if id.contains("bc") => {
		 pre_name = "Bitter Coast ";
	     },
	     id if id.contains("bm") => {
		 pre_name = "Solstheim ";
	     },
	     id if id.contains("mh") => {
		 pre_name = "Mournhold ";
	     },
	     id if id.starts_with("in_") => {
		 pre_name = "Underground ";
	     }
	     &_ => {
		 pre_name = "";
	     }
	 }

	 match &stat.id {
	     id if id.contains("parasol") => {
		 cont_name = "Emperor Parasol".to_owned();
	     },
	     id if id.contains("ashtree") || id.contains("ash_log") => {
		 cont_name = "Ash ".to_owned() + if id.contains("tree") {"Tree"} else {"Log"};
		 mat2_name = "JWS_Charcoal Drops";
		 rare_count = 3;
	     },
	     id if id.contains("_grass_") => {
		 cont_name = pre_name.to_owned() + &"Grass";
		 mat1_name = "JWS_Plant Drops"
	     },
	     id if id.contains("tree") => {
		 let tree_type = if id.contains("Dead") {"Dead".to_owned()} else {"".to_owned()} + "Tree" + if id.contains("stump") {" Stump"} else {""};
		 cont_name = pre_name.to_owned() + &tree_type;
	     },
	     id if id.contains("rock") || id.contains("boulder") => {
		 let rock_type =  if id.contains("boulder") {"Boulder".to_string()} else {"Rock".to_string()};
		 cont_name = pre_name.to_owned() + &rock_type;
		 mat1_name = "JWS_Rock Drops";
		 mat2_name = "JWS_Rock_Ore Drops";
		 if id.starts_with("in") {rare_count = 3}
	     },
	     // id if id.contains("ice") => {
	     // 	 cont_name = "Ice Block";
	     // 	 mat_name = "JW_Ice Drops"
             // },
	     // id if id.contains("muck") => {
	     // 	 cont_name = "Swamp Water";
	     // 	 mat_name = "JW_Swamp Drops"
	     // },
	     _ => {
		 continue;
	     },
	 }

	 let mut cont_stat = Container {
	     id: stat.id.clone(),
	     mesh: stat.mesh.clone(),
	     name: cont_name.to_string(),
	     container_flags: 0x000b,
	     ..Default::default()
	 };

	 // Add common drops
	 cont_stat.inventory.push((5, FixedString(mat1_name.to_string())));

	 // Add rare drops
	 if !stat.id.contains("_grass_") {
	     cont_stat.inventory.push((rare_count, FixedString(mat2_name.to_string())));
	 }

	 stat.flags = ObjectFlags::DELETED;

	 stack.push(tes3::esp::TES3Object::Container(cont_stat));
     }


    for cont in stack {
	plugin.objects.push(cont.clone());
	println!("Adding container {:?}", cont);
    }

    plugin.save_path("statCONTS.esp")?;

    Ok(())
}
