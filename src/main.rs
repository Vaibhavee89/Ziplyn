extern crate flate2;
mod compression;
mod decompression;
mod utils;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: <compress|decompress> <source> <destination>");
        return;
    }

    let action = &args[1];
    let source = &args[2];
    let destination = &args[3];

    match action.as_str() {
        "compress" => {
            if let Err(e) = compression::gzip::compress(source, destination) {
                eprintln!("Compression failed: {}", e);
            }
        }
        "decompress" => {
            if let Err(e) = decompression::gzip::decompress(source, destination) {
                eprintln!("Decompression failed: {}", e);
            }
        }
        _ => eprintln!("Invalid action. Use 'compress' or 'decompress'."),
    }
}


