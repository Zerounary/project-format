-- liquibase formatted sql

-- 创建表结构
create table {{prefix}}{{name}} (
{{#each columns }}
{{#unless dbSkip }}
{{#if (isId name)}}
    {{name}} {{dbType}} {{dbTypeWith}} AUTO_INCREMENT,
{{else}}
    {{name}} {{dbType}} {{dbTypeWith}},
{{/if}}
{{/unless}}
{{/each}}
    tenant_id int8,
    created_time datetime,
    created int8,
    updated_time datetime,
    updated int8,
    is_active bool default true,
    primary key (id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;


-- 创建表约束
create index idx_{{name}}_tenant_id on {{prefix}}{{name}}(tenant_id);
{{#each ddls }}
{{this}}
{{/each}}