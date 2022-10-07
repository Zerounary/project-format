use serde::{Deserialize, Serialize};
use struct_convert::Convert;


use crate::{
    entities::{{lower this.name}}_bo::{{upperCamel this.name}}BO,
    entities::{{lower this.name}}_opt_bo::{{upperCamel this.name}}OptionBO,
    service::{{lower this.name}}_service::{Create{{upperCamel this.name}}Input, Update{{upperCamel this.name}}Input},
};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
#[derive(Convert)]
#[convert(into = "Create{{upperCamel this.name}}Input")]
pub struct Create{{upperCamel this.name}}VO {
    {{#each columns}}
    {{#unless (isId name)}}
    pub {{name}}: {{type}},
    {{/unless}}
    {{/each}}
}
