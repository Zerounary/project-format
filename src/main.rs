pub mod log;
pub mod config;
pub mod parser;
pub mod render;

use clap::Parser;
use itertools::Itertools;
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

    let data = if let Some(whitelist) = yaml.project.includes {
        if whitelist.is_empty() {
            yaml.project.data.clone()
        } else {
            let mut render_data = yaml.project.data.clone();
            if let Some(tables) = render_data.get_mut("tables") {
               let mut list = tables.as_array_mut().unwrap();
               let filter_tables = list.clone().into_iter().filter(|e| {
                    let name = e.get("name").unwrap().as_str().unwrap().to_string();
                    whitelist.contains(&name)
                }).collect_vec();

                render_data.as_object_mut().unwrap().insert("tables".to_string(), serde_json::Value::Array(filter_tables) )  ;
                render_data
            }else {
                yaml.project.data.clone()
            }
        }
    }else {
        yaml.project.data.clone()
    };

    render.copy_render(&input, &output, &data);

    log_text_ok("所有文件生成");
}
