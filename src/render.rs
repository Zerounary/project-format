use convert_case::{Case, Casing};
use handlebars::{handlebars_helper, Handlebars};
use jsonpath_rust::JsonPathQuery;
use serde_json::{json, Value};
use std::ffi::OsStr;
use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;

use similar_asserts::assert_eq;

use crate::parser::gen_file_name::{parse_file_name, to_case};

handlebars_helper!(upper: |s: String| s.to_case(Case::Upper));
handlebars_helper!(lower: |s: String| s.to_case(Case::Lower));
handlebars_helper!(snake: |s: String| s.to_case(Case::Snake));
handlebars_helper!(upperSnake: |s: String| s.to_case(Case::UpperSnake));
handlebars_helper!(camel: |s: String| s.to_case(Case::Camel));
handlebars_helper!(upperCamel: |s: String| s.to_case(Case::UpperCamel));
handlebars_helper!(kebab: |s: String| s.to_case(Case::Kebab));

pub struct Render<'a> {
    pub extension: String,
    pub h: Handlebars<'a>,
}

impl<'a> Default for Render<'a> {
    fn default() -> Self {
        Self {
            extension: "rs".to_string(),
            h: Default::default(),
        }
    }
}

impl<'a> Render<'a> {
    pub fn new() -> Self {
        let mut h = Handlebars::new();
        h.register_helper("upper", Box::new(upper));
        h.register_helper("lower", Box::new(lower));
        h.register_helper("snake", Box::new(snake));
        h.register_helper("upperSnake", Box::new(upperSnake));
        h.register_helper("camel", Box::new(camel));
        h.register_helper("upperCamel", Box::new(upperCamel));
        h.register_helper("kebab", Box::new(kebab));
        let render: Render = Self {
            h,
            ..Default::default()
        };
        render
    }

    pub fn set_template_extension(&mut self, extension: &str) {
        self.extension = extension.to_string()
    }

    pub fn copy_render(self, path_from: &str, path_to: &str, data: &Value) {
        let from = Path::new(path_from);
        let to = Path::new(path_to);
        if !to.exists() {
            fs::create_dir(to);
        }
        visit_dirs(from, &mut |e| {
            println!("{:?}", e.path());
            let mut target = to.join(e.path().strip_prefix(from.to_str().unwrap()).unwrap());
            if e.path().extension() == Some(OsStr::new("hbs")) {
                if !target.parent().unwrap().exists() {
                    fs::create_dir(target.parent().unwrap());
                }
                let result = fs::read_to_string(e.path());
                match result {
                    Ok(template_string) => {
                        let file_name = target.file_name().map(|x| x.to_str().unwrap()).unwrap();
                        if file_name.contains("{") {
                            let (list_key, item_key) = get_var_name(file_name);
                            data.clone()
                                .path(&format!("$.{:}", list_key))
                                .map(|v| match v.get(0) {
                                    Some(l) => l.to_owned(),
                                    None => json!([]),
                                })
                                .expect(&format!("project.data.{:} 未找到", list_key))
                                .as_array()
                                .map(|list| {
                                    for item in list {
                                        println!("{:}", item.to_string());
                                        let item_name = item
                                            .clone()
                                            .path(&format!("$.{:}", item_key))
                                            .map(|v| match v.get(0) {
                                                Some(name) => name.to_owned(),
                                                None => item.clone(),
                                            })
                                            .expect(&format!("{:}.{:}不存在", list_key, item_key))
                                            .as_str()
                                            .map(|v| v.to_string());
                                        let item_name = match item_name {
                                            Some(name) => name.to_string(),
                                            None => match item.as_str() {
                                                Some(name) => name.to_string(),
                                                None => {
                                                    panic!("{:?}.{:?}不存在", list_key, item_key)
                                                }
                                            },
                                        };
                                        let item_target = target
                                            .parent()
                                            .map(|p| p.join(replace_var(file_name, &item_name)))
                                            .unwrap();

                                        let contents = self
                                            .h
                                            .render_template(&template_string, &item)
                                            .expect(&format!("渲染对象{:?}", item));
                                        fs::write(item_target, contents);
                                    }
                                })
                                .expect(&format!("不能遍历对象{:?}", list_key))
                        } else {
                            let result = self.h.render_template(&template_string, data);
                            match result {
                                Ok(contents) => {
                                    // 某些文件是需要根据数据生成多个文件的
                                    let file_name = target.file_name().unwrap().to_str().unwrap();
                                    target.set_extension(self.extension.clone());
                                    fs::write(target, contents);
                                }
                                Err(e) => {
                                    println!("{:?}", e);
                                    panic!("不能写入文件{:?}", target)
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!("{:?}", e);
                        panic!("不能读取文件{:?}", target)
                    }
                }
            } else {
                // from = ./tempalte
                // to = ./prj
                // e = ./tempalte/a.hbs
                // target = ./prj/a.hbs
                fs::copy(e.path(), target).expect("copy file error");
            }
        })
        .expect("visit dirs error");
    }
}

pub fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&DirEntry)) -> io::Result<()> {
    if !dir.exists() {
        panic!("路径 {:?} 不存在", dir)
    }
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

/// '{tables;name}' => ('tables', 'name')
pub fn get_var_name(s: &str) -> (String, String) {
    let expr = parse_file_name(s);
    (expr.list_path, expr.item_path)
}

/// '{upperCamel obj.a.names;name.b}' => {{upperCamel this}} => Book
pub fn replace_var(s: &str, v: &str) -> String {
    let expr = parse_file_name(s);
    let start = s.find('{').map(|x| x).unwrap_or(0);
    let end = s.find('}').map(|x| x + 1).unwrap_or(s.len());
    let mut x = s.to_string();
    let replace_value = match to_case(&expr.case) {
        Some(case) => v.to_case(case),
        None => v.to_string(),
    };
    println!("{:?}", replace_value);
    x.replace_range(start..end, &replace_value);
    x
}

#[test]
fn test_render() {
    let render = Render::new();
    let mut data = json!({
        "test": "test"
    });
    render.copy_render("./templates", "./output", &data);
}

#[test]
fn test_path() {
    let path = Path::new("./templates/a.hbs");
}

#[test]
fn test_replace_var() {
    let file_name = "I{upperCamel obj.a.names;name.b}Bo.hbs";
    assert_eq!("IBookBo.hbs".to_string(), replace_var(file_name, "book"));
}

#[test]
fn test_file_name() {
    // let file_name = "{{a.tables:b.name}}Bo.hbs";
    let file_name = "I{tables;name}Bo.hbs";

    let get_vars = |s: &str| -> (String, String) {
        let start = s.find('{').map(|x| x + 1).unwrap_or(0);
        let end = s.find('}').unwrap_or(s.len());
        let var = s.to_string().drain(start..end).collect::<String>();
        var.split_once(';')
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .unwrap_or_default()
    };

    assert_eq!(
        ("tables".to_string(), "name".to_string()),
        get_vars(file_name)
    );

    let file_name = "I{upperCamel tables;name}Bo.hbs";
    assert_eq!("IBookBo.hbs".to_string(), replace_var(file_name, "book"));

    // a.tables 生成 列表

    // b.name 生

    let file_name_temp = r#"
      {{#each a.tables}}
        {{this.b.name}}Bo.hbs
      {{/each}}
    "#;

    // 获得一个数组，元素是每个文件的名字
}
