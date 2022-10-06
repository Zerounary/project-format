{{#each tables}}
pub mod {{name}}_vo;
pub mod {{name}}_opt_vo;
pub mod {{name}}_create_vo;
pub mod {{name}}_update_vo;
{{/each}}