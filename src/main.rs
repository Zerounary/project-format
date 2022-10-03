pub mod log;
pub mod config;
pub mod parser;
pub mod render;

use clap::Parser;
use log::log_text_ok;
use render::Render;
// use itertools::Itertools;
use crate::{config::{read_yaml_file, Template}, log::log_path_ok};

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
    let args = Args::parse();
    let config = args.config.unwrap_or("./project.yml".to_string());

    let yaml = read_yaml_file(&config);
    log_path_ok("读取配置文件", &config);

    let render = Render::new();

    let mut input = "templates".to_string();
    let mut output = "output".to_string();

    if let Some(Template {
        dir,
        output: temp_out,
    }) = yaml.project.template
    {
        input = dir;
        output = temp_out;
    }

    render.copy_render(&input, &output, &yaml.project.data);

    log_text_ok("所有文件生成");
}
