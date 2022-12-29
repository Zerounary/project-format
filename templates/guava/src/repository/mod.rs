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
use crate::macros::repository::{impl_repo_select_list, impl_repo_select};
use crate::{macros::repository::impl_repo_select_one, server::auth::SessionUser};
use crate::drivers::db::DB; 
impl_repo_select_one!(SessionUser{select_session_user_by_name(name:&str) => 
r#"
`where name = #{name}`
"#});

impl_repo_select_list!(MenuBO{select_menu_by_ids(ids:&str) => 
r#"
` join sys_role_menu srm on srm.menu_id = t.id `
`where srm.role_id in( ${ids} )`
"#});

impl Repository {
    pub async fn delete_role_menu_by_role_id(&self, db: &mut dyn rbatis::executor::Executor, id: i64) -> Result<(), rbatis::Error> {
        db.fetch("DELETE FROM `sys_role_menu` where role_id = ?", vec![to_value!(id)])
            .await
            .unwrap();
        Ok(())
    }

    pub async fn delete_user_role_by_user_id(&self, db: &mut dyn rbatis::executor::Executor, user_id: i64) -> Result<(), rbatis::Error> {
        db.fetch("DELETE FROM `sys_user_role` where user_id = ?", vec![to_value!(user_id)])
            .await
            .unwrap();
        Ok(())
    }
}
{{/if}}