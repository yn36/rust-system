<template>
  <common-layout>
    <div class="top">
      <div class="header">
        <!-- <img alt="logo" class="logo" src="@/assets/img/logo.png" /> -->
        <div style="width: 135px; display: inline-block; position: relative">
          <svg
            style="position: absolute; left: -20px"
            t="1614062526036"
            class="icon"
            viewBox="0 0 1024 1024"
            version="1.1"
            xmlns="http://www.w3.org/2000/svg"
            p-id="1471"
            width="50"
            height="50"
          >
            <path
              d="M851 264.8H171.3c-59.4 0-107.6 48.3-107.6 107.7v278.8C63.7 710.7 112 759 171.3 759H851c59.4 0 107.7-48.3 107.7-107.7V372.5c0-59.4-48.3-107.7-107.7-107.7zM119.6 651.3V372.5c0-28.5 23.2-51.7 51.7-51.7h339.8V703H171.3c-28.5 0-51.7-23.2-51.7-51.7z"
              p-id="1472"
              fill="#1890ff"
            ></path>
            <path
              d="M205.6 404.1h54.2l51.4 86.8 51.1-86.8h53.8l-81.5 135.4v84.3h-48.3v-85.5l-80.7-134.2z"
              p-id="1473"
              fill="#1890ff"
            ></path>
            <path
              d="M679.4 404.1l91.7 147.4h0.6V404.1h45.2v219.7h-48.3l-91.4-147.1h-0.6v147.1h-45.2V404.1h48z"
              p-id="1474"
              fill="white"
            ></path>
          </svg>
          <span class="title" style="float: right">{{ systemName }}</span>
        </div>
      </div>
      <div class="desc">YNOS Web 后台管理系统</div>
    </div>
    <div class="login">
      <a-form @submit="onSubmit" :form="form">
        <a-tabs
          size="large"
          :tabBarStyle="{ textAlign: 'center' }"
          style="padding: 0 2px"
        >
          <a-tab-pane tab="账户密码登录" key="1">
            <a-alert
              type="error"
              :closable="true"
              v-show="error"
              :message="error"
              showIcon
              style="margin-bottom: 24px"
            />
            <a-form-item>
              <a-input
                autocomplete="autocomplete"
                size="large"
                placeholder="用户名/手机/邮箱"
                v-decorator="[
                  'name',
                  {
                    rules: [
                      {
                        required: true,
                        message: '请输入账户名',
                        whitespace: true,
                      },
                    ],
                  },
                ]"
              >
                <a-icon slot="prefix" type="user" />
              </a-input>
            </a-form-item>
            <a-form-item>
              <a-input
                size="large"
                placeholder="密码"
                autocomplete="autocomplete"
                type="password"
                v-decorator="[
                  'password',
                  {
                    rules: [
                      {
                        required: true,
                        message: '请输入密码',
                        whitespace: true,
                      },
                    ],
                  },
                ]"
              >
                <a-icon slot="prefix" type="lock" />
              </a-input>
            </a-form-item>
          </a-tab-pane>
          <!-- <a-tab-pane tab="手机号登录" key="2">
            <a-form-item>
              <a-input size="large" placeholder="mobile number">
                <a-icon slot="prefix" type="mobile" />
              </a-input>
            </a-form-item>
            <a-form-item>
              <a-row :gutter="8" style="margin: 0 -4px">
                <a-col :span="16">
                  <a-input size="large" placeholder="captcha">
                    <a-icon slot="prefix" type="mail" />
                  </a-input>
                </a-col>
                <a-col :span="8" style="padding-left: 4px">
                  <a-button
                    style="width: 100%"
                    class="captcha-button"
                    size="large"
                    >获取验证码</a-button
                  >
                </a-col>
              </a-row>
            </a-form-item>
          </a-tab-pane> -->
        </a-tabs>
        <div>
          <a-checkbox :checked="AutomaticLogin" v-model="AutomaticLogin"
            >自动登录</a-checkbox
          >
          <!-- <a style="float: right">忘记密码</a> -->
        </div>
        <a-form-item>
          <a-button
            :loading="logging"
            style="width: 100%; margin-top: 24px"
            size="large"
            htmlType="submit"
            type="primary"
            >登录</a-button
          >
        </a-form-item>
        <!-- <div>
          其他登录方式
          <a-icon class="icon" type="alipay-circle" />
          <a-icon class="icon" type="taobao-circle" />
          <a-icon class="icon" type="weibo-circle" />
          <router-link style="float: right" to="/reg"
            >注册账户</router-link
          >
        </div> -->
      </a-form>
    </div>
  </common-layout>
</template>

<script>
import CommonLayout from "@/layouts/CommonLayout";
import { login, getRoutesConfig } from "@/api/user";
import { loadRoutes } from "@/utils/routerUtil";
import { mapMutations, mapActions, mapState } from "vuex";
import notification from "ant-design-vue/es/notification";
import { ACCESS_TOKEN } from "@/store/mutation-types";
import Vue from "vue";
import { getThemeFind } from "@/api/system-serve";
import md5 from "md5";
import base64 from "base-64";

export default {
  name: "Login",
  components: { CommonLayout },
  data() {
    return {
      logging: false,
      error: "",
      form: this.$form.createForm(this),
      /** 是否自动登陆 */
      AutomaticLogin: false,
    };
  },
  computed: {
    systemName() {
      return this.$store.state.setting.systemName;
    },
    ...mapState("setting", [
      "theme",
      "layout",
      "animate",
      "animates",
      "palettes",
      "multiPage",
      "weekMode",
      "fixedHeader",
      "fixedSideBar",
      "hideSetting",
    ]),
  },
  methods: {
    ...mapMutations("account", [
      "setUser",
      "setPermissions",
      "setRoles",
      "setToken",
    ]),
    ...mapMutations("setting", [
      "setTheme",
      "setLayout",
      "setMultiPage",
      "setWeekMode",
      "setFixedSideBar",
      "setFixedHeader",
      "setAnimate",
      "setHideSetting",
    ]),
    ...mapActions(["Login", "Logout"]),
    /** 查找用户主题 */
    async doQuery() {
      let data = await getThemeFind()
        .then((res) => res.data)
        .catch((e) => {
          /** 如果未成功 */
          this.auto = false;
          Object.assign(this.theme, {
            mode: "light",
            color: "#1890ff",
          });
          this.setTheme({ ...this.theme, mode: "auto" });
          this.setLayout(data.body.navigate);
          this.setFixedHeader(data.body.fixedHeader);
          this.setWeekMode(data.body.weekMode);
          this.setMultiPage(data.body.multiPages);
          this.$nextTick(() => {
            this.isload = true;
          });
        });
      if (data.success) {
        if (data.body.appearance === "auto") {
          this.auto = true;
          let isLight =
            window.matchMedia &&
            window.matchMedia("(prefers-color-scheme: light)").matches;
          Object.assign(this.theme, {
            mode: isLight ? "light" : "night",
            // color: data.body.theme_color,
            color: isLight ? "#1890ff" : "#13c2c2",
          });
          this.setTheme({ ...this.theme, mode: isLight ? "light" : "night" });
        } else {
          this.auto = false;
          Object.assign(this.theme, {
            mode: data.body.appearance,
            color: data.body.theme_color,
          });
          this.setTheme({ ...this.theme, mode: data.body.appearance });
        }
        this.setLayout(data.body.navigate);
        this.setFixedHeader(data.body.fixedHeader);
        this.setWeekMode(data.body.weekMode);
        this.setMultiPage(data.body.multiPages);
        this.$nextTick(() => {
          this.isload = true;
        });
      }
    },
    onSubmit(e) {
      e.preventDefault();
      this.form.validateFields((err) => {
        if (!err) {
          this.logging = true;
          const username = this.form.getFieldValue("name");
          // TODO 此处需要加密
          // const password = base64.encode(
          //   md5(this.form.getFieldValue("password"))
          // );
          const password = this.form.getFieldValue("password");
          this.$store
            .dispatch("account/Login", { username, password })
            .then((res) => {
              this.afterLogin(res);
            })
            .catch((err) => {
              this.logging = false;
              // notification.error({
              //   message: "错误",
              //   description: err.response.data.message,
              // });
            });
        }
      });
    },
    afterLogin(res) {
      this.logging = false;
      const loginRes = res.data;
      if (loginRes.success) {
        // this.doQuery();
        this.setUser(loginRes.body.user_info);
        // this.setRoles([{ id: "admin", operation: ["add", "edit", "delete"] }]);
        this.setToken(loginRes.body.token);
        if (this.AutomaticLogin) {
          Vue.ls.set(
            ACCESS_TOKEN,
            loginRes.body.token,
            7 * 24 * 60 * 60 * 1000
          );
        }
        Vue.ss.set(ACCESS_TOKEN, loginRes.body.token, 7 * 24 * 60 * 60 * 1000);
        Vue.ck.set(ACCESS_TOKEN, loginRes.body.token, 7 * 24 * 60 * 60 * 1000);

        if (this.$route.query.redirect) {
          this.$router.replace({
            path: this.$route.query.redirect,
          });
        } else if (this.$route.query.backurl) {
          let url = this.$route.hash;
          if (url.indexOf("?") < 0) {
            url = `${url}?`;
          }
          let goUrl = `${this.$route.query.backurl}${url}&token=${loginRes.body.token}`;
          window.location.href = goUrl;
        } else {
          this.$router.replace({
            path: "/account/PersonalSettings/BasicSettings",
          });
        }
        // this.$router.push("/account/PersonalSettings/BasicSettings");
        // this.$router.replace({
        //   path: "/",
        // });
        this.$message.success(loginRes.message, 3);
      } else {
        this.error = loginRes.message;
      }
    },
  },
};
</script>

<style lang="less" scoped>
.common-layout {
  .top {
    text-align: center;
    .header {
      height: 44px;
      line-height: 44px;
      a {
        text-decoration: none;
      }
      .logo {
        height: 44px;
        vertical-align: top;
        margin-right: 16px;
      }
      .title {
        font-size: 33px;
        color: @title-color;
        font-family: "Myriad Pro", "Helvetica Neue", Arial, Helvetica,
          sans-serif;
        font-weight: 600;
        position: relative;
        top: 2px;
      }
    }
    .desc {
      font-size: 14px;
      color: @text-color-second;
      margin-top: 12px;
      margin-bottom: 40px;
    }
  }
  .login {
    width: 368px;
    margin: 0 auto;
    @media screen and (max-width: 576px) {
      width: 95%;
    }
    @media screen and (max-width: 320px) {
      .captcha-button {
        font-size: 14px;
      }
    }
    .icon {
      font-size: 24px;
      color: @text-color-second;
      margin-left: 16px;
      vertical-align: middle;
      cursor: pointer;
      transition: color 0.3s;

      &:hover {
        color: @primary-color;
      }
    }
  }
}
</style>
