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
  <div class="flex-1 bg-base-100 text-base-content flex items-center justify-center p-6">
    
    <!-- Centered card -->
    <div class="w-full max-w-md bg-base-200 border border-neutral rounded-none p-8 sm:p-10">
      
      <!-- Logo Area -->
      <div class="mb-8 text-center flex flex-col items-center">
        <div class="w-16 h-16 rounded-none bg-base-300 flex items-center justify-center mb-4">
           <div class="w-8 h-8 rounded-none bg-primary opacity-80"></div>
        </div>
        <h1 class="text-2xl font-bold tracking-tight text-base-content mb-2">Cliptzy</h1>
        <p class="text-xs text-secondary uppercase tracking-wider font-semibold">
          Video Clipper & Auto Uploader
        </p>
      </div>

      <div class="mb-8 text-center">
        <p class="text-sm text-secondary leading-relaxed">Connect with Google to sync your configurations and enable YouTube automation.</p>
      </div>

            <!-- Action Area -->
      <CButton
        @click="handleLogin"
        :loading="isLoggingIn"
        :disabled="isLoggingIn"
        variant="primary"
        block
        
        class="mb-4"
      >
        Authenticate with Google
      </CButton>

      <CButton
        @click="handleOfflineLogin"
        :disabled="isLoggingIn"
        variant="secondary"
        block
        
      >
        Continue in Offline Mode
      </CButton>

      <!-- Error State -->
      <div v-if="auth.loginError" class="mt-5 p-4 border border-error/50 rounded-none bg-error/10 text-error text-xs text-center">
        <span class="block font-bold mb-1">Authentication Failed</span>
        {{ auth.loginError }}
      </div>

    </div>
  </div>
</template>


