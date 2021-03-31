<!--
 * @Description: 个人中心
 * @Version: 1.0
 * @Autor: jiajun wu
 * @Date: 2020-08-27 20:27:12
 * @LastEditors: JiaJun Wu
 * @LastEditTime: 2021-02-04 14:48:31
-->
<!-- 个人中心 -->
<template>
  <div class="clearfix">
    <a-upload :file-list="fileList" :remove="handleRemove" :before-upload="beforeUpload">
      <a-button>
        <a-icon type="upload" />Select File
      </a-button>
    </a-upload>
    <a-button
      type="primary"
      :disabled="fileList.length === 0"
      :loading="uploading"
      style="margin-top: 16px"
      @click="handleUpload"
    >{{ uploading ? 'Uploading' : 'Start Upload' }}</a-button>

    <div>
      <p v-for="item in fileList" :key="item._id" @click="onfile(item)">{{item.name}}</p>
    </div>
  </div>
</template>

<script>
import { FilefindWith, uploadFile } from "@/api/system-serve";
import { request } from "@/utils/request";

export default {
  components: {},
  data() {
    return {
      fileList: [],
      uploading: false,
      files: {},
    };
  },
  //创建完成 访问当前this实例
  created() {
    this.getfile();
  },
  //挂载完成 访问DOM元素
  mounted() {},
  //方法集合
  methods: {
    handleRemove(file) {
      const index = this.fileList.indexOf(file);
      const newFileList = this.fileList.slice();
      newFileList.splice(index, 1);
      this.fileList = newFileList;
      return false;
    },
    beforeUpload(file) {
      this.fileList = [...this.fileList, file];
      return false;
    },
    handleUpload() {
      const { fileList } = this;
      const formData = new FormData();
      console.log(fileList);
      formData.append("file", fileList[0]);
      formData.append("fileType", fileList[0].type);
      // this.uploading = true;

      uploadFile(formData).then((res) => {
        console.log(res);
      });
      return false;
    },

    async getfile() {
      let data = await FilefindWith({
        body: { createdBy: "5f449a4fea72c5591e988fca" },
      }).then((res) => res.data);
      if (data.success) {
        data.body.map((v) => {
          Object.assign(v, {
            uid: v._id,
            name: v.name,
            status: "done",
            url: `/system/annex/view/${v._id}`,
          });
        });
        this.fileList = data.body;
      }
    },
    async onfile(item) {
      let data = await request(`/system/annex/view/${item._id}`, "get", null, {
        responseType: "blob",
        headers: {
          "Content-Type": "application/octet-stream",
        },
      }).then((res) => res.data);

      console.log(data, "data");
      const binaryData = [];
      binaryData.push(data);
      // //获取blob链接
      let url = encodeURIComponent(
        window.URL.createObjectURL(
          new Blob(binaryData, { type: "application/pdf" })
        )
      );
      window.open(`/pdf/web/viewer.html?file=${url}`);
    },
  },
};
</script>
<style lang='less' scoped>
</style>