use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{self, Read, Write},
};

use openmw_config::OpenMWConfiguration;
use vfstool_lib::VFS;

type PluginReferences = Vec<HashMap<String, Vec<String>>>;

fn main() -> io::Result<()> {
    let use_paths = env::args()
        .collect::<Vec<String>>()
        .contains(&String::from("--use-paths"));

    let config = match OpenMWConfiguration::from_env() {
        Err(err) => panic!("{err}"),
        Ok(config) => config,
    };

    let vfs = VFS::from_directories(config.data_directories(), None);

    let mut references = PluginReferences::new();

    config.content_files_iter().for_each(|content_file| {
        let vfs_entry = match vfs.get_file(content_file.value()) {
            Some(file) => file,
            None => {
                let fail_str = format!("Failed to locate plugin: {} in the provided VFS. Bailing out on requiredDataFiles generation.", content_file.value());

                let result = native_dialog::DialogBuilder::message()
                    .set_text(&fail_str)
                    .set_title("Couldn't locate plugin")
                    .alert()
                    .show();

                if let Err(_) = result {
                    eprintln!("{fail_str}");
                }

                std::process::exit(256);
            }
        };

        if let Some(ext) = vfs_entry.path().extension() {
            if ext.eq_ignore_ascii_case("omwscripts") {
                let fail_str = format!("Incompatible plugin found: {} cannot be used in TES3MP.", content_file.value());

                let result = native_dialog::DialogBuilder::message()
                    .set_text(&fail_str)
                    .set_title("Incompatible modlist detected!")
                    .alert()
                    .show();

                if let Err(_) = result {
                    eprintln!("{fail_str}")
                }

                std::process::exit(255);
            }
        }

        let mut buffer = Vec::new();

        let mut file = match File::open(vfs_entry.path()) {
            Ok(file) => file,
            Err(error) => panic!("{error}")
        };

        buffer.clear();
        file.read_to_end(&mut buffer).expect(&format!("Failed reading bytes for plugin {}! Bailing!", content_file.value()));

        let hash_str = format!("0x{:X}", crc32fast::hash(&buffer));

        let output_file_name = match use_paths {
            true => vfs_entry.path().to_string_lossy().to_string(),
            false => content_file.value().clone(),
        };

        references.push(HashMap::from([(
            output_file_name, vec![hash_str]
        )]));
    });

    let mut required_data_files = File::create("requiredDataFiles.json").unwrap();

    write!(
        required_data_files,
        "{}",
        serde_json::to_string_pretty(&references).expect("Failed to generate JSON String!")
    )?;

    Ok(())
}
