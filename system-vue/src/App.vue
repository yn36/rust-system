<template>
  <a-configProvider :locale="locale">
    <router-view />
  </a-configProvider>
</template>

<script>
import { enquireScreen } from "./utils/util";
import { mapState, mapMutations } from "vuex";
import themeUtil from "@/utils/themeUtil";
import { getI18nKey } from "@/utils/routerUtil";
import store from "@/store";
import i18n from "./router/i18n";

export default {
  name: "App",
  data() {
    return {
      locale: {},
      isLight:
        window.matchMedia &&
        window.matchMedia("(prefers-color-scheme: light)").matches,
    };
  },
  created() {
    this.setHtmlTitle();
    this.setLanguage(this.lang);
    enquireScreen((isMobile) => this.setDevice(isMobile));
  },
  watch: {
    isLight: {
      deep: true,
      handler(value) {
        console.log(value);
      },
    },
  },
  mounted() {
    this.setWeekModeTheme(this.weekMode);
    let isLight =
      window.matchMedia &&
      window.matchMedia("(prefers-color-scheme: light)").matches;
    if (!store.getters["account/user"]) {
      Object.assign(this.theme, {
        mode: isLight ? "light" : "night",
        color: isLight ? "#1890ff" : "#13c2c2",
      });
      this.setTheme({ ...this.theme, mode: isLight ? "light" : "night" });
    }
    /** 实时监听浏览器模式 */
    window
      .matchMedia("(prefers-color-scheme: dark)")
      .addEventListener("change", (event) => {
        if (store.getters["account/user"] && this.theme.appearance != "auto") {
          return false;
        }
        if (event.matches) {
          isLight = false;
          Object.assign(this.theme, {
            mode: isLight ? "light" : "night",
            color: isLight ? "#1890ff" : "#13c2c2",
          });
          this.setTheme({
            ...this.theme,
            mode: isLight ? "light" : "night",
          });
        } else {
          isLight = true;
          Object.assign(this.theme, {
            mode: isLight ? "light" : "night",
            color: isLight ? "#1890ff" : "#13c2c2",
          });
          this.setTheme({
            ...this.theme,
            mode: isLight ? "light" : "night",
          });
        }
      });
  },
  watch: {
    weekMode(val) {
      this.setWeekModeTheme(val);
    },
    lang(val) {
      this.setLanguage(val);
      this.setHtmlTitle();
    },
    $route() {
      this.setHtmlTitle();
    },
    "theme.mode": function (val) {
      // let closeMessage = this.$message.loading(
      //   `您选择了主题模式 ${val}, 正在切换...`
      // );
      themeUtil.changeThemeColor(this.theme.color, val).then(() => {
        // setTimeout(closeMessage, 1000);
      });
    },
    "theme.color": function (val) {
      // let closeMessage = this.$message.loading(
      //   `您选择了主题色 ${val}, 正在切换...`
      // );
      themeUtil.changeThemeColor(val, this.theme.mode).then(() => {
        // setTimeout(closeMessage, 1000);
      });
    },
  },
  computed: {
    ...mapState("setting", ["theme", "weekMode", "lang"]),
  },
  methods: {
    ...mapMutations("setting", ["setDevice", "setTheme"]),
    setWeekModeTheme(weekMode) {
      if (weekMode) {
        document.body.classList.add("week-mode");
      } else {
        document.body.classList.remove("week-mode");
      }
    },
    setLanguage(lang) {
      this.$i18n.locale = lang;
      switch (lang) {
        case "CN":
          this.locale = require("ant-design-vue/es/locale-provider/zh_CN").default;
          break;
        case "HK":
          this.locale = require("ant-design-vue/es/locale-provider/zh_TW").default;
          break;
        case "US":
        default:
          this.locale = require("ant-design-vue/es/locale-provider/en_US").default;
          break;
      }
    },
    setHtmlTitle() {
      const route = this.$route;
      const key =
        route.matched &&
        route.matched.length &&
        route.matched[route.matched.length - 1].meta.i18n
          ? route.matched[route.matched.length - 1].meta.i18n[this.$i18n.locale]
          : route.name;
      document.title = process.env.VUE_APP_NAME + " | " + this.$t(key);
    },
  },
};
</script>

<style lang="less" scoped>
#id {
}
</style>
