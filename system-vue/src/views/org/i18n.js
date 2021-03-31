/*
 * @Description: 机构 国际化
 * @Version: 1.0
 * @Autor: JiaJun Wu
 * @Date: 2021-02-19 11:36:43
 * @LastEditors: JiaJun Wu
 * @LastEditTime: 2021-02-19 13:57:07
 */
module.exports = {
  messages: {
    CN: {
      name: {
        label: '机构名称',
        message: '请输入机构名称',
        placeholder: '请输入机构名称'
      },
      alias: {
        label: '机构别名',
        message: '请输入机构别名',
        placeholder: '请输入机构别名'
      },
      org_code: {
        label: '机构编码',
        message: '请输入机构编码',
        placeholder: '请输入机构编码'
      },
      types: {
        label: '机构类型',
        message: '请选择机构类型',
        placeholder: '请选择机构类型',
        options: [
          { value: 1, label: '企业' },
          { value: 2, label: '机构' },
          { value: 3, label: '部门' },
        ]
      },
      addr: {
        label: '机构地址',
        message: '请输入机构地址',
        placeholder: '请输入机构地址'
      },
      tel: {
        label: '机构电话',
        message: '请输入机构电话',
        placeholder: '请输入机构电话'
      },
      ordered: {
        label: '序号',
        message: '请输入序号',
        placeholder: '请输入序号... 最大值999'
      },
      desc: {
        label: '描述',
        message: '请输入描述',
        placeholder: '请输入描述'
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
        label: '机构名称',
        message: '请输入机构名称',
        placeholder: '请输入机构名称'
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
        label: '机构名称',
        message: '请输入机构名称',
        placeholder: '请输入机构名称'
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
