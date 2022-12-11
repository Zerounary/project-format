{{#each tables}}
pub mod {{this.name}}_repo;
{{/each}}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Repository {}

impl Repository {
    pub fn new() -> Repository {
        Repository {}
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