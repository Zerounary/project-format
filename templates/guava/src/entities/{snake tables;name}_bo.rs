// 业务的实体 自动生成，不能修改

use serde::{Serialize, Deserialize};
use smart_default::SmartDefault;
use rbatis::rbdc::{decimal::Decimal, date::Date};

#[derive(Debug, SmartDefault, Clone, Serialize, Deserialize)]
pub struct {{upperCamel name}}BO {
    {{#each columns}}
    {{#if default}}
    #[default(_code = "{{default}}")]
    {{/if}}
    pub {{name}}: {{type}},
    {{/each}}
}