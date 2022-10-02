use convert_case::Case;
use pest::Parser;

#[derive(Parser)]
#[grammar = "parser/gen_file_name.pest"]
pub struct GenFileParser;

#[derive(Default, Debug, PartialEq, Eq)]
pub struct FileNameExpr {
    pub case: String,
    pub list_path: String,
    pub item_path: String,
}

pub fn to_case(name: &str) -> Option<Case> {
    match name {
        "upper" => Some(Case::Upper),
        "lower" => Some(Case::Lower),
        "snake" => Some(Case::Snake),
        "upperSnake" => Some(Case::UpperSnake),
        "camel" => Some(Case::Camel),
        "upperCamel" => Some(Case::UpperCamel),
        "kebab" => Some(Case::Kebab),
        "upperKebab" => Some(Case::UpperKebab),
        _ => None,
    }
}

pub fn parse_file_name(file_name: &str) -> FileNameExpr {
    let pairs = GenFileParser::parse(Rule::file_name, file_name).expect("解析文件名表达式失败");
    let mut expr = FileNameExpr::default();
    for pair in pairs {
        match pair.as_rule() {
            Rule::case => {
                expr.case = pair.as_str().to_string();
            }
            Rule::list_path => {
                expr.list_path = pair.as_str().to_string();
            }
            Rule::item_path => {
                expr.item_path = pair.as_str().to_string();
            }
            _ => {
                panic!("未知格式")
            }
        }
    }
    expr
}

#[test]
fn test_parse_file_name() {
    let result = parse_file_name("I{upperCamel obj.a.names ; name.b}Bo.hbs");
    debug_assert_eq!(
        FileNameExpr {
            case: "upperCamel".to_string(),
            list_path: "obj.a.names".to_string(),
            item_path: "name.b".to_string()
        },
        result
    );
}

#[test]
fn test_parse_digit() {
    let result = GenFileParser::parse(Rule::file_name, "I{upperCamel obj.a.names ; name.b}Bo.hbs");
    println!("{:#?}", result);
}
