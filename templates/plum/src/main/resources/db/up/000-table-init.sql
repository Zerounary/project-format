-- liquibase formatted sql

-- 创建表结构
{{#each tables }}
-- changeset codegen:a{{@index}}
create table {{prefix}}{{name}} (
{{#each columns }}
    {{name}} {{dbType}} {{dbTypeWith}},
{{/each}}
    tenant_id INT8,
    created_time DATE,
    created INT8,
    updated_time DATE,
    updated INT8,
    isactive bool default true,
    primary key (id)
);

{{/each}}


-- 创建表约束
{{#each tables }}
-- changeset codegen:b{{@index}}
{{#each ddls }}
{{this}}
{{/each}}

{{/each}}

