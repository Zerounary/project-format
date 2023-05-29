-- liquibase formatted sql

-- 创建表结构
-- changeset codegen:a
CREATE SEQUENCE seq_{{prefix}}{{name}}
START WITH 1
INCREMENT BY 1
NO MINVALUE
NO MAXVALUE
CACHE 1;

create table {{prefix}}{{name}} (
{{#each columns }}
    {{snake name}} {{dbType}} {{dbTypeWith}},
{{/each}}
    tenant_id INT8,
    created_time TIMESTAMP,
    created INT8,
    updated_time TIMESTAMP,
    updated INT8,
    isactive bool default true,
    primary key (id)
);


-- 创建表约束
create index idx_{{name}}_tenant_id on {{prefix}}{{name}}(tenant_id);
{{#if ddls}}
-- changeset codegen:b{{@index}}
{{#each ddls }}
{{this}}
{{/each}}
{{/if}}