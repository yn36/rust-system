<!--
 * @Description: 基本设置
 * @Version: 1.0
 * @Autor: jiajun wu
 * @Date: 2021-01-06 21:10:58
 * @LastEditors: JiaJun Wu
 * @LastEditTime: 2021-02-23 11:47:24
-->
<template>
  <div class="account-settings-info-view">
    <a-row :gutter="16">
      <a-col :md="24" :lg="16">
        <a-form layout="vertical" :form="form" @submit="handleSubmit">
          <a-form-item :label="$t('name')">
            <a-input
              :placeholder="$t('namePlaceholder')"
              v-decorator="['realname']"
            />
          </a-form-item>
          <a-form-item :label="$t('desc')">
            <a-textarea
              rows="4"
              :placeholder="$t('descsPlaceholder')"
              v-decorator="['desc']"
            />
          </a-form-item>

          <a-form-item :label="$t('Email')" :required="false">
            <a-input :placeholder="$t('Email')" v-decorator="['email']" />
          </a-form-item>

          <a-form-item :label="$t('birthday')" :required="false">
            <a-date-picker
              :placeholder="$t('birthday')"
              v-decorator="['birthday']"
            />
          </a-form-item>

          <a-form-item :label="$t('sex')" :required="false">
            <a-radio-group v-decorator="['sex']">
              <a-radio :value="1">{{ $t("man") }}</a-radio>
              <a-radio :value="0">{{ $t("female") }}</a-radio>
              <a-radio :value="3">{{ $t("secrecy") }}</a-radio>
            </a-radio-group>
          </a-form-item>

          <a-form-item :label="$t('phone')" :required="false">
            <a-input
              :placeholder="$t('phone')"
              allow-clear
              v-decorator="['phone']"
            />
          </a-form-item>

          <a-form-item>
            <a-button
              type="primary"
              html-type="submit"
              style="margin-left: 8px"
              >{{ $t("Save") }}</a-button
            >
          </a-form-item>
        </a-form>
      </a-col>
      <a-col
        :md="24"
        :lg="8"
        :style="{ minHeight: '180px', textAlign: 'center' }"
      >
        <a-upload
          accept=".png, .jpg, .gif, .jpeg"
          class="ArticleUpload"
          list-type="picture-card"
          @preview="handlePreview"
          :remove="handleRemove"
          @change="handleChange"
          :file-list="fileList"
          :before-upload="
            () => {
              return false;
            }
          "
        >
          <div v-if="fileList.length < 1">
            <a-icon type="plus" />
            <div class="ant-upload-text">图片上传</div>
          </div>
        </a-upload>
        <a-modal
          :visible="previewVisible"
          :footer="null"
          @cancel="handleCancel"
        >
          <img alt="example" style="width: 100%" :src="previewImage" />
        </a-modal>
      </a-col>
    </a-row>
  </div>
</template>

<script>
import AvatarModal from "./AvatarModal";
import {
  isAuthed,
  userUpdate,
  UploadAvatarWith,
  deleteFile,
  FilefindWith,
} from "@/api/system-serve";
import moment from "moment";
import { mapMutations, mapActions } from "vuex";

function getBase64(file) {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.readAsDataURL(file);
    reader.onload = () => resolve(reader.result);
    reader.onerror = (error) => reject(error);
  });
}
export default {
  i18n: require("./i18n"),
  components: {
    AvatarModal,
  },
  data() {
    return {
      form: this.$form.createForm(this, { name: "base" }),
      // 文件
      fileList: [],
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
      // 是否上传图片
      ishandleChange: false,
      // 移除的图片sid
      FileRemoveSID: null,
      /** 图片 */
      previewImage: "",
      previewVisible: false,
      /** 当前登陆人信息 */
      principal: {},
    };
  },
  created() {
    this.$nextTick(() => {
      this.principal = this.$store.getters["account/user"];
      this.doQuery();
    });
  },
  methods: {
    ...mapMutations("account", ["setUser", "setPermissions", "setRoles"]),
    /** 查询当前用户信息 */
    async doQuery() {
      if (this.principal._id) {
        let birth_year = `${this.principal.birth_year}-${
          this.principal.birth_month > 9
            ? `${this.principal.birth_month}`
            : `0${this.principal.birth_month}`
        }-${
          this.principal.birth_day > 9
            ? `${this.principal.birth_day}`
            : `0${this.principal.birth_day}`
        }`;
        this.form.setFieldsValue({
          realname: this.principal.realname,
          desc: this.principal.desc,
          email: this.principal.email,
          birthday: this.principal.birth_year
            ? moment(`${birth_year} 00:00:00`)
            : "",
          sex: this.principal.sex,
          phone: this.principal.phone,
        });
        /** 查询头像 */
        if (this.principal.portrait) {
          FilefindWith({
            body: { _id: this.principal.portrait },
            page: 1,
            pageSize: 10,
          }).then((res) => {
            if (res.data.success && res.data.body.length > 0) {
              Object.assign(res.data.body[0], {
                uid: res.data.body[0]._id,
                name: res.data.body[0].name,
                status: "done",
                url: `/annex/annex/view/${res.data.body[0]._id}`,
              });
              this.fileList = res.data.body;
            }
          });
        }
      }
    },
    /** 保存按钮 */
    handleSubmit(e) {
      e.preventDefault();
      this.form.validateFields((error, values) => {
        if (!error) {
          let obj = {
            ...values,
            birth_year: values.birthday
              ? parseInt(moment(values.birthday).format("YYYY"))
              : undefined,
            birth_month: values.birthday
              ? parseInt(moment(values.birthday).format("MM"))
              : undefined,
            birth_day: values.birthday
              ? parseInt(moment(values.birthday).format("DD"))
              : undefined,
            _id: this.principal._id,
          };
          userUpdate(obj).then((res) => {
            if (res.data.success) {
              if (!this.ishandleChange) {
                this.setUser(res.data.data);
              }
              this.$message.success("修改成功");
            }
          });
          if (this.ishandleChange) {
            this.uploadChange(this.principal._id);
          }
        }
      });
    },
    //===============  图片上传（start） =============
    handleRemove(file) {
      if (file.sid) {
        this.FileRemoveSID = file.sid;
      }
      this.fileList = [];
    },
    handleCancel() {
      this.previewVisible = false;
    },
    async handlePreview(file) {
      if (!file.url && !file.preview) {
        file.preview = await getBase64(file.originFileObj);
      }
      this.previewImage = file.url || file.preview;
      this.previewVisible = true;
    },
    handleChange({ fileList }) {
      console.log(fileList, "fileList");
      this.fileList = [];
      this.ishandleChange = true;
      fileList.map((v) => {
        Object.assign(v, {
          status: "done",
        });
      });
      this.fileList = fileList;
    },
    /**
     * 附件上传
     */
    uploadChange(affiliationId) {
      const formData = new FormData();
      formData.append("file", this.fileList[0].originFileObj);
      formData.append("fileType", this.fileList[0].originFileObj.type);
      UploadAvatarWith(formData, affiliationId).then((res) => {
        if (res.success) {
          this.ishandleChange = false;
          this.setUser(res.body);
          location.reload([false]);
        }
      });
    },
    //=============== 图片上传（end） =============
  },
};
</script>

<style lang="less">
.ArticleUpload {
  width: 170px;
  height: 170px;
  border-radius: 50%;
  .ant-upload.ant-upload-select.ant-upload-select-picture-card {
    height: 100%;
    width: 100%;
    border-radius: 50%;
    .ant-upload-list-item-info {
      border-radius: 50%;
      &::before {
        left: 0;
      }
    }
    i {
      font-size: 32px;
      color: #999;
    }
    .ant-upload-text {
      font-size: 20px;
    }
  }
  .ant-upload-list-picture-card-container {
    height: 100%;
    width: 100%;
    border-radius: 50%;
    .ant-upload-list-item.ant-upload-list-item-done.ant-upload-list-item-list-type-picture-card {
      height: 100%;
      width: 100%;
      border-radius: 50%;
      .ant-upload-list-item-info {
        border-radius: 50%;
        &::before {
          left: 0;
        }
      }
      i {
        font-size: 32px;
        margin: 0 10px;
      }
    }
  }
}
</style>
<style lang="less" scoped>
#myfile {
  display: none;
}
.avatar-upload-wrapper {
  height: 200px;
  width: 100%;
}

.ant-upload-preview {
  position: relative;
  margin: 0 auto;
  width: 100%;
  max-width: 180px;
  border-radius: 50%;
  box-shadow: 0 0 4px #ccc;

  .upload-icon {
    position: absolute;
    top: 0;
    right: 10px;
    font-size: 1.4rem;
    padding: 0.5rem;
    background: rgba(222, 221, 221, 0.7);
    border-radius: 50%;
    border: 1px solid rgba(0, 0, 0, 0.2);
  }
  .mask {
    opacity: 0;
    position: absolute;
    background: rgba(0, 0, 0, 0.4);
    cursor: pointer;
    transition: opacity 0.4s;

    &:hover {
      opacity: 1;
    }

    i {
      font-size: 2rem;
      position: absolute;
      top: 50%;
      left: 50%;
      margin-left: -1rem;
      margin-top: -1rem;
      color: #d6d6d6;
    }
  }

  img,
  .mask {
    width: 100%;
    max-width: 180px;
    height: 100%;
    border-radius: 50%;
    overflow: hidden;
  }
}
</style>
