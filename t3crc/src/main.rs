use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    // Get the filename from the command-line arguments
    let args: Vec<String> = env::args().collect();
    let use_paths = args.contains(&String::from("--use-paths"));
    let config = openmw_cfg::get_config().expect("Failed to get config");
    let plugins = openmw_cfg::get_plugins(&config).expect("Failed to get plugins");

    let mut json = String::from("[\n");
    let mut buffer = Vec::new();
    let mut required_data_files = File::create("requiredDataFiles.json").unwrap();
    // We are going to be charitable and assume these files will never actually be present for us to check
    let mut base_plugins = HashMap::new();
    base_plugins.insert("Morrowind.esm", "0x7B6AF5B9");
    base_plugins.insert("Tribunal.esm", "0xF481F334");
    base_plugins.insert("Bloodmoon.esm", "0x43DD2132");

    for (index, filename) in plugins.iter().enumerate() {
        // Open the file
        let mut filename = filename.to_str().unwrap();

        // if let Some((_, checksum)) = base_plugins.get_key_value(get_filename_from_path(filename)) {
        //     if !use_paths {
        //         filename = get_filename_from_path(filename);
        //     }
        //     json.push_str(&format!("{{\"{}\": [\"{}\"]}}", filename, checksum));
        //     if index != plugins.len() - 1 {
        //         json.push_str(",\n");
        //     }
        // } else {
        let mut file = File::open(filename)?;

        // Read the contents of the file into a buffer
        buffer.clear();
        file.read_to_end(&mut buffer)?;

        if !use_paths {
            filename = get_filename_from_path(filename);
        }

        json.push_str(&json_string(filename, crc32fast::hash(&buffer)));

        if index != plugins.len() - 1 {
            json.push_str(",\n");
        }
        // }
    }

    json.push_str("\n]");
    write!(required_data_files, "{}", json)?;

    Ok(())
}

fn json_string(filename: &str, checksum: u32) -> String {
    format!("{{\"{}\": [\"0x{:X}\"]}}", filename, checksum)
}

fn get_filename_from_path(path: &str) -> &str {
    if let Some(index) = path.rfind('/') {
        return &path[(index + 1)..];
    }
    path
}
