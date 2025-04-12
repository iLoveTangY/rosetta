<template>
  <div id="app" class="app-container">
    <el-container>
      <el-row>
        <el-col :span="24">
          <el-card class="card">
            <h2 class="card-title">百度翻译</h2>
            <el-form :model="form" :rules="rules" class="form">
              <el-form-item label="AppID" prop="appid">
                <el-input v-model="form.appid" placeholder="请输入 AppID" class="input"></el-input>
              </el-form-item>

              <el-form-item label="密钥" prop="secretKey">
                <el-input v-model="form.secretKey" placeholder="请输入密钥" type="password" class="input"></el-input>
              </el-form-item>

              <el-form-item>
                <el-button type="primary" @click="submitForm" class="submit-button">提交</el-button>
              </el-form-item>
            </el-form>
          </el-card>
        </el-col>
      </el-row>
    </el-container>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { store } from '../store/sidebar';
const form = ref({
  appid: '',
  secretKey: ''
});
const rules = {
  appid: [
    { required: true, message: '请输入 AppID', trigger: 'blur' }
  ],
  secretKey: [
    { required: true, message: '请输入密钥', trigger: 'blur' }
  ]
};

const submitForm = async () => {
  // 这里你可以处理表单提交的逻辑
  console.log('提交的数据：', form.value);
  // 例如，你可以发送请求到后端接口，传递 AppID 和 密钥
  await store.set("baidu", form.value);
};
</script>

<style scoped>
/* 背景渐变 */
.app-container {
  height: 100vh;
  display: flex;
  justify-content: center;
  align-items: center;
  font-family: 'Helvetica Neue', Arial, sans-serif;
}

/* 卡片样式 */
.card {
  border-radius: 15px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  padding: 20px;
}

/* 标题样式 */
.card-title {
  font-size: 24px;
  font-weight: bold;
  color: #4A4A4A;
  margin-bottom: 20px;
  text-align: center;
}

/* 表单样式 */
.form {
  max-width: 100%;
  margin: 0 auto;
  width: 100%;
}

/* 输入框样式 */
.input {
  border-radius: 10px;
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.1);
  transition: all 0.3s ease;
}

/* 输入框聚焦时的样式 */
.input:focus {
  border-color: #409EFF;
  box-shadow: 0 0 5px rgba(64, 158, 255, 0.5);
}

/* 按钮样式 */
.submit-button {
  width: 100%;
  background-color: #409EFF;
  border-radius: 10px;
  padding: 12px;
  font-size: 16px;
  color: #fff;
  transition: background-color 0.3s ease;
}

/* 按钮 hover 效果 */
.submit-button:hover {
  background-color: #66b1ff;
}

/* 按钮点击时的效果 */
.submit-button:active {
  background-color: #3a8ee6;
}
</style>
