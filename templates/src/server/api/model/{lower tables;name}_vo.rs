use serde::{Deserialize, Serialize};
use struct_convert::Convert;


use crate::{
    entities::{{lower this.name}}_bo::{{upperCamel this.name}}BO,
    entities::{{lower this.name}}_opt_bo::{{upperCamel this.name}}OptionBO,
    service::{{lower this.name}}_service::{Create{{upperCamel this.name}}Input, Update{{upperCamel this.name}}Input},
};

#[derive(Debug, Default, Serialize, Deserialize)]
#[derive(Convert)]
#[convert(from = "{{upperCamel this.name}}BO")]
pub struct {{upperCamel this.name}}VO {
    #[convert_field(to_string)]
    {{#each columns}}
    {{#if (isId name) }}
    pub {{name}}: String,
    {{else}}
    pub {{name}}: {{type}},
    {{/if}}
    {{/each}}
}