import {{camel name}}Columns from './columns/{{camel name}}Columns';
import {{camel name}}Create from './create/{{camel name}}Create';
import {{camel name}}Update from './update/{{camel name}}Update';

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
                ...{{camel name}}Create,
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
        ...{{camel name}}Columns,
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
                  data: {
                    "id": "${id}"
                  },
                  api: 'patch:/api/{{name}}',
                  body: [
                    ...{{camel name}}Update,
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
