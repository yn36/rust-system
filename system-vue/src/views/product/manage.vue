<!--
 * @Description: 门户产品
 * @Version: 1.0
 * @Autor: JiaJun Wu
 * @Date: 2021-02-05 13:37:20
 * @LastEditors: JiaJun Wu
 * @LastEditTime: 2021-02-24 10:25:39
-->
<!-- 门户产品 -->
<template>
  <div class="job-manage">
    <div v-if="showType == 0">
      <advanced-search
        :config="searchConfig"
        @onSearch="onSearch"
        @onAdd="showItem(null, 1)"
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
              <a>{{ $t("empty") }}</a>
            </span>
            &nbsp;&nbsp;
            <span v-if="selectedRowKeys.length > 0">
              <a-popconfirm
                :title="$t('delete.titles')"
                :ok-text="$t('delete.yes')"
                :cancel-text="$t('delete.cancel')"
                @confirm="doDelete(null)"
              >
                <a href="#" class="yn-operation-btn">
                  {{ $t("delete.title") }}
                </a>
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
            <router-link
              class="yn-operation-btn"
              :to="{ path: '/menu/manage', query: { prod_id: record._id } }"
            >
              {{ $t("menu") }}
            </router-link>

            <a class="yn-operation-btn" @click="showItem(record, 2)">
              {{ $t("edit") }}
            </a>
            <a-popconfirm
              :title="$t('delete.titles')"
              :ok-text="$t('delete.yes')"
              :cancel-text="$t('delete.cancel')"
              @confirm="doDelete(record)"
            >
              <a href="#" class="yn-operation-btn">
                {{ $t("delete.title") }}
              </a>
            </a-popconfirm>
          </template>
        </a-table>
      </a-card>
    </div>
    <a-card v-if="showType == 1">
      <advanced-form
        :config="formConfig"
        @goBack="Return_btn"
        @onSave="onSave"
        :formData="formData"
        :formType="type"
      />
    </a-card>
  </div>
</template>

<script>
import BasicMixins from "@/utils/basic-mixins";
import {
  getProductList,
  getProductSave,
  getProductDelete,
} from "@/api/system-serve";

export default {
  i18n: require("./i18n"),
  mixins: [BasicMixins],
  data() {
    return {};
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
          title: this.$t("codes"),
          dataIndex: "codes",
          align: "center",
        },
        {
          title: this.$t("vers_no"),
          dataIndex: "vers_no",
          align: "center",
        },
        {
          title: this.$t("operation"),
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
          name: this.$t("codes"),
          key: "codes",
          searchFilterType: "string",
          defaultValue: "",
          placeholder: this.$t("codes"),
          rules: [{ required: false, message: "请选择/请输入" }],
          colSpan: 8,
          labelCol: 6,
        },
      ];
    },
    formConfig() {
      return [
        {
          type: "a-input",
          name: this.$t("name"),
          key: "name",
          searchFilterType: "string",
          defaultValue: "",
          placeholder: this.$t("name"),
          rules: [{ required: true, message: "请选择/请输入" }],
          colSpan: 24,
          labelCol: 4,
        },
        {
          type: "a-input",
          name: this.$t("codes"),
          key: "codes",
          searchFilterType: "string",
          defaultValue: "",
          placeholder: this.$t("codes"),
          rules: [{ required: true, message: "请选择/请输入" }],
          colSpan: 24,
          labelCol: 4,
        },
        {
          type: "a-input",
          name: this.$t("welcome"),
          key: "welcome",
          searchFilterType: "string",
          defaultValue: "",
          placeholder: this.$t("welcome"),
          rules: [{ required: false, message: "请选择/请输入" }],
          colSpan: 24,
          labelCol: 4,
        },
        {
          type: "a-input",
          name: this.$t("vers_no"),
          key: "vers_no",
          searchFilterType: "string",
          defaultValue: "",
          placeholder: this.$t("vers_no"),
          rules: [{ required: false, message: "请选择/请输入" }],
          colSpan: 24,
          labelCol: 4,
        },
        {
          type: "a-textarea",
          name: this.$t("vers_desc"),
          key: "vers_desc",
          searchFilterType: "string",
          defaultValue: "",
          placeholder: this.$t("vers_desc"),
          rules: [{ required: false, message: "请选择/请输入" }],
          colSpan: 24,
          labelCol: 4,
        },
        {
          type: "a-textarea",
          name: this.$t("prod_desc"),
          key: "prod_desc",
          searchFilterType: "string",
          defaultValue: "",
          placeholder: this.$t("prod_desc"),
          rules: [{ required: false, message: "请选择/请输入" }],
          colSpan: 24,
          labelCol: 4,
        },
        {
          type: "a-textarea",
          name: this.$t("sfc_desc"),
          key: "sfc_desc",
          searchFilterType: "string",
          defaultValue: "",
          placeholder: this.$t("sfc_desc"),
          rules: [{ required: false, message: "请选择/请输入" }],
          colSpan: 24,
          labelCol: 4,
        },
        {
          type: "a-textarea",
          name: this.$t("inds_desc"),
          key: "inds_desc",
          searchFilterType: "string",
          defaultValue: "",
          placeholder: this.$t("inds_desc"),
          rules: [{ required: false, message: "请选择/请输入" }],
          colSpan: 24,
          labelCol: 4,
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
      let res = await getProductList({
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

    /**
     * @description: 新增、编辑和查看
     * @param {object} record 数据
     * @param {number} type 1:添加 2:编辑 3:查看
     */
    showItem(record, type) {
      this.formData = {};
      this.showType = 1;
      this.type = type;
      if (record && type != 1) {
        this.formData = record;
      }
    },

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
      let data = await getProductDelete({ ids: list.toString() }).then(
        (res) => res.data
      );
      if (data.success) {
        this.doQuery();
        this.selectedRowKeys = [];
        this.$message.success(data.message);
      } else {
        this.$message.error(data.message);
      }
    },

    /** 提交 */
    async onSave(param) {
      let data = await getProductSave({ ...param }).then((res) => res.data);
      if (data.success) {
        this.showType = 0;
        this.doQuery();
        this.$message.success(data.message);
      } else {
        this.$message.error(data.message);
      }
    },
  },
};
</script>
<style lang='less' scoped>
</style>