<script setup lang="ts">
import { ref } from 'vue';
import { useAuthStore } from '../stores/auth';
import { useRouter } from 'vue-router';
import EngineStatus from '../components/EngineStatus.vue';
import LogViewer from '../components/LogViewer.vue';

import IconDashboard from '~icons/lucide/layout-dashboard';
import IconScissors from '~icons/lucide/scissors';
import IconLayers from '~icons/lucide/layers';
import IconUpload from '~icons/lucide/upload-cloud';
import IconSettings from '~icons/lucide/settings';
import IconChevronRight from '~icons/lucide/chevron-right';
import IconChevronLeft from '~icons/lucide/chevron-left';
import IconLogOut from '~icons/lucide/log-out';
import IconMenu from '~icons/lucide/menu';

const auth = useAuthStore();
const router = useRouter();

const isSidebarExpanded = ref(false); // Default to collapsed
const isMobileMenuOpen = ref(false);

const toggleSidebar = () => {
  isSidebarExpanded.value = !isSidebarExpanded.value;
};

const handleLogout = async () => {
  await auth.logout();
  router.push('/login');
};

const navItems = [
  { name: 'Dashboard', path: '/', icon: IconDashboard, color: 'bg-[#4285F4]' },
  { name: 'Clipper', path: '/clipper', icon: IconScissors, color: 'bg-[#EA4335]' },
  { name: 'Compilation', path: '/compilation', icon: IconLayers, color: 'bg-[#FBBC04]' },
  { name: 'Upload', path: '/upload', icon: IconUpload, color: 'bg-[#34A853]' },
  { name: 'Settings', path: '/settings', icon: IconSettings, color: 'bg-black dark:bg-[#E8EAED]' },
];
</script>

<template>
  <div class="h-screen w-full bg-[#F8F9FA] dark:bg-[#121212] flex font-sans text-black dark:text-white transition-colors overflow-hidden">
    
    <!-- Mobile Header (Visible only on small screens) -->
    <div class="md:hidden fixed top-0 left-0 right-0 h-16 bg-white dark:bg-[#1E1E1E] border-b-[3px] border-black dark:border-[#3C4043] z-50 flex items-center justify-between px-4 transition-colors">
      <div class="flex items-center gap-3">
        <span class="text-xl font-black tracking-tighter bg-black dark:bg-[#E8EAED] text-white dark:text-black px-3 py-1 rounded-full border-[2px] border-black dark:border-transparent">C.</span>
      </div>
      <button @click="isMobileMenuOpen = !isMobileMenuOpen" class="p-2 border-[2px] border-black dark:border-[#5F6368] rounded-full hover:bg-gray-100 dark:hover:bg-[#3C4043]">
        <IconMenu class="w-5 h-5" />
      </button>
    </div>

    <!-- Sidebar -->
    <aside 
      :class="[
        'fixed md:static top-0 left-0 h-full bg-white dark:bg-[#1E1E1E] border-r-[3px] border-black dark:border-[#3C4043] transition-all duration-300 z-40 flex flex-col',
        isSidebarExpanded ? 'w-64' : 'w-20',
        isMobileMenuOpen ? 'translate-x-0' : '-translate-x-full md:translate-x-0'
      ]"
    >
      <!-- Logo Area -->
      <div class="h-20 flex items-center justify-center border-b-[3px] border-black dark:border-[#3C4043] relative transition-colors">
        <span 
          v-if="isSidebarExpanded"
          class="text-2xl font-black tracking-tighter bg-black dark:bg-[#E8EAED] text-white dark:text-black px-4 py-1.5 rounded-full border-[2px] border-black dark:border-transparent transition-all truncate"
        >
          CLIPTZY
        </span>
        <span 
          v-else
          class="text-2xl font-black tracking-tighter bg-black dark:bg-[#E8EAED] text-white dark:text-black px-3 py-1 rounded-full border-[2px] border-black dark:border-transparent transition-all"
        >
          C.
        </span>

        <!-- Expand Toggle Button (Hidden on Mobile) -->
        <button 
          @click="toggleSidebar"
          class="hidden md:flex absolute -right-4 top-1/2 -translate-y-1/2 bg-white dark:bg-[#28292C] border-[3px] border-black dark:border-[#3C4043] rounded-full p-1 hover:bg-gray-100 dark:hover:bg-[#3C4043] transition-colors z-50 text-black dark:text-white"
        >
          <IconChevronLeft v-if="isSidebarExpanded" class="w-4 h-4" />
          <IconChevronRight v-else class="w-4 h-4" />
        </button>
      </div>

      <!-- Navigation Links -->
      <nav class="flex-1 py-6 px-3 flex flex-col gap-3 overflow-y-auto overflow-x-hidden">
        <router-link 
          v-for="item in navItems" 
          :key="item.path"
          :to="item.path" 
          v-slot="{ isActive, navigate }"
          custom
        >
          <button
            @click="() => { navigate(); isMobileMenuOpen = false; }"
            :class="[
              'flex items-center rounded-full border-[2px] transition-all duration-200 group relative shrink-0',
              isSidebarExpanded ? 'px-4 py-3 justify-start gap-4' : 'p-3 justify-center w-12 h-12 mx-auto',
              isActive 
                ? `${item.color} text-white border-black dark:border-transparent dark:text-black` 
                : 'border-transparent hover:bg-gray-100 dark:hover:bg-[#3C4043] text-gray-700 dark:text-gray-300'
            ]"
            :title="!isSidebarExpanded ? item.name : ''"
          >
            <!-- Settings dark mode logic for text contrast -->
            <component :is="item.icon" :class="['w-5 h-5 shrink-0', isActive && item.name === 'Settings' ? 'dark:text-black' : '']" />
            <span 
              v-if="isSidebarExpanded" 
              class="font-bold whitespace-nowrap"
            >
              {{ item.name }}
            </span>
          </button>
        </router-link>
      </nav>

      <!-- User Profile & Logout -->
      <div class="p-4 border-t-[3px] border-black dark:border-[#3C4043] flex flex-col gap-4 bg-white dark:bg-[#1E1E1E] transition-colors">
        <div 
          :class="[
            'flex items-center gap-3 border-[2px] border-black dark:border-[#5F6368] rounded-full bg-gray-50 dark:bg-[#28292C] transition-all overflow-hidden',
            isSidebarExpanded ? 'px-3 py-2' : 'p-2 justify-center mx-auto w-12 h-12 shrink-0'
          ]"
        >
          <img v-if="auth.avatarUrl" :src="auth.avatarUrl" class="w-7 h-7 rounded-full border border-gray-300 dark:border-gray-600 shrink-0" alt="Avatar">
          <div v-else class="w-7 h-7 rounded-full bg-gray-300 dark:bg-gray-600 shrink-0"></div>
          
          <div v-if="isSidebarExpanded" class="flex flex-col min-w-0">
            <span class="text-xs font-bold text-gray-800 dark:text-gray-200 truncate">{{ auth.displayName || 'User' }}</span>
          </div>
        </div>

        <button 
          @click="handleLogout" 
          :class="[
            'flex items-center justify-center gap-2 border-[2px] border-black dark:border-transparent rounded-full bg-white dark:bg-[#EA4335] text-black dark:text-white hover:bg-[#EA4335] hover:text-white dark:hover:brightness-110 transition-colors shrink-0',
            isSidebarExpanded ? 'px-4 py-2' : 'p-2 mx-auto w-12 h-12'
          ]"
          :title="!isSidebarExpanded ? 'Logout' : ''"
        >
          <IconLogOut class="w-5 h-5 shrink-0" />
          <span v-if="isSidebarExpanded" class="text-xs font-bold">LOGOUT</span>
        </button>
      </div>
    </aside>

    <!-- Mobile Overlay -->
    <div 
      v-if="isMobileMenuOpen" 
      @click="isMobileMenuOpen = false"
      class="fixed inset-0 bg-black/50 z-30 md:hidden"
    ></div>

    <!-- Main Content Area -->
    <div class="flex-1 flex flex-col h-screen overflow-hidden pt-16 md:pt-0 relative bg-[#F8F9FA] dark:bg-[#121212] transition-colors">
      <main class="flex-1 overflow-y-auto p-4 md:p-8">
        <router-view />
      </main>

      <!-- Bottom Status Area (Log + Status Bar) -->
      <div class="shrink-0 flex flex-col">
        <!-- Log Viewer -->
        <div class="h-32 bg-gray-900 border-t-[3px] border-black dark:border-gray-800 z-20 w-full shrink-0">
          <LogViewer />
        </div>

        <!-- Status Bar -->
        <div class="shrink-0 w-full z-30">
          <EngineStatus />
        </div>
      </div>
    </div>

  </div>
</template>
