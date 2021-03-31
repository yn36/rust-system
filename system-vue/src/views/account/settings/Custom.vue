<!--
 * @Description: 个性化设置
 * @Version: 1.0
 * @Autor: jiajun wu
 * @Date: 2020-08-29 13:50:48
 * @LastEditors: JiaJun Wu
 * @LastEditTime: 2021-02-04 14:49:54
-->
<!-- 个性化设置 -->
<template>
  <div v-if="isload">
    <a-divider />
    <setting-item :title="$t('theme.title')">
      <img-checkbox-group
        @change="(values) => setMode(values)"
        :default-values="[theme.mode]"
      >
        <img-checkbox
          :title="$t('theme.light')"
          img="https://gw.alipayobjects.com/zos/rmsportal/jpRkZQMyYRryryPNtyIC.svg"
          value="light"
        />
        <img-checkbox
          :title="$t('theme.night')"
          img="https://gw.alipayobjects.com/zos/antfincdn/hmKaLQvmY2/LCkqqYNmvBEbokSDscrm.svg"
          value="night"
        />
      </img-checkbox-group>
    </setting-item>
    <setting-itemH :title="$t('theme.auto')">
      <a-switch :checked="auto" size="small" @change="setAuto" />
    </setting-itemH>
    <setting-item :title="$t('theme.color')">
      <color-checkbox-group
        @change="(values, colors) => setColors(colors)"
        :defaultValues="[palettes.indexOf(theme.color)]"
        :multiple="false"
      >
        <color-checkbox
          v-for="(color, index) in palettes"
          :key="index"
          :color="color"
          :value="index"
        />
      </color-checkbox-group>
    </setting-item>
    <setting-item :title="$t('navigate.title')">
      <img-checkbox-group
        @change="(values) => setNavigate(values)"
        :default-values="[layout]"
      >
        <img-checkbox
          :title="$t('navigate.side')"
          img="https://gw.alipayobjects.com/zos/rmsportal/JopDzEhOqwOjeNTXkoje.svg"
          value="side"
        />
        <img-checkbox
          :title="$t('navigate.head')"
          img="https://gw.alipayobjects.com/zos/rmsportal/KDNDBbriJhLwuqMoxcAr.svg"
          value="head"
        />
      </img-checkbox-group>
    </setting-item>
    <setting-itemH :title="$t('navigate.fixedHeader')">
      <a-switch :checked="fixedHeader" size="small" @change="setHeader" />
    </setting-itemH>
    <setting-itemH :title="$t('other.weekMode')">
      <a-switch :checked="weekMode" size="small" @change="setweekmode" />
    </setting-itemH>
    <setting-itemH :title="$t('other.multiPages')">
      <a-switch :checked="multiPage" size="small" @change="setmultipage" />
    </setting-itemH>
  </div>
</template>

<script>
import { UpdateTheme, getThemeFind } from "@/api/system-serve";
import SettingItem from "@/components/setting/SettingItem";
import SettingItemH from "@/components/setting/SettingItemH";
import { mapState, mapMutations } from "vuex";
import { ColorCheckbox, ImgCheckbox } from "@/components/checkbox";
import store from "@/store";
import { theme } from "@/config/config";

const ColorCheckboxGroup = ColorCheckbox.Group;
const ImgCheckboxGroup = ImgCheckbox.Group;

export default {
  components: {
    ImgCheckboxGroup,
    ImgCheckbox,
    ColorCheckboxGroup,
    ColorCheckbox,
    SettingItem,
    SettingItemH,
  },
  i18n: require("@/components/setting/i18n"),
  data() {
    return {
      /** 用户 */
      user_theme: {},
      auto: false,
      isload: false,
    };
  },
  //创建完成 访问当前this实例
  created() {},
  //挂载完成 访问DOM元素
  mounted() {
    this.doQuery();
  },
  computed: {
    directions() {
      return this.animates.find((item) => item.name == this.animate.name)
        .directions;
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
  watch: {
    "animate.name": function (val) {
      this.setAnimate({ name: val, direction: this.directions[0] });
    },
    "theme.mode": {
      deep: true,
      immediate: true,
      handler(value) {
        this.isload = false;
        this.$nextTick(() => {
          this.isload = true;
        });
      },
    },
  },
  //方法集合
  methods: {
    /** 查找用户主题 */
    async doQuery() {
      let data = await getThemeFind().then((res) => res.data);
      if (data.success) {
        if (data.body.appearance === "auto") {
          this.auto = true;
          let isLight =
            window.matchMedia &&
            window.matchMedia("(prefers-color-scheme: light)").matches;
          Object.assign(this.theme, {
            mode: isLight ? "light" : "night",
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
        this.setFixedHeader(data.body.fixed_header);
        this.setWeekMode(data.body.week_mode);
        this.setMultiPage(data.body.multi_pages);
        this.$nextTick(() => {
          this.isload = true;
        });
      }
    },
    /** 设置主题 */
    async setMode(values) {
      let data = await UpdateTheme({ appearance: `${values[0]}` }).then(
        (res) => res.data
      );
      if (data.success) {
        this.setTheme({ ...this.theme, mode: values[0] });
      }
    },
    /** 自动按钮触发 */
    async setAuto(values) {
      this.auto = values;
      let body = {};
      let isLight =
        window.matchMedia &&
        window.matchMedia("(prefers-color-scheme: light)").matches;
      if (values) {
        Object.assign(body, {
          appearance: "auto",
          theme_color: isLight ? "#1890ff" : "#13c2c2",
        });
      } else {
        Object.assign(body, { appearance: `${this.theme.mode}` });
      }
      let data = await UpdateTheme({ ...body }).then((res) => res.data);
      if (data.success) {
        this.isload = false;
        let theme = JSON.parse(JSON.stringify(this.theme));
        Object.assign(theme, {
          mode: isLight ? "light" : "night",
          color: isLight ? "#1890ff" : "#13c2c2",
        });
        this.setTheme({
          ...theme,
          mode: isLight ? "light" : "night",
          appearance: data.body.appearance,
        });
        this.$nextTick(() => {
          this.isload = true;
        });
      }
    },
    /** 修改颜色 */
    async setColors(colors) {
      let data = await UpdateTheme({ theme_color: `${colors[0]}` }).then(
        (res) => res.data
      );
      if (data.success) {
        this.setTheme({ ...this.theme, color: colors[0] });
      }
    },

    /** 更改导航设置 */
    async setNavigate(values) {
      let data = await UpdateTheme({ navigate: `${values[0]}` }).then(
        (res) => res.data
      );
      if (data.success) {
        this.setLayout(values[0]);
      }
    },

    /** 设置是否固定Header */
    async setHeader(value) {
      let data = await UpdateTheme({ fixed_header: value }).then(
        (res) => res.data
      );
      if (data.success) {
        this.setFixedHeader(value);
      }
    },

    /** 设置色弱模式 */
    async setweekmode(value) {
      let data = await UpdateTheme({ week_mode: value }).then(
        (res) => res.data
      );
      if (data.success) {
        this.setWeekMode(value);
      }
    },

    /** 设置多标签模式 */
    async setmultipage(value) {
      let data = await UpdateTheme({ multi_pages: value }).then(
        (res) => res.data
      );
      if (data.success) {
        this.setMultiPage(value);
      }
    },

    ...mapMutations("account", ["setUser", "setPermissions", "setRoles"]),
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
  },
};
</script>
<style lang='less' scoped>
.side-setting {
  min-height: 100%;
  background-color: @base-bg-color;
  padding: 24px;
  font-size: 14px;
  line-height: 1.5;
  word-wrap: break-word;
  position: relative;
  .flex {
    display: flex;
  }
  .select-item {
    width: 80px;
  }
}
</style>