/*
 * @Description: 用户管理 国际化
 * @Version: 1.0
 * @Autor: JiaJun Wu
 * @Date: 2021-02-04 16:22:33
 * @LastEditors: JiaJun Wu
 * @LastEditTime: 2021-02-20 14:18:04
 */
module.exports = {
  messages: {
    CN: {
      index: '序号',
      username: '用户账号',
      realname: '用户姓名',
      role: '角色',
      phone: '联系电话',
      enabled: {
        title: '状态',
        options: [{ value: 1, label: '正常' }, { value: 2, label: '禁用' }, { value: 3, label: '注销' }]
      },
      password: "密码",
      sex: {
        title: '性别',
        options: [{ value: 1, label: '男' }, { value: 0, label: '女' }, { value: 3, label: '保密' }]
      },
      birthday: '生日',
      org_id: '所在部门',
      types: {
        title: '用户类型',
        options: [{ value: 1, label: '管理员' }, { value: 2, label: '普通用户' }, { value: 3, label: '其他' }]
      },
      desc: '描述/备注',
      qq: '联系QQ',
      email: '电子邮箱',
      operation: {
        title: '操作',
        role: '角色分配',
        menu: '菜单分配'
      }
    },
    HK: {
      index: '序號',
      username: "帳號",
      realname: '用戶姓名',
      role: '角色',
      phone: '聯繫電話',
      enabled: {
        title: '狀態',
        options: [{ value: 1, label: '正常' }, { value: 2, label: '禁用' }, { value: 3, label: '註銷' }]
      },
      password: "密碼",
      sex: {
        title: '性別',
        options: [{ value: 1, label: '男' }, { value: 0, label: '女' }, { value: 3, label: '保密' }]
      },
      birthday: '生日',
      org_id: '所在部門',
      types: {
        title: '用戶類型',
        options: [{ value: 1, label: '管理員' }, { value: 2, label: '普通用戶' }, { value: 3, label: '其他' }]
      },
      desc: '描述/備註',
      qq: '聯繫QQ',
      email: '電子郵箱',
      operation: '操作'
    },
    US: {
      index: 'sort',
      username: 'User account',
      realname: 'User Name',
      role: 'roles',
      phone: 'phone',
      enabled: {
        title: 'state',
        options: [{ value: 1, label: 'normal' }, { value: 2, label: 'Disable' }, { value: 3, label: 'Logout' }]
      },
      password: "password",
      sex: {
        title: 'gender',
        options: [{ value: 1, label: 'male' }, { value: 0, label: 'Female' }, { value: 3, label: 'Keep secret' }]
      },
      birthday: 'birthday',
      org_id: 'Department',
      types: {
        title: 'User Type',
        options: [{ value: 1, label: 'administrator' }, { value: 2, label: 'general user' }, { value: 3, label: 'other' }]
      },
      desc: 'Description/Remarks',
      qq: 'Contact QQ',
      email: 'E-mail',
      operation: 'operation'
    }
  }
}
