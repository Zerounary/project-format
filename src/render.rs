use convert_case::{Case, Casing};
use handlebars::{handlebars_helper, Handlebars};
use itertools::Itertools;
use jsonpath_rust::JsonPathQuery;
use regex::Regex;
use serde_json::{json, Value};
use std::fmt::format;
use std::fs::{self, create_dir_all, DirEntry};
use std::io;
use std::path::{Path, PathBuf};

use crate::log::{log_fail, log_path_ok};
use crate::parser::gen_file_name::{parse_file_name, to_case};

// case convert
handlebars_helper!(upper: |s: String| s.to_case(Case::Upper));
handlebars_helper!(lower: |s: String| s.to_case(Case::Lower));
handlebars_helper!(snake: |s: String| s.to_case(Case::Snake));
handlebars_helper!(upperSnake: |s: String| s.to_case(Case::UpperSnake));
handlebars_helper!(camel: |s: String| s.to_case(Case::Camel));
handlebars_helper!(upperCamel: |s: String| s.to_case(Case::UpperCamel));
handlebars_helper!(kebab: |s: String| s.to_case(Case::Kebab));

// stringify
handlebars_helper!(stringify: |s: Value| s.to_string());

// string convert
handlebars_helper!(fkTable: |s: String| s.trim_end_matches("_id").trim_end_matches("_ids").to_string());

// test
handlebars_helper!(isId: |s: String| s.to_lowercase().eq("id"));
handlebars_helper!(isIds: |s: String| s.to_lowercase().ends_with("_ids"));
handlebars_helper!(isFk: |s: String| s.to_lowercase().ends_with("_id") || s.to_lowercase().ends_with("_ids"));
handlebars_helper!(isDate: |s: String| s.to_lowercase().eq("date"));
handlebars_helper!(isDateTime: |s: String| s.to_lowercase().eq("datetime"));
handlebars_helper!(isDecimal: |s: String| s.to_lowercase().starts_with("decimal"));
handlebars_helper!(defaultDbType: |s: String| {
    if s.to_lowercase().eq("date") {
        "Date(fastdate::Date::from(fastdate::DateTime::now()))".to_string()
    } else if  s.to_lowercase().eq("datetime") {
        "FastDateTime::now()".to_string()
    }
    else if s.to_lowercase().starts_with("decimal"){
        "Decimal(String::from(0.to_string()))".to_string()
    }else {
        "".to_string()
    }
});

pub struct Render<'a> {
    pub h: Handlebars<'a>,
}

impl<'a> Default for Render<'a> {
    fn default() -> Self {
        Self {
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
        h.register_helper("isId", Box::new(isId));
        h.register_helper("isIds", Box::new(isIds));
        h.register_helper("isFk", Box::new(isFk));
        h.register_helper("isDate", Box::new(isDate));
        h.register_helper("isDateTime", Box::new(isDateTime));
        h.register_helper("isDecimal", Box::new(isDecimal));

        h.register_helper("stringify", Box::new(stringify));
        h.register_helper("fkTable", Box::new(fkTable));
        h.register_helper("defaultDbType", Box::new(defaultDbType));

        let render: Render = Self {
            h,
            ..Default::default()
        };
        render
    }

    pub fn copy_render(self, path_from: &str, path_to: &str, data: &Value) {
        let from = Path::new(path_from);
        let to = Path::new(path_to);
        visit_dirs(from, &mut |e| {
            let target = to.join(e.path().strip_prefix(from.to_str().unwrap()).unwrap());
            if !target.parent().unwrap().exists() {
                create_dir_all(target.parent().unwrap()).expect("创建父目录失败");
            }
            let result = fs::read_to_string(e.path());
            match result {
                Ok(template_string) => {
                    let file_name = target.file_name().map(|x| x.to_str().unwrap()).unwrap();
                    if file_name.contains("{") {
                        let (list_key, item_key) = get_var_name(file_name);
                        let list = data
                            .clone()
                            .path(&format!("$.{:}[*]", list_key))
                            .expect(&format!("jsonpath: '$.{:}[*]' 不合法", list_key));
                        if let Some(list) = list.as_array() {
                            if list.is_empty() {
                                panic!("{:}中数据列表{:}不存在", file_name, list_key)
                            }
                            for item in list {
                                let item_name = item
                                    .clone()
                                    .path(&format!("$.{:}", item_key))
                                    .map(|v| match v.get(0) {
                                        Some(l) => l.to_owned(),
                                        None => json!(""),
                                    })
                                    .expect(&format!("{:}.{:}不存在", list_key, item_key));

                                let item_target = target
                                    .parent()
                                    .map(|p| {
                                        p.join(replace_var(file_name, item_name.as_str().unwrap()))
                                    })
                                    .unwrap();

                                // 支持插槽
                                let slots = get_slots_content(item_target.clone());
                                let mut item = item.clone();
                                for (index, slot) in slots.iter().enumerate() {
                                    if let Some(obj) = item.as_object_mut() {
                                        obj.insert(
                                            format!("slot{}", index),
                                            Value::String(slot.to_string()),
                                        );
                                    }
                                }
                                let contents = self
                                    .h
                                    .render_template(&template_string, &item)
                                    .expect(&format!("渲染对象{:?}", item));
                                fs::write(&item_target, contents).expect("生成表达式文件失败");
                                log_path_ok("生成文件:", item_target.as_os_str().to_str().unwrap());
                            }
                        }
                    } else {
                        // 支持插槽
                        let slots = get_slots_content(target.clone());
                        let mut data = data.clone();
                        for (index, slot) in slots.iter().enumerate() {
                            if let Some(obj) = data.as_object_mut() {
                                obj.insert(
                                    format!("slot{}", index),
                                    Value::String(slot.to_string()),
                                );
                            }
                        }
                        let contents = self
                            .h
                            .render_template(&template_string, &data)
                            .expect(&format!("不能生成文件{}", e.path().to_str().unwrap()));
                        fs::write(&target, contents).expect("生成文件失败");
                        log_path_ok("生成文件", target.as_os_str().to_str().unwrap());
                    }
                }
                Err(_e) => {
                    fs::copy(e.path(), &target).expect("copy file error");
                    log_path_ok("复制文件(UTF8不可读)", target.as_os_str().to_str().unwrap());
                }
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
    x.replace_range(start..end, &replace_value);
    x
}

///
/// 解析源码中的slot
/// ```rust
/// //<slot>
/// hello
/// //</slot>
/// ```
/// 获取槽中的代码
pub fn get_slots_content(target: PathBuf) -> Vec<String> {
    let commet_keyword = if let Some(extension) = target.extension() {
        match extension.to_str().unwrap_or_default() {
            "sql" => "--",
            _ => "//",
        }
    } else {
        "//"
    };
    let pattern = &format!(r"{0}<slot>([\s\S]*?){0}</slot>", commet_keyword.to_string());
    let TAG_REGEX = Regex::new(pattern).unwrap();
    let target_str = fs::read_to_string(target).expect("未能读取到文件");
    let mut result = vec![];
    for captures in TAG_REGEX.captures_iter(&target_str) {
        let tag_content = captures.get(1).unwrap().as_str();
        result.push(tag_content.to_string());
    }
    result
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
