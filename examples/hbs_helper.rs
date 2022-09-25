use std::error::Error;

use convert_case::{Casing, Case};
use handlebars::{handlebars_helper, Handlebars, JsonRender};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};


// a helper returns number of provided parameters
handlebars_helper!(nargs: |*args| args.len());

handlebars_helper!(snake: |s: String| s.to_case(Case::Snake));

// a helper joins all values, using both hash and parameters
handlebars_helper!(join: |{sep:str=","}, *args|
                   args.iter().map(|a| a.render()).collect::<Vec<String>>().join(sep)
);

handlebars_helper!(isdefined: |v: Value| !v.is_null());

#[derive(Serialize, Deserialize)]
struct A {
    name: String
}

fn main() -> Result<(), Box<dyn Error>> {
    // create the handlebars registry
    let mut handlebars = Handlebars::new();

    handlebars.register_helper("nargs", Box::new(nargs));
    handlebars.register_helper("join", Box::new(join));
    handlebars.register_helper("isdefined", Box::new(isdefined));
    handlebars.register_helper("snake", Box::new(snake));

    println!("{}", handlebars.render_template("{{nargs 1 2 3 4}}", &())?);
    let name =  "store_task".to_string();
    let a = A{
        name
    };
    println!("{:?}", handlebars.render_template("{{snake this}}", &"storeTask".to_string()));

    println!(
        "{}",
        handlebars.render_template("{{join 1 2 3 4 sep=\"|\" }}", &())?
    );

    println!(
        "{}",
        handlebars.render_template(
            r#"{{isdefined a}} {{isdefined b}}
{{#if (isdefined a)}}a{{/if}} {{#if (isdefined b)}}b{{/if}}"#,
            &json!({"a": 1})
        )?
    );

    Ok(())
}