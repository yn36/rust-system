<!--
 * @Description: 菜单按钮列表
 * @Version: 1.0
 * @Autor: JiaJun Wu
 * @Date: 2021-03-05 16:35:27
 * @LastEditors: JiaJun Wu
 * @LastEditTime: 2021-03-05 17:58:00
-->
<!-- 菜单按钮列表 -->
<template>
  <div class="btn-manage">
    <advanced-search
      :config="searchConfig"
      @onSearch="onSearch"
      @onAdd="showItem(null, 1)"
      @onHandleReset="emptyBtn"
      extra_title="返回菜单"
      @extra="
        () => {
          this.$emit('onExtra');
        }
      "
    />
    <a-card class="yn-table">
      <a-alert type="info">
        <div slot="message" class="yn-alert-message">
          <span>
            已选择
            <a>{{ selectedRowKeys.length }}</a>
            项
          </span>
          &nbsp;&nbsp;
          <span v-if="selectedRowKeys.length > 0" @click="tableEmpty">
            <a>清空</a>
          </span>
          &nbsp;&nbsp;
          <span v-if="selectedRowKeys.length > 0">
            <a-popconfirm
              title="你确定删除这些数据吗?"
              ok-text="是"
              cancel-text="否"
              @confirm="doDelete(null)"
            >
              <a href="#" class="yn-operation-btn">删除</a>
            </a-popconfirm>
          </span>
        </div>
      </a-alert>
      <a-table
        :columns="columns"
        :data-source="listData"
        :pagination="pagination"
        :loading="loading"
        bordered
        @change="handleTableChange"
        :rowKey="(record, index) => record._id"
        :row-selection="{
          selectedRowKeys: selectedRowKeys,
          onChange: onSelectChange,
        }"
      >
        <template slot="operation" slot-scope="text, record">
          <a class="yn-operation-btn" @click="showItem(record, 2)">编辑</a>
          <a-popconfirm
            title="你确定删除吗?"
            ok-text="是"
            cancel-text="否"
            @confirm="doDelete(record)"
          >
            <a href="#" class="yn-operation-btn">删除</a>
          </a-popconfirm>
        </template>
      </a-table>
    </a-card>
  </div>
</template>

<script>
import BasicMixins from "@/utils/basic-mixins";
import { getButtonFind } from "@/api/system-serve";

export default {
  props: {
    menu_id: {
      type: String,
      required: true,
    },
  },
  mixins: [BasicMixins],
  components: {},
  i18n: require("./btn-i18n"),
  data() {
    return {};
  },
  computed: {
    searchConfig() {
      return [
        {
          type: "a-input",
          name: this.$t("name.label"),
          key: "name",
          searchFilterType: "string",
          defaultValue: "",
          placeholder: this.$t("name.label"),
          rules: [{ required: false, message: "请选择/请输入" }],
          colSpan: 8,
          labelCol: 6,
        },
        // {
        //   type: "a-select",
        //   name: this.$t("enabled.title"),
        //   key: "enabled",
        //   searchFilterType: "string",
        //   defaultValue: "",
        //   placeholder: this.$t("enabled.title"),
        //   options: this.$t("enabled.options"),
        //   rules: [{ required: false, message: "请选择/请输入" }],
        //   colSpan: 8,
        //   labelCol: 6,
        // },
      ];
    },
    columns() {
      return [
        {
          title: this.$t("index"),
          dataIndex: "index",
          align: "center",
          width: 70,
        },
        {
          title: this.$t("name.label"),
          dataIndex: "name",
          align: "center",
        },
        {
          title: this.$t("title.label"),
          dataIndex: "title",
          align: "center",
        },
        {
          title: this.$t("alias.label"),
          dataIndex: "alias",
          align: "center",
        },
        {
          title: this.$t("status.label"),
          dataIndex: "status",
          align: "center",
          customRender: (text) => {
            // 1.正常，2.禁用.
            let value = this.$t("status.options").find(
              (e) => e.value == parseInt(text)
            ).label;
            return value;
          },
        },
        {
          title: this.$t("operation"),
          dataIndex: "operation",
          scopedSlots: { customRender: "operation" },
          align: "center",
        },
      ];
    },
  },
  //创建完成 访问当前this实例
  created() {},
  //挂载完成 访问DOM元素
  mounted() {
    console.log(this.$route.query.prod_id);
    console.log(this.menu_id);
    this.doQuery();
  },
  //方法集合
  methods: {
    /** 按钮数据查询 */
    async doQuery() {
      let data = await getButtonFind({
        body: {
          prod_id: this.$route.query.prod_id,
          menu_id: this.menu_id,
          ...this.searchData,
        },
        page: this.pagination.current,
        limit: this.pagination.pageSize,
      }).then((res) => res.data);
      console.log(data, "data");
    },

    /**
     * @description: 新增、编辑和查看
     * @param {object} record 数据
     * @param {number} type 1:添加 2:编辑 3:查看
     */
    showItem(record, type) {},

    /** 删除 */
    doDelete(record) {
      let data = [];
      if (record) {
        data.push(record._id);
      } else {
        data = this.selectedRowKeys;
      }
      this.$nextTick(() => {
        this.toDelete(data);
      });
    },

    async toDelete(list) {
      // let data = await getUserDelete({ ids: list.toString() }).then(
      //   (res) => res.data
      // );
      // if (data.success) {
      //   this.doQuery();
      //   this.selectedRowKeys = [];
      //   this.$message.success(data.message);
      // } else {
      //   this.$message.error(data.message);
      // }
    },
  },
};
</script>
<style lang='less' scoped>
</style>