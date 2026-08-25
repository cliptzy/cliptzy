<script setup lang="ts">
import { computed } from 'vue';
import { useRoute } from 'vue-router';
import { useAuthStore } from './stores/auth';
import MainLayout from './layouts/MainLayout.vue';
import BlankLayout from './layouts/BlankLayout.vue';

const route = useRoute();
const auth = useAuthStore();

const layout = computed(() => {
  return route.name === 'login' ? BlankLayout : MainLayout;
});
</script>

<template>
  <div class="fixed top-0 right-0 z-[9999] bg-black text-white p-2 text-xs font-mono">
    Route: {{ String(route.name) }} | LoggedIn: {{ auth.isLoggedIn }} | Checking: {{ auth.isChecking }}
  </div>
  <component :is="layout" />
</template>