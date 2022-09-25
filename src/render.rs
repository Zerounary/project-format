use handlebars::Handlebars;
use serde_yaml::Value;
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;
// use similar_asserts::assert_eq;

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
        let h = Handlebars::new();
        let render: Render = Self {
            h,
            ..Default::default()
        };
        render
    }

    pub fn set_template_extension(&mut self, extension: &str) {
        self.extension = extension.to_string()
    }

    pub fn copy_render<T: serde::ser::Serialize>(self, path_from: &str, path_to: &str, data: &T) {
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
                        println!("{:?}", result);
                        match result {
                            Ok(contents) => {
                                target.set_extension(self.extension.clone());
                                println!("{:?}", target);
                                fs::write(target, contents);
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
                println!("copy {:?} to {:?}", e.path(), target);
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
    data.insert("test", "test");
    render.copy_render("./templates", "./output", &data);
}

#[test]
fn test_path() {
    let path = Path::new("./templates/a.hbs");
}
