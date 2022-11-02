-- liquibase formatted sql

-- 创建表结构
{{#each tables }}
-- changeset codegen:a{{@index}}
create table {{prefix}}{{name}} (
{{#each columns }}
{{#unless dbSkip }}
    {{name}} {{dbType}} {{dbTypeWith}},
{{/unless}}
{{/each}}
    tenant_id int8,
    created_time datetime,
    created int8,
    updated_time datetime,
    updated int8,
    is_active bool default true,
    primary key (id)
);

{{/each}}


-- 创建表约束
{{#each tables }}
{{#each ddls }}
{{this}}
{{/each}}
{{/each}}