-- liquibase formatted sql

-- 创建定时任务
{{#each tables }}
{{#if cron }}
-- changeset {{../project_name}}:d{{@index}}
insert into SYS_SCHEDULE_CRON
(id, name, cron, created_time, updated_time)
select get_sequences('SYS_SCHEDULE_CRON'), '{{name}}', '{{cron}}', sysdate, sysdate
from dual
{{/if}}
{{/each}}