<template>
  <div id="app">
    <el-container style="height: 100vh;">
      <el-aside v-if="sidebarState.isVisible" width="200px">
        <el-menu :default-active="activeMenu" class="el-menu-vertical-demo" @select="handleSelect">
          <el-menu-item index="translate-settings">
            <i class="el-icon-document"></i>
            <span slot="title">翻译设置</span>
          </el-menu-item>
          <el-menu-item index="ocr-settings">
            <i class="el-icon-camera"></i>
            <span slot="title">OCR 设置</span>
          </el-menu-item>
          <el-menu-item index="general-settings">
            <i class="el-icon-setting"></i>
            <span slot="title">通用设置</span>
          </el-menu-item>
        </el-menu>
      </el-aside>
      <el-main>
        <router-view></router-view>
      </el-main>
    </el-container>
  </div>
</template>

<script lang="ts">
import { computed } from 'vue';
import { sidebarState } from './store/sidebar';

export default {
  name: 'App',
  data() {
    return {
      activeMenu: 'translate-settings',
      sidebarState: computed(() => sidebarState)
    };
  },
  methods: {
    handleSelect(key: string) {
      this.$router.push({ path: `/${key}` });
      this.activeMenu = key;
    }
  }
};
</script>

<style>
#app {
  font-family: 'Avenir', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
}

.el-menu-vertical-demo:not(.el-menu--collapse) {
  width: 200px;
  min-height: 400px;
}

.el-main {
  padding: 20px;
}
</style>
