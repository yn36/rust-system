<!--
 * @Description: 用户角色 权限管理
 * @Version: 1.0
 * @Autor: JiaJun Wu
 * @Date: 2021-02-20 14:03:41
 * @LastEditors: JiaJun Wu
 * @LastEditTime: 2021-02-25 09:39:43
-->
<!-- 用户角色 权限管理 -->
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
        :row-selection="{
          selectedRowKeys: selectedRowKeys,
          onChange: onSelectChange,
        }"
      >
        <template slot="operation" slot-scope="text, record">
          <a
            class="yn-operation-btn"
            :title="$t('operation.role')"
            @click="showDrawer(record)"
          >
            <a-icon type="solution" class="doactionIcons" />
          </a>
          <router-link
            class="yn-operation-btn"
            :title="$t('operation.menu')"
            :to="{
              path: '/auth/menumanage',
              query: { user_id: record._id, user: record.realname },
            }"
          >
            <a-icon type="cluster" class="doactionIcons" />
          </router-link>
        </template>
      </a-table>
    </a-card>

    <!-- 用户分配 -->
    <a-drawer
      :title="`用户角色分配（${recordName}）`"
      placement="right"
      :width="720"
      @close="onClose"
      :visible="visible"
    >
      <div class="role-wrap">
        <a-alert message="已有角色列表" type="success" style="margin: 5px 0">
          <p slot="description">
            <a-tag
              v-for="(item, index) in hasRole"
              :key="index"
              closable
              @close.prevent="toLog(item)"
              >{{ item.name }}</a-tag
            >
            <span v-if="hasRole.length == 0">暂无数据</span>
          </p>
        </a-alert>
        <a-alert
          message="新增角色列表"
          type="info"
          class="mt-md"
          style="margin: 5px 0"
        >
          <p slot="description">
            <a-tag
              v-for="(item, index) in newTag"
              :key="index"
              closable
              @close.prevent="toNewLog(item)"
              >{{ item.name }}</a-tag
            >
            <span v-if="newTag.length == 0">暂无数据</span>
          </p>
        </a-alert>
        <a-alert
          message="删除角色列表"
          type="error"
          class="mt-md"
          style="margin: 5px 0"
        >
          <p slot="description">
            <a-tag
              v-for="(item, index) in deletesTag"
              :key="index"
              closable
              @close.prevent="toDelLog(item)"
              >{{ item.name }}</a-tag
            >
            <span v-if="deletesTag.length == 0">暂无数据</span>
          </p>
        </a-alert>
      </div>

      <a-divider />

      <div class="role">
        <div class="query-form">
          <a-form layout="inline" style="margin-bottom: 5px; line-height: 40px">
            <a-row :gutter="24">
              <a-form-item label="角色名">
                <a-input v-model="params.name" placeholder />
              </a-form-item>
              <a-form-item label="角色编码">
                <a-input v-model="params.role_code" placeholder />
              </a-form-item>

              <a-button
                style="margin-right: 10px"
                type="primary"
                @click="doQueryRole"
                >查询</a-button
              >
              <a-button @click="toClean">重置</a-button>
            </a-row>
          </a-form>
        </div>

        <a-alert type="info" :showIcon="true" class="query-batch-opt">
          <template slot="message">
            <span class="mr-sm">
              新增角色:
              <a class="selected">{{ selectedRoles.length }}</a>
            </span>
            <a-button
              style="margin-left: 10px"
              :disabled="selectedRoles.length < 1"
              @click="alltoAdd"
              >批量添加</a-button
            >
          </template>
        </a-alert>

        <div class="query-result mt-sm">
          <a-table
            v-if="Update"
            :row-selection="roleSelection"
            :rowKey="(record, index) => record._id"
            :columns="userColumns"
            :dataSource="roleDatas"
            :pagination="{ pageSize: 10 }"
            size="small"
            bordered
          >
            <span slot="doaction" slot-scope="doaction, record">
              <a
                href="javascript:;"
                :disabled="
                  hasRole.findIndex((e) => e.role_id === record._id) != -1 ||
                  newTag.findIndex((e) => e.role_id === record._id) != -1
                "
                @click="toAddRole(record)"
                >添加</a
              >
            </span>
          </a-table>
        </div>

        <a-row type="flex" justify="end" class="mt-lg">
          <a-col>
            <a-button
              type="primary"
              icon="save"
              :disabled="newTag.length == 0 && deletesTag.length == 0"
              @click="doSave()"
              >提交</a-button
            >
          </a-col>
        </a-row>
      </div>
    </a-drawer>
  </div>
</template>

<script>
import BasicMixins from "@/utils/basic-mixins";
import {
  getUserList,
  getRoleList,
  getauthSaveRoles,
  getauthUserRoles,
} from "@/api/system-serve";

export default {
  i18n: require("./i18n"),
  mixins: [BasicMixins],
  data() {
    return {
      visible: false,
      recordName: "",
      user_id: null,
      roleDatas: [], // 角色列表
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
          title: this.$t("org_id"),
          dataIndex: "org_name",
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
          type: "a-input",
          name: this.$t("realname"),
          key: "realname",
          searchFilterType: "string",
          defaultValue: "",
          placeholder: this.$t("realname"),
          rules: [{ required: false, message: "请选择/请输入" }],
          colSpan: 8,
          labelCol: 6,
        },
      ];
    },
    userColumns() {
      return [
        {
          title: "角色名",
          dataIndex: "name",
          align: "center",
        },
        {
          title: "角色编码",
          dataIndex: "role_code",
          align: "center",
        },
        {
          title: "操作",
          width: 120,
          align: "center",
          dataIndex: "doaction",
          scopedSlots: {
            customRender: "doaction",
          },
        },
      ];
    },

    // 用户分配多选
    roleSelection() {
      const { newTag, hasRole, deletesTag } = this;
      return {
        onChange: (selectedRoleKeys, selectedRoles) => {
          this.selectedRoles = selectedRoles;
          // this.selectedRoleKeys = selectedRoleKeys;
        },
        getCheckboxProps: (record) => {
          return {
            props: {
              disabled:
                hasRole.findIndex((e) => e.role_id === record._id) != -1 ||
                newTag.findIndex((e) => e.role_id === record._id) != -1 ||
                deletesTag.findIndex((e) => e.role_id === record._id) != -1,
              name: record.name,
            },
          };
        },
      };
    },
  },
  //挂载完成 访问DOM元素
  mounted() {
    this.doQuery();
    this.doQueryRole();
  },
  //方法集合
  methods: {
    /** 数据请求 */
    async doQuery() {
      let res = await getUserList({
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

    /** 查询角色 */
    async doQueryRole() {
      let data = await getRoleList({
        body: {
          ...this.params,
        },
      }).then((r) => r.data);
      if (data.success) {
        this.roleDatas = data.body;
      } else {
        this.$message.error(data.message);
      }
    },

    /** 角色列表条件查询重置 */
    toClean() {},

    /**
     * 展示用户分配抽屉
     */
    showDrawer(record) {
      console.log(record);
      this.user_id = record._id;
      // this.doQuery(2)
      this.recordName = record.realname;
      this.doRoles();
      this.visible = true;
    },

    getRoleArray(arr) {
      let temp = [];
      for (let item of arr) {
        console.log(item, "item");
        temp.push({
          _id: item.role_id,
          name: item.name,
        });
      }
      return temp;
    },

    /**
     * 提交
     */
    doSave() {
      const adds = this.getRoleArray(this.newTag);
      const deletes = this.getRoleArray(this.deletesTag);
      let userIds = [];
      userIds.push(this.user_id);
      getauthSaveRoles({
        user_id: userIds,
        adds: adds,
        deletes: deletes,
      })
        .then((result) => {
          let data = result.data;
          if (data.success) {
            this.doRoles(); // 加载最新已有角色
            this.newTag = [];
            this.deletesTag = [];
            this.$message.success("提交成功");
          } else {
            this.$message.warning(data.message);
          }
        })
        .catch((error) => {
          this.$message.warning("数据加载异常.." + error);
        });
    },

    /**
     *查询用户列表
     */
    doRoles() {
      getauthUserRoles({ body: { user_id: this.user_id } })
        .then((result) => {
          let data = result.data;
          if (data.success) {
            this.Update = false;
            this.hasRole = data.body;
            // this.hasRole = JSON.parse(
            //   JSON.stringify(this.hasRole).replace(/_id/g, "roleId")
            // );
            this.$nextTick(() => {
              this.Update = true;
            });
          } else {
            this.$message.warning("数据加载失败..");
          }
        })
        .catch((error) => {
          this.$message.warning("数据加载异常.." + error);
        });
    },

    /**
     * 已存在列表--删除标签
     */
    toLog(tag) {
      const arr = [];
      this.hasRole.forEach((r) => {
        if (r.role_id != tag.role_id) {
          arr.push(r);
        }
      });
      this.Update = false;
      this.hasRole = arr;
      this.deletesTag.push({
        name: tag.name,
        role_id: tag.role_id,
      });
      this.hasRecord = tag;
      this.$nextTick(() => {
        this.Update = true;
      });
    },

    /**
     * 增加标签
     */
    toAddRole(val) {
      let flag = false;
      // 已有列表是否存在该角色
      this.hasRole.map((item) => {
        if (item.role_id == val._id) {
          flag = true;
          this.$message.error("已有角色列表存在该角色");
        }
      });
      this.newTag.map((item) => {
        if (item.role_id == val._id) {
          flag = true;
          this.$message.error("新增角色列表存在该角色");
        }
      });
      // 删除列表是否存在该角色
      this.deletesTag.map((item, index) => {
        if (item.role_id == val._id) {
          flag = true;
          this.toDelLog(this.hasRecord);
          if (index == 0) {
            this.deletesTag.shift(); //删除并返回数组的第一个元素
            return this.deletesTag;
          } else if (index == length - 1) {
            this.deletesTag.pop(); //删除并返回数组的最后一个元素
            return this.deletesTag;
          } else {
            this.deletesTag.splice(index, 1); //删除下标为i的元素
            return this.deletesTag;
          }
        }
      });
      if (!flag) {
        this.Update = false;
        this.$set(this.newTag, this.newTag.length, {
          name: val.name,
          role_id: val._id,
        });
        this.newTag = JSON.parse(JSON.stringify(this.newTag));
        this.$nextTick(() => {
          this.Update = true;
          this.selectedRoles = [];
        });
      }
    },
    /**
     * 新增列表--删除标签
     */
    toNewLog(tag) {
      console.log("toNewLog");
      const arr = [];
      this.newTag.forEach((r) => {
        if (r.role_id != tag.role_id) {
          arr.push(r);
        }
      });
      this.Update = false;
      this.newTag = arr;
      this.$nextTick(() => {
        this.Update = true;
      });
    },

    /**
     * 删除列表--删除标签
     */
    toDelLog(tag) {
      const arr = [];
      this.deletesTag.forEach((r) => {
        if (r.role_id != tag.role_id) {
          arr.push(r);
        }
      });
      this.Update = false;
      this.deletesTag = arr;
      this.hasRole.push({
        name: tag.name,
        role_id: tag.role_id,
      });
      this.$nextTick(() => {
        this.Update = true;
      });
    },

    // 批量添加
    alltoAdd() {
      this.Update = false;
      this.selectedRoles.map((v) => {
        this.$set(this.newTag, this.newTag.length, {
          name: v.name,
          role_id: v._id,
        });
        this.newTag = JSON.parse(JSON.stringify(this.newTag));
      });
      this.$nextTick(() => {
        this.Update = true;
        this.selectedRoles = [];
      });
    },

    /**
     * 隐藏用户分配抽屉
     */
    onClose() {
      this.visible = false;
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