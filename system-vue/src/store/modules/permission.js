/*
 * @Description: 菜单权限
 * @Version: 1.0
 * @Autor: JiaJun Wu
 * @Date: 2021-02-26 14:37:13
 * @LastEditors: JiaJun Wu
 * @LastEditTime: 2021-03-01 17:51:56
 */
import { getAuthProdsMenusFind } from '@/api/system-serve'

function setRedirect(routers) {
  const redirectFun = function (router) {
    if (router.children && router.children.length > 0) {
      return redirectFun(router.children[0])
    } else {
      return router.path
    }
  }
  return redirectFun(routers)
}

/**
 * 过滤账户是否拥有某一个权限，并将菜单从加载列表移除
 *
 * @param permission
 * @param route
 * @returns {boolean}
 */
function hasPermission(permission, route) {
  if (route.meta && route.meta.permission) {
    let flag = false
    for (let i = 0, len = permission.length; i < len; i++) {
      flag = route.meta.permission.includes(permission[i])
      if (flag) {
        return true
      }
    }
    return false
  }
  return true
}

function filterAsyncRouter(routerMap, roles) {
  const accessedRouters = routerMap.filter(route => {
    if (hasPermission(roles.permissionList, route)) {
      if (route.children && route.children.length) {
        route.children = filterAsyncRouter(route.children, roles)
      }
      return true
    }
    return false
  })
  return accessedRouters
}

export default {
  state: {
    routers: [],
    addRouters: []
  },
  mutations: {
    SET_ROUTERS: (state, routers) => {
      state.addRouters = routers
    }
  },
  actions: {
    async GenerateRoutes({ commit }, data) {
      const res = await getAuthProdsMenusFind({ prod_id: "6037581f0007219a00e4004f" }).then(res => res.data) //通过固定sid获取对应产品菜单
      if (res.success) {
        const menus = res.body
        let asyncRouter = generateOptions(menus)

        if (asyncRouter && asyncRouter.length > 0) {
          const childrens = asyncRouter.find(e => e.name === "system").children
          asyncRouter.find(e => e.name === "system").redirect = setRedirect(childrens[0])
          childrens.forEach(e => {
            if (e.path !== '/' && e.children && e.children.length > 0) {
              e.redirect = e.subMenu ? e.redirect : setRedirect(e.children[0])
            }
          })
        }
        asyncRouter.push(
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
          }) // 异常页面组件（定向）

        return new Promise(resolve => {
          const { roles } = data
          const accessedRouters = filterAsyncRouter(asyncRouter, roles)
          // commit('SET_ROUTERS', accessedRouters)
          resolve(accessedRouters)
        })
      } else {
        this.$message.error(res.message)
      }
    }
  }
}

/** 初始化数据 构建树结构 */
export function generateOptions(params) {
  //生成Cascader级联数据
  var result = [];
  for (let param of params) {
    if (param.parent_id == "-1" || !param.parent_id) {
      let meta = param.config ? JSON.parse(param.config) : {}

      let component = param.is_iframe != '1' ? param.component ? param.component : 'views/index' : 'components/iframePage/index'
      if (param.url == '/') {
        component = `layouts/tabs/TabsView`
      }
      //判断是否为顶层节点
      var parent = {
        // ...param,
        //转换成el-Cascader可以识别的数据结构
        path: param.url,
        _id: param._id,
        // desc: param.desc,
        // is_external: param.is_external,
        hidden: param.is_hidden,
        // is_iframe: param.is_iframe,
        name: param.name,
        // title: param.title,
        parent_id: param.parent_id,
        hideChildrenInMenu: meta.hideChildrenInMenu,
        sort: param.sort,
        hideHeader: meta.hideHeader,
        component: () => import(`@/${component}.vue`).catch(e => {
          console.error(`接收了一个无效的组件地址：'/${component}.vue'`)
          return import('@/views/exception/404')
        }),
        meta
      };
      parent.children = getchilds(param._id, params); //获取子节点
      parent.redirect = meta.redirect ? meta.redirect : parent.children ? parent.children[0].path : null
      result.push(parent);
    }
  }
  return result.sort((a, b) => {
    return a.sort - b.sort
  });
}

function getchilds(id, array) {
  let childs = new Array();
  for (let arr of array) {
    //循环获取子节点
    if (arr.parent_id == id) {
      let meta = arr.config ? JSON.parse(arr.config) : {}
      let component = arr.is_iframe != '1' ? arr.component ? arr.component : 'views/index' : 'components/iframePage/index'
      childs.push({
        // ...arr,
        path: arr.url,
        _id: arr._id,
        // desc: arr.desc,
        // is_external: arr.is_external,
        hidden: arr.is_hidden,
        // is_iframe: arr.is_iframe,
        name: arr.name,
        // title: arr.title,
        sort: arr.sort,
        parent_id: arr.parent_id,
        hideChildrenInMenu: meta.hideChildrenInMenu,
        hideHeader: meta.hideHeader,
        component: () => import(`@/${component}.vue`).catch(e => {
          console.error(`接收了一个无效的组件地址：'/${component}.vue'`)
          return import('@/views/exception/404')
        }),
        meta,
      });
    }
  }
  for (let child of childs) {
    //获取子节点的子节点
    let childscopy = getchilds(child._id, array); //递归获取子节点
    if (childscopy.length > 0) {
      child.children = childscopy;
      child.redirect = child.meta.redirect ? child.meta.redirect : child.children ? child.children[0].path : null
    }
  }
  return childs.sort((a, b) => {
    return a.sort - b.sort
  });
  // return childs;
}
