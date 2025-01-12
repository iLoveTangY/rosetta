import { createRouter, createWebHistory } from 'vue-router';
import TranslateSettings from '../views/TranslateSettings.vue';
import OcrSettings from '../views/OcrSettings.vue';
import GeneralSettings from '../views/GeneralSettings.vue';
import TranslateWindow from '../views/TranslateWindow.vue';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
const appWindow = getCurrentWebviewWindow()

const routes = [
  {
    path: '/',
    // FIXME: don't use any
    redirect: (_to: any) => {
      const label = appWindow.label;
      if (label === 'main') {
        return { name: 'TranslateSettings' }
      } else {
        return { name: 'TranslateWindow' }
      }
    }
  },
  {
    path: '/translate-settings',
    name: 'TranslateSettings',
    component: TranslateSettings
  },
  {
    path: '/ocr-settings',
    name: 'OcrSettings',
    component: OcrSettings
  },
  {
    path: '/general-settings',
    name: 'GeneralSettings',
    component: GeneralSettings
  },
  {
    path: '/translate-window',
    name: 'TranslateWindow',
    component: TranslateWindow,
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes
});

console.log(`${appWindow.label}`);

export default router;
