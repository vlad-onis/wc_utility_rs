use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short)]
    pub c: Option<PathBuf>,

    #[arg(short, required(false))]
    pub l: Option<PathBuf>,

    #[arg(short, required(false))]
    pub w: Option<PathBuf>,

    #[arg(short, required(false))]
    pub m: Option<PathBuf>,
}
