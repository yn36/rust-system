/*
 * @Description: 基础配置
 * @Version: 1.0
 * @Autor: JiaJun Wu
 * @Date: 2021-02-04 11:55:13
 * @LastEditors: jiajun wu
 * @LastEditTime: 2021-02-06 22:58:24
 */
import AdvancedSearch from "@/components/AdvancedSearch/index.vue";
import AdvancedForm from "@/components/advancedForm/index.vue";

export default {
  components: {
    AdvancedSearch,
    AdvancedForm
  },
  data() {
    return {
      pagination: {
        total: 0,
        defaultPageSize: 10,
        showTotal: total => `共 ${total} 条数据`,
        showSizeChanger: true,
        pageSizeOptions: ['10', '20', '40'],
        // eslint-disable-next-line no-return-assign
        onShowSizeChange: (current, pageSize) => (this.pageSize = pageSize)
      },
      loading: false,
      // 保存一个对象返回表单显示
      formData: {},
      // 保存搜索栏的数据
      searchData: {},
      // showType  0.显示表格  1.显示添加、修改、查看页面
      showType: 0,
      // type 按钮类型  1:添加 2:修改 3:查看
      type: null,
      // 存储数据
      listData: [],
      /** 添加弹窗视图 */
      modaleVisible: false,
      /** 列表已选 */
      selectedRowKeys: []
    }
  },
  methods: {
    doQuery() {
      console.log(this.searchData);
    },

    /** 默认查询功能 */
    onSearch(param) {
      this.searchData = param
      this.pagination.current = 1
      this.doQuery()
    },
    /**
     * @description: emptyBtn 清空搜索栏并查询
     * @author: jiajun wu
     */
    emptyBtn() {
      this.searchData = {}
      this.pagination.current = 1
      this.doQuery()
    },
    // 返回
    Return_btn() {
      this.showType = 0
      this.modaleVisible = false
    },
    // 表格下一页
    handleTableChange(pagination) {
      this.pagination = pagination
      this.doQuery()
    },
    /** 列表清空 */
    tableEmpty() {
      this.selectedRowKeys = []
    },
    onSelectChange(selectedRowKeys) {
      this.selectedRowKeys = selectedRowKeys;
    },
  }
}