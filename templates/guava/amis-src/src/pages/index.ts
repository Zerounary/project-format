{{#each tables }}
import {{camel name}}Schema from './schema/{{camel name}}Schema';
{{/each}}

export let hasPerm = (perm: string) => {
  let perms = JSON.parse(localStorage.getItem('perms') || '[]');
  let result = perms.find(e => e.perms === perm)
  return !!result;
}
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
        visible: hasPerm('{{camel tableName}}:view'),
        schema: {{camel tableName}}Schema
      },
{{/each}}
    ]
  },
{{/each}}
];
