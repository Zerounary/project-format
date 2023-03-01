{{#if cron }}
--changeset data:a
insert into SYS_SCHEDULE_CRON
(id, name, cron, created_time, updated_time)
select get_sequences('SYS_SCHEDULE_CRON'), '{{name}}', '{{cron}}', sysdate, sysdate
from dual;
{{/if}}