{{#each pages }}
{{#each children }}
insert into `sys_menu`
(pid, menu_name, path, component, perms, icon, ty, sort, description)
values
(0, '{{label}} - 查看', '', '', '{{tableName}}:view', '', '', 0, ''),
(0, '{{label}} - 新增', '', '', '{{tableName}}:add', '', '', 0, ''),
(0, '{{label}} - 修改', '', '', '{{tableName}}:update', '', '', 0, ''),
(0, '{{label}} - 删除', '', '', '{{tableName}}:delete', '', '', 0, ''),
(0, '{{label}} - 作废', '', '', '{{tableName}}:invalid', '', '', 0, ''),
(0, '{{label}} - 提交', '', '', '{{tableName}}:submit', '', '', 0, ''),
(0, '{{label}} - 导入', '', '', '{{tableName}}:import', '', '', 0, ''),
(0, '{{label}} - 导出', '', '', '{{tableName}}:export', '', '', 0, '');
{{/each}}
{{/each}}