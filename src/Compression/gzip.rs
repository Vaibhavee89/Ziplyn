extern crate flate2;

use flate2::write::GzEncoder;
use ::flate2::Compression;
use std::fs::File;
use std::io::{self, BufReader, Write};
// use crate::utils::ensure_gz_extension;

pub fn compress(source: &str, destination: &str) -> io::Result<()> {
    let destination = crate::utils::ensure_gz_extension(destination);

    // Pass a reference (&destination) to avoid moving the value
    let output_file = File::create(&destination)?;
    let input_file = File::open(source)?;
    let mut input = BufReader::new(input_file);
    let mut encoder = GzEncoder::new(output_file, Compression::default());

    io::copy(&mut input, &mut encoder)?;
    encoder.finish()?;
    println!("Compression completed: {} -> {}", source, destination);
    Ok(())
}

