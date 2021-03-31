<!--
 * @Description: 高级搜索组件
 * @Version: 1.0
 * @Autor: JiaJun Wu
 * @Date: 2021-02-04 10:50:23
 * @LastEditors: JiaJun Wu
 * @LastEditTime: 2021-03-05 18:00:13
-->
<!-- 高级搜索组件 -->
<template>
  <a-card :title="$t('form_title')">
    <a-button
      type="link"
      slot="extra"
      v-if="extra_title"
      @click="
        () => {
          this.$emit('extra');
        }
      "
    >
      {{ extra_title }}
    </a-button>
    <a-form
      class="ant-advanced-search-form"
      :form="form"
      @submit="handleSubmit"
    >
      <a-row :gutter="[10, 10]">
        <template v-for="(item, index) in config">
          <a-col
            :key="item.key"
            :span="item.colSpan"
            :offset="item.offset"
            :pull="item.offset"
            v-if="!item.isHidden && index < count"
          >
            <a-form-item
              :label-col="{
                xs: { span: 24 },
                sm: { span: 24 },
                md: { span: item.labelCol || 6 },
              }"
              :wrapper-col="{
                xs: { span: 24 },
                sm: { span: 24 },
                md: { span: 24 - item.labelCol || 17 },
              }"
              :label="item.name"
            >
              <component
                :is="item.type"
                v-decorator="[
                  `${item.key}`,
                  {
                    rules: [...item.rules],
                  },
                ]"
                :placeholder="item.placeholder"
                :style="{ width: item.width ? item.width : '100%' }"
                :options="item.options ? item.options : null"
                :format="item.format"
                :showTime="item.showTime"
                :max="item.type === 'a-input-number' ? item.max : null"
                :allowClear="item.allowClear || true"
                :showSearch="item.showSearch || false"
              />
            </a-form-item>
          </a-col>
        </template>
      </a-row>
      <a-row>
        <a-col
          :style="{
            marginTop: ' 5px',
            paddingRight: '10px',
            textAlign: 'right',
            ...btnStyle,
          }"
        >
          <a-button
            style="margin-right: 8px"
            type="primary"
            html-type="submit"
            icon="search"
            >{{ $t("search") }}</a-button
          >
          <a-button
            style="margin-right: 8px"
            @click="handleReset"
            icon="delete"
            >{{ $t("reset") }}</a-button
          >
          <a-button
            style="margin-right: 8px"
            v-if="!disabledAdd"
            @click="
              () => {
                this.$emit('onAdd');
              }
            "
            icon="plus"
            >{{ AddbtnName || $t("add") }}</a-button
          >
          <slot name="btn" />
          <a
            v-if="
              (limit && config.length > limit) || (!limit && config.length > 6)
            "
            :style="{ marginLeft: '8px', fontSize: '12px' }"
            @click="toggle"
          >
            {{ expand ? $t("up") : $t("down") }}
            <a-icon :type="expand ? 'up' : 'down'" />
          </a>
        </a-col>
      </a-row>
    </a-form>
  </a-card>
</template>

<script>
/**
 * @description: 高级搜索表单
 * @param {type, name, key, searchFilterType, defaultValue, placeholder, rules, options} config设置
 * @return {key}
 *
 * # Example
 *
 * config:[
 *   {
 *      type: "a-input",
 *      name: "用户名",
 *      key: "username",
 *      searchFilterType: "string",
 *      defaultValue: "",
 *      placeholder: "用户名",
 *      rules: [{ required: false, message: "请选择/请输入" }],
 *      colSpan: 8,
 *      labelCol: 6,
 *   },
 *   {
 *      type: "a-select",
 *      name: "用户名状态",
 *      key: "type",
 *      searchFilterType: "string",
 *      defaultValue: "",
 *      placeholder: "用户名状态",
 *      options:[
 *          {value:1,label:'有效'}
 *      ]
 *      rules: [{ required: false, message: "请选择/请输入" }],
 *      colSpan: 8,
 *      labelCol: 6,
 *   }
 * ]
 *
 * type: "text",
 * [必填] 标题名称
 * name: '名称',
 * [必填] 接口key名
 * key: 'name',
 * [可选] 字符类型, 针对input框的输入限制 string|select|number
 * searchFilterType: "string",
 * [可选] 初始化值
 * defaultValue: "",
 * [可选] 提示文字 rangepicker/rangepickerMonth:使用数组形式，例：['开始', '结束']
 * placeholder: "请输入名称",
 * [可选] form.getFieldDecorator第二参数中的rules属性
 * rules: [{ required: true, message: 'Please input your name!' }]
 * [可选] type为select时，options选项
 * options: []
 */
export default {
  props: {
    config: {
      type: Array,
      default: () => [],
    },
    disabledAdd: {
      type: Boolean,
      default: false,
    },
    /** 添加按钮名称 */
    AddbtnName: {
      type: String,
    },
    /** 自定义限制最多显示多少搜索条件 */
    limit: {
      type: Number,
      default: () => null,
    },
    /** 按钮组样色 */
    btnStyle: {
      type: Object,
      default: () => {},
    },
    /** 卡片右上角按钮 */
    extra_title: {
      type: String,
    },
  },
  components: {},
  i18n: require("./i18n"),
  data() {
    return {
      expand: false,
      form: this.$form.createForm(this),
    };
  },
  computed: {
    count() {
      return this.expand ? this.config.length : this.limit ? this.limit : 6;
    },
  },
  //创建完成 访问当前this实例
  created() {},
  //挂载完成 访问DOM元素
  mounted() {
    this.setDefaultValue();
  },
  //方法集合
  methods: {
    /** 表单提交事件 */
    handleSubmit(e) {
      e.preventDefault();
      this.form.validateFields((err, values) => {
        if (!err) {
          this.$emit("onSearch", values);
        }
      });
    },

    /** 表单重置 */
    handleReset() {
      this.form.resetFields();
      this.setDefaultValue();
      /**
       * @description: 返回取消按钮回调
       * @author: jiajun wu
       */
      this.$emit("onHandleReset", true);
    },

    toggle() {
      this.expand = !this.expand;
    },

    /** 设置默认值数据 */
    setDefaultValue() {
      /** 需要设置默认值的列表 */
      let setValueList = this.config.filter((e) => e.defaultValue);
      /** 设置默认值的对象 */
      let setValueObj = {};
      setValueList.map((v) => {
        Object.assign(setValueObj, {
          [v.key]:
            v.searchFilterType == "number"
              ? parseInt(v.defaultValue)
              : v.defaultValue,
        });
      });
      this.setValue(setValueObj);
    },

    /** 设置默认值 */
    setValue(data) {
      this.form.setFieldsValue(data);
    },
  },
};
</script>
<style lang='less' scoped>
</style>