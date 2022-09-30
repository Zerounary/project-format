use jsonpath_rust::{JsonPathQuery};
use serde_json::json;


fn main() {
  let data = json!({
    "data": {
      "table_names": [
        "book",
        "demand"
      ]
    }
  });

  let tables = &data.path("$.data.table_names").unwrap();
  println!("{:?}",tables.to_string());
}