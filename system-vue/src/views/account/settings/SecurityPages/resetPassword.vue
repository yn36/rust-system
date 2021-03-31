<!--
 * @Description: 重置密码
 * @Version: 1.0
 * @Autor: jiajun wu
 * @Date: 2020-09-12 17:44:15
 * @LastEditors: JiaJun Wu
 * @LastEditTime: 2021-02-04 14:49:39
-->
<!-- 重置密码 -->
<template>
  <div>
    <a-form :form="form" @submit="handleSubmit">
      <a-row :gutter="10">
        <a-col :span="22">
          <a-form-item :label-col="{span:6}" :wrapper-col="{span:15}" :label="$t('passwordOld')">
            <a-input-password
              :maxLength="16"
              :minLength="3"
              :placeholder="$t('passwordOldPlaceholder')"
              v-decorator="['passwordOld', { rules: [{ required: true, message: $t('passwordOldPlaceholder') }] }]"
            />
          </a-form-item>
        </a-col>
        <a-col :span="22">
          <a-form-item :label-col="{span:6}" :wrapper-col="{span: 15}" :label="$t('newPassword')">
            <a-input-password
              :maxLength="16"
              :minLength="3"
              @blur="(e)=>PasswordMinLength(e)"
              @input="(e)=>onInput(e)"
              :placeholder="$t('newPasswordPlaceholder')"
              v-decorator="['newPassword', { rules: [{ required: true, message: $t('newPasswordPlaceholder') },] }]"
            />
            <template slot="extra">
              <div class="input_span">
                <span id="one"></span>
                <span id="two"></span>
                <span id="three"></span>
              </div>
            </template>
          </a-form-item>
        </a-col>
        <a-col :span="22">
          <a-form-item :label-col="{span:6}" :wrapper-col="{span: 15}" :label="$t('Reconfirm')">
            <a-input-password
              :maxLength="16"
              :minLength="3"
              :placeholder="$t('ReconfirmPasswordPlaceholder')"
              v-decorator="['confirm', { rules: [{ required: true, message: $t('ReconfirmPasswordPlaceholder') }, 
                  {
                    validator: compareToFirstPassword,
                  }]
                 }]"
            />
          </a-form-item>
        </a-col>
        <a-col :span="22">
          <div class="ant-row ant-form-item">
            <div class="ant-col ant-col-6 ant-form-item-label wariness">
              <label :title="$t('Tips')">⚠️{{$t('Tips')}}</label>
            </div>
            <div class="ant-col ant-col-15 ant-form-item-control-wrapper wariness">
              <div class="ant-form-item-control">{{$t('TipsMessage')}}</div>
            </div>
          </div>
        </a-col>
      </a-row>
      <a-row>
        <a-col :span="22" :style="{ textAlign: 'right',margin: '15px 0'}">
          <a-button
            :style="{ marginLeft: '8px' }"
            type="primary"
            html-type="submit"
            :loading="loading"
          >{{$t('Save')}}</a-button>
          <a-button :style="{ marginLeft: '8px' }" @click="handleCancel">{{$t('return')}}</a-button>
        </a-col>
      </a-row>
    </a-form>
  </div>
</template>

<script>
import { mapMutations } from "vuex";
import { checkStrong } from "@/utils/util";
import { userUpdatePassword } from "@/api/system-serve";
import { removeAuthorization } from "@/utils/request";
import { ACCESS_TOKEN } from "@/store/mutation-types";
import Vue from "vue";

export default {
  components: {},
  i18n: require("./i18n"),
  data() {
    return {
      form: this.$form.createForm(this, { name: "form" }),
      loading: false,
      /** 密码强度 */
      passwordrule: 0,
    };
  },
  //创建完成 访问当前this实例
  created() {},
  //挂载完成 访问DOM元素
  mounted() {},
  //方法集合
  methods: {
    ...mapMutations("account", ["setUser"]),
    /** 提交 */
    handleSubmit(e) {
      e.preventDefault();
      this.form.validateFields((error, values) => {
        console.log(error);
        if (!error) {
          // console.log(values);
          let obj = {
            passwordOld: values.passwordOld,
            newPassword: values.newPassword,
            passwordrule: this.passwordrule,
          };
          console.log(obj);
          userUpdatePassword({ ...obj }).then((res) => {
            console.log(res);
            if (res.data.success) {
              this.$success({
                title: this.$t("successTitle"),
                content: this.$t("successContent"),
                onOk: () => {
                  Vue.ss.remove(ACCESS_TOKEN);
                  localStorage.removeItem(process.env.VUE_APP_ROUTES_KEY);
                  localStorage.removeItem(process.env.VUE_APP_PERMISSIONS_KEY);
                  localStorage.removeItem(process.env.VUE_APP_ROLES_KEY);
                  removeAuthorization();
                  this.setUser(null);
                  this.$router.push({ path: "/login" });
                },
              });
            }
          });
        }
      });
    },
    /** 返回 */
    handleCancel() {
      this.$emit("return_btn");
    },
    compareToFirstPassword(rule, value, callback) {
      const form = this.form;
      if (value && value !== form.getFieldValue("newPassword")) {
        callback(this.$t("ToFirstPassword"));
      } else {
        callback();
      }
    },
    PasswordMinLength(e) {
      if (e.target.value && e.target.value.length < 5) {
        this.form.setFields({
          newPassword: {
            value: e.target.value,
            errors: [new Error(this.$t("PasswordMinLength"))],
          },
        });
      }
    },

    /** 输入密码 */
    onInput(e) {
      let msgText = checkStrong(e.target.value);
      if (msgText > 1 || msgText == 1) {
        document.getElementById("one").style.background = "red";
      } else {
        document.getElementById("one").style.background = "#eee";
      }
      if (msgText > 2 || msgText == 2) {
        document.getElementById("two").style.background = "orange";
        document.getElementById("one").style.background = "orange";
      } else {
        document.getElementById("two").style.background = "#eee";
      }
      if (msgText == 4) {
        document.getElementById("two").style.background = "#00D1B2";
        document.getElementById("one").style.background = "#00D1B2";
        document.getElementById("three").style.background = "#00D1B2";
      } else {
        document.getElementById("three").style.background = "#eee";
      }
      this.passwordRule = msgText;
    },
  },
};
</script>
<style lang='less' scoped>
.wariness {
  color: red;
  label {
    color: red;
  }
}
.input_span span {
  display: inline-block;
  width: 33%;
  height: 10px;
  background: #eee;
  line-height: 20px;
  transition: all 0.5s;
}

#one {
  border-top-left-radius: 5px;
  border-bottom-left-radius: 5px;
  border-right: 0px solid;
  margin-left: 5px;
}

#two {
  border-left: 0px solid;
  border-right: 0px solid;
  margin-left: -5px;
  margin-right: 3px;
}

#three {
  border-top-right-radius: 5px;
  border-bottom-right-radius: 5px;
  border-left: 0px solid;
  margin-left: -5px;
}
</style>