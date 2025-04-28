import { appConfigDir, join } from '@tauri-apps/api/path';
import { Store } from '@tauri-apps/plugin-store';
import { reactive } from 'vue';

export enum TranslaterService {
  BAIDU = "baidu",
  HUOSHAN = "huoshan"
}

export const sidebarState = reactive({
  isVisible: true
});

export function showSidebar() {
  sidebarState.isVisible = true;
}

export function hideSidebar() {
  sidebarState.isVisible = false;
}
