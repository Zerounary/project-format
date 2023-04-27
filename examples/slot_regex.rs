use regex::Regex;

extern crate regex;

fn main() {
    let TAG_REGEX = Regex::new(r"//<slot>([\s\S]*?)//</slot>").unwrap();
    // let TAG_REGEX = Regex::new(r"<tag>(.*?)</tag>").unwrap();

    let target_str = r#"
    import a;
    import b;

    fn add() {
        //<slot>
        hello world
        print('abc', 123);
        //</slot>
    }

    fn select() {
        //<slot>
        select * from a;
        //</slot>
    }
    <tag>hello world</tag>
    "#;
    for captures in TAG_REGEX.captures_iter(target_str) {
        let tag_content = captures.get(1).unwrap().as_str();
        println!("{}", tag_content);
    }
}
