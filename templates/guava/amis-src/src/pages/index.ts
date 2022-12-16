{{#each tables }}
import {{camel name}}Schema from './schema/{{camel name}}Schema';
{{/each}}
export default [
{{#each pages }}
  {
    label: '{{label}}',
    url: '{{url}}',
    children: [

{{#each children }}
      {
        label: '{{label}}',
        url: '/{{camel tableName}}',
        schema: {{camel tableName}}Schema
      },
{{/each}}
    ]
  },
{{/each}}
];
