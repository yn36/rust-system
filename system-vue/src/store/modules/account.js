import Vue from 'vue'
import { login, isAuthed } from '@/api/system-serve'
import { ACCESS_TOKEN } from '@/store/mutation-types'

export default {
  namespaced: true,
  state: {
    user: undefined,
    permissions: null,
    roles: null,
    routesConfig: null,
    token: null
  },
  getters: {
    user: state => {
      if (!state.user) {
        try {
          const user = localStorage.getItem(process.env.VUE_APP_USER_KEY)
          state.user = JSON.parse(user)
        } catch (e) {
          console.error(e)
        }
      }
      return state.user
    },
    permissions: state => {
      if (!state.permissions) {
        try {
          const permissions = localStorage.getItem(process.env.VUE_APP_PERMISSIONS_KEY)
          state.permissions = JSON.parse(permissions)
          state.permissions = state.permissions ? state.permissions : []
        } catch (e) {
          console.error(e.message)
        }
      }
      return state.permissions
    },
    roles: state => {
      // if (!state.roles) {
      //   try {
      //     const roles = localStorage.getItem(process.env.VUE_APP_ROLES_KEY)
      //     state.roles = JSON.parse(roles)
      //     state.roles = state.roles ? state.roles : []
      //   } catch (e) {
      //     console.error(e.message)
      //   }
      // }
      return state.roles
    },
    routesConfig: state => {
      if (!state.routesConfig) {
        try {
          const routesConfig = localStorage.getItem(process.env.VUE_APP_ROUTES_KEY)
          state.routesConfig = JSON.parse(routesConfig)
          state.routesConfig = state.routesConfig ? state.routesConfig : []
        } catch (e) {
          console.error(e.message)
        }
      }
      return state.routesConfig
    },
  },
  mutations: {
    setUser(state, user) {
      state.user = user
      // localStorage.setItem(process.env.VUE_APP_USER_KEY, JSON.stringify(user))
    },
    setPermissions(state, permissions) {
      state.permissions = permissions
      // localStorage.setItem(process.env.VUE_APP_PERMISSIONS_KEY, JSON.stringify(permissions))
    },
    setRoles(state, roles) {
      state.roles = roles
      // localStorage.setItem(process.env.VUE_APP_ROLES_KEY, JSON.stringify(roles))
    },
    setRoutesConfig(state, routesConfig) {
      state.routesConfig = routesConfig
      // localStorage.setItem(process.env.VUE_APP_ROUTES_KEY, JSON.stringify(routesConfig))
    },
    setToken(state, token) {
      state.token = token
    },
  },
  actions: {
    /** 判断是否已登录 */
    isAuthed({ commit }) {
      return new Promise((resolve, reject) => {
        const token = Vue.ss.get(ACCESS_TOKEN) || Vue.ls.get(ACCESS_TOKEN) || Vue.ck.get(ACCESS_TOKEN)
        if (token) {
          isAuthed().then(res => {
            let result = res.data
            if (result.success) {
              commit('setUser', result.body)
              resolve(result)
            } else {
              resolve()
            }
          }).catch(() => {
            reject()
          })
        } else {
          reject()
        }
      })
    },

    /** 用户登录 */
    Login({ commit }, userInfo) {
      return login(userInfo)
    },

    /** 退出登录 */
    Logout({ commit, state }) {
      return new Promise(resolve => {
        commit('setToken', '')
        commit('setUser', null)
        commit('setRoles', null)

        Vue.ss.remove(ACCESS_TOKEN);
        Vue.ls.remove(ACCESS_TOKEN);
        Vue.ck.remove(ACCESS_TOKEN);
        window.location.href = "#/login"
        resolve()
      })
    },
  }
}
