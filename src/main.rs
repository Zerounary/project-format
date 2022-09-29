pub mod config;
pub mod parser;
pub mod render;

use clap::Parser;
use render::Render;
// use itertools::Itertools;
use crate::config::{read_yaml_file, Template};

extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser, value_name = "YAML")]
    config: Option<String>,
}

fn main() {
    let mut args = Args::parse();
    let config = args.config.unwrap_or("./project.yml".to_string());

    let yaml = read_yaml_file(&config);
    println!("{:?}", yaml);

    let mut render = Render::new();

    let mut input = "templates".to_string();
    let mut output = "output".to_string();
    let mut extension = "rs".to_string();

    if let Some(Template {
        dir,
        output: temp_out,
        extension: out_extension,
    }) = yaml.project.template
    {
        input = dir;
        output = temp_out;
        extension = out_extension;
    }

    render.set_template_extension(&extension);

    render.copy_render(&input, &output, &yaml.project.data);
}
