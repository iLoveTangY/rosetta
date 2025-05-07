<template>
  <div id="app" class="app-container">
    <el-container>
      <el-main class="main-container">
        <div class="settings-header">
          <h2>翻译设置</h2>
          <p class="settings-description">配置翻译服务的相关参数</p>
        </div>
        <el-card class="settings-card">
          <div class="service-section">
            <div class="service-header">
              <img src="https://fanyi-cdn.cdn.bcebos.com/static/trans-pc/static/media/logo.f7f354dc.svg" alt="百度翻译"
                class="service-icon" />
              <h3>百度翻译</h3>
            </div>

            <el-form :model="form" :rules="rules" class="form">
              <el-form-item label="AppID" prop="appid">
                <el-input v-model="form.appid" placeholder="请输入 AppID" class="settings-input"></el-input>
                <div class="input-description">在百度翻译开放平台获取的应用ID</div>
              </el-form-item>

              <el-form-item label="密钥" prop="secretKey">
                <el-input v-model="form.secret" placeholder="请输入密钥" class="settings-input" show-password></el-input>
                <div class="input-description">在百度翻译开放平台获取的应用密钥</div>
              </el-form-item>

              <el-form-item class="form-actions">
                <el-button type="primary" @click="submitForm" class="submit-button">
                  保存设置
                </el-button>
              </el-form-item>
            </el-form>
          </div>
        </el-card>
      </el-main>
    </el-container>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { TranslaterService } from '../store/sidebar';
import { invoke } from '@tauri-apps/api/core';
import { BaiduConfig } from '../types/TranslateConfigs';
import { ElMessage } from 'element-plus';

const form = ref<BaiduConfig>({
  appid: '',
  secret: ''
});

const rules = {
  appid: [
    { required: true, message: '请输入 AppID', trigger: 'blur' }
  ],
  secretKey: [
    { required: true, message: '请输入密钥', trigger: 'blur' }
  ]
};

const initForm = async () => {
  console.log('init form');
  const res = await invoke("get_config", { key: TranslaterService.BAIDU }) as BaiduConfig;
  console.log("res = ", res);
  form.value.appid = res.appid;
  form.value.secret = res.secret;
}

const submitForm = async () => {
  try {
    await invoke("set_config", { key: TranslaterService.BAIDU, value: form.value });
    ElMessage({
      message: '保存成功',
      type: 'success',
      duration: 2000,
      center: true
    });
  } catch (e: unknown) {
    ElMessage({
      message: `保存失败: ${(e as Error).message}`,
      type: 'error',
      duration: 2000,
      center: true
    });
  }
};

initForm();
</script>

<style scoped>
.app-container {
  min-height: 100vh;
  /* background-color: #f5f5f7; */
  padding: 3px;
}

.main-container {
  /* max-width: 800px; */
  min-width: fit-content;
  margin: 0 auto;
  /* padding: 20px; */
}

.settings-card {
  background: #ffffff;
  border-radius: 12px;
  /* border: 1px solid #e8e8e8; */
  /* box-shadow: none; */
}

.settings-header {
  /* margin-bottom: 30px; */
  padding-bottom: 3px;
  /* border-bottom: 1px solid #e8e8e8; */
}

.settings-header h2 {
  font-size: 24px;
  color: #1d1d1f;
  margin: 0 0 8px 0;
  font-weight: 500;
}

.settings-description {
  color: #86868b;
  font-size: 14px;
  margin: 0;
}

.service-section {
  padding: 3px 0;
}

.service-header {
  display: flex;
  align-items: center;
  margin-bottom: 24px;
}

.service-icon {
  width: 32px;
  height: 32px;
  margin-right: 12px;
}

.service-header h3 {
  font-size: 18px;
  color: #1d1d1f;
  margin: 0;
  font-weight: 500;
}

.settings-input {
  transition: all 0.3s ease;
  border-radius: 8px;
}

.settings-input:hover {
  border-color: #d2d2d7;
}

.settings-input:focus {
  border-color: #0071e3;
  box-shadow: 0 0 0 3px rgba(0, 113, 227, 0.1);
}

.input-description {
  font-size: 12px;
  color: #86868b;
  margin-top: 4px;
}

.form-actions {
  margin-top: 15px;
  width: 50%;
}

.submit-button {
  background-color: #0071e3;
  border-color: #0071e3;
  border-radius: 8px;
  padding: 10px 20px;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.3s ease;
  display: inline-block;
  width: 50%;
  margin-left: 70%;
  /* justify-content: center; */
}

.submit-button:hover {
  background-color: #0077ed;
  border-color: #0077ed;
  transform: translateY(-1px);
}

.submit-button:active {
  background-color: #006edb;
  border-color: #006edb;
  transform: translateY(0);
}

:deep(.el-form-item__label) {
  font-weight: 500;
  color: #1d1d1f;
}

:deep(.el-input__inner) {
  border-radius: 8px;
}

:deep(.el-form-item) {
  margin-bottom: 16px;
}

:deep(.el-card) {
  box-shadow: none !important;
  border: 1px solid #e8e8e8;
  background-color: #ffffff;
}

:deep(.el-card__body) {
  padding: 10px;
}
</style>
