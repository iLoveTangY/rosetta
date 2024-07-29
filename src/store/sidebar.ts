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
