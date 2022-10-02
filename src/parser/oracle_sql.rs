use pest::Parser;

#[derive(Parser)]
#[grammar = "parser/oracle_sql.pest"]
pub struct OracleSQLParser;


#[test]
fn test_parse_digit() {
  let result = OracleSQLParser::parse(Rule::creat_table, include_str!("./test/create_table.sql"));
  println!("{:#?}",result);
}