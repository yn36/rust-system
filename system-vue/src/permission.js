import Vue from "vue";
import { ACCESS_TOKEN } from "@/store/mutation-types";
import $store from '@/store'
import Router from 'vue-router'
import router from '@/router/index'
import { mergeI18nFromRoutes } from '@/utils/i18n'
import { initI18n } from '@/utils/i18n'
import notification from 'ant-design-vue/es/notification'

import { getThemeFind } from '@/api/system-serve'

import NProgress from 'nprogress'
import "nprogress/nprogress.css";

export const i18n = initI18n('CN', 'US', 'HK')

NProgress.configure({ showSpinner: false })
const whiteList = ['/login', '登录页']

//递归汇总路径，用于判断前往路由是否具有权限
const sumPath = (Routers, path) => {
  let bool = false
  const sumP = function (routers) {
    if (routers && routers.length) {
      routers.some(router => {
        sumP(router.children)
        bool = bool ? bool : router.path === path
        return bool
      })
    } else {
      return
    }
  }
  sumP(Routers)
  return bool
}

/**
 * 跳转统一登录
 * @param {String} backurl 
 */
const gotoLogin = (backurl) => {
  let url = process.env.VUE_APP_MBP_LOGIN_URL
  if (url.indexOf("?") < 0) {
    url = `${url}?`
  }
  backurl = `${window.location.protocol}//${window.location.hostname}:${window.location.port}${window.location.pathname}#${backurl}`;
  backurl = decodeURIComponent(backurl);
  window.location.href = `${url}&backurl=${backurl}`
}

router.beforeEach((to, from, next) => {
  NProgress.start()

  if (!Vue.ss.get(ACCESS_TOKEN) && to.path === '/login') {
    // gotoLogin("/")
    NProgress.done()
  }

  const token = to.query.token || from.query.token;
  if (token) {
    const flag = Vue.ss.get(ACCESS_TOKEN) != token
    Vue.ss.set(ACCESS_TOKEN, token);
    $store.commit("account/setToken", token);
    if (flag) {
      $store.dispatch("account/isAuthed").then(res => {
        setUserTheme(res)
      })
    }
  }

  // 验证是否已登录
  if (Vue.ss.get(ACCESS_TOKEN)) {
    if (to.path === '/login') {
      // gotoLogin("/")
      next()
    } else {
      // debugger
      if (!$store.getters['account/roles']) {
        $store.dispatch("account/isAuthed").then(res => {
          // debugger
          setUserTheme(res)
          let roles = []
          res.body.roles.map(v => {
            roles.push(v._id)
          })
          $store.commit('account/setRoles', roles)

          $store.dispatch("GenerateRoutes", { roles }).then((asyncRouter) => {

            // debugger
            router.options = {
              routes: asyncRouter
            }
            router.matcher = new Router({ routes: [] }).matcher
            router.addRoutes(asyncRouter)

            // 提取路由国际化数据
            mergeI18nFromRoutes(i18n, router.options.routes)
            // 初始化Admin后台菜单数据
            const rootRoute = router.options.routes.find(item => item.path === '/')
            const menuRoutes = rootRoute && rootRoute.children
            if (menuRoutes) {
              $store.commit('setting/setMenuData', menuRoutes)
            }

            const redirect = from.query.redirect ? decodeURIComponent(from.query.redirect) : null

            if (redirect) {
              // 跳转到目的路由
              next({ path: redirect })
              // // hack方法 确保addRoutes已完成 ,set the replace: true so the navigation will not leave a history record
              // next({ ...to, replace: true })
            } else {
              //修复to.path不为当前path的BUG
              if (sumPath(asyncRouter, to.path)) {
                next({ ...to, replace: true })
              } else {
                next({ path: asyncRouter.find(e => e.name === 'system').redirect })
              }
              // sumPath(asyncRouter, to.path) ? next({ ...to, replace: true }) : next({ path: asyncRouter.find(e => e.name === 'system').redirect })
            }

            // next({ path: '/account/PersonalSettings/BasicSettings' })
            // next({ path: '/prod/product/manage' })
          })
        }).catch(() => {
          notification.error({
            message: '错误',
            description: '请求用户信息失败，请重试'
          })
          $store.dispatch('account/Logout').then(() => {
            next({ path: '/login', query: { redirect: to.fullPath, _useiframe: false } })
          })
        })
      } else {
        next()
      }

    }
  } else {
    if (whiteList.includes(to.name)) {
      next()
    } else {
      // gotoLogin("/")
      next({
        path: '/login',
        query: { redirect: to.fullPath, _useiframe: false }
      })
    }
  }
})

router.afterEach(() => {
  NProgress.done()
})

function setUserTheme(user_info) {
  if (user_info.success) {
    /** 主题 */
    getThemeFind().then(res => {
      let result = res.data
      if (result.success) {
        let theme = result.body
        /** 如果用户主题设置为自动 */
        if (theme.appearance == 'auto') {
          let isLight =
            window.matchMedia &&
            window.matchMedia("(prefers-color-scheme: light)").matches;
          Object.assign(theme, {
            mode: isLight ? "light" : "night",
            color: isLight ? "#1890ff" : "#13c2c2",
          });
          $store.commit('setting/setTheme', { ...theme, mode: isLight ? "light" : "night" });
        } else {
          Object.assign(theme, {
            ...$store.state.setting.theme,
            mode: theme.appearance,
            color: theme.theme_color
          })
          $store.commit('setting/setTheme', { ...theme });
          $store.commit('setting/setTheme', { ...theme });
        }
        $store.commit('setting/setLayout', theme.navigate);
        $store.commit('setting/setFixedHeader', theme.fixed_header);
        $store.commit('setting/setWeekMode', theme.week_mode);
        $store.commit('setting/setMultiPage', theme.multi_pages);

        $store.commit('account/setUser', user_info.body)
      }
    })
  } else {
    Vue.error = user_info.message;
  }
}