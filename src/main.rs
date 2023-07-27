pub mod cli;

use core::num;
use std::fs::File;
use std::io::{prelude::*, BufReader};
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

pub fn count_lines(file_path: &PathBuf) -> u32 {
    if !file_path.is_file() {
        return 0;
    }

    let mut file = std::fs::File::open(file_path).expect("Could not upen the provided file");
    let file = BufReader::new(file);
    let lines = file.lines();

    let lines_count = lines.count();

    info!("The number of lines of the file is: {}", lines_count);
    lines_count as u32
}

fn count_words(file_path: &PathBuf) -> u32 {
    let file = File::open(file_path).expect("error opening the file");
    let reader = BufReader::new(file);
    let mut word_count: usize = 0;

    reader.lines().for_each(|line| {
        let line = line.unwrap();
        word_count += line.split(' ').filter(|word| word.len() > 0).count();
    });

    return word_count as u32;
}

pub fn count_characters(file_path: &PathBuf) -> u32 {
    if !file_path.is_file() {
        return 0;
    }

    let mut file = std::fs::File::open(file_path).expect("Could not upen the provided file");
    let mut reader = BufReader::new(file);

    let mut content = String::new();
    let _count = reader.read_to_string(&mut content).unwrap();
    content.chars().count() as u32
}

fn main() {
    let args = Args::parse();
    set_tracing();

    if let Some(path) = args.c {
        info!("Counting bytes in file: {:?}", path);
        let res = count_bytes(&path);
        info!("Read {} bytes from {:?}", res, path);
    }

    if let Some(path) = args.l {
        info!("Counting lines in file: {:?}", path);
        let res = count_lines(&path);
        info!("found {} lines in {:?}", res, path);
    }

    if let Some(path) = args.w {
        info!("Counting words in file: {:?}", path);
        let res = count_words(&path);
        info!("Read {} words from {:?}", res, path);
    }

    if let Some(path) = args.m {
        info!("Counting characters in file: {:?}", path);
        let res = count_characters(&path);
        info!("Read {} characters from {:?}", res, path);
    }
}
