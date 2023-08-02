use serde::{Deserialize, Serialize};
use struct_convert::Convert;
use rbatis::rbdc::{decimal::Decimal, datetime::FastDateTime, date::Date, json::Json};


use crate::{
    entities::{{snake this.name}}_bo::{{upperCamel this.name}}BO,
    entities::{{snake this.name}}_opt_bo::{{upperCamel this.name}}OptionBO,
    service::{{snake this.name}}_service::{Create{{upperCamel this.name}}Input, Update{{upperCamel this.name}}Input},
};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
#[derive(Convert)]
#[convert(into = "Create{{upperCamel this.name}}Input")]
pub struct Create{{upperCamel this.name}}VO {
    {{#each columns}}
    {{#unless (isId name)}}
    {{#if skip.[0]}}
    {{#if skip.[4]}}
    pub {{name}}: Option<{{type}}>,
    {{else}}
    pub {{name}}: {{type}},
    {{/if}}
    {{/if}}
    {{/unless}}
    {{/each}}
}
