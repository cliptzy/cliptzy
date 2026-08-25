<script setup lang="ts">
import { onMounted, computed } from 'vue';
import { useAuthStore } from './stores/auth';
import { useRouter, useRoute } from 'vue-router';
import AppLayout from './layouts/AppLayout.vue';
import AuthLayout from './layouts/AuthLayout.vue';

const auth = useAuthStore();
const router = useRouter();
const route = useRoute();

onMounted(async () => {
  await auth.checkAuthStatus();
  if (auth.isLoggedIn && router.currentRoute.value.path === '/login') {
    router.push('/');
  } else if (!auth.isLoggedIn && router.currentRoute.value.meta.requiresAuth) {
    router.push('/login');
  }
});

const layout = computed(() => {
  return route.meta.layout === 'AuthLayout' ? AuthLayout : AppLayout;
});
</script>

<template>
  <component :is="layout" />
</template>