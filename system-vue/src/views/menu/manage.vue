<!--
 * @Description: 菜单管理
 * @Version: 1.0
 * @Autor: JiaJun Wu
 * @Date: 2021-02-20 11:19:26
 * @LastEditors: JiaJun Wu
 * @LastEditTime: 2021-03-05 18:02:19
-->
<!-- 菜单管理 -->
<template>
  <div class="menu-manage">
    <div class="yn-menu-tree beauty-scroll" style="width: 22%">
      <a-card class="tree" style="height: 100%">
        <a-tree
          showLine
          :tree-data="treeData"
          @select="onSelect"
          :default-expanded-keys="['-1']"
        >
          <a-icon slot="folder-open" type="folder-open" />
          <a-icon slot="file" type="file" />
          <template slot="titleSlot" slot-scope="node">
            <span @mouseenter="showOperation(node)">{{ node.title }}</span>
            <span
              v-if="isShowMenu"
              :style="{
                display: enterId === node._id ? 'inline-block' : 'none',
              }"
              class="tree-node-opts-menu"
              style="margin-right: 10px"
            >
              <a-icon
                title="删除菜单"
                type="tool"
                v-if="node.key != '-1'"
                @click="toBtnMange(node, $event)"
              />
              <a-icon
                title="删除菜单"
                type="minus-circle"
                v-if="node.key != '-1'"
                @click="toDelete(node, $event)"
              />
              <a-icon
                title="新增菜单"
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
    <div class="yn-menu-form beauty-scroll" style="width: 78%">
      <a-card style="height: 100%">
        <a-spin :spinning="spinning" v-if="isShowMenu">
          <a-form :form="form" @submit="handleSubmit">
            <a-form-item
              :label="$t('title.label')"
              :label-col="{ span: 5 }"
              :wrapper-col="{ span: 16 }"
            >
              <a-input
                :maxLength="30"
                :disabled="disabled"
                v-decorator="[
                  'title',
                  { rules: [{ required: true, message: $t('title.message') }] },
                ]"
                :placeholder="$t('title.placeholder')"
              />
            </a-form-item>
            <a-form-item
              :label="$t('name.label')"
              :label-col="{ span: 5 }"
              :wrapper-col="{ span: 16 }"
            >
              <a-input
                :maxLength="30"
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
                :maxLength="30"
                :disabled="disabled"
                v-decorator="['alias']"
                :placeholder="$t('alias.placeholder')"
              />
            </a-form-item>
            <a-form-item
              :label="$t('url.label')"
              :label-col="{ span: 5 }"
              :wrapper-col="{ span: 16 }"
            >
              <a-input
                :disabled="disabled"
                v-decorator="[
                  'url',
                  {
                    rules: [{ required: true, message: $t('url.message') }],
                  },
                ]"
                :placeholder="$t('url.placeholder')"
              />
            </a-form-item>
            <a-form-item
              :label="$t('component.label')"
              :label-col="{ span: 5 }"
              :wrapper-col="{ span: 16 }"
            >
              <a-input
                :disabled="disabled"
                v-decorator="['component']"
                :placeholder="$t('component.placeholder')"
              />
            </a-form-item>
            <a-form-item
              :label="$t('config.label')"
              :label-col="{ span: 5 }"
              :wrapper-col="{ span: 16 }"
            >
              <vue-json-editor
                style="width: 100%"
                v-model="configs"
                :mode="'code'"
                :show-btns="false"
                :exapndedOnStart="false"
              />
            </a-form-item>
            <a-form-item
              :label="$t('sort.label')"
              :label-col="{ span: 5 }"
              :wrapper-col="{ span: 16 }"
            >
              <a-input-number
                style="width: 100%"
                :disabled="disabled"
                :min="1"
                :max="999"
                v-decorator="['sort']"
                :placeholder="$t('sort.placeholder')"
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
                v-decorator="['desc']"
                :placeholder="$t('desc.placeholder')"
              />
            </a-form-item>
            <a-form-item
              style="margin-bottom: 0"
              :label="$t('is_hidden.label')"
              :label-col="{ span: 5 }"
              :wrapper-col="{ span: 16 }"
            >
              <a-switch
                :disabled="disabled"
                v-decorator="['is_hidden', { valuePropName: 'checked' }]"
                :placeholder="$t('is_hidden.placeholder')"
              />
            </a-form-item>
            <a-form-item
              style="margin-bottom: 0"
              :label="$t('is_iframe.label')"
              :label-col="{ span: 5 }"
              :wrapper-col="{ span: 16 }"
            >
              <a-switch
                :disabled="disabled"
                v-decorator="['is_iframe', { valuePropName: 'checked' }]"
                :placeholder="$t('is_iframe.placeholder')"
              />
            </a-form-item>
            <a-form-item
              style="margin-bottom: 0"
              :label="$t('is_external.label')"
              :label-col="{ span: 5 }"
              :wrapper-col="{ span: 16 }"
            >
              <a-switch
                :disabled="disabled"
                v-decorator="['is_external', { valuePropName: 'checked' }]"
                :placeholder="$t('is_external.placeholder')"
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
        <btn-list
          v-if="isShowBtnList"
          :menu_id="formData._id"
          @onExtra="onExtra"
        ></btn-list>
      </a-card>
    </div>
  </div>
</template>

<script>
import { getMenuSave, getMenuList, getMenuDelete } from "@/api/system-serve";
import { mapState } from "vuex";
import vueJsonEditor from "vue-json-editor";
import BtnList from "./btnList.vue";

export default {
  components: { vueJsonEditor, BtnList },
  i18n: require("./i18n"),
  data() {
    return {
      form: this.$form.createForm(this), // 表单对象
      treeData: [
        {
          title: "菜单管理",
          key: "-1",
          _id: "-1",
          isLeaf: false,
          scopedSlots: { title: "titleSlot" },
          children: [],
        },
      ],
      /** 菜单数据 */
      formData: {},
      prod_id: this.$route.query.prod_id,
      enterId: "",
      /** 父节点数据 */
      ParentForm: {},
      spinning: false,
      configs: null,
      /** 是否显示菜单 */
      isShowMenu: true,
      /** 是否显示按钮列表 */
      isShowBtnList: false,
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
    //       if (document.querySelector(".yn-menu-tree")) {
    //         let treeDom = document.querySelector(".yn-menu-tree");
    //         let formDom = document.querySelector(".yn-menu-form");
    //         let orgDom = document.querySelector(".menu-manage");
    //         let treeWidth = treeDom.scrollWidth;
    //         console.log(orgDom.scrollWidth - treeWidth);
    //         formDom.style.width = `${orgDom.scrollWidth - treeWidth - 144}px`;
    //       }
    //     } else {
    //       if (document.querySelector(".yn-menu-tree")) {
    //         let treeDom = document.querySelector(".yn-menu-tree");
    //         let formDom = document.querySelector(".yn-menu-form");
    //         let orgDom = document.querySelector(".menu-manage");
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
      let data = await getMenuList({ body: { prod_id: this.prod_id } }).then(
        (res) => res.data
      );
      if (data.success) {
        let list = this.generateOptions(data.body);
        this.treeData[0].children = list;
      } else {
        this.$message.error(data.message);
      }
    },

    /** 查询按钮 */
    async doQueryBtnList() {},

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
            label: param.title,
            value: param._id,
            title: param.title,
            scopedSlots: { title: "titleSlot" },
          };
          parent.children = this.getchilds(param._id, params); //获取子节点
          result.push(parent);
        }
      }
      return result.sort((a, b) => {
        return a.sort - b.sort;
      });
      // return result;
    },

    getchilds(id, array) {
      let childs = new Array();
      for (let arr of array) {
        //循环获取子节点
        if (arr.parent_id == id) {
          childs.push({
            ...arr,
            label: arr.title,
            value: arr._id,
            title: arr.title,
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
      // return childs;
      return childs.sort((a, b) => {
        return a.sort - b.sort;
      });
    },
    onSelect(selectedKeys, { node }) {
      if (selectedKeys[0] == "-1") {
        return false;
      }
      // console.log(node.dataRef);
      this.configs = null;
      this.formData = node.dataRef;
      if (node.dataRef.config) {
        this.configs = JSON.parse(node.dataRef.config);
      } else {
        this.configs = null;
      }
      this.form.setFieldsValue({
        title: node.dataRef.title,
        name: node.dataRef.name,
        alias: node.dataRef.alias,
        url: node.dataRef.url,
        component: node.dataRef.component,
        sort: node.dataRef.sort,
        is_hidden: node.dataRef.is_hidden,
        is_iframe: node.dataRef.is_iframe,
        is_external: node.dataRef.is_external,
        desc: node.dataRef.desc,
      });
    },

    /** 鼠标点击 */
    onMousedown(event) {
      let bDrag = true;
      // console.log(event, "event");
      /** 初始值 */
      let initClientX = event.clientX;
      let treeDom = document.querySelector(".yn-menu-tree");
      let formDom = document.querySelector(".yn-menu-form");
      let orgDom = document.querySelector(".menu-manage");
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
          let configs = "{}";
          if (this.configs) {
            configs = JSON.stringify(this.configs);
            // if (configs == "{}") {
            //   this.$message.error("请输入正确的路由配置信息");
            //   return false;
            // }
          }

          let data = {
            ...this.formData,
            ...values,
            parent_id: this.formData.parent_id
              ? this.formData.parent_id
              : this.ParentForm._id
              ? this.ParentForm._id
              : "-1",
            config: configs,
            prod_id: this.prod_id,
          };

          this.spinning = true;
          getMenuSave(data)
            .then((r) => {
              let res = r.data;
              this.spinning = false;
              if (res.success) {
                this.$message.success(res.message);
                this.doQuery();
                this.formData = res.body;
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
     * 打开按钮列表
     */
    toBtnMange(node, event) {
      event.stopPropagation();
      this.isShowMenu = false;
      this.isShowBtnList = true;
      this.formData = node;
    },

    /** 返回菜单 */
    onExtra() {
      this.isShowMenu = true;
      this.isShowBtnList = false;
      this.formData = {};
      this.configs = null;
    },

    /**
     * 删除记录
     */
    toDelete(node, event) {
      event.stopPropagation();
      this.$confirm({
        title: "菜单操作",
        content: "确定要删除该菜单么?",
        okType: "danger",
        onOk: () => {
          if (node.children && node.children.length > 0) {
            this.$message.warning("该菜单无法直接删除, 请删除子菜单");
            return false;
          }
          getMenuDelete({ ids: node._id }).then((r) => {
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
     * 添加菜单
     */
    toAdd(node, event) {
      event.stopPropagation();
      this.form.resetFields();
      this.ParentForm = node;
      this.formData = {
        key: Math.ceil(Math.random(10) * 100),
      };
      this.form.setFieldsValue({
        title: "新增菜单",
      });
    },
    showOperation(record) {
      this.enterId = record._id;
    },
  },
};
</script>
<style lang='less' scoped>
.menu-manage {
  width: 100%;
  height: 100%;
  overflow: hidden;

  .yn-menu-tree {
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

    .tree-node-opts-menu {
      position: absolute;
      i {
        margin-left: 5px;
        color: rgba(0, 0, 0, 0.65);
      }
    }
  }

  .yn-menu-form {
    width: calc(100% - 20%);
    height: 900px;
    overflow-y: auto;
    float: right;
  }
}
</style>