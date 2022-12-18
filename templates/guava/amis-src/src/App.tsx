import React from 'react';

import '@fortawesome/fontawesome-free/css/all.css';
import '@fortawesome/fontawesome-free/css/v4-shims.css';

import 'amis/lib/themes/cxd.css';
import 'amis/lib/helper.css';
import 'amis/sdk/iconfont.css';

import axios from 'axios';
import copy from 'copy-to-clipboard';

import {render as renderAmis, ToastComponent, AlertComponent} from 'amis';
import {alert, confirm, toast} from 'amis-ui';
import pages from './pages';

// amis 环境配置
const env = {
  // 下面三个接口必须实现
  fetcher: ({
    url, // 接口地址
    method, // 请求方法 get、post、put、delete
    data, // 请求数据
    responseType,
    config, // 其他配置
    headers // 请求头
  }: any) => {
    config = config || {};
    config.withCredentials = true;
    responseType && (config.responseType = responseType);

    if (config.cancelExecutor) {
      config.cancelToken = new (axios as any).CancelToken(
        config.cancelExecutor
      );
    }

    config.headers = headers || {};

    if (method !== 'post' && method !== 'put' && method !== 'patch') {
      if (data) {
        config.params = data;
      }
      return (axios as any)[method](url, config);
    } else if (data && data instanceof FormData) {
      config.headers = config.headers || {};
      config.headers['Content-Type'] = 'multipart/form-data';
    } else if (
      data &&
      typeof data !== 'string' &&
      !(data instanceof Blob) &&
      !(data instanceof ArrayBuffer)
    ) {
      data = JSON.stringify(data);
      config.headers = config.headers || {};
      config.headers['Content-Type'] = 'application/json';
    }

    return (axios as any)[method](url, data, config);
  },
  isCancel: (value: any) => (axios as any).isCancel(value),
  copy: (content: string) => {
    copy(content);
    toast.success('内容已复制到粘贴板');
  },

  // 后面这些接口可以不用实现

  // notify: (
  //   type: 'error' | 'success' /**/,
  //   msg: string /*提示内容*/
  // ) => {
  //   toast[type]
  //     ? toast[type](msg, type === 'error' ? '系统错误' : '系统消息')
  //     : console.warn('[Notify]', type, msg);
  // },
  // alert,
  // confirm,
};

class AMISComponent extends React.Component<any, any> {
  render() {
    return renderAmis(
      // 这里是 amis 的 Json 配置。
      {
        type: 'service',
        api: {
          url: '/api/auth',
          adaptor: function(payload, response) {
            if(location.href.endsWith('/login')){
              return {
                ...payload,
                status: 0
              };
            }else {
              if(payload.status == 401) {
                console.log(payload);
                localStorage.removeItem('auth')
                location.href = '/login'
                return payload;
              }else {
                return {
                  ...payload,
                  status: 0
                }
              }
            }
          }
        },
        body: [
          {
            type: 'page',
            cssVars: {
              '--Page-body-padding': '0'
            },
            body: [
              {
                type: 'flex',
                className: 'w-screen h-screen',
                hiddenOn: '${!!ls:auth}',
                items: [
                  {
                    type: 'form',
                    className: 'w-96',
                    api: {
                      method: 'get',
                      url: '/api/login?username=${username}&password=${password}',
                      adaptor: function (payload, response) {
                        if (payload.data) {
                          localStorage.setItem('auth', true)
                          location.href = '/';
                          return payload;
                        } else {
                          return payload;
                        }
                      }
                    },
                    body: [
                      {
                        type: 'input-text',
                        name: 'username',
                        label: '用户名：'
                      },
                      {
                        name: 'password',
                        type: 'input-password',
                        label: '密码：'
                      }
                    ]
                  }
                ]
              },
              {
                type: 'app',
                brandName: '管理系统',
                logo: '/assets/logo.png',
                hiddenOn: '${!ls:auth}',
                header: {
                  type: 'tpl',
                  inline: false,
                  className: 'w-full',
                  tpl: '<div class="flex justify-between"><div>顶部区域左侧</div><div>顶部区域右侧</div></div>'
                },
                pages
              }
            ]
          },
        ]
      },
      {
        // props...
      },
      env
    );
  }
}

class APP extends React.Component<any, any> {
  render() {
    return (
      <>
        <ToastComponent key="toast" position={'top-right'} />
        <AlertComponent key="alert" />
        <AMISComponent />
      </>
    );
  }
}

export default APP;
