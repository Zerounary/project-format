use convert_case::{Case, Casing};
use handlebars::{handlebars_helper, Handlebars};
use serde_json::Value;
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;

// use similar_asserts::assert_eq;

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

    pub fn copy_render(self, path_from: &str, path_to: &str, data: &BTreeMap<String, Value>) {
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
                        let result = self.h.render_template(&template_string, data);
                        match result {
                            Ok(contents) => {
                                // 某些文件是需要根据数据生成多个文件的
                                let file_name = target.file_name().unwrap().to_str().unwrap();
                                if file_name.contains("{{") {
                                    // todo 拿到列表 循环渲染列表 到模板 data.tables
                                    // data.get("tables").unwrap().as_sequence().
                                    // let v = Value::from(data);

                                    // println!("{:?}", target_file_name);
                                } else {
                                    target.set_extension(self.extension.clone());
                                    fs::write(target, contents);
                                }
                            }
                            Err(e) => {
                                println!("{:?}", e);
                                panic!("不能写入文件{:?}", target)
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

#[test]
fn test_render() {
    let render = Render::new();
    let mut data = BTreeMap::new();
    data.insert("test".to_string(), Value::from("test"));
    render.copy_render("./templates", "./output", &data);
}

#[test]
fn test_path() {
    let path = Path::new("./templates/a.hbs");
}

#[test]
fn test_file_name() {
    let file_name = "{{a.tables 'b.name'}}Bo.hbs";

    // a.tables 生成 列表

    // b.name 生

    let file_name_temp = r#"
      {{#each a.tables}}
        {{this.b.name}}Bo.hbs
      {{/each}}
    "#;

    // 获得一个数组，元素是每个文件的名字
}
