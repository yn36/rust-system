<!--
 * @Description: 角色 权限管理
 * @Version: 1.0
 * @Autor: JiaJun Wu
 * @Date: 2021-02-20 14:03:41
 * @LastEditors: JiaJun Wu
 * @LastEditTime: 2021-02-25 14:20:31
-->
<!-- 角色 权限管理 -->
<template>
  <div class="user-manage">
    <advanced-search
      :config="searchConfig"
      @onSearch="onSearch"
      :disabledAdd="true"
      @onHandleReset="emptyBtn"
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
      >
        <template slot="operation" slot-scope="text, record">
          <router-link
            class="yn-operation-btn"
            :title="$t('operation.menu')"
            :to="{
              path: '/auth/menumanage',
              query: { role_id: record._id, user: record.name },
            }"
          >
            <a-icon type="cluster" class="doactionIcons" />
          </router-link>
        </template>
      </a-table>
    </a-card>
  </div>
</template>

<script>
import BasicMixins from "@/utils/basic-mixins";
import { getRoleList } from "@/api/system-serve";

export default {
  i18n: require("./i18n"),
  mixins: [BasicMixins],
  data() {
    return {
      visible: false,
      recordName: "",
      user_id: null,
      hasRole: [], // 已有角色数组
      newTag: [], // 已有角色标签
      deletesTag: [], // 删除角色标签
      params: {}, // 查询参数
      Update: true,
      selectedRoles: [],
    };
  },
  computed: {
    columns() {
      return [
        {
          title: this.$t("index"),
          dataIndex: "index",
          align: "center",
          width: 70,
        },
        {
          title: this.$t("name"),
          dataIndex: "name",
          align: "center",
        },
        {
          title: this.$t("role_code"),
          dataIndex: "role_code",
          align: "center",
        },
        {
          title: this.$t("desc"),
          dataIndex: "desc",
          align: "center",
        },
        {
          title: this.$t("create_time"),
          dataIndex: "create_time",
          align: "center",
        },
        {
          title: this.$t("operation.title"),
          dataIndex: "operation",
          scopedSlots: { customRender: "operation" },
          align: "center",
        },
      ];
    },
    searchConfig() {
      return [
        {
          type: "a-input",
          name: this.$t("name"),
          key: "name",
          searchFilterType: "string",
          defaultValue: "",
          placeholder: this.$t("name"),
          rules: [{ required: false, message: "请选择/请输入" }],
          colSpan: 8,
          labelCol: 6,
        },
        {
          type: "a-input",
          name: this.$t("role_code"),
          key: "role_code",
          searchFilterType: "string",
          defaultValue: "",
          placeholder: this.$t("role_code"),
          rules: [{ required: false, message: "请选择/请输入" }],
          colSpan: 8,
          labelCol: 6,
        },
      ];
    },
  },
  //挂载完成 访问DOM元素
  mounted() {
    this.doQuery();
  },
  //方法集合
  methods: {
    /** 数据请求 */
    async doQuery() {
      let res = await getRoleList({
        body: {
          ...this.searchData,
        },
        page: this.pagination.current,
        page_size: this.pagination.pageSize,
      });
      if (res.data.success) {
        this.listData = res.data.body;
        this.listData.map((v, i) => {
          Object.assign(v, {
            index: this.pagination.current
              ? (this.pagination.current - 1) *
                  (this.pagination.pageSize ? this.pagination.pageSize : 10) +
                (i + 1)
              : i + 1,
          });
        });
        this.loading = false;
        this.pagination.total = parseInt(res.data.total);
      } else {
        this.$message.error(data.message);
      }
    },
  },
};
</script>
<style lang='less' scoped>
.doactionIcons {
  font-size: 20px;
  margin-left: 10px;
}
.query-result {
  margin-bottom: 20px;
  margin-top: 5px;
}
</style>