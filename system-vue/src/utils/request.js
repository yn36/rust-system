import Vue from 'vue'
import axios from 'axios'
import store from '@/store'
import { VueAxios } from './axios'
import notification from 'ant-design-vue/es/notification'
import { ACCESS_TOKEN } from '@/store/mutation-types'


// 创建 axios 实例
const service = axios.create({
  baseURL: process.env.VUE_APP_CONTEXT || "",
  timeout: 60000, // 请求超时时间
})

const err = error => {
  if (error.response) {
    const data = error.response.data
    const token = Vue.ss.get(ACCESS_TOKEN) || Vue.ls.get(ACCESS_TOKEN) || Vue.ck.get(ACCESS_TOKEN)
    if (error.response.status === 404) {
      notification.error({
        message: 'NotFound',
        description: data.message
      })
    } else if (error.response.status === 403) {
      notification.error({
        message: 'Forbidden',
        description: data.message
      })
    } else if (error.response.status === 402) {
      notification.error({
        message: 'License验证失败',
        description: data.message
      })
    } else if (error.response.status === 401) {
      notification.error({
        message: '未经授权',
        description: '授权验证失败'
      })
      if (token) {
        store.dispatch('account/Logout').then(() => {
          setTimeout(() => {
            window.location.reload()
          }, 1500)
        })
      } else {
        window.location.href = "#/login"
      }
    } else if (error.response.status === 400) {
      notification.error({
        message: '错误',
        description: data.message
      })
      Vue.prototype.$message.error(data.message)
    } else if (error.response.status === 500) {
      notification.error({
        message: '服务器错误',
        description: "服务器错误"
      })
    }
  }
  return Promise.reject(error)
}

// request interceptor
service.interceptors.request.use(config => {
  const token = Vue.ss.get(ACCESS_TOKEN)
  if (token) {
    Vue.ck.set(ACCESS_TOKEN, token)
    // config.headers[ACCESS_TOKEN] = token // 让每个请求携带自定义 token 请根据实际情况自行修改
    // config.headers["Authorization"] = 'Bearer ' + token // 让每个请求携带自定义 token 请根据实际情况自行修改
  }
  // config.headers['X-Axios-With'] = false
  return config
}, err)

// response interceptor
service.interceptors.response.use(response => {
  if (response.data === '') {
    store.dispatch('account/Logout').then(() => {
      setTimeout(() => {
        window.location.reload()
      }, 500)
    })
  } else {
    return response
  }
  // return response.data
}, err)

const installer = {
  vm: {},
  install(Vue) {
    Vue.use(VueAxios, service)
  }
}

export { installer as VueAxios, service as axios }
