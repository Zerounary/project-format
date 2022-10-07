// 业务的实体 自动生成，不能修改

use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct {{upperCamel name}}OptionBO {
    {{#each columns}}
    pub {{name}}: Option<{{type}}>,
    {{/each}}
}