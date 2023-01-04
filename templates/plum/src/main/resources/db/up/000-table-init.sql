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


-- 创建函数
-- changeset codegen:c1  endDelimiter:/
CREATE OR REPLACE FUNCTION create_seq(p_tablename IN VARCHAR2)
    RETURN NUMBER AS
    PRAGMA AUTONOMOUS_TRANSACTION;
    v_id NUMBER(10);
BEGIN
    BEGIN
        EXECUTE IMMEDIATE 'drop sequence SEQ_' || upper(p_tablename);
    EXCEPTION
        WHEN OTHERS THEN
            NULL;
    END;

    BEGIN
        EXECUTE IMMEDIATE 'select nvl(max(id),0) from ' || upper(p_tablename)
            INTO v_id;
    EXCEPTION
        WHEN OTHERS THEN
            v_id := 0;
    END;

    EXECUTE IMMEDIATE '
    CREATE sequence SEQ_' || upper(p_tablename) ||
                      ' minvalue 1 maxvalue 999999999999 START WITH ' || (v_id + 2) ||
                      ' increment BY 1 cache 5';
    COMMIT;

    RETURN v_id + 1;
END;
/
CREATE OR REPLACE FUNCTION get_sequences(p_tablename IN VARCHAR2)
    RETURN NUMBER AS
    v_tablename    VARCHAR2(30);
    v_seq_name     VARCHAR2(40);
    v_tableid      NUMBER(10);
    v_tableid_loop NUMBER(10);
    v_flowno       NUMBER(10);
    v_count        NUMBER(10);
BEGIN
    v_tablename := upper(p_tablename);
    v_seq_name := 'SEQ_' || v_tablename;

    BEGIN
        EXECUTE IMMEDIATE '
        SELECT ' || v_seq_name || '.nextval
        FROM dual'
            INTO v_tableid;
    EXCEPTION
        WHEN OTHERS THEN
            v_tableid := create_seq(v_tablename);
    END;

    RETURN v_tableid;
END;
/