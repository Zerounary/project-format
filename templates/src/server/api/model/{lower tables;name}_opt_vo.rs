use serde::{Deserialize, Serialize};
use struct_convert::Convert;


use crate::{
    entities::{{lower this.name}}_bo::{{upperCamel this.name}}BO,
    entities::{{lower this.name}}_opt_bo::{{upperCamel this.name}}OptionBO,
    service::{{lower this.name}}_service::{Create{{upperCamel this.name}}Input, Update{{upperCamel this.name}}Input},
};


#[derive(Debug, Default, Serialize, Deserialize)]
#[derive(Convert)]
#[convert(into = "{{upperCamel this.name}}OptionBO")]
pub struct {{upperCamel this.name}}OptionVO {
    #[convert_field(ignore)]
    {{#each columns}}
    pub {{name}}: Option<{{type}}>,
    {{/each}}
}
