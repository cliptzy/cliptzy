<template>
  <aside 
    class="flex-shrink-0 bg-[var(--color-surface)] border-t md:border-t-0 md:border-r border-[var(--color-subtle)] transition-all duration-300 z-40 flex md:flex-col"
    :class="[
      appStore.isSidebarCollapsed ? 'md:w-20' : 'md:w-64',
      'w-full md:w-auto md:h-full h-16 order-last md:order-first shadow-[4px_0_24px_rgba(0,0,0,0.5)]'
    ]"
  >
    <!-- Logo Area (Hidden on mobile) -->
    <div class="hidden md:flex h-[72px] items-center relative px-4" :class="[appStore.isSidebarCollapsed ? 'justify-center' : 'justify-between']">
      <span class="font-black tracking-[0.2em] text-sm text-[var(--color-text-main)] truncate transition-all duration-300">
        {{ appStore.isSidebarCollapsed ? 'C.' : 'CLIPTZY' }}
      </span>
      <button @click="appStore.toggleSidebar()" class="absolute -right-3 top-1/2 -translate-y-1/2 w-6 h-6 bg-[var(--color-surface)] border border-[var(--color-subtle)] rounded-full flex items-center justify-center text-[var(--color-text-muted)] hover:text-gray-900 dark:text-gray-100 hover:border-[var(--color-accent)] transition-all shadow-lg z-50 focus:outline-none" >
        <div class="w-1.5 h-1.5 rounded-full bg-current transition-colors"></div>
      </button>
    </div>

    <!-- Navigation Links -->
    <nav class="flex-1 px-2 py-2 flex md:flex-col justify-around md:justify-start gap-3 overflow-y-auto md:mt-2">
      <router-link 
        v-for="item in navItems" 
        :key="item.path"
        :to="item.path" 
        v-slot="{ isActive, navigate }"
        custom
      >
        <button @click="navigate" class="relative flex items-center rounded-xl transition-all duration-300 group overflow-hidden" :class="[ appStore.isSidebarCollapsed ? 'p-3 justify-center' : 'px-4 py-3 justify-center md:justify-start gap-4', 'flex-1 md:flex-none h-12 md:h-auto' ]" :title="appStore.isSidebarCollapsed ? item.name : ''" >
          <!-- Active Glow Indicator -->
          <div 
            class="absolute left-0 top-1/2 -translate-y-1/2 w-1 h-[60%] bg-gray-200 dark:bg-gray-800 rounded-r-full transition-all duration-300 shadow-[0_0_12px_var(--color-accent)]"
            :class="isActive ? 'opacity-100 scale-y-100' : 'opacity-0 scale-y-0'"
          ></div>
          
          <div 
            class="absolute inset-0 bg-black/5 dark:bg-white/5 transition-opacity duration-300 backdrop-blur-sm"
            :class="isActive ? 'opacity-100' : 'opacity-0 group-hover:opacity-100'"
          ></div>

          <component 
            :is="item.icon" 
            class="w-5 h-5 shrink-0 transition-all duration-300 z-10" 
            :class="isActive ? 'text-gray-900 dark:text-gray-100 drop-shadow-[0_0_8px_rgba(217,249,157,0.5)]' : 'text-[var(--color-text-muted)] group-hover:text-[var(--color-text-main)]'" 
          />
          <span 
            v-if="!appStore.isSidebarCollapsed" 
            class="font-medium text-sm whitespace-nowrap z-10 hidden md:block transition-colors duration-300"
            :class="isActive ? 'text-[var(--color-text-main)]' : 'text-[var(--color-text-muted)] group-hover:text-[var(--color-text-main)]'"
          >
            {{ item.name }}
          </span>
        </button>
      </router-link>

      <div class="mt-auto pt-4 md:flex hidden flex-col">
        <button @click="toggleDarkMode" class="relative flex items-center rounded-xl transition-all duration-300 group overflow-hidden" :class="[ appStore.isSidebarCollapsed ? 'p-3 justify-center' : 'px-4 py-3 justify-center md:justify-start gap-4', 'flex-1 md:flex-none h-12 md:h-auto' ]" :title="appStore.isSidebarCollapsed ? 'Toggle Theme' : ''" >
          <div class="absolute inset-0 bg-black/5 dark:bg-white/5 opacity-0 group-hover:opacity-100 transition-opacity duration-300 backdrop-blur-sm"></div>
          <IconSun v-if="isDark" class="w-5 h-5 shrink-0 transition-all duration-300 z-10 text-[var(--color-text-muted)] group-hover:text-[var(--color-text-main)]" />
          <IconMoon v-else class="w-5 h-5 shrink-0 transition-all duration-300 z-10 text-[var(--color-text-muted)] group-hover:text-[var(--color-text-main)]" />
          <span 
            v-if="!appStore.isSidebarCollapsed" 
            class="font-medium text-sm whitespace-nowrap z-10 hidden md:block transition-colors duration-300 text-[var(--color-text-muted)] group-hover:text-[var(--color-text-main)]"
          >
            {{ isDark ? 'Light Mode' : 'Dark Mode' }}
          </span>
        </button>
      </div>
    </nav>
  </aside>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useAppStore } from '../stores/app';
import IconDashboard from '~icons/lucide/layout-dashboard';
import IconScissors from '~icons/lucide/scissors';
import IconLibrary from '~icons/lucide/library';
import IconSettings from '~icons/lucide/settings';
import IconInfo from '~icons/lucide/info';
import IconSun from '~icons/lucide/sun';
import IconMoon from '~icons/lucide/moon';

const appStore = useAppStore();

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
