<script setup lang="ts">
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { useAuthStore } from '../stores/auth';
import IconGoogle from '~icons/logos/google-icon';

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
</script>

<template>
  <div class="min-h-screen bg-[#F8F9FA] text-black flex items-center justify-center p-6 relative overflow-hidden font-sans">
    
    <!-- Decorative background blocks (Google I/O 2024 inspired) -->
    <!-- Large arch on the left -->
    <div class="absolute bottom-0 left-[5%] w-64 h-80 bg-[#4285F4] border-[3px] border-black rounded-t-full -z-10"></div>
    <!-- Circle on top right -->
    <div class="absolute top-[15%] right-[12%] w-32 h-32 bg-[#FBBC04] border-[3px] border-black rounded-full -z-10"></div>
    <!-- Pill shape bottom right -->
    <div class="absolute bottom-[20%] right-[8%] w-48 h-20 bg-[#34A853] border-[3px] border-black rounded-full -z-10 rotate-[-15deg]"></div>
    <!-- Semi circle top left -->
    <div class="absolute top-0 left-[20%] w-40 h-20 bg-[#EA4335] border-[3px] border-black border-t-0 rounded-b-full -z-10"></div>

    <!-- Main Login Block -->
    <div class="relative z-10 w-full max-w-md bg-white border-[3px] border-black p-10 rounded-[32px]">
      
      <!-- Title Area -->
      <div class="mb-10 text-center flex flex-col items-center">
        <!-- Logo placeholder with Google colors -->
        <div class="flex gap-1.5 mb-6">
          <div class="w-5 h-5 rounded-full bg-[#4285F4] border-[2px] border-black"></div>
          <div class="w-5 h-5 rounded-full bg-[#EA4335] border-[2px] border-black"></div>
          <div class="w-5 h-5 rounded-full bg-[#FBBC04] border-[2px] border-black"></div>
          <div class="w-5 h-5 rounded-full bg-[#34A853] border-[2px] border-black"></div>
        </div>
        <h1 class="text-5xl font-black tracking-tighter mb-4 text-black">Cliptzy</h1>
        <div class="px-5 py-1.5 border-[2px] border-black rounded-full bg-[#F8F9FA]">
          <p class="text-xs font-bold text-black uppercase tracking-widest">
            AI Engine Workspace
          </p>
        </div>
      </div>

      <div class="mb-10 text-center">
        <h2 class="text-2xl font-bold mb-3 tracking-tight">Welcome Back</h2>
        <p class="text-sm font-medium text-gray-600 leading-relaxed px-2">Sign in to sync your AI configuration and connect to YouTube automations.</p>
      </div>

      <!-- Google Login Button -->
      <button
        @click="handleLogin"
        :disabled="isLoggingIn"
        class="w-full flex justify-center items-center gap-3 py-4 px-6 border-[3px] border-black rounded-full text-base font-bold bg-white hover:bg-[#F8F9FA] text-black transition-colors disabled:opacity-70 disabled:cursor-not-allowed group"
      >
        <template v-if="!isLoggingIn">
          <IconGoogle class="w-6 h-6 group-hover:scale-110 transition-transform" />
          Continue with Google
        </template>
        <template v-else>
          <svg class="animate-spin -ml-1 mr-2 h-6 w-6 text-black" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
          <span>Waiting for Browser...</span>
        </template>
      </button>

      <div v-if="auth.loginError" class="mt-6 p-4 border-[3px] border-black rounded-2xl bg-[#FCE8E6] text-black font-bold text-center">
        ⚠️ {{ auth.loginError }}
      </div>

    </div>
  </div>
</template>
