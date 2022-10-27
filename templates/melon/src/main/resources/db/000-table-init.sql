-- liquibase formatted sql

-- 创建表结构
{{#each tables }}
-- changeset codegen:a{{@index}}
create table {{prefix}}{{name}} (
{{#each columns }}
    {{name}} {{dbType}} {{dbTypeWith}},
{{/each}}
    tenant_id NUMBER(10),
    created_time DATE,
    created NUMBER(10),
    updated_time DATE,
    updated NUMBER(10),
    isactive CHAR(1) default 'Y',
    primary key (id)
);

{{/each}}


-- 创建表约束
{{#each tables }}
-- changeset codegen:b{{@index}}
{{#each ddls }}
{{this}}
{{/each}}

{{#unless skipAuto ~}}
alter table {{prefix}}{{name}} add constraint FK_{{name}}_tenant foreign key (tenant_id) references sys_tenant(id);
alter table {{prefix}}{{name}} add constraint FK_{{name}}_created foreign key (created) references sys_user(id);
alter table {{prefix}}{{name}} add constraint FK_{{name}}_updated foreign key (updated) references sys_user(id);
{{~/unless}}
{{/each}}