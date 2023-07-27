use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short)]
    pub c: Option<PathBuf>,

    #[arg(short, required(false))]
    pub l: Option<PathBuf>,
}
