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