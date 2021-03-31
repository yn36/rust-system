<!--
 * @Description: 产品菜单授权
 * @Version: 1.0
 * @Autor: JiaJun Wu
 * @Date: 2021-02-23 15:17:35
 * @LastEditors: JiaJun Wu
 * @LastEditTime: 2021-03-01 17:51:32
-->
<!-- 产品菜单授权 -->
<template>
  <a-card :bordered="false" :title="`菜单授权（${realName}）`">
    <template slot="extra">
      <a-button
        shape="circle"
        icon="rollback"
        class="mr-sm"
        @click="toBack"
      ></a-button>
      <a-button
        type="primary"
        shape="circle"
        icon="save"
        @click="doSave()"
      ></a-button>
    </template>

    <a-table
      v-if="listData.length"
      :defaultExpandAllRows="false"
      :defaultExpandedRowKeys="[listData[0]._id]"
      :rowKey="(record) => record._id"
      :rowSelection="{
        selectedRowKeys: selectedRowKeys,
        onSelectAll: onSelectAll,
        onSelect: onSelect,
      }"
      :columns="columns"
      :dataSource="listData"
      :pagination="false"
    >
    </a-table>
  </a-card>
</template>

<script>
import { getAuthProdsMenu, getAuthProdsMenuSave } from "@/api/system-serve";

export default {
  components: {},
  data() {
    return {
      realName: this.$route.query.user,
      /// 数据列表
      listData: [],
      dataMap: {}, //表格数据map
      selectedRows: [],
      selectedRowKeys: [],

      /** 查询参数 */
      searchData: {},
      /** 产品id 固定 系统管理 */
      prod_id: "6037581f0007219a00e4004f",
      role_id: this.$route.query.role_id,
      user_id: this.$route.query.user_id,
      orignData: [], //原始数据
    };
  },
  computed: {
    columns() {
      return [
        {
          title: "菜单名称",
          dataIndex: "title",
          align: "left",
        },
      ];
    },
  },
  //创建完成 访问当前this实例
  created() {
    //角色授权 or 用户授权
    this.searchData = this.user_id
      ? { prod_id: this.prod_id, user_id: [this.user_id] }
      : { prod_id: this.prod_id, role_id: [this.role_id] };
    this.doQuery();
  },
  //挂载完成 访问DOM元素
  mounted() {},
  //方法集合
  methods: {
    /** 查询表格数据 */
    async doQuery() {
      let result = await getAuthProdsMenu({ ...this.searchData }).then(
        (res) => res.data
      );
      if (result.success) {
        this.orignData = JSON.parse(JSON.stringify(result.body));
        this.listData = this.toTree(result.body);

        //选中列表赋值
        this.selectedRows = result.body.filter((item) => item.checked);
        this.selectedRowKeys = result.body
          .filter((item) => item.checked)
          .map((item) => item._id);
      } else {
        this.$message.error(result.message);
      }
    },
    //将数据处理成树形结构
    toTree(data) {
      // 删除 所有 children,以防止多次调用
      data.forEach((item) => {
        delete item.children;
      });
      //给dataMap赋值
      data.forEach((item) => {
        this.dataMap[item._id] = item;
      });

      var val = [];
      data.forEach((item) => {
        let parent = this.dataMap[item.parent_id];
        if (parent) {
          (parent.children || (parent.children = [])).push(item);
          parent.children.sort((a, b) => {
            return a.sort - b.sort;
          });
        } else {
          val.push(item);
        }
      });
      return val.sort((a, b) => {
        return a.sort - b.sort;
      });
      // return val;
    },
    //提交
    async doSave() {
      //处理数据
      let adds = [];
      let deletes = [];
      let updates = [];
      let selected = JSON.parse(JSON.stringify(this.selectedRows));
      let origin = JSON.parse(
        JSON.stringify(this.orignData.filter((item) => item.checked))
      );

      //1.当前选中，初始未选中的全部加入adds（menu + btn）
      let addRows = selected.filter((item) =>
        origin.every((itemX) => itemX._id !== item._id)
      );
      addRows.forEach((item) => {
        //加入menu
        adds.push({
          _id: item._id,
          name: item.name,
        });
      });
      //2.当前未选中，初始选中的全部加入deletes（menu + btn）
      let deleteRows = origin.filter((item) =>
        selected.every((itemX) => itemX._id !== item._id)
      );
      deleteRows.forEach((item) => {
        //加入menu
        deletes.push({
          _id: item._id,
          name: item.name,
        });
      });
      let result = await getAuthProdsMenuSave({
        user_id: this.user_id ? [this.user_id] : undefined,
        prod_id: this.prod_id,
        role_id: this.role_id ? [this.role_id] : undefined,
        adds,
        deletes,
      }).then((res) => res.data);
      if (result.success) {
        this.doQuery();
        this.$message.success("提交成功");
      } else {
        this.$message.warning(result.message);
      }
    },
    //单选
    onSelect(record, selected) {
      //递归获取当前节点的父节点和子节点
      let { parentKeys = [], parentList = [] } = this.getParentList(
        [],
        [],
        record.parent_id
      );
      let { childKeys = [], childList = [] } = this.getChildList(
        [],
        [],
        record._id
      );
      if (selected) {
        //父子节点以及当前节点全勾选，去除重复key
        this.selectedRowKeys = [
          ...this.selectedRowKeys,
          ...parentKeys,
          ...childKeys,
          record._id,
        ].filter((item, index, arr) => arr.indexOf(item) === index);
        this.selectedRows = [
          ...this.selectedRows,
          ...parentList,
          ...childList,
          record,
        ].filter((item, index, arr) => arr.indexOf(item) === index);
      } else {
        //子节点和当前节点取消勾选,父节点不取消
        this.selectedRowKeys = this.selectedRowKeys.filter(
          (item) => childKeys.indexOf(item) === -1 && item !== record._id
        );
        this.selectedRows = this.selectedRows.filter(
          (item) =>
            childList.every((itemY) => itemY._id !== item._id) &&
            item._id !== record._id
        );
        //将子节点和当前节点 操作按钮 和 可传递选择 清空,并将checked设置为false
        childList.forEach((child) => {
          child.checked = false;
        });
        record.checked = false;
      }
      this.changeProps(this.selectedRows, "checked", selected);
    },
    //全选
    onSelectAll(selected) {
      if (selected) {
        const data = this.listData;
        const keys = [];
        const rows = [];
        setVal(data, keys, rows);
        this.selectedRowKeys = keys;
        this.selectedRows = rows;
      } else {
        this.selectedRowKeys = [];
        this.selectedRows = [];
        //全部节点的操作按钮 和 可传递选择 清空(用dataMap可以避免递归)
        for (let key in this.dataMap) {
          if (!this.dataMap.hasOwnProperty(key)) continue;
          let item = this.dataMap[key];
          item.checked = false;
        }
      }
      //改变数据的checked值
      this.changeProps(this.selectedRows, "checked", selected);

      function setVal(list, keys, rows) {
        list.forEach((v) => {
          keys.push(v._id);
          rows.push(v);
          if (v.children) {
            setVal(v.children, keys, rows);
          }
        });
      }
    },
    //遍历改变对象的某个属性
    changeProps(data, key, value) {
      data.length &&
        data.forEach((item) => {
          item[key] = value;
        });
    },

    //递归获取父节点(父节点keys，父节点list,父节点sid)
    getParentList(parentKeys, parentList, pid) {
      if (pid === "-1") return {};
      let target = this.dataMap[pid];
      if (target) {
        parentKeys.push(pid);
        parentList.push(target);
        this.getParentList(parentKeys, parentList, target["parent_id"]);
      }
      return {
        parentKeys,
        parentList,
      };
    },

    //递归获取子节点(子节点keys，子节点list,当前节点(父节点)id)
    getChildList(childKeys, childList, _id) {
      let current = this.dataMap[_id];
      if (current && current.children && current.children.length) {
        current.children.forEach((item) => {
          childKeys.push(item._id);
          childList.push(item);
          this.getChildList(childKeys, childList, item._id);
        });
      }
      return {
        childKeys,
        childList,
      };
    },
    toBack() {
      this.$router.back(-1);
    },
  },
};
</script>
<style lang='less' scoped>
</style>