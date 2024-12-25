extern crate flate2;

use flate2::read::GzDecoder;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};

pub fn decompress(source: &str, destination: &str) -> io::Result<()> {
    let input_file = File::open(source)?;
    let output_file = File::create(destination)?;
    let mut decoder = GzDecoder::new(BufReader::new(input_file));
    let mut writer = BufWriter::new(output_file);

    io::copy(&mut decoder, &mut writer)?;
    println!("Decompression completed: {} -> {}", source, destination);
    Ok(())
}
