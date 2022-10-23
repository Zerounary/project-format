{{#each tables }}
{{#each ddls }}
{{this}}
{{/each}}
alter table {{name}} ADD tenant_id int8;
alter table {{name}} ADD create_time datetime;
alter table {{name}} ADD create_user_id int8;
alter table {{name}} ADD modify_time datetime;
alter table {{name}} ADD modify_user_id int8;
{{/each}}