export default {
  type: 'page',
  body: [
    {
      type: 'crud',
      api: 'get:/api/user/page',
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
              api: 'post:/api/user',
              body: [
                {
                  type: 'input-text',
                  name: 'name',
                  label: '用户名'
                },
                {
                  type: 'input-password',
                  name: 'password',
                  label: '密码'
                },
                {
                  type: 'input-number',
                  name: 'store_id',
                  label: '所属店铺'
                },
                {
                  type: 'input-number',
                  name: 'dept_id',
                  label: '所属部门'
                }
              ]
            }
          }
        },
        'bulkActions',
        // 导出按钮 在打包后有效
        {
          type: 'export-excel',
          label: '导出Excel',
          api: '/api/user/list'
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
          api: 'delete:/api/user/${ids}',
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
        {
          name: 'name',
          label: '用户名',
          searchable: {
            type: 'input-text',
            name: 'name',
            label: '用户名',
            placeholder: '输入用户名'
          }
        },
        {
          name: 'password',
          label: '密码',
          type: 'input-password',
          static: true
        },
        {
          name: 'store_id',
          label: '所属店仓'
        },
        {
          name: 'dept_id',
          label: '所属部门'
        },
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
                  api: 'post:/api/user',
                  body: [
                    {
                      type: 'input-text',
                      name: 'name',
                      label: '用户名'
                    },
                    {
                      type: 'input-password',
                      name: 'password',
                      label: '密码'
                    },
                    {
                      type: 'input-number',
                      name: 'store_id',
                      label: '所属店铺'
                    },
                    {
                      type: 'input-number',
                      name: 'dept_id',
                      label: '所属部门'
                    }
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
              api: 'delete:/api/user/${id}'
            }
          ]
        }
      ]
    }
  ]
};
