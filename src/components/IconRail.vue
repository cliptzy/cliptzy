<template>
  <aside 
    class="w-12 h-full bg-base-200 flex flex-col items-center py-4 shrink-0 z-40"
    style="border-right: 1px solid color-mix(in srgb, var(--color-base-content) 8%, transparent)"
  >
    <!-- Nav Icons -->
    <nav class="flex-1 flex flex-col items-center gap-3 w-full px-2">
      <router-link
        v-for="item in navItems"
        :key="item.path"
        :to="item.path"
        v-slot="{ isActive, navigate }"
        custom
      >
        <button
          @click="navigate"
          class="relative w-10 h-10 flex items-center justify-center rounded-none transition-all duration-200 group"
          :class="
            isActive
              ? 'text-base-content bg-base-300'
              : 'text-base-content/60 hover:text-base-content hover:bg-base-300/50'
          "
          :title="item.name"
        >
          <!-- Active indicator bar -->
          <div
            v-if="isActive"
            class="absolute left-0 top-1/2 -translate-y-1/2 w-0.5 h-6 bg-primary rounded-none"
          ></div>
          <component :is="item.icon" class="w-5 h-5 shrink-0" />
        </button>
      </router-link>
    </nav>
    <div class="flex flex-col items-center gap-2 mt-4">
    <button @click="toggleDarkMode"
      class="w-10 h-10 flex items-center justify-center rounded-none text-base-content bg-base-200 hover:bg-base-300 transition-colors duration-200"
      :title="isDark ? 'Switch to Light mode' : 'Switch to Dark mode'">
      <component :is="isDark ? IconMoon : IconSun" class="w-5 h-5"/>
    </button>
  </div>
</aside>
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
  { name: 'Studio', path: '/studio/clipper', icon: IconScissors },
  { name: 'Library', path: '/library', icon: IconLibrary },
  { name: 'Settings', path: '/settings', icon: IconSettings },
  { name: 'About', path: '/about', icon: IconInfo },
];

const isDark = ref(true);

const toggleDarkMode = () => {
  isDark.value = !isDark.value;
  if (isDark.value) {
    document.documentElement.classList.add('dark');
    document.documentElement.setAttribute('data-theme', 'dark');
    localStorage.setItem('theme', 'dark');
  } else {
    document.documentElement.classList.remove('dark');
    document.documentElement.setAttribute('data-theme', 'light');
    localStorage.setItem('theme', 'light');
  }
};

onMounted(() => {
  const savedTheme = localStorage.getItem('theme');
  if (savedTheme === 'light') {
    isDark.value = false;
    document.documentElement.classList.remove('dark');
    document.documentElement.setAttribute('data-theme', 'light');
  } else {
    isDark.value = true;
    document.documentElement.classList.add('dark');
    document.documentElement.setAttribute('data-theme', 'dark');
  }
});
</script>
