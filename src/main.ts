import { createApp } from 'vue';
import App from './App.vue';
import router from './router';
import ElementPlus from 'element-plus';
import 'element-plus/dist/index.css';
import { appWindow } from '@tauri-apps/api/window';

console.log(`appWindow.label = ${appWindow.label}`);

const app = createApp(App);

app.use(router);
app.use(ElementPlus);

app.mount('#app');
