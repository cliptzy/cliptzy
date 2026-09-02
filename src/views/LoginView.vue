<script setup lang="ts">
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { useAuthStore } from '../stores/auth';

const auth = useAuthStore();
const router = useRouter();
const isLoggingIn = ref(false);

const handleLogin = async () => {
  isLoggingIn.value = true;
  const success = await auth.login();
  isLoggingIn.value = false;
  if (success) {
    router.push('/');
  }
};

const handleOfflineLogin = () => {
  auth.$patch({
    isLoggedIn: true,
    email: 'offline@cliptzy.com',
    displayName: 'Offline User',
    avatarUrl: null,
    loginError: null
  });
  router.push('/');
};
</script>

<template>
  <div class="flex-1 bg-[var(--color-base)] text-[var(--color-text-main)] flex items-center justify-center p-6 relative font-sans overflow-hidden">
    
    <!-- Spatial background decoration -->
    <div class="absolute inset-0 z-0 opacity-20 pointer-events-none" 
         style="background-image: radial-gradient(circle at center, var(--color-surface) 0%, var(--color-base) 70%);"></div>

    <!-- Spatial Bento Box Container -->
    <div class="relative z-10 w-full max-w-md bg-[var(--color-surface)] border border-[var(--color-border-subtle)] p-8 sm:p-10 rounded-[24px] shadow-2xl backdrop-blur-md">
      
      <!-- Logo Area -->
      <div class="mb-10 text-center flex flex-col items-center">
        <div class="w-16 h-16 rounded-2xl bg-[var(--color-base)] border border-[var(--color-border-subtle)] flex items-center justify-center mb-6 shadow-inner">
           <!-- Simple geometric logo -->
           <div class="w-8 h-8 rounded-full bg-gray-200 dark:bg-gray-800 animate-pulse opacity-90 blur-[2px]"></div>
           <div class="w-4 h-4 rounded-full bg-[var(--color-base)] absolute"></div>
        </div>
        <h1 class="text-3xl font-bold tracking-tight text-[var(--color-text-main)] mb-2">Cliptzy Studio</h1>
        <div class="px-3 py-1 border border-[var(--color-border-subtle)] rounded-full bg-[var(--color-base)]">
          <p class="text-[10px] font-semibold text-[var(--color-text-main)] uppercase tracking-widest">
            Native AI Engine
          </p>
        </div>
      </div>

      <div class="mb-10 text-center">
        <h2 class="text-xl font-medium mb-2 text-gray-200 tracking-tight">System Authentication</h2>
        <p class="text-sm font-normal text-[var(--color-text-muted)] leading-relaxed">Establish connection with your Google account to sync configurations and YouTube automations.</p>
      </div>

      <!-- Action Area -->
      <button @click="handleLogin" :disabled="isLoggingIn" class="w-full relative flex justify-center items-center gap-3 py-4 px-6 border border-[var(--color-border-subtle)] rounded-xl text-sm font-semibold bg-[var(--color-base)] hover:border-[var(--color-accent)] hover:text-[var(--color-text-main)] text-[var(--color-text-main)] transition-all duration-300 disabled:opacity-50 disabled:cursor-not-allowed group overflow-hidden" >
        <div class="absolute inset-0 bg-gray-200 dark:bg-gray-800 opacity-0 group-hover:opacity-[0.03] transition-opacity"></div>
        
        <template v-if="!isLoggingIn">
          <!-- Custom minimal Google Icon -->
          <svg class="w-5 h-5 transition-transform group-hover:scale-110" viewBox="0 0 24 24" fill="currentColor">
            <path d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z" fill="#4285F4" />
            <path d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z" fill="#34A853" />
            <path d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z" fill="#FBBC04" />
            <path d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z" fill="#EA4335" />
          </svg>
          Authenticate with Google
        </template>
        <template v-else>
          <svg class="animate-spin -ml-1 mr-2 h-5 w-5 text-[var(--color-text-main)]" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
          <span class="text-[var(--color-text-main)]">Establishing Connection...</span>
        </template>
      </button>

      <button @click="handleOfflineLogin" :disabled="isLoggingIn" class="w-full mt-4 relative flex justify-center items-center gap-3 py-4 px-6 border border-[var(--color-border-subtle)] rounded-xl text-sm font-semibold bg-[var(--color-base)] hover:border-[var(--color-accent)] hover:text-[var(--color-text-main)] text-[var(--color-text-muted)] transition-all duration-300 disabled:opacity-50 disabled:cursor-not-allowed group overflow-hidden">
        Continue in Offline Mode
      </button>

      <!-- Error State -->
      <div v-if="auth.loginError" class="mt-5 p-4 border border-red-900/50 rounded-xl bg-red-950/20 text-red-400 text-xs text-center backdrop-blur-sm">
        <span class="block font-bold mb-1">Authentication Failed</span>
        {{ auth.loginError }}
      </div>

    </div>
  </div>
</template>
