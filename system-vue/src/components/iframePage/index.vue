<template>
  <iFrame class="iframe" :src="url" width="100%" height="100%" frameborder="no" border="0" scrolling="auto"></iFrame>
</template>

<script>
import Vue from 'vue'
import { ACCESS_TOKEN } from '@/store/mutation-types'

export default {
  name: 'IframePage',
  data() {
    return {
      url: this.$route.meta.url
    }
  },
  created() {
    if (Vue.ss.get(ACCESS_TOKEN)) {
      this.url = this.url + `&token=${Vue.ss.get(ACCESS_TOKEN)}`
    }
  },
  watch: {
    $route(to, from) {
      if (Vue.ss.get(ACCESS_TOKEN)) {
        this.url = to.meta.url + `&token=${Vue.ss.get(ACCESS_TOKEN)}`
      }
    }
  }
}
</script>
<style lang='less' scoped>
.iframe {
  //height: calc(100vh - 90px);
}
</style>