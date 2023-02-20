-- liquibase formatted sql

-- 创建表结构
{{#each tables }}
-- changeset codegen:a{{@index}}
create table {{prefix}}{{name}} (
{{#each columns }}
    {{name}} {{dbType}} {{dbTypeWith}},
{{/each}}
    created_time DATE,
    updated_time DATE,
    isactive CHAR(1) default 'Y',
    primary key (id)
);

{{/each}}