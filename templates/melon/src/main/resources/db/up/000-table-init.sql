-- liquibase formatted sql

-- 创建表结构
{{#each tables }}
-- changeset codegen:a{{@index}}
create table {{prefix}}{{name}} (
{{#each columns }}
    {{snake name}} {{dbType}} {{{dbTypeWith}}},
{{/each}}
    created_time DATE,
    updated_time DATE,
    isactive CHAR(1) default 'Y',
    primary key (id)
);

{{/each}}


-- changeset codegen:z1
create table SYS_SCHEDULE_CRON (
    id NUMBER(15) ,
    name VARCHAR2(120) ,
    cron VARCHAR2(120) ,
    created_time DATE,
    updated_time DATE,
    isactive CHAR(1) default 'Y',
    primary key (id)
);