export default [
  {{#each columns}}
  {{#if ui}}
  {{#if (isFk name)}}
  {
    name: '{{name}}',
    type: 'select',
    label: '{{ui.label}}',
    searchable: true,
    source: {
      method: 'get',
      url: '/api/{{fkTable name}}/list',
    },
    labelField: 'name',
    valueField: 'id',
  },
  {{else}}
  {
    name: '{{name}}',
    ...{{{stringify ui}}}
  },
  {{/if}}
  {{/if}}
  {{/each}}
];
