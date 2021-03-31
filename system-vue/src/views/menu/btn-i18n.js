/*
 * @Description: 按钮 国际化 
 * @Version: 1.0
 * @Autor: JiaJun Wu
 * @Date: 2021-03-05 17:16:32
 * @LastEditors: JiaJun Wu
 * @LastEditTime: 2021-03-05 17:33:20
 */

/// 名称 标题 别名 图标 状态 操作
module.exports = {
  messages: {
    CN: {
      index: '序号',
      name: {
        label: '按钮名称',
        message: '请输入按钮名称',
        placeholder: '请输入按钮名称'
      },
      title: {
        label: '按钮标题',
        message: '请输入按钮标题',
        placeholder: '请输入按钮标题'
      },
      alias: {
        label: '按钮别名',
        message: '请输入按钮别名',
        placeholder: '请输入按钮别名'
      },
      config: {
        label: '按钮配置信息',
        message: '请输入按钮配置信息',
        placeholder: '请输入按钮配置信息'
      },
      status: {
        label: '按钮状态',
        message: '请输入按钮状态',
        options: [
          { value: 1, label: '正常' },
          { value: 2, label: '禁用' },
        ],
        placeholder: '请输入按钮状态'
      },
      pic_min: {
        label: '按钮图标',
        message: '请输入按钮图标',
        placeholder: '请输入按钮图标'
      },
      btn_code: {
        label: '按钮编码',
        message: '请输入按钮编码',
        placeholder: '请输入按钮编码'
      },
      sort: {
        label: '排序',
        message: '排序',
        placeholder: '排序'
      },
      desc: {
        label: '描述',
        message: '描述',
        placeholder: '描述'
      },
      operation: '操作',
      empty: '清空',
      delete: {
        title: '删除机构',
        titles: '你确定删除这些数据吗?',
        yes: '是',
        cancel: '否'
      },
      edit: '编辑',
    },
    HK: {
      name: {
        label: '菜单标题',
        message: '请输入菜单标题/名称',
        placeholder: '请输入菜单标题/名称'
      },
      job_code: '编码',
      desc: '描述',
      index: '序号',
      operation: '操作',
      empty: '清空',
      delete: {
        title: '删除机构',
        titles: '你确定删除这些数据吗?',
        yes: '是',
        cancel: '否'
      },
      edit: '编辑',
    },
    US: {
      name: {
        label: '菜单标题',
        message: '请输入菜单标题/名称',
        placeholder: '请输入菜单标题/名称'
      },
      job_code: 'Code',
      desc: 'desc',
      index: 'sort',
      operation: 'operation',
      empty: 'empty',
      delete: {
        title: 'delete',
        titles: 'Are you sure you want to delete the data?',
        yes: 'yes',
        cancel: 'no'
      },
      edit: 'edit',
    }
  }
}
