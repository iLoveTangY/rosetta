<template>
  <div id="app">
    <el-container style="height: 100vh;">
      <el-aside v-if="sidebarState.isVisible">
        <el-menu :default-active="'translate-settings'" class="el-menu-vertical-demo">
          <RouterLink to="/translate-settings">
            <el-menu-item index="translate-settings">
              <el-icon><Document /></el-icon>
              <span slot="title">翻译设置</span>
            </el-menu-item>
          </RouterLink>

          <RouterLink to="/ocr-settings">
            <el-menu-item index="ocr-settings">
              <el-icon><Camera /></el-icon>
              <span slot="title">OCR 设置</span>
            </el-menu-item>
          </RouterLink>
          
          <RouterLink to="general-settings">
            <el-menu-item index="general-settings">
              <el-icon><Setting /></el-icon>
              <span slot="title">通用设置</span>
            </el-menu-item>
          </RouterLink>
        </el-menu>
      </el-aside>
      <el-main>
        <router-view></router-view>
      </el-main>
    </el-container>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { sidebarState as s } from './store/sidebar';
import { RouterLink } from 'vue-router';
const activeMenu = ref("translate-settings");
const sidebarState = computed(() => s);

// export default {
//   name: 'App',
//   data() {
//     return {
//       activeMenu: 'translate-settings',
//       sidebarState: computed(() => sidebarState)
//     };
//   },
//   methods: {
//     handleSelect(key: string) {
//       this.$router.push({ path: `/${key}` });
//       this.activeMenu = key;
//     }
//   }
// };
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
  width: 150px;
  min-height: 400px;
  align-content: center;
}

.el-main {
  padding: 20px;
}
</style>
