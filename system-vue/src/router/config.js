import TabsView from '@/layouts/tabs/TabsView'
import BlankView from '@/layouts/BlankView'
import PageView from '@/layouts/PageView'

// 路由配置
const options = {
  routes: [
    {
      path: '/login',
      name: '登录页',
      component: () => import('@/views/login')
    },
    {
      path: '*',
      name: '404',
      component: () => import('@/views/exception/404'),
    },
    {
      path: '/403',
      name: '403',
      component: () => import('@/views/exception/403'),
    },
    {
      path: '/',
      name: '首页',
      component: TabsView,
      redirect: '/account/PersonalSettings/BasicSettings',
      children: [
        {
          path: '/prod',
          name: '产品',
          component: () => import('@/views'),
          meta: {
            title: '产品', keepAlive: false,
            i18n: {
              CN: '产品',
              US: 'prod',
              HK: '產品'
            }
          },
          children: [
            {
              path: 'product/manage',
              name: '产品管理',
              component: () => import('@/views/product/manage.vue'),
              meta: {
                title: '产品管理', keepAlive: false,
                i18n: {
                  CN: '产品管理',
                  US: 'management',
                  HK: '產品管理'
                }
              },
            },
          ]
        },
        {
          path: '/account',
          name: '帐号',
          component: () => import('@/views'),
          meta: { title: '帐号', keepAlive: false, },
          children: [
            {
              path: 'PersonalCenter',
              name: '个人中心',
              component: () => import('@/views/account/center'),
              meta: { title: '个人中心', keepAlive: false },
            },
            {
              path: 'PersonalSettings',
              name: '个人设置',
              component: () => import('@/views/account/settings/index'),
              meta: { title: '个人设置', hideHeader: true, },
              redirect: '/account/PersonalSettings/BasicSettings',
              hideChildrenInMenu: true,
              children: [
                {
                  path: 'BasicSettings',
                  name: '基本设置',
                  component: () => import('@/views/account/settings/BaseSetting'),
                  meta: { title: '基本设置', invisible: true, }
                },
                {
                  path: 'SecuritySettings',
                  name: '安全设置',
                  component: () => import('@/views/account/settings/Security'),
                  meta: { title: '安全设置', invisible: true, keepAlive: false, }
                },
                {
                  path: 'Personalization',
                  name: '个性化设置',
                  component: () => import('@/views/account/settings/Custom'),
                  meta: { title: '个性化设置', invisible: true, keepAlive: false, }
                },
                {
                  path: 'AccountBinding',
                  name: '账户绑定',
                  component: () => import('@/views/account/settings/Binding'),
                  meta: { title: '账户绑定', invisible: true, keepAlive: false, }
                },
                // {
                //   path: '/account/settings/notification',
                //   name: '新消息通知',
                //   component: () => import('@/views/account/settings/Notification'),
                //   meta: { title: '新消息通知', invisible: true, keepAlive: false, }
                // }
              ]
            }
          ]
        },
        {
          path: '/basic',
          name: '基础管理',
          component: () => import('@/views'),
          meta: {
            title: '基础管理', keepAlive: false,
          },
          children: [
            {
              path: 'user/manage',
              name: '用户管理',
              component: () => import('@/views/user/manage.vue'),
              meta: { title: '用户管理', keepAlive: false },
            },
            {
              path: 'role/manage',
              name: '角色管理',
              component: () => import('@/views/role/manage.vue'),
              meta: { title: '角色管理', keepAlive: false },
            },
          ]
        },
        {
          path: 'job/manage',
          name: '职务/岗位管理',
          component: () => import('@/views/job/manage.vue'),
          meta: { title: '职务/岗位管理', keepAlive: false },
        },
        {
          path: 'org/manage',
          name: '机构管理',
          component: () => import('@/views/org/manage.vue'),
          meta: { title: '机构管理', keepAlive: false },
        },
        {
          path: 'menu/manage',
          name: '菜单管理',
          component: () => import('@/views/menu/manage.vue'),
          meta: { title: '菜单管理', keepAlive: false, },
          hidden: true
        },
        {
          path: '/auth',
          name: '权限管理',
          component: () => import('@/views'),
          meta: { title: '权限管理', keepAlive: false, },
          children: [
            {
              path: 'usermanage',
              name: '用户权限管理',
              component: () => import('@/views/auth/userRole/manage.vue'),
              meta: { title: '用户权限管理', keepAlive: false },
            },
            {
              path: 'menumanage',
              name: '菜单权限管理',
              component: () => import('@/views/auth/menu/index.vue'),
              meta: { title: '菜单权限管理', keepAlive: false },
              hidden: true
            },
            {
              path: 'rolemanage',
              name: '角色权限管理',
              component: () => import('@/views/auth/roleMenu/manage.vue'),
              meta: { title: '角色权限管理', keepAlive: false },
            },
          ]
        },
        {
          path: 'dashboard',
          name: 'Dashboard',
          meta: {
            icon: 'dashboard'
          },
          component: BlankView,
          children: [
            // {
            //   path: 'workplace',
            //   name: '工作台',
            //   component: () => import('@/views/dashboard/workplace'),
            // },
            {
              path: 'analysis',
              name: '分析页',
              component: () => import('@/views/dashboard/analysis'),
            }
          ]
        },
        {
          path: 'form',
          name: '表单页',
          meta: {
            icon: 'form',
          },
          component: PageView,
          children: [
            {
              path: 'basic',
              name: '基础表单',
              component: () => import('@/views/form/basic'),
            },
            {
              path: 'step',
              name: '分步表单',
              component: () => import('@/views/form/step'),
            },
            {
              path: 'advance',
              name: '高级表单',
              component: () => import('@/views/form/advance'),
            }
          ]
        },
        {
          path: 'list',
          name: '列表页',
          meta: {
            icon: 'table'
          },
          component: PageView,
          children: [
            {
              path: 'query',
              name: '查询表格',
              meta: {
                authority: 'queryForm',
              },
              component: () => import('@/views/list/QueryList'),
            },
            {
              path: 'primary',
              name: '标准列表',
              component: () => import('@/views/list/StandardList'),
            },
            {
              path: 'card',
              name: '卡片列表',
              component: () => import('@/views/list/CardList'),
            },
            {
              path: 'search',
              name: '搜索列表',
              component: () => import('@/views/list/search/SearchLayout'),
              children: [
                {
                  path: 'article',
                  name: '文章',
                  component: () => import('@/views/list/search/ArticleList'),
                },
                {
                  path: 'application',
                  name: '应用',
                  component: () => import('@/views/list/search/ApplicationList'),
                },
                {
                  path: 'project',
                  name: '项目',
                  component: () => import('@/views/list/search/ProjectList'),
                }
              ]
            }
          ]
        },
        {
          path: 'details',
          name: '详情页',
          meta: {
            icon: 'profile'
          },
          component: BlankView,
          children: [
            {
              path: 'basic',
              name: '基础详情页',
              component: () => import('@/views/detail/BasicDetail')
            },
            {
              path: 'advance',
              name: '高级详情页',
              component: () => import('@/views/detail/AdvancedDetail')
            }
          ]
        },
        {
          path: 'result',
          name: '结果页',
          meta: {
            icon: 'check-circle-o',
          },
          component: PageView,
          children: [
            {
              path: 'success',
              name: '成功',
              component: () => import('@/views/result/Success')
            },
            {
              path: 'error',
              name: '失败',
              component: () => import('@/views/result/Error')
            }
          ]
        },
        {
          path: 'exception',
          name: '异常页',
          meta: {
            icon: 'warning',
          },
          component: BlankView,
          children: [
            {
              path: '404',
              name: 'Exp404',
              component: () => import('@/views/exception/404')
            },
            {
              path: '403',
              name: 'Exp403',
              component: () => import('@/views/exception/403')
            },
            {
              path: '500',
              name: 'Exp500',
              component: () => import('@/views/exception/500')
            }
          ]
        },
        {
          path: 'components',
          name: '小组件',
          meta: {
            icon: 'appstore-o'
          },
          component: PageView,
          children: [
            {
              path: 'taskCard',
              name: '任务卡片',
              component: () => import('@/views/components/TaskCard')
            },
            {
              path: 'palette',
              name: '颜色复选框',
              component: () => import('@/views/components/Palette')
            }
          ]
        },
        // {
        //   name: '验权表单',
        //   path: 'auth/form',
        //   meta: {
        //     icon: 'file-excel',
        //     authority: {
        //       permission: 'form'
        //     }
        //   },
        //   component: () => import('@/views/form/basic')
        // }
      ]
    },
  ]
}

export default options
