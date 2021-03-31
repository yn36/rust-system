import axios from 'axios'
import Cookie from 'js-cookie'
import notification from 'ant-design-vue/es/notification'
import { ACCESS_TOKEN } from '@/store/mutation-types'

// 跨域认证信息 header 名
const xsrfHeaderName = 'Authorization'

axios.defaults.timeout = 60000
axios.defaults.withCredentials = true
axios.defaults.xsrfHeaderName = xsrfHeaderName
axios.defaults.xsrfCookieName = xsrfHeaderName
axios.defaults.baseURL = process.env.VUE_APP_CONTEXT || ''

// 认证类型
const AUTH_TYPE = {
  BEARER: 'Bearer',
  BASIC: 'basic',
  AUTH1: 'auth1',
  AUTH2: 'auth2',
}

// http method
const METHOD = {
  GET: 'get',
  POST: 'post'
}

/**
 * axios请求
 * @param url 请求地址
 * @param method {METHOD} http method
 * @param data 请求参数
 * @param config 请求头数据
 * @returns {Promise<AxiosResponse<T>>}
 */
async function request(url, method, data, config) {
  return axios({
    url,
    method,
    data,
    ...config
  })
}

/**
 * 设置认证信息
 * @param auth {Object}
 * @param authType {AUTH_TYPE} 认证类型，默认：{AUTH_TYPE.BEARER}
 */
function setAuthorization(auth, authType = AUTH_TYPE.BEARER) {
  switch (authType) {
    case AUTH_TYPE.BEARER:
      Cookie.set(xsrfHeaderName, 'Bearer ' + auth.token, { expires: auth.expireAt })
      break
    case AUTH_TYPE.BASIC:
    case AUTH_TYPE.AUTH1:
    case AUTH_TYPE.AUTH2:
    default:
      break
  }
}

/**
 * 移出认证信息
 * @param authType {AUTH_TYPE} 认证类型
 */
function removeAuthorization(authType = AUTH_TYPE.BEARER) {
  switch (authType) {
    case AUTH_TYPE.BEARER:
      Cookie.remove(xsrfHeaderName)
      break
    case AUTH_TYPE.BASIC:
    case AUTH_TYPE.AUTH1:
    case AUTH_TYPE.AUTH2:
    default:
      break
  }
}

/**
 * 检查认证信息
 * @param authType
 * @returns {boolean}
 */
function checkAuthorization(authType = AUTH_TYPE.BEARER) {
  switch (authType) {
    case AUTH_TYPE.BEARER:
      if (Cookie.get(xsrfHeaderName)) {
        return true
      }
      break
    case AUTH_TYPE.BASIC:
    case AUTH_TYPE.AUTH1:
    case AUTH_TYPE.AUTH2:
    default:
      break
  }
  return false
}

/**
 * 加载 axios 拦截器
 * @param interceptors
 * @param options
 */
function loadInterceptors(interceptors, options) {
  const { request, response } = interceptors
  // 加载请求拦截器
  request.forEach(item => {
    let { onFulfilled, onRejected } = item
    if (!onFulfilled || typeof onFulfilled !== 'function') {
      onFulfilled = config => config
    }
    if (!onRejected || typeof onRejected !== 'function') {
      onRejected = error => Promise.reject(error)
    }
    axios.interceptors.request.use(
      config => {
        console.log(config, 'options', options);
        onFulfilled(config, options)
      },
      // error => {
      //   notification.error({
      //     message: "错误",
      //     description: error.response.data.message,
      //   });
      // }
    )
  })
  // 加载响应拦截器
  response.forEach(item => {
    let { onFulfilled, onRejected } = item
    if (!onFulfilled || typeof onFulfilled !== 'function') {
      onFulfilled = response => response
    }
    if (!onRejected || typeof onRejected !== 'function') {
      onRejected = error => Promise.reject(error)
    }
    axios.interceptors.response.use(
      response => onFulfilled(response, options),
      error => {
        console.log(error.response, 'error');
        // return onRejected(error, options)
        notification.error({
          message: "错误",
          description: error.response.data.message,
        });
      }
    )
  })
}

export {
  METHOD,
  AUTH_TYPE,
  request,
  setAuthorization,
  removeAuthorization,
  checkAuthorization,
  loadInterceptors
}
