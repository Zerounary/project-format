export default [
  {{#each columns}}
    {{#if ui}}
    {{#if (isFk name)}}
  {
    type: 'service',
    label: '{{ui.label}}',
    api: {
      method: 'get',
      url: '/api/{{fkTable name}}/${ {{name}} }',
      adaptor: function (payload, response) {
        return {
          ...payload,
          data: {
            fk_{{name}}: payload.data?.name
          },
        };
      },
      cache: 2000
    },
    body: {
      type: 'tpl',
      tpl: '${fk_{{name}} }'
    },
  },
    {{else}}
  {
    name: '{{name}}',
    static: true,
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
