insert into `sys_menu`
(menu_name, perms)
values
{{#each pages }}
{{#each children }}
('{{label}}', '{{tableName}}:view'),
-- ('{{label}}', '{{tableName}}:add'),
-- ('{{label}}', '{{tableName}}:update'),
-- ('{{label}}', '{{tableName}}:delete'),
-- ('{{label}}', '{{tableName}}:invalid'),
-- ('{{label}}', '{{tableName}}:submit'),
-- ('{{label}}', '{{tableName}}:import'),
-- ('{{label}}', '{{tableName}}:export'),
{{/each}}
{{/each}};