use serde::{Deserialize, Serialize};
use struct_convert::Convert;
use rbatis::rbdc::{decimal::Decimal, datetime::FastDateTime, date::Date};


use crate::{
    entities::{{snake this.name}}_bo::{{upperCamel this.name}}BO,
    entities::{{snake this.name}}_opt_bo::{{upperCamel this.name}}OptionBO,
    service::{{snake this.name}}_service::{Create{{upperCamel this.name}}Input, Update{{upperCamel this.name}}Input},
};


#[derive(Debug, Serialize, Deserialize)]
#[derive(Convert)]
#[convert(into = "{{upperCamel this.name}}OptionBO")]
pub struct {{upperCamel this.name}}OptionVO {
    #[convert_field(ignore)]
    {{#each columns}}
    {{#if skip.[2]}}
    pub {{name}}: Option<{{type}}>,
    {{/if}}
    {{/each}}
}
