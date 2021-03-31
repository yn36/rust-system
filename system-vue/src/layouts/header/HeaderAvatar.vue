<template>
  <a-dropdown style="zindex: 9999">
    <div class="header-avatar" style="cursor: pointer">
      <a-avatar
        class="avatar"
        size="small"
        shape="circle"
        :src="user ? `${process}/annex/annex/view/${user.portrait}` : ''"
      />
      <span class="name">{{ user ? user.realname : "" }}</span>
    </div>
    <a-menu :class="['avatar-menu']" slot="overlay">
      <a-menu-item @click="goTo('/account/PersonalCenter')">
        <a-icon type="user" />
        <span>{{ $t("PersonalCenter") }}</span>
      </a-menu-item>
      <a-menu-item @click="goTo('/account/PersonalSettings/BasicSettings')">
        <a-icon type="setting" />
        <span>{{ $t("PersonalSettings") }}</span>
      </a-menu-item>
      <a-menu-divider />
      <a-menu-item @click="logout">
        <a-icon style="margin-right: 8px" type="poweroff" />
        <span>{{ $t("signOut") }}</span>
      </a-menu-item>
    </a-menu>
  </a-dropdown>
</template>

<script>
import { mapGetters, mapMutations } from "vuex";
import { Modal } from "ant-design-vue";
import { ACCESS_TOKEN } from "@/store/mutation-types";
// import { removeAuthorization } from "@/utils/request";
import Vue from "vue";

export default {
  name: "HeaderAvatar",
  i18n: require("./i18n.js"),
  data() {
    return {
      process: process.env.NODE_ENV === "production" ? "" : "",
    };
  },
  computed: {
    ...mapGetters("account", ["user"]),
  },
  methods: {
    ...mapMutations("account", ["setUser"]),
    logout() {
      Modal.confirm({
        title: this.$t("dialog.title"),
        content: this.$t("dialog.content"),
        onOk: () => {
          // logout().then((res) => {
          //   if (res.data.success) {
          // localStorage.removeItem(process.env.VUE_APP_ROUTES_KEY);
          // localStorage.removeItem(process.env.VUE_APP_PERMISSIONS_KEY);
          // localStorage.removeItem(process.env.VUE_APP_ROLES_KEY);
          // removeAuthorization();
          // storage.remove(ACCESS_TOKEN);
          // Vue.ss.remove(ACCESS_TOKEN);
          // this.setUser(null);
          // this.$router.push("/login");
          //   } else {
          //     this.$message.error(res.data.message);
          //   }
          // });
          // this.$router.push("/login");
          // return new Promise((resolve, reject) => {
          //   setTimeout(Math.random() > 0.5 ? resolve : reject, 1500)
          // }).catch(() => console.log('Oops errors!'))
          return this.$store.dispatch("account/Logout").then(() => {
            localStorage.removeItem(process.env.VUE_APP_ROUTES_KEY);
            localStorage.removeItem(process.env.VUE_APP_PERMISSIONS_KEY);
            localStorage.removeItem(process.env.VUE_APP_ROLES_KEY);
            Vue.ss.remove(ACCESS_TOKEN);
            Vue.ls.remove(ACCESS_TOKEN);
            Vue.ck.remove(ACCESS_TOKEN);
            this.setUser(null);
            this.$router.push("/login");
          });
        },
        onCancel() {},
      });
    },
    goTo(path) {
      this.$router.push(path);
    },
  },
};
</script>

<style lang="less">
.header-avatar {
  display: inline-flex;
  position: relative;
  .avatar,
  .name {
    align-self: center;
  }
  .avatar {
    margin-right: 8px;
  }
  .name {
    font-weight: 500;
  }
}
.avatar-menu {
  width: 150px;
  z-index: 9999;
}
.ant-dropdown.ant-dropdown-placement-bottomRight {
  z-index: 9999;
}

.ant-modal-mask {
  z-index: 9999;
}
.ant-modal-wrap {
  z-index: 9999;
}
</style>
