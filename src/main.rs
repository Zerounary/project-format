pub mod parser;
pub mod render;

use clap::Parser;
use render::Render;

extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser, value_name = "FILE")]
    input: Option<String>,

    #[clap(short, long, value_parser, value_name = "EXTENSION")]
    extension: Option<String>,

    #[clap(short, long, value_parser, value_name = "FILE")]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();
    let input = args.input.unwrap_or("./templates".to_string());
    let output = args.output.unwrap_or("./output".to_string());
    let extension = args.extension.unwrap_or("rs".to_string());

    let mut render = Render::new();
    render.set_template_extension(&extension);

    render.copy_render(&input, &output, "test".to_string());
}
