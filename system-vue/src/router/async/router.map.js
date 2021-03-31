// 视图组件
const view = {
  tabs: () => import('@/layouts/tabs'),
  blank: () => import('@/layouts/BlankView'),
  page: () => import('@/layouts/PageView')
}

// 路由组件注册
const routerMap = {
  login: {
    authority: '*',
    path: '/login',
    component: () => import('@/views/login')
  },
  root: {
    path: '/',
    name: '首页',
    redirect: '/login',
    component: view.tabs
  },
  dashboard: {
    name: 'Dashboard',
    component: view.blank
  },
  workplace: {
    name: '工作台',
    component: () => import('@/views/dashboard/workplace')
  },
  analysis: {
    name: '分析页',
    component: () => import('@/views/dashboard/analysis')
  },
  form: {
    name: '表单页',
    icon: 'form',
    component: view.page
  },
  basicForm: {
    path: 'basic',
    name: '基础表单',
    component: () => import('@/views/form/basic')
  },
  stepForm: {
    path: 'step',
    name: '分步表单',
    component: () => import('@/views/form/step')
  },
  advanceForm: {
    path: 'advance',
    name: '高级表单',
    component: () => import('@/views/form/advance')
  },
  list: {
    name: '列表页',
    icon: 'table',
    component: view.page
  },
  queryList: {
    path: 'query',
    name: '查询表格',
    component: () => import('@/views/list/QueryList')
  },
  primaryList: {
    path: 'primary',
    name: '标准列表',
    component: () => import('@/views/list/StandardList')
  },
  cardList: {
    path: 'card',
    name: '卡片列表',
    component: () => import('@/views/list/CardList')
  },
  searchList: {
    path: 'search',
    name: '搜索列表',
    component: () => import('@/views/list/search/SearchLayout')
  },
  article: {
    name: '文章',
    component: () => import('@/views/list/search/ArticleList')
  },
  application: {
    name: '应用',
    component: () => import('@/views/list/search/ApplicationList')
  },
  project: {
    name: '项目',
    component: () => import('@/views/list/search/ProjectList')
  },
  details: {
    name: '详情页',
    icon: 'profile',
    component: view.blank
  },
  basicDetails: {
    path: 'basic',
    name: '基础详情页',
    component: () => import('@/views/detail/BasicDetail')
  },
  advanceDetails: {
    path: 'advance',
    name: '高级详情页',
    component: () => import('@/views/detail/AdvancedDetail')
  },
  result: {
    name: '结果页',
    icon: 'check-circle-o',
    component: view.page
  },
  success: {
    name: '成功',
    component: () => import('@/views/result/Success')
  },
  error: {
    name: '失败',
    component: () => import('@/views/result/Error')
  },
  exception: {
    name: '异常页',
    icon: 'warning',
    component: view.blank
  },
  exp403: {
    authority: '*',
    name: 'exp403',
    path: '403',
    component: () => import('@/views/exception/403')
  },
  exp404: {
    name: 'exp404',
    path: '404',
    component: () => import('@/views/exception/404')
  },
  exp500: {
    name: 'exp500',
    path: '500',
    component: () => import('@/views/exception/500')
  },
  components: {
    name: '小组件',
    icon: 'appstore-o',
    component: view.page
  },
  taskCard: {
    name: '任务卡片',
    component: () => import('@/views/components/TaskCard')
  },
  palette: {
    name: '颜色复选框',
    component: () => import('@/views/components/Palette')
  }
}
export default routerMap

