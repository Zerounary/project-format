use serde::{Deserialize, Serialize};
use struct_convert::Convert;


use crate::{
    entities::{{lower this.name}}_bo::{{upperCamel this.name}}BO,
    entities::{{lower this.name}}_opt_bo::{{upperCamel this.name}}OptionBO,
    service::{{lower this.name}}_service::{Create{{upperCamel this.name}}Input, Update{{upperCamel this.name}}Input},
};

#[derive(Convert)]
#[convert(into = "Update{{upperCamel this.name}}Input")]
#[derive(Debug, Deserialize)]
pub struct Update{{upperCamel this.name}}VO {
    #[convert_field(unwrap)]
    pub id: Option<i64>,
    {{#each columns}}
    {{#unless (isId name) }}
    {{#if skip.[1]}}
    pub {{name}}: {{type}},
    {{/if}}
    {{/unless}}
    {{/each}}
}