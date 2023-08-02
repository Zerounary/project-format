export default [
  {{#each columns}}
  {{#if ui}}
  {{#if (isFk name)}}
  {
    name: '{{name}}',
    type: 'select',
    label: '{{ui.label}}',
    searchable: true,
    {{#if (isIds name)}}
    multiple: true,
    source: {
      method: 'get',
      url: '/api/{{fkTable name}}/list',
      adaptor: function(payload, response) {
        return {
          ...payload,
          data: {
            rows: [
              ...payload.data.rows.map(e => ({
                ...e,
                id: `${e.id}`,
              }))
            ]
          }
        };
      }
    },
    {{else}}
    source: {
      method: 'get',
      url: '/api/{{fkTable name}}/list',
    },
    {{/if}}
    labelField: 'name',
    valueField: 'id',

  },
  {{else}}
  {
    name: '{{name}}',
    ...{{{stringify ui}}},
    {{#if (isEq ui.type "json-schema-editor")}}
      pipeIn: (value, data) => {
        if (typeof value === 'string') {
          return JSON.parse(value);
        }
        return value;
      },
    {{/if}}
  },
  {{/if}}
  {{/if}}
  {{/each}}
];
