<!--
 * @Description: 职务/岗位管理
 * @Version: 1.0
 * @Autor: JiaJun Wu
 * @Date: 2021-02-05 13:37:20
 * @LastEditors: JiaJun Wu
 * @LastEditTime: 2021-02-19 11:14:02
-->
<!-- 职务/岗位管理 -->
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
import { getJobList, getJobSave, getJobDelete } from "@/api/system-serve";

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
          dataIndex: "job_name",
          align: "center",
        },
        {
          title: this.$t("job_code"),
          dataIndex: "job_code",
          align: "center",
        },
        {
          title: this.$t("desc"),
          dataIndex: "job_desc",
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
          key: "job_name",
          searchFilterType: "string",
          defaultValue: "",
          placeholder: this.$t("name"),
          rules: [{ required: false, message: "请选择/请输入" }],
          colSpan: 8,
          labelCol: 6,
        },
        {
          type: "a-input",
          name: this.$t("job_code"),
          key: "job_code",
          searchFilterType: "string",
          defaultValue: "",
          placeholder: this.$t("job_code"),
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
          key: "job_name",
          searchFilterType: "string",
          defaultValue: "",
          placeholder: this.$t("name"),
          rules: [{ required: true, message: "请选择/请输入" }],
          colSpan: 24,
          labelCol: 4,
        },
        {
          type: "a-input",
          name: this.$t("job_code"),
          key: "job_code",
          searchFilterType: "string",
          defaultValue: "",
          placeholder: this.$t("job_code"),
          rules: [{ required: true, message: "请选择/请输入" }],
          colSpan: 24,
          labelCol: 4,
        },
        {
          type: "a-textarea",
          name: this.$t("desc"),
          key: "job_desc",
          searchFilterType: "string",
          defaultValue: "",
          placeholder: this.$t("desc"),
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
      let res = await getJobList({
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
      let data = await getJobDelete({ ids: list.toString() }).then(
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
      console.log(param, "param");
      let data = await getJobSave({ ...param }).then((res) => res.data);
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