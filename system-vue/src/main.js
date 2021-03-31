import Vue from 'vue'
import App from './App.vue'
import router from '@/router/index'
// import { initRouter } from './router/index'
import './theme/index.less'
import Antd from 'ant-design-vue'
import Viser from 'viser-vue'
import store from './store'
import 'animate.css/source/animate.css'
import Plugins from '@/plugins'
// import bootstrap from '@/bootstrap'
import VueStorage from 'vue-ls'
import config from '@/config/config'
import { i18n } from './permission'

import './permission'

// const router = initRouter(store.state.setting.asyncRoutes)

import '@/components/global.less'

// 引入 sessionstorage
import VueSessionStorage from 'vue-sessionstorage'
Vue.use(VueSessionStorage)
Vue.ss = Vue.prototype.$session

//引入Cookies
import Cookies from 'js-cookie'
Vue.ck = Cookies

Vue.use(VueStorage, config.storageOptions)
Vue.prototype.$store = store

Vue.use(Antd)
Vue.config.productionTip = false
Vue.use(Viser)
Vue.use(Plugins)

// bootstrap({ router, store, i18n, message: Vue.prototype.$message })

new Vue({
  router,
  store,
  i18n,
  render: h => h(App),
}).$mount('#app')
