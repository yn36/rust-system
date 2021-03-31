import { hasPermission, hasRole } from '@/utils/authority-utils'
import { loginIgnore } from '@/router/index'
import { ACCESS_TOKEN } from '@/store/mutation-types'
import store from '@/store'
import Vue from 'vue'

/**
 * 登录守卫
 * @param to
 * @param form
 * @param next
 * @param options
 */
const loginGuard = (to, from, next, options) => {
  const { message } = options
  /** 使用 sessionStorage、Cookie 还是 localStorage */
  if (Vue.ss.get(ACCESS_TOKEN) || Vue.ck.get(ACCESS_TOKEN) || Vue.ls.get(ACCESS_TOKEN)) {
    // 判断前端是否获取到用户信息
    if (!store.getters["account/user"]) {
      store.dispatch('account/isAuthed')
        .then(() => {
          // next({path: decodeURIComponent(redirectto || to.path)});
          next({ path: decodeURIComponent(to.path) });
        }).catch(() => {
          // 跳转到登录页面
          // gotoLogin(to.fullPath)
          NProgress.done() // if current page is login will not trigger afterEach hook, so manually handle it
        })
      return;
    }
  }
  next()
  /** 使用 sessionStorage、Cookie 还是 localStorage */
  if (!loginIgnore.includes(to) && (!Vue.ss.get(ACCESS_TOKEN) || !Vue.ck.get(ACCESS_TOKEN)) && !Vue.ls.get(ACCESS_TOKEN)) {
    message.warning('登录已失效，请重新登录')
    Vue.ls.remove(ACCESS_TOKEN)
    Vue.ss.remove(ACCESS_TOKEN)
    Vue.ck.remove(ACCESS_TOKEN)
    next({ path: '/login' })
  } else {
    next()
  }
}

/**
 * 权限守卫
 * @param to
 * @param form
 * @param next
 * @param options
 */
const authorityGuard = (to, from, next, options) => {
  const { store, message } = options
  const permissions = store.getters['account/permissions']
  const roles = store.getters['account/roles']
  // if (!hasPermission(to, permissions) && !hasRole(to, roles)) {
  //   message.warning(`对不起，您无权访问页面: ${to.fullPath}，请联系管理员`)
  //   next({ path: '/403' })
  // } else {
  //   next()
  // }
  next()
}

export default {
  beforeEach: [loginGuard, authorityGuard],
  afterEach: []
}
