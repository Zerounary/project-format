-- 删库语句
{{#each tables }}
drop table {{prefix}}{{name}};
{{/each}}
drop table DATABASECHANGELOG;
drop table DATABASECHANGELOGLOCK;
drop table SYS_SCHEDULE_CRON;