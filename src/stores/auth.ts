import { defineStore } from 'pinia';
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

export const useAuthStore = defineStore('auth', () => {
  const isLoggedIn = ref<boolean>(false);
  const email = ref<string | null>(null);
  const displayName = ref<string | null>(null);
  const avatarUrl = ref<string | null>(null);
  const isChecking = ref<boolean>(true);
  const loginError = ref<string | null>(null);

  async function checkAuthStatus() {
    isChecking.value = true;
    try {
      const res = await invoke<any>('get_user_info');
      if (res) {
        isLoggedIn.value = true;
        email.value = res.email;
        displayName.value = res.display_name;
        avatarUrl.value = res.avatar_url;
      } else {
        isLoggedIn.value = false;
        email.value = null;
        displayName.value = null;
        avatarUrl.value = null;
      }
    } catch (e: any) {
      console.error('Failed to check auth status:', e);
      isLoggedIn.value = false;
    } finally {
      isChecking.value = false;
    }
  }

  async function login() {
    loginError.value = null;
    try {
      const success = await invoke<boolean>('login_with_google');
      if (success) {
        await checkAuthStatus();
        return true;
      }
    } catch (e: any) {
      console.error('Login failed:', e);
      loginError.value = e.toString() || 'Login failed or was cancelled.';
      return false;
    }
    return false;
  }

  async function logout() {
    try {
      await invoke('logout');
    } catch (e) {
      console.error('Logout failed:', e);
    }
    isLoggedIn.value = false;
    email.value = null;
    displayName.value = null;
    avatarUrl.value = null;
  }

  return { 
    isLoggedIn, 
    email, 
    displayName, 
    avatarUrl, 
    isChecking, 
    loginError, 
    checkAuthStatus, 
    login, 
    logout 
  };
});
