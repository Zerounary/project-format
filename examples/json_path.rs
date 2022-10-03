use jsonpath_rust::JsonPathQuery;
use serde_json::json;

fn main() {
    let data = json!({
      "data": {
        "tables": [
          "book",
          "demand"
        ]
      }
    });

    let tables = &data.path("$.data.tables").unwrap();
    println!("{:?}", tables.to_string());

    let data = json!({"id":1,"name":{"b":"book1"}});
    // let data = &data.path("$.name.b").unwrap();
    // println!("{:?}", data.to_string());

    let path = &format!("$.{:}", "name.b");
    println!("{:?}", path);
    let item_name = data
        .clone()
        .path(path)
        .map(|v| match  v.get(0) {
            Some(l) => l.to_owned(),
            None => json!(""),
        })
        .expect("null");
    
    println!("{:?}", item_name);
}
