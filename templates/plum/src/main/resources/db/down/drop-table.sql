-- 删库语句
{{#each tables }}
drop table {{prefix}}{{name}};
drop SEQUENCE seq_{{prefix}}{{name}};
{{/each}}
drop table DATABASECHANGELOG;
drop table DATABASECHANGELOGLOCK;