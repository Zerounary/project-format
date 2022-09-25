pub mod parser;
pub mod render;

use clap::{Parser};
use std::path::PathBuf;

extern crate pest;
#[macro_use]
extern crate pest_derive;


#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser, value_name = "FILE")]
    input: Option<PathBuf>,

    #[clap(short, long, value_parser, value_name = "FILE")]
    output: Option<PathBuf>
}

fn main() {
  let args = Args::parse();
  
}
