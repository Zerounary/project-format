export default {
  type: 'page',
  body: [
    {
      type: 'crud',
      api: 'get:/api/{{name}}/page',
      headerToolbar: [
        {
          label: '新增',
          type: 'button',
          actionType: 'dialog',
          level: 'primary',
          dialog: {
            title: '新增表单',
            body: {
              type: 'form',
              api: 'post:/api/{{name}}',
              body: [
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
              ]
            }
          }
        },
        'bulkActions',
        // 导出按钮 在打包后有效
        {
          type: 'export-excel',
          label: '导出Excel',
          api: '/api/{{name}}/list'
        },
        {
          type: 'columns-toggler',
          align: 'right',
          draggable: true,
          icon: 'fas fa-cog',
          overlay: true,
          footerBtnSize: 'sm'
        }
      ],
      bulkActions: [
        {
          label: '批量删除',
          actionType: 'ajax',
          api: 'delete:/api/{{name}}/${ids}',
          confirmText: '确定要批量删除?'
        }
      ],
      autoGenerateFilter: true,
      columns: [
        {
          label: '序号',
          type: 'tpl',
          tpl: '${index + 1}'
        },
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
                  fk_{{name}}: payload.data.name
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
          ...{{{stringify ui}}}
        },
          {{/if}}
          {{/if}}
        {{/each}}
        {
          type: 'operation',
          label: '操作',
          buttons: [
            {
              label: '详情',
              type: 'button',
              level: 'link',
              actionType: 'dialog',
              dialog: {
                title: '查看详情',
                body: {
                  type: 'form',
                  api: 'put:/api/{{name}}/${id}',
                  body: [
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
                  ]
                }
              }
            },
            {
              label: '删除',
              type: 'button',
              level: 'link',
              className: 'text-danger',
              confirmText: '确认要删除？',
              actionType: 'ajax',
              api: 'delete:/api/{{name}}/${id}'
            }
          ]
        }
      ]
    }
  ]
};
