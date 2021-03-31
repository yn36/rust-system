/*
 * @Description: 系统管理接口
 * @Version: 1.0
 * @Autor: jiajun wu
 * @Date: 2020-08-29 19:26:50
 * @LastEditors: JiaJun Wu
 * @LastEditTime: 2021-03-05 17:46:14
 */
import { axios } from '@/utils/request'
import $store from '@/store'
const prefix = '/system'

/**
 * 登录服务
 * @param name 账户名
 * @param password 账户密码
 * @returns {Promise<AxiosResponse<T>>}
 */
export async function login(data) {
  return axios({
    url: `${prefix}/auth/login`,
    method: 'post',
    data
  })
}

/** 认证当前登陆人 */
export function isAuthed() {
  return axios({
    url: `${prefix}/auth/isAuthed`,
    method: 'post'
  })
}

/** 用户信息修改 */
export function userUpdate(data) {
  return axios({
    url: `${prefix}/auth/update`,
    method: 'post',
    data
  })
}

/** 查询用户 */
export function getUserList(data) {
  return axios({
    url: `${prefix}/auth/find`,
    method: 'post',
    data
  })
}

/** 用户保存 */
export function getUserSave(data) {
  let url = `${prefix}/auth/reg`
  if (data._id) {
    url = `${prefix}/auth/update`
  }
  return axios({
    url,
    method: 'post',
    data
  })
}

/** 删除用户 */
export function getUserDelete(data) {
  return axios({
    url: `${prefix}/auth/delete`,
    method: 'delete',
    data
  })
}

/** 头像上传 */
export function UploadAvatarWith(file, affiliationId) {
  // return request(`${prefix}/annex/avatar/upload?affiliationId=${affiliationId}`, 'post', file).then(res => res.data)
  return axios({
    url: `/annex/annex/avatar/upload?affiliationId=${affiliationId}`,
    method: 'post',
    data: file
  })
}

/** 角色查询 */
export function getRoleList(data) {
  return axios({
    url: `${prefix}/role/find`,
    method: 'post',
    data
  })
}

/** 角色保存 */
export function getRoleSave(data) {
  let url = `${prefix}/role/save`
  if (data._id) {
    url = `${prefix}/role/update`
  }
  return axios({
    url,
    method: 'post',
    data
  })
}

/** 角色删除 */
export function getRoleDelete(data) {
  return axios({
    url: `${prefix}/role/delete`,
    method: 'delete',
    data
  })
}

/** 职务/岗位查询 */
export function getJobList(data) {
  return axios({
    url: `${prefix}/job/find`,
    method: 'post',
    data
  })
}

/** 职务/岗位保存 */
export function getJobSave(data) {
  let url = `${prefix}/job/save`
  if (data._id) {
    url = `${prefix}/job/update`
  }
  return axios({
    url,
    method: 'post',
    data
  })
}

/** 职务/岗位删除 */
export function getJobDelete(data) {
  return axios({
    url: `${prefix}/job/delete`,
    method: 'delete',
    data
  })
}

/** 机构保存 */
export function getOrgSave(data) {
  let url = `${prefix}/org/save`
  if (data._id) {
    url = `${prefix}/org/update`
  }
  return axios({
    url,
    method: 'post',
    data
  })
}

/** 机构查询 */
export function getOrgList(data) {
  return axios({
    url: `${prefix}/org/findAll`,
    // url: `${prefix}/org/findTree`,
    method: 'post',
    data
  })
}

/** 机构删除 */
export function getOrgDelete(data) {
  return axios({
    url: `${prefix}/org/delete`,
    method: 'delete',
    data
  })
}

/** 菜单查询 */
export function getMenuList(data) {
  return axios({
    url: `${prefix}/menu/findAll`,
    // url: `${prefix}/menu/findTree`,
    method: 'post',
    data
  })
}

/** 菜单保存 */
export function getMenuSave(data) {
  let url = `${prefix}/menu/save`
  if (data._id) {
    url = `${prefix}/menu/update`
  }
  return axios({
    url,
    method: 'post',
    data
  })
}

/** 菜单删除 */
export function getMenuDelete(data) {
  return axios({
    url: `${prefix}/menu/delete`,
    method: 'delete',
    data
  })
}

/** 用户权限保存 */
export function getauthSaveRoles(data) {
  return axios({
    url: `${prefix}/auth/save/roles`,
    method: 'post',
    data
  })
}

/** 用户权限 查询 */
export function getauthUserRoles(data) {
  return axios({
    url: `${prefix}/auth/find/roles`,
    method: 'post',
    data
  })
}

/** 产品查询 */
export function getProductList(data) {
  return axios({
    url: `${prefix}/portalProduct/find`,
    method: 'post',
    data
  })
}

/** 产品保存 */
export function getProductSave(data) {
  let url = `${prefix}/portalProduct/save`
  if (data._id) {
    url = `${prefix}/portalProduct/update`
  }
  return axios({
    url,
    method: 'post',
    data
  })
}

/** 产品删除 */
export function getProductDelete(data) {
  return axios({
    url: `${prefix}/portalProduct/delete`,
    method: 'delete',
    data
  })
}

/** 菜单权限查询 */
export function getAuthProdsMenu(data) {
  return axios({
    url: `${prefix}/auth/prods/menu/find`,
    method: 'post',
    data
  })
}

/** 菜单权限 保存 */
export function getAuthProdsMenuSave(data) {
  return axios({
    url: `${prefix}/auth/prods/menu/save`,
    method: 'post',
    data
  })
}

/** 菜单权限 路由查询 */
export function getAuthProdsMenusFind(data) {
  return axios({
    url: `${prefix}/auth/prods/menus`,
    method: 'post',
    data
  })
}

/** 按钮查询 */
export function getButtonFind(data) {
  return axios({
    url: `${prefix}/button/find`,
    method: 'post',
    data
  })
}


/** 文件上传 */
export function uploadFile(file) {
  return request(`${prefix}/annex/upload`, 'post', file).then(res => res.data)
}

/** 用户修改密码 */
export function userUpdatePassword(data) {
  return request(`${prefix}/user/updatepassword`, 'post', data)
}

/** 文件删除 */
export function deleteFile(data) {
  return request(`${prefix}/annex/deleteFile`, 'delete', data)
}

/** 文件查询 */
export function FilefindWith(data) {
  // return request(`${prefix}/annex/find`, 'post', data)
  return axios({
    url: `/annex/annex/find`,
    method: 'post',
    data
  })
}

/** 修改主题 */
export function UpdateTheme(data) {
  return axios({
    url: `${prefix}/theme/update`,
    method: 'post',
    data: {
      ...data,
      _id: $store.state.setting.theme._id
    }
  })
}

/** 主题查找 */
export function getThemeFind(data) {
  return axios({
    url: `${prefix}/theme/find`,
    method: 'post',
    data
  })
}
