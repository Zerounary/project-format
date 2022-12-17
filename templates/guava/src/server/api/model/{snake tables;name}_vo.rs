use serde::{Deserialize, Serialize};
use struct_convert::Convert;
use smart_default::SmartDefault;
use rbatis::rbdc::{decimal::Decimal, date::Date};


use crate::{
    entities::{{snake this.name}}_bo::{{upperCamel this.name}}BO,
    entities::{{snake this.name}}_opt_bo::{{upperCamel this.name}}OptionBO,
    service::{{snake this.name}}_service::{Create{{upperCamel this.name}}Input, Update{{upperCamel this.name}}Input},
};

#[derive(Debug, SmartDefault, Serialize, Deserialize)]
#[derive(Convert)]
#[convert(from = "{{upperCamel this.name}}BO")]
pub struct {{upperCamel this.name}}VO {
    {{#each columns}}
    {{#if (isId name) }}
    pub {{name}}: i64,
    {{else}}
    {{#if skip.[3]}}
    {{#if default}}
    #[default(_code = "{{default}}")]
    {{/if}}
    pub {{name}}: {{type}},
    {{/if}}
    {{/if}}
    {{/each}}
}