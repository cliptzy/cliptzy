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
      <span class="font-black tracking-[0.2em] text-sm text-white truncate transition-all duration-300">
        {{ appStore.isSidebarCollapsed ? 'C.' : 'CLIPTZY' }}
      </span>
      <button 
        @click="appStore.toggleSidebar()"
        class="absolute -right-3 top-1/2 -translate-y-1/2 w-6 h-6 bg-[var(--color-surface)] border border-[var(--color-subtle)] rounded-full flex items-center justify-center text-gray-400 hover:text-[var(--color-accent)] hover:border-[var(--color-accent)] transition-all shadow-lg z-50 focus:outline-none"
      >
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
        <button
          @click="navigate"
          class="relative flex items-center rounded-xl transition-all duration-300 group overflow-hidden"
          :class="[
            appStore.isSidebarCollapsed ? 'p-3 justify-center' : 'px-4 py-3 justify-center md:justify-start gap-4',
            'flex-1 md:flex-none h-12 md:h-auto'
          ]"
          :title="appStore.isSidebarCollapsed ? item.name : ''"
        >
          <!-- Active Glow Indicator -->
          <div 
            class="absolute left-0 top-1/2 -translate-y-1/2 w-1 h-[60%] bg-[var(--color-accent)] rounded-r-full transition-all duration-300 shadow-[0_0_12px_var(--color-accent)]"
            :class="isActive ? 'opacity-100 scale-y-100' : 'opacity-0 scale-y-0'"
          ></div>
          
          <div 
            class="absolute inset-0 bg-white/5 transition-opacity duration-300 backdrop-blur-sm"
            :class="isActive ? 'opacity-100' : 'opacity-0 group-hover:opacity-100'"
          ></div>

          <component 
            :is="item.icon" 
            class="w-5 h-5 shrink-0 transition-all duration-300 z-10" 
            :class="isActive ? 'text-[var(--color-accent)] drop-shadow-[0_0_8px_rgba(217,249,157,0.5)]' : 'text-gray-400 group-hover:text-white'" 
          />
          <span 
            v-if="!appStore.isSidebarCollapsed" 
            class="font-medium text-sm whitespace-nowrap z-10 hidden md:block transition-colors duration-300"
            :class="isActive ? 'text-white' : 'text-gray-400 group-hover:text-white'"
          >
            {{ item.name }}
          </span>
        </button>
      </router-link>
    </nav>
  </aside>
</template>

<script setup lang="ts">
import { useAppStore } from '../stores/app';
import IconDashboard from '~icons/lucide/layout-dashboard';
import IconScissors from '~icons/lucide/scissors';
import IconLibrary from '~icons/lucide/library';
import IconSettings from '~icons/lucide/settings';
import IconInfo from '~icons/lucide/info';

const appStore = useAppStore();

const navItems = [
  { name: 'Dashboard', path: '/', icon: IconDashboard },
  { name: 'Studio', path: '/studio', icon: IconScissors },
  { name: 'Library', path: '/library', icon: IconLibrary },
  { name: 'Settings', path: '/settings', icon: IconSettings },
  { name: 'About', path: '/about', icon: IconInfo },
];
</script>
