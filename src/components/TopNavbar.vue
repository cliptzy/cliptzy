<template>
  <header class="w-full flex justify-center py-4 px-6 z-40 transition-all duration-300">
    <nav class="bg-[var(--color-surface)] shadow-sm rounded-full px-2 py-1.5 border border-[var(--color-subtle)] flex items-center gap-1">
      <!-- Logo -->
      <div class="px-4 flex items-center mr-2">
        <span class="font-black tracking-[0.2em] text-sm text-[var(--color-text-main)]">CLIPTZY</span>
      </div>
      
      <!-- Nav Items -->
      <router-link 
        v-for="item in navItems" 
        :key="item.path"
        :to="item.path" 
        v-slot="{ isActive, navigate }"
        custom
      >
        <button @click="navigate" class="relative px-4 py-2 rounded-full flex items-center gap-2 transition-all duration-300 text-sm font-medium" :class="isActive ? 'bg-gray-200 dark:bg-gray-800 text-white shadow-md' : 'text-[var(--color-text-muted)] hover:text-[var(--color-text-main)] hover:bg-black/5 dark:hover:bg-white/5'" >
          <component :is="item.icon" class="w-4 h-4 shrink-0" :class="isActive ? 'text-white' : ''" />
          <span>{{ item.name }}</span>
        </button>
      </router-link>

      <!-- Theme Toggle -->
      <div class="w-px h-5 bg-[var(--color-subtle)] mx-2"></div>
      <button @click="toggleDarkMode" class="p-2 rounded-full text-[var(--color-text-muted)] hover:text-[var(--color-text-main)] hover:bg-black/5 dark:hover:bg-white/5 transition-all duration-300" >
        <IconSun v-if="isDark" class="w-4 h-4" />
        <IconMoon v-else class="w-4 h-4" />
      </button>
    </nav>
  </header>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import IconDashboard from '~icons/lucide/layout-dashboard';
import IconScissors from '~icons/lucide/scissors';
import IconLibrary from '~icons/lucide/library';
import IconSettings from '~icons/lucide/settings';
import IconInfo from '~icons/lucide/info';
import IconSun from '~icons/lucide/sun';
import IconMoon from '~icons/lucide/moon';

const navItems = [
  { name: 'Dashboard', path: '/', icon: IconDashboard },
  { name: 'Studio', path: '/studio', icon: IconScissors },
  { name: 'Library', path: '/library', icon: IconLibrary },
  { name: 'Settings', path: '/settings', icon: IconSettings },
  { name: 'About', path: '/about', icon: IconInfo },
];

const isDark = ref(true);

const toggleDarkMode = () => {
  isDark.value = !isDark.value;
  if (isDark.value) {
    document.documentElement.classList.add('dark');
    localStorage.setItem('theme', 'dark');
  } else {
    document.documentElement.classList.remove('dark');
    localStorage.setItem('theme', 'light');
  }
};

onMounted(() => {
  const savedTheme = localStorage.getItem('theme');
  if (savedTheme === 'light') {
    isDark.value = false;
    document.documentElement.classList.remove('dark');
  } else {
    isDark.value = true;
    document.documentElement.classList.add('dark');
  }
});
</script>
