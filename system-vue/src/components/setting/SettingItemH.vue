<template>
  <div class="setting-item">
    <h3 class="title">
      {{title}}:
      <span class="slot">
        <slot></slot>
      </span>
    </h3>
  </div>
</template>

<script>
export default {
  name: "SettingItem",
  props: ["title"],
};
</script>

<style lang="less" scoped>
.setting-item {
  margin-bottom: 24px;
  overflow: hidden;
  .title {
    display: inline-block;
    float: left;
    font-size: 14px;
    color: @title-color;
    line-height: 22px;
    margin-bottom: 12px;
    width: 30%;
  }
  .slot {
    margin-left: 20px;
    float: right;
  }
}
</style>
