pub mod cli;

use core::num;
use std::io::prelude::*;
use std::{io::BufRead, path::PathBuf};

use clap::Parser;
use cli::Args;

use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

pub fn set_tracing() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

pub fn count_bytes(file_path: &PathBuf) -> u32 {
    if !file_path.is_file() {
        return 0;
    }

    // prepare the buffer for buffered reading
    const BUFF_SIZE: usize = 512;
    let mut buffer = [0u8; BUFF_SIZE];

    let mut file = std::fs::File::open(file_path).expect("Could not upen the provided file");
    let mut number_of_total_read_bytes = 0;
    loop {
        let read_count = file.read(&mut buffer).unwrap();

        if read_count == 0 {
            break;
        }

        number_of_total_read_bytes += read_count;
    }

    number_of_total_read_bytes as u32
}

fn main() {
    let args = Args::parse();
    set_tracing();

    if args.c.is_file() {
        info!("Counting bytes in file: {:?}", args.c);
        let res = count_bytes(&args.c);
        info!("Read {} bytes from {:?}", res, args.c);
    }
}
