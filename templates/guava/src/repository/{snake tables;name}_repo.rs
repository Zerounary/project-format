use super::Repository;
use crate::{
    drivers::db::DB, 
    entities::{{snake name}}_bo::{{upperCamel this.name}}BO,
    entities::{{snake this.name}}_opt_bo::{{upperCamel this.name}}OptionBO,
    macros::repository::{impl_repo_insert, impl_repo_update, impl_repo_select_one, impl_repo_select_page, impl_repo_select_list, impl_repo_delete},
};
use itertools::Itertools;


impl_repo_select_one!({{upperCamel this.name}}BO => "count(1)" => i64 {count_{{snake name}}() => r#""#});
// impl_repo_select!(UserBO{select_bo_by_id(id: i64) -> Option => "`where id = #{id}`"});
impl_repo_select_one!({{upperCamel this.name}}BO{select_{{snake this.name}}_by_id});
//impl_repo_select_one!({{upperCamel this.name}}BO{select_{{snake this.name}}_one(code:&str) => 
//r#"
//`where code = #{code}`
//"#});
impl_repo_select_list!({{upperCamel this.name}}BO{select_{{snake this.name}}_list(bo:{{upperCamel this.name}}OptionBO) => 
    // 此处 可以用 py_sql 和 html_sql 对比使用
r#"
where:
{{#each columns}}
  if bo.{{name}} != null && bo.{{name}} != '':
    `and {{name}} = #{ bo.{{name}} }`
{{/each}}
  "#});

impl_repo_select_page!({{upperCamel this.name}}BO{select_{{snake this.name}}_page(bo:{{upperCamel this.name}}OptionBO) => 
    // 此处 可以用 py_sql 和 html_sql 对比使用
r#"
where:
{{#each columns}}
  if bo.{{name}} != null && bo.{{name}} != '':
    `and {{name}} = #{ bo.{{name}} }`
{{/each}}
` order by id `
` limit #{page_start}, #{page_size}`
  "#});

impl_repo_update!({{upperCamel this.name}}BO{update_{{snake this.name}}_by_id(id: i64) => "`where id = #{id}`"});

impl_repo_update!({{upperCamel this.name}}OptionBO{update_{{snake this.name}}_opt_by_id(id: i64) => "`where id = #{id}`"});

impl_repo_insert!({{upperCamel this.name}}BO, create_{{snake this.name}}, create_{{snake this.name}}_batch);

impl_repo_delete!({{upperCamel this.name}}BO{delete_{{snake this.name}}_ids});

impl_repo_delete!({{upperCamel this.name}}BO{delete_{{snake this.name}}(id: i64) => "` where id = #{id}`"});

