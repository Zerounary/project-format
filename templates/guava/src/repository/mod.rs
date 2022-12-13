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
use crate::{macros::repository::impl_repo_select_one, server::auth::SessionUser};
use crate::drivers::db::DB; 
impl_repo_select_one!(SessionUser{select_session_user_by_name(name:&str) => 
r#"
`where name = #{name}`
"#});
{{/if}}