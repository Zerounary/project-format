// 业务的实体 自动生成，不能修改

use serde::{Serialize, Deserialize};
use smart_default::SmartDefault;
use rbatis::{rbdc::{decimal::Decimal, date::Date}, crud};

use crate::macros::repository::to_sql_table_name;

#[derive(Debug, SmartDefault, Clone, Serialize, Deserialize)]
pub struct {{upperCamel name}}OptionBO {
    {{#each columns}}
    pub {{name}}: Option<{{type}}>,
    {{/each}}
}

crud!({{upperCamel name}}OptionBO{}, to_sql_table_name("`{{upperCamel name}}OptionBO`"));