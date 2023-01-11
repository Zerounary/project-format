-- liquibase formatted sql

-- 创建表结构
{{#each tables }}
-- changeset codegen:a{{@index}}
CREATE SEQUENCE seq_{{prefix}}{{name}}
START WITH 1
INCREMENT BY 1
NO MINVALUE
NO MAXVALUE
CACHE 1;

create table {{prefix}}{{name}} (
{{#each columns }}
    {{name}} {{dbType}} {{dbTypeWith}},
{{/each}}
    tenant_id INT8,
    created_time TIMESTAMP,
    created INT8,
    updated_time TIMESTAMP,
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

-- changeset codegen:c1
insert into sys_user
(id, name, password, tenant_id)
values (nextval('seq_sys_user'), 'mrbird', '123456', 1);