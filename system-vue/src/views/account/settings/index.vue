<template>
  <div class="page-header-index-wide">
    <a-card
      :bordered="false"
      :bodyStyle="{ padding: '16px 0', height: '100%' }"
      :style="{ height: '100%' }"
    >
      <div class="account-settings-info-main" :class="{ mobile: isMobile }">
        <div class="account-settings-info-left">
          <a-menu
            :mode="isMobile ? 'horizontal' : 'inline'"
            :style="{ border: '0', width: isMobile ? '560px' : 'auto' }"
            :selectedKeys="selectedKeys"
            type="inner"
            @openChange="onOpenChange"
          >
            <a-menu-item key="/account/PersonalSettings/BasicSettings">
              <router-link to="/account/PersonalSettings/BasicSettings">{{
                $t("BasicSettings")
              }}</router-link>
            </a-menu-item>
            <!-- <a-menu-item key="/account/PersonalSettings/SecuritySettings">
              <router-link
                to="/account/PersonalSettings/SecuritySettings"
              >{{$t('SecuritySettings')}}</router-link>
            </a-menu-item> -->
            <a-menu-item key="/account/PersonalSettings/Personalization">
              <router-link to="/account/PersonalSettings/Personalization">{{
                $t("Personalization")
              }}</router-link>
            </a-menu-item>
            <!-- <a-menu-item key="/account/PersonalSettings/AccountBinding">
              <router-link to="/account/PersonalSettings/AccountBinding">{{$t('AccountBinding')}}</router-link>
            </a-menu-item> -->
            <!-- <a-menu-item key="/account/PersonalSettings/notification">
              <router-link :to="{ name: 'NotificationSettings' }">新消息通知</router-link>
            </a-menu-item>-->
          </a-menu>
        </div>
        <div class="account-settings-info-right">
          <div class="account-settings-info-title">
            <span class="title">{{ $route.meta.title }}</span>
          </div>
          <router-view />
        </div>
      </div>
    </a-card>
  </div>
</template>

<script>
export default {
  // components: {
  //   RouteView,
  // },
  i18n: require("./i18n"),
  data() {
    return {
      mode: "inline",
      openKeys: [],
      selectedKeys: [],
      // cropper
      preview: {},
      option: {
        img: "/avatar2.jpg",
        info: true,
        size: 1,
        outputType: "jpeg",
        canScale: false,
        autoCrop: true,
        // 只有自动截图开启 宽度高度才生效
        autoCropWidth: 180,
        autoCropHeight: 180,
        fixedBox: true,
        // 开启宽度和高度比例
        fixed: true,
        fixedNumber: [1, 1],
      },

      pageTitle: "",
      isMobile: false,
    };
  },
  mounted() {
    this.updateMenu();
  },
  methods: {
    onOpenChange(openKeys) {
      this.openKeys = openKeys;
    },
    updateMenu() {
      const routes = this.$route.matched.concat();
      this.selectedKeys = [routes.pop().path];
    },
  },
  watch: {
    $route(val) {
      this.updateMenu();
    },
  },
};
</script>

<style lang="less" scoped>
.page-header-index-wide {
  height: 100%;
  .account-settings-info-main {
    width: 100%;
    display: flex;
    height: 100%;
    overflow: auto;

    &.mobile {
      display: block;

      .account-settings-info-left {
        border-right: unset;
        border-bottom: 1px solid #e8e8e8;
        width: 100%;
        height: 50px;
        overflow-x: auto;
        overflow-y: scroll;
      }
      .account-settings-info-right {
        padding: 20px 40px;
      }
    }

    .account-settings-info-left {
      border-right: 1px solid #e8e8e8;
      width: 224px;
    }

    .account-settings-info-right {
      flex: 1 1;
      padding: 8px 40px;

      .account-settings-info-title {
        color: rgba(0, 0, 0, 0.85);
        font-size: 20px;
        font-weight: 500;
        line-height: 28px;
        margin-bottom: 12px;
        .title {
          color: @text-color;
        }
      }
      .account-settings-info-view {
        padding-top: 12px;
      }
    }
  }
}
</style>
