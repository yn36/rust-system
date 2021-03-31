<!--
 * @Description: 用户管理
 * @Version: 1.0
 * @Autor: JiaJun Wu
 * @Date: 2021-02-04 10:46:41
 * @LastEditors: JiaJun Wu
 * @LastEditTime: 2021-02-25 17:02:14
-->
<!-- 用户管理 -->
<template>
  <div class="user-manage">
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
  getUserList,
  getUserSave,
  getUserDelete,
  getOrgList,
} from "@/api/system-serve";
import moment from "moment";

export default {
  i18n: require("./i18n"),
  mixins: [BasicMixins],
  data() {
    return {
      orgList: [],
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
          title: this.$t("username"),
          dataIndex: "username",
          align: "center",
        },
        {
          title: this.$t("realname"),
          dataIndex: "realname",
          align: "center",
        },
        {
          title: this.$t("role"),
          dataIndex: "roles",
          align: "center",
          customRender: (text) => {
            let arr = [];
            text.map((v) => {
              arr.push(v.name);
            });
            return arr.toString();
          },
        },
        {
          title: this.$t("phone"),
          dataIndex: "phone",
          align: "center",
        },
        {
          title: this.$t("enabled.title"),
          dataIndex: "enabled",
          align: "center",
          customRender: (text) => {
            // 1.正常，2.禁用，3.注销
            let value = this.$t("enabled.options").find(
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
    searchConfig() {
      return [
        {
          type: "a-input",
          name: this.$t("username"),
          key: "username",
          searchFilterType: "string",
          defaultValue: "",
          placeholder: this.$t("username"),
          rules: [{ required: false, message: "请选择/请输入" }],
          colSpan: 8,
          labelCol: 6,
        },
        {
          type: "a-select",
          name: this.$t("enabled.title"),
          key: "enabled",
          searchFilterType: "string",
          defaultValue: "",
          placeholder: this.$t("enabled.title"),
          options: this.$t("enabled.options"),
          rules: [{ required: false, message: "请选择/请输入" }],
          colSpan: 8,
          labelCol: 6,
        },
      ];
    },
    formConfig: {
      get() {
        return [
          {
            type: "a-input",
            name: this.$t("username"),
            key: "username",
            searchFilterType: "string",
            defaultValue: "",
            placeholder: this.$t("username"),
            rules: [{ required: true, message: "请选择/请输入" }],
            disabled: this.type != 1,
            colSpan: 8,
            labelCol: 6,
          },
          {
            type: "a-input-password",
            name: this.$t("password"),
            key: "password",
            searchFilterType: "string",
            defaultValue: "",
            placeholder: this.$t("password"),
            rules: [{ required: true, message: "请选择/请输入" }],
            disabled: this.type != 1,
            isHide: this.type != 1,
            colSpan: 8,
            labelCol: 6,
          },
          {
            type: "a-input",
            name: this.$t("realname"),
            key: "realname",
            searchFilterType: "string",
            defaultValue: "",
            placeholder: this.$t("realname"),
            rules: [{ required: true, message: "请选择/请输入" }],
            colSpan: 8,
            labelCol: 6,
          },
          {
            type: "a-input",
            name: this.$t("phone"),
            key: "phone",
            searchFilterType: "string",
            defaultValue: "",
            placeholder: this.$t("phone"),
            rules: [{ required: true, message: "请选择/请输入" }],
            colSpan: 8,
            labelCol: 6,
          },
          {
            type: "a-select",
            name: this.$t("sex.title"),
            key: "sex",
            searchFilterType: "string",
            defaultValue: "",
            placeholder: this.$t("sex.title"),
            options: this.$t("sex.options"),
            rules: [{ required: true, message: "请选择/请输入" }],
            colSpan: 8,
            labelCol: 6,
          },
          {
            type: "a-date-picker",
            name: this.$t("birthday"),
            key: "birthday",
            searchFilterType: "string",
            defaultValue: "",
            placeholder: this.$t("birthday"),
            rules: [{ required: false, message: "请选择/请输入" }],
            colSpan: 8,
            labelCol: 6,
          },
          {
            type: "a-select",
            name: this.$t("org_id"),
            key: "org_id",
            searchFilterType: "string",
            defaultValue: "",
            placeholder: this.$t("org_id"),
            options: this.orgList,
            rules: [{ required: true, message: "请选择/请输入" }],
            colSpan: 8,
            labelCol: 6,
          },
          {
            type: "a-input",
            name: this.$t("email"),
            key: "email",
            searchFilterType: "string",
            defaultValue: "",
            placeholder: this.$t("email"),
            rules: [{ required: false, message: "请选择/请输入" }],
            colSpan: 8,
            labelCol: 6,
          },
          {
            type: "a-select",
            name: this.$t("types.title"),
            key: "types",
            searchFilterType: "string",
            defaultValue: "",
            placeholder: this.$t("types.title"),
            options: this.$t("types.options"),
            rules: [{ required: true, message: "请选择/请输入" }],
            colSpan: 8,
            labelCol: 6,
          },
          {
            type: "a-select",
            name: this.$t("enabled.title"),
            key: "enabled",
            searchFilterType: "string",
            defaultValue: "",
            placeholder: this.$t("enabled.title"),
            options: this.$t("enabled.options"),
            rules: [{ required: true, message: "请选择/请输入" }],
            colSpan: 8,
            labelCol: 6,
          },
          {
            type: "a-textarea",
            name: this.$t("desc"),
            key: "desc",
            searchFilterType: "string",
            defaultValue: "",
            placeholder: this.$t("desc"),
            rules: [{ required: false, message: "请选择/请输入" }],
            colSpan: 24,
            labelCol: 2,
          },
        ];
      },
    },
  },
  //挂载完成 访问DOM元素
  mounted() {
    this.doQuery();
    this.doOrg();
  },
  //方法集合
  methods: {
    /** 机构/部门数据请求 */
    async doOrg() {
      let data = await getOrgList().then((res) => res.data);
      if (data.success) {
        data.body.map((v) => {
          Object.assign(v, {
            value: v._id,
            label: v.name,
          });
        });
        this.orgList = data.body;
      }
    },
    /** 数据请求 */
    async doQuery() {
      let res = await getUserList({
        body: {
          ...this.searchData,
        },
        page: this.pagination.current,
        limit: this.pagination.pageSize,
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
      this.formData = {
        types: 2,
      };
      this.showType = 1;
      this.type = type;
      if (record && type != 1) {
        this.formData = record;
        let birth_year = `${record.birth_year}-${
          record.birth_month > 9
            ? `${record.birth_month}`
            : `0${record.birth_month}`
        }-${
          record.birth_day > 9 ? `${record.birth_day}` : `0${record.birth_day}`
        }`;
        Object.assign(this.formData, {
          birthday: record.birth_year ? moment(`${birth_year} 00:00:00`) : null,
        });
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
      let data = await getUserDelete({ ids: list.toString() }).then(
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
      let org_name = this.orgList.find((e) => e._id === param.org_id)?.name;
      let data = await getUserSave({
        ...param,
        org_name,
        birth_year: param.birthday
          ? parseInt(moment(param.birthday).format("YYYY"))
          : undefined,
        birth_month: param.birthday
          ? parseInt(moment(param.birthday).format("MM"))
          : undefined,
        birth_day: param.birthday
          ? parseInt(moment(param.birthday).format("DD"))
          : undefined,
      }).then((res) => res.data);
      if (data.success) {
        this.showType = 0;
        this.doQuery();
        this.$message.success(data.message);
      }
    },
  },
};
</script>
<style lang='less' scoped>
</style>