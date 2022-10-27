-- 删库语句
{{#each tables }}
drop table {{prefix}}{{name}};
{{/each}}