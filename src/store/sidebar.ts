import { Store } from '@tauri-apps/plugin-store';
import { reactive } from 'vue';

export const sidebarState = reactive({
  isVisible: true
});

export function showSidebar() {
  sidebarState.isVisible = true;
}

export function hideSidebar() {
  sidebarState.isVisible = false;
}

export const store = await Store.load("config.bin");
