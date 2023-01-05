{{#each tables}}
pub mod {{this.name}}_repo;
{{/each}}

#[derive(Debug)]
pub struct Repository {
    user: Option<SessionUser>,
}

impl Repository {
    pub fn new(user: Option<SessionUser>) -> Repository {
        Repository {
            user
        }
    }


}

{{#if auth}}
use rbs::to_value;
use crate::entities::menu_bo::MenuBO;
use crate::entities::role_menu_bo::RoleMenuBO;
use crate::entities::user_role_bo::UserRoleBO;
use crate::macros::repository::{impl_repo_select_list, impl_repo_select, impl_repo_delete};
use crate::{macros::repository::impl_repo_select_one, server::auth::SessionUser};
use crate::drivers::db::DB; 
impl_repo_select_one!(SessionUser => "id, name, password, tenant_id, role_ids" => {select_session_user_by_name(name:&str) => 
r#"
`where name = #{name}`
"#});

impl_repo_select_list!(MenuBO => "t.*" => {select_menu_by_ids(ids:&str) => 
r#"
` join sys_role_menu srm on srm.menu_id = t.id `
`where srm.role_id in( ${ids} )`
"#});

impl_repo_delete!(RoleMenuBO{delete_role_menu_by_role_id(role_id: i64) => "`where role_id = #{role_id}`"});

impl_repo_delete!(UserRoleBO{delete_user_role_by_user_id(user_id: i64) => "`where user_id = #{user_id}`"});

{{/if}}