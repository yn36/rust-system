<!--
 * @Description: 机构管理
 * @Version: 1.0
 * @Autor: JiaJun Wu
 * @Date: 2021-02-19 11:06:48
 * @LastEditors: JiaJun Wu
 * @LastEditTime: 2021-02-20 11:00:06
-->
<!-- 机构管理 -->
<template>
  <div class="org-manage">
    <div class="yn-org-tree beauty-scroll" style="width: 20%">
      <a-card class="tree" style="height: 100%">
        <a-tree
          showIcon
          :tree-data="treeData"
          @select="onSelect"
          :default-expanded-keys="['-1']"
        >
          <a-icon slot="folder-open" type="folder-open" />
          <a-icon slot="file" type="file" />
          <template slot="titleSlot" slot-scope="node">
            <span @mouseenter="showOperation(node)">{{ node.title }}</span>
            <span
              :style="{
                display: enterId === node._id ? 'inline-block' : 'none',
              }"
              class="tree-node-opts-org"
              style="margin-right: 10px"
            >
              <a-icon
                title="删除机构"
                type="minus-circle"
                v-if="node.key != '-1'"
                @click="toDelete(node, $event)"
              />
              <a-icon
                title="新增机构"
                style="margin-left: 5px"
                type="plus-circle"
                @click="toAdd(node, $event)"
              />
            </span>
          </template>
        </a-tree>
      </a-card>
      <div class="rigbr" @mousedown="onMousedown"></div>
    </div>
    <div class="yn-org-form" style="width: 80%">
      <a-card style="height: 100%">
        <a-spin :spinning="spinning">
          <a-form :form="form" @submit="handleSubmit">
            <a-form-item
              :label="$t('name.label')"
              :label-col="{ span: 5 }"
              :wrapper-col="{ span: 16 }"
            >
              <a-input
                :maxLength="15"
                :disabled="disabled"
                v-decorator="[
                  'name',
                  { rules: [{ required: true, message: $t('name.message') }] },
                ]"
                :placeholder="$t('name.placeholder')"
              />
            </a-form-item>
            <a-form-item
              :label="$t('alias.label')"
              :label-col="{ span: 5 }"
              :wrapper-col="{ span: 16 }"
            >
              <a-input
                :maxLength="15"
                :disabled="disabled"
                v-decorator="['alias']"
                :placeholder="$t('alias.placeholder')"
              />
            </a-form-item>
            <a-form-item
              :label="$t('org_code.label')"
              :label-col="{ span: 5 }"
              :wrapper-col="{ span: 16 }"
            >
              <a-input
                :maxLength="15"
                :disabled="disabled"
                v-decorator="[
                  'org_code',
                  {
                    rules: [
                      { required: true, message: $t('org_code.message') },
                    ],
                  },
                ]"
                :placeholder="$t('org_code.placeholder')"
              />
            </a-form-item>
            <a-form-item
              :label="$t('types.label')"
              :label-col="{ span: 5 }"
              :wrapper-col="{ span: 16 }"
            >
              <a-select
                :disabled="disabled"
                v-decorator="[
                  'types',
                  {
                    rules: [{ required: true, message: $t('types.message') }],
                  },
                ]"
                :options="$t('types.options')"
                :placeholder="$t('types.placeholder')"
              />
            </a-form-item>
            <a-form-item
              :label="$t('addr.label')"
              :label-col="{ span: 5 }"
              :wrapper-col="{ span: 16 }"
            >
              <a-input
                :maxLength="15"
                :disabled="disabled"
                v-decorator="[
                  'addr',
                  {
                    rules: [{ required: true, message: $t('addr.message') }],
                  },
                ]"
                :placeholder="$t('addr.placeholder')"
              />
            </a-form-item>
            <a-form-item
              :label="$t('tel.label')"
              :label-col="{ span: 5 }"
              :wrapper-col="{ span: 16 }"
            >
              <a-input
                :maxLength="15"
                :disabled="disabled"
                v-decorator="[
                  'tel',
                  {
                    rules: [{ required: true, message: $t('tel.message') }],
                  },
                ]"
                :placeholder="$t('tel.placeholder')"
              />
            </a-form-item>
            <a-form-item
              :label="$t('ordered.label')"
              :label-col="{ span: 5 }"
              :wrapper-col="{ span: 16 }"
            >
              <a-input-number
                style="width: 100%"
                :disabled="disabled"
                :min="1"
                :max="999"
                v-decorator="['ordered']"
                :placeholder="$t('ordered.placeholder')"
              />
            </a-form-item>
            <a-form-item
              :label="$t('desc.label')"
              :label-col="{ span: 5 }"
              :wrapper-col="{ span: 16 }"
            >
              <a-textarea
                :maxLength="500"
                :disabled="disabled"
                :rows="4"
                v-decorator="[
                  'desc',
                  {
                    rules: [{ required: false, message: $t('desc.message') }],
                  },
                ]"
                :placeholder="$t('desc.placeholder')"
              />
            </a-form-item>
            <div class="ant-row ant-form-item" style="text-align: center">
              <a-button
                type="primary"
                :disabled="disabled"
                icon="save"
                html-type="submit"
                >提交</a-button
              >
            </div>
          </a-form>
        </a-spin>
      </a-card>
    </div>
  </div>
</template>

<script>
import { getOrgSave, getOrgList, getOrgDelete } from "@/api/system-serve";
import { mapState } from "vuex";

export default {
  components: {},
  i18n: require("./i18n"),
  data() {
    return {
      form: this.$form.createForm(this), // 表单对象
      treeData: [
        {
          title: "机构管理",
          key: "-1",
          _id: "-1",
          isLeaf: false,
          scopedSlots: { title: "titleSlot" },
          children: [],
        },
      ],
      /** 机构数据 */
      formData: {},
      enterId: "",
      /** 父节点数据 */
      ParentForm: {},
      spinning: false,
    };
  },
  //创建完成 访问当前this实例
  created() {
    this.doQuery();
  },
  //挂载完成 访问DOM元素
  mounted() {
    /** 拖拽是禁止文字被选中 */
    document.body.ondrag = () => {
      return false;
    };
  },
  computed: {
    ...mapState("setting", ["collapsed"]),
    disabled() {
      return !this.formData._id && !this.formData.key;
    },
  },
  watch: {
    // collapsed: {
    //   deep: true,
    //   immediate: true,
    //   handler(v) {
    //     console.log(v);
    //     if (v) {
    //       if (document.querySelector(".yn-org-tree")) {
    //         let treeDom = document.querySelector(".yn-org-tree");
    //         let formDom = document.querySelector(".yn-org-form");
    //         let orgDom = document.querySelector(".org-manage");
    //         let treeWidth = treeDom.scrollWidth;
    //         console.log(orgDom.scrollWidth - treeWidth);
    //         formDom.style.width = `${orgDom.scrollWidth - treeWidth - 144}px`;
    //       }
    //     } else {
    //       if (document.querySelector(".yn-org-tree")) {
    //         let treeDom = document.querySelector(".yn-org-tree");
    //         let formDom = document.querySelector(".yn-org-form");
    //         let orgDom = document.querySelector(".org-manage");
    //         let treeWidth = treeDom.scrollWidth;
    //         formDom.style.width = `${orgDom.scrollWidth - treeWidth + 136}px`;
    //       }
    //     }
    //   },
    // },
  },
  //方法集合
  methods: {
    async doQuery() {
      let data = await getOrgList().then((res) => res.data);
      if (data.success) {
        let list = this.generateOptions(data.body);
        this.treeData[0].children = list;
      } else {
        this.$message.error(data.message);
      }
    },
    /** 初始化数据 构建树结构 */
    generateOptions(params) {
      //生成Cascader级联数据
      var result = [];
      for (let param of params) {
        if (param.parent_id == "-1" || !param.parent_id) {
          //判断是否为顶层节点
          var parent = {
            //转换成el-Cascader可以识别的数据结构
            ...param,
            label: param.name,
            value: param._id,
            title: param.name,
            scopedSlots: { title: "titleSlot" },
          };
          parent.children = this.getchilds(param._id, params); //获取子节点
          result.push(parent);
        }
      }
      return result;
    },

    getchilds(id, array) {
      let childs = new Array();
      for (let arr of array) {
        //循环获取子节点
        if (arr.parent_id == id) {
          childs.push({
            ...arr,
            label: arr.name,
            value: arr._id,
            title: arr.name,
            scopedSlots: { title: "titleSlot" },
          });
        }
      }
      for (let child of childs) {
        //获取子节点的子节点
        let childscopy = this.getchilds(child.value, array); //递归获取子节点
        if (childscopy.length > 0) {
          child.children = childscopy;
        }
      }
      return childs;
    },
    onSelect(selectedKeys, { node }) {
      if (selectedKeys[0] == "-1") {
        return false;
      }
      console.log(node.dataRef);
      this.formData = node.dataRef;
      this.form.setFieldsValue({
        name: node.dataRef.name,
        alias: node.dataRef.alias,
        org_code: node.dataRef.org_code,
        types: node.dataRef.types,
        addr: node.dataRef.addr,
        tel: node.dataRef.tel,
        ordered: node.dataRef.ordered,
        desc: node.dataRef.desc,
      });
    },

    /** 鼠标点击 */
    onMousedown(event) {
      let bDrag = true;
      // console.log(event, "event");
      /** 初始值 */
      let initClientX = event.clientX;
      let treeDom = document.querySelector(".yn-org-tree");
      let formDom = document.querySelector(".yn-org-form");
      let orgDom = document.querySelector(".org-manage");
      let treeWidth = treeDom.scrollWidth;

      // 开始移动
      document.onmousemove = (ev) => {
        if (!bDrag) {
          return false;
        }
        if (ev.clientX > initClientX) {
          let left = ev.clientX - initClientX;
          if (treeWidth + left < 850) {
            treeDom.style.width = `${treeWidth + left}px`;
            formDom.style.width = `${orgDom.scrollWidth - treeWidth - left}px`;
          }
        } else {
          let left = initClientX - ev.clientX;
          if (treeWidth - left > 100) {
            treeDom.style.width = `${treeWidth - left}px`;
            formDom.style.width = `${orgDom.scrollWidth - treeWidth + left}px`;
          }
        }
      };
      // 鼠标弹起
      document.onmouseup = (ev) => {
        if (!bDrag) return false;
        bDrag = false;
      };
      return false;
    },

    /**
     * 保存 doSave
     */
    handleSubmit(e) {
      e.preventDefault();
      this.form.validateFields((err, values) => {
        if (!err) {
          let data = {
            ...this.formData,
            ...values,
            parent_id: this.formData.parent_id
              ? this.formData.parent_id
              : this.ParentForm._id
              ? this.ParentForm._id
              : "-1",
          };
          this.spinning = true;
          getOrgSave(data)
            .then((r) => {
              let res = r.data;
              this.spinning = false;
              if (res.success) {
                this.$message.success(res.message);
                this.doQuery();
              } else {
                this.$message.error(res.message);
              }
            })
            .catch((e) => {
              this.spinning = false;
            });
        }
      });
    },
    /**
     * 删除记录
     */
    toDelete(node, event) {
      event.stopPropagation();
      this.$confirm({
        title: "机构操作",
        content: "确定要删除该机构么?",
        okType: "danger",
        onOk: () => {
          if (node.children && node.children.length > 0) {
            this.$message.warning("该机构无法直接删除, 请删除子机构");
            return false;
          }
          getOrgDelete({ ids: node._id }).then((r) => {
            let data = r.data;
            if (data.success) {
              this.doQuery();
              this.$message.success(data.message);
            } else {
              this.$message.error(data.message);
            }
          });
        },
      });
    },
    /**
     * 添加机构
     */
    toAdd(node, event) {
      event.stopPropagation();
      this.form.resetFields();
      console.log("add");
      console.log(node);
      this.ParentForm = node;
      this.formData = {
        key: Math.ceil(Math.random(10) * 100),
      };
      this.form.setFieldsValue({
        name: "新增机构",
      });
    },
    showOperation(record) {
      this.enterId = record._id;
    },
  },
};
</script>
<style lang='less' scoped>
.org-manage {
  width: 100%;
  height: 100%;
  overflow: hidden;

  .yn-org-tree {
    height: 900px;
    overflow-y: auto;
    float: left;

    .tree {
      width: calc(100% - 5px);
      float: left;
    }
    .rigbr {
      width: 5px;
      height: 100%;
      float: right;
      cursor: ew-resize;
      background: transparent;
    }

    .tree-node-opts-org {
      position: absolute;
      i {
        margin-left: 5px;
        color: rgba(0, 0, 0, 0.65);
      }
    }
  }

  .yn-org-form {
    width: calc(100% - 20%);
    height: 900px;
    overflow-y: auto;
    float: right;
  }
}
</style>