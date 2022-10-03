use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fs};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub project: Project,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub template: Option<Template>,
    pub data: Value,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Template {
    pub dir: String,
    pub output: String,
}

pub fn read_yaml_file(yaml_path: &str) -> Config {
    let yaml_content =
        fs::read_to_string(yaml_path).expect(&format!("找不到渲染配置文件{:?}", yaml_path));
    let result: Config = serde_yaml::from_str(&yaml_content).expect("read yaml file error");
    result
}
