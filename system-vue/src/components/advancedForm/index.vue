<!--
 * @Description: 高级表单组件
 * @Version: 1.0
 * @Autor: JiaJun Wu
 * @Date: 2021-02-05 13:53:17
 * @LastEditors: jiajun wu
 * @LastEditTime: 2021-02-06 22:46:02
-->
<!-- 高级表单组件 -->
<template>
  <a-card>
    <a-form :form="form" @submit="handleSubmit">
      <a-row :gutter="[10, 10]">
        <template v-for="item in config">
          <a-col
            :key="item.key"
            :offset="item.offset"
            :pull="item.offset"
            :xs="{ span: 24 }"
            :sm="{ span: 24 }"
            :md="{ span: 12 }"
            :lg="{ span: 12 }"
            :xl="{ span: item.colSpan || 6 }"
            v-if="!item.isHide"
          >
            <a-form-item
              :label-col="{
                xs: { span: 24 },
                sm: { span: 24 },
                md: { span: 6 },
                xl: { span: item.labelCol || 6 },
              }"
              :wrapper-col="{
                xs: { span: 24 },
                sm: { span: 24 },
                md: { span: 24 - 6 || 18 },
                xl: { span: 24 - item.labelCol || 18 },
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
                :disabled="item.disabled || disabled"
                :showTime="item.showTime"
                :max="item.type === 'a-input-number' ? item.max : null"
                :allowClear="item.allowClear || true"
                :showSearch="item.showSearch || false"
                :rows="item.rows || 4"
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
          }"
        >
          <a-button
            style="margin-right: 8px"
            type="primary"
            html-type="submit"
            icon="check"
            >保存</a-button
          >
          <a-button
            style="margin-right: 8px"
            type="primary"
            @click="onSubmit"
            icon="check"
            v-if="isSubmitBtn"
            >提交</a-button
          >
          <a-button
            style="margin-right: 8px"
            type="primary"
            icon="undo"
            @click="
              () => {
                this.$emit('goBack');
              }
            "
            >返回</a-button
          >
        </a-col>
      </a-row>
    </a-form>
  </a-card>
</template>

<script>
export default {
  props: {
    config: {
      type: Array,
      default: () => [],
    },
    /** 是否需要提交 */
    isSubmitBtn: {
      type: Boolean,
      default: () => false,
    },
    /** 回显的数据 */
    formData: {
      type: Object,
      default: () => {},
    },
    /** 1:添加 2:修改 3:查看 */
    formType: {
      type: Number,
      default: 3,
    },
  },
  computed: {
    disabled() {
      return this.formType == 3;
    },
  },
  data() {
    return {
      form: this.$form.createForm(this),
    };
  },
  //创建完成 访问当前this实例
  created() {},
  //挂载完成 访问DOM元素
  mounted() {
    this.$nextTick(() => {
      this.setFormData();
    });
  },
  //方法集合
  methods: {
    /** 表单提交事件
     * @param {Number} statusType :提交状态 0.保存 1.提交
     */
    handleSubmit(e, statusType = 0) {
      e.preventDefault();
      this.form.validateFields((err, values) => {
        if (!err) {
          let data = {
            ...this.formData,
            ...values,
          };
          this.$emit("onSave", data, statusType);
        }
      });
    },
    setFormData() {
      let keys = this.config.filter((e) => e.key).map((v) => v.key);
      let data = {};
      keys.map((v) => {
        Object.assign(data, {
          [v]: this.formData[v],
        });
      });
      this.$nextTick(() => {
        this.form.setFieldsValue(data);
      });
    },
    /** 提交 */
    onSubmit(e) {
      this.handleSubmit(e, 1);
    },
  },
};
</script>
<style lang='less' scoped>
</style>