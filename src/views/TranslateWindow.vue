<template>
  <el-container class="custom-container">
    <el-header>
      <el-input v-model="inputText" placeholder="Enter text" clearable></el-input>
    </el-header>
    <el-main class="custom-main">
      <el-card class="box-card">
        <div slot="header" class="clearfix">
          <span>OpenAI</span>
          <el-button @click="removeTranslation('openai')" type="text" class="card-close">X</el-button>
        </div>
        <div>
          <p>{{ translatedTextOpenAI }}</p>
          <el-button icon="el-icon-copy-document"></el-button>
          <el-button icon="el-icon-mic"></el-button>
        </div>
      </el-card>
      <el-card class="box-card">
        <div slot="header" class="clearfix">
          <span>Google Translate</span>
          <el-button @click="removeTranslation('google')" type="text" class="card-close">X</el-button>
        </div>
        <div>
          <p>{{ translatedTextGoogle }}</p>
          <el-button icon="el-icon-copy-document"></el-button>
          <el-button icon="el-icon-mic"></el-button>
        </div>
      </el-card>
      <el-card class="box-card">
        <div slot="header" class="clearfix">
          <span>Huoshan Translate</span>
          <el-button @click="removeTranslation('huoshan')" type="text" class="card-close">X</el-button>
        </div>
        <div>
          <p>{{ translatedTextHuoshan }}</p>
          <el-button icon="el-icon-copy-document"></el-button>
          <el-button icon="el-icon-mic"></el-button>
        </div>
      </el-card>
    </el-main>
  </el-container>
</template>

<script setup lang="ts">
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { hideSidebar, showSidebar } from '../store/sidebar';
import { listen } from '@tauri-apps/api/event';
import { onMounted, onUnmounted, ref } from 'vue';
const appWindow = getCurrentWebviewWindow()

console.log(`appWindow.label = ${appWindow.label}`);
listen("tauri://blur", async () => {
  console.log('blur event happend');
})

const inputText = ref('Hello world');

function updateText(newText: string) {
  inputText.value = newText;
}

function removeTranslation(service: string) {
  if (service === 'openai') {
    translatedTextOpenAI.value = '';
  } else if (service === 'google') {
    translatedTextGoogle.value = '';
  } else if (service === 'huoshan') {
    translatedTextHuoshan.value = '';
  }
}

const translatedTextOpenAI = ref("你好，世界");
const translatedTextGoogle = ref("你好，世界");
const translatedTextHuoshan = ref("你好，世界");

listen("new_text", (event) => {
  appWindow.show();
  appWindow.setFocus();
  const text = event.payload as string;
  console.log("received text: ", text);
  updateText(text);
});

onMounted(() => {
  hideSidebar();
});

onUnmounted(() => {
  showSidebar();
});

</script>

<style>
html,
body {
  -ms-overflow-style: none;
  /* IE and Edge */
  scrollbar-width: none;
}

.custom-container {
  width: 100%;
  /* height: 100vh; */
  display: flex;
  /* flex-direction: column; */
  background-color: white;
  /* overflow: hidden; */
}

.box-card {
  margin-bottom: 20px;
}

.card-close {
  float: right;
  margin: 0;
  padding: 0;
}

.el-button {
  margin-right: 10px;
}

/* 隐藏滚动条的样式 */
.custom-main::-webkit-scrollbar {
  width: 0px;
  height: 0px;
}

.custom-main {
  scrollbar-width: none;
  /* 适用于 Firefox */
}

.custom-main {
  -ms-overflow-style: none;
  /* 适用于 Internet Explorer 和 Edge */
}
</style>