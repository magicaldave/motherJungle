use std::env;
use crc32fast::Hasher;
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // Get the filename from the command-line arguments
    let args: Vec<String> = env::args().collect();
    let filename = args
        .get(1)
        .expect("Please provide a filename as the first argument");

    // Open the file
    let mut file = File::open(filename)?;

    // Read the contents of the file into a buffer
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Calculate the CRC32 checksum of the file's contents
    let mut hasher = Hasher::new();
    hasher.update(&buffer);
    let checksum = hasher.finalize();

    // Print the checksum
    println!("CRC32 checksum: {:x}", checksum);

    Ok(())
}
