-- liquibase formatted sql

-- 创建表结构
create table sys_tenant (
    id int8  AUTO_INCREMENT,
    code varchar(120) ,
    name varchar(120) ,
    ty varchar(120) ,
    description varchar(120) ,
    tenant_id int8,
    created_time datetime,
    created int8,
    updated_time datetime,
    updated int8,
    is_active bool default true,
    primary key (id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

{{#each tables }}
-- changeset codegen:a{{@index}}
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

{{/each}}


-- 创建表约束
{{#each tables }}
create index idx_{{name}}_tenant_id on {{prefix}}{{name}}(tenant_id);
{{#each ddls }}
{{this}}
{{/each}}
{{/each}}