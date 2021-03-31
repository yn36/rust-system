// import { request, METHOD, removeAuthorization } from '@/utils/request'
import { axios } from '@/utils/request'

/**
 * 登录服务
 * @param name 账户名
 * @param password 账户密码
 * @returns {Promise<AxiosResponse<T>>}
 */
export async function login(name, password) {
  return axios({
    url: `/system/auth/login`,
    method: 'post',
    data: {
      username: name,
      password: password
    }
  })
}

export async function getRoutesConfig() {
  return request(ROUTES, METHOD.GET)
}

export default {
  login,
  getRoutesConfig
}
