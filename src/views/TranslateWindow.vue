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

<script lang="ts">
import { appWindow } from '@tauri-apps/api/window';
import { hideSidebar, showSidebar } from '../store/sidebar';
import { listen } from '@tauri-apps/api/event';
import { ref } from 'vue';

console.log(`appWindow.label = ${appWindow.label}`);
listen("tauri://blur", async () => {
  console.log('blur event happend');
})

export default {
  name: 'TranslateWindow',
  setup() {
    console.log("call setup");
    const inputText = ref("Hello World");

    const updateText = (new_text: string) => {
      inputText.value = new_text;
    }

    listen("new_text", (event) => {
      appWindow.show();
      appWindow.setFocus();
      const text = event.payload as string;
      console.log("received text: ", text);
      updateText(text);
    });

    return {
      inputText,
    }
  },

  data() {
    return {
      translatedTextOpenAI: '你好，世界',
      translatedTextGoogle: '你好世界',
      translatedTextHuoshan: '你好世界',
    };
  },
  methods: {
    // TODO: 细化类型
    removeTranslation(service: string) {
      if (service === 'openai') {
        this.translatedTextOpenAI = '';
      } else if (service === 'google') {
        this.translatedTextGoogle = '';
      } else if (service === 'huoshan') {
        this.translatedTextHuoshan = '';
      }
    },
  },
  mounted() {
    hideSidebar();
  },
  unmounted() {
    showSidebar();
  }
};
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