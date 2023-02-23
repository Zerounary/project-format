create table {{prefix}}{{name}} (
{{#each columns }}
    {{snake name}} {{dbType}} {{{dbTypeWith}}},
{{/each}}
    created_time DATE,
    updated_time DATE,
    isactive CHAR(1) default 'Y',
    primary key (id)
);