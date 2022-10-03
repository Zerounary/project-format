use ansi_term::{
    ANSIGenericString,
    Colour::{Green, Red},
};

pub fn println(tag: ANSIGenericString<str>, text: &str, path: &str) {
    println!("{} {} {}", tag, text, Green.paint(path));
}

pub fn log_text_ok(text: &str) {
    let tag = Green.paint("[OK] ");
    println(tag, text, "");
}

pub fn log_path_ok(text: &str, path: &str) {
    let tag = Green.paint("[OK] ");
    println(tag, text, path);
}

pub fn log_fail(text: &str, path: &str) {
    let tag = Red.paint("[ERR]");
    println(tag, text, path);
}
