<template>
  <header class="w-full flex justify-center py-4 px-6 z-40 transition-all duration-300">
    <nav class="bg-white/70 dark:bg-[#18181b]/70 backdrop-blur-xl shadow-sm rounded-full px-2 py-1.5 border border-white/20 dark:border-white/10 flex items-center gap-1 transition-all">
      <div class="px-4 flex items-center mr-2">
        <span class="font-black tracking-[0.2em] text-sm text-[var(--color-text-main)] drop-shadow-sm">CLIPTZY</span>
      </div>

      <template v-for="item in navItems" :key="item.path || item.name">
        <!-- Studio dropdown -->
        <div v-if="item.children" class="relative" ref="studioMenuRef">
          <button
            @click="studioMenuOpen = !studioMenuOpen"
            class="relative px-5 py-2 rounded-full flex items-center gap-2 transition-all duration-300 text-sm font-bold"
            :class="isStudioActive
              ? 'bg-[var(--color-accent)] text-white shadow-[0_4px_15px_rgba(232,115,137,0.3)]'
              : 'text-[var(--color-text-muted)] hover:text-[var(--color-text-main)] hover:bg-black/5 dark:hover:bg-white/10'"
          >
            <component :is="item.icon" class="w-4 h-4 shrink-0" :class="isStudioActive ? 'scale-110' : ''" />
            <span>{{ item.name }}</span>
            <IconChevronDown class="w-3.5 h-3.5 transition-transform" :class="studioMenuOpen ? 'rotate-180' : ''" />
          </button>

          <transition name="fade-scale">
            <div
              v-if="studioMenuOpen"
              class="absolute top-full left-1/2 -translate-x-1/2 mt-2 min-w-[200px] bg-white dark:bg-[#1E293B] rounded-2xl shadow-lg border border-gray-200/50 dark:border-white/10 p-1.5 z-50"
            >
              <router-link
                v-for="child in item.children"
                :key="child.path"
                :to="child.path"
                @click="studioMenuOpen = false"
                v-slot="{ isActive, navigate }"
                custom
              >
                <button
                  @click="navigate"
                  class="w-full text-left px-4 py-2.5 rounded-xl text-sm font-bold transition-all flex items-center gap-2"
                  :class="isActive
                    ? 'bg-[var(--color-accent)]/15 text-[var(--color-text-main)]'
                    : 'text-[var(--color-text-muted)] hover:bg-black/5 dark:hover:bg-white/10 hover:text-[var(--color-text-main)]'"
                >
                  <component :is="child.icon" class="w-4 h-4" />
                  {{ child.name }}
                </button>
              </router-link>
            </div>
          </transition>
        </div>

        <!-- Regular nav item -->
        <router-link
          v-else
          :to="item.path!"
          v-slot="{ isActive, navigate }"
          custom
        >
          <button
            @click="navigate"
            class="relative px-5 py-2 rounded-full flex items-center gap-2 transition-all duration-300 text-sm font-bold"
            :class="isActive
              ? 'bg-[var(--color-accent)] text-white shadow-[0_4px_15px_rgba(232,115,137,0.3)]'
              : 'text-[var(--color-text-muted)] hover:text-[var(--color-text-main)] hover:bg-black/5 dark:hover:bg-white/10'"
          >
            <component :is="item.icon" class="w-4 h-4 shrink-0 transition-transform" :class="isActive ? 'scale-110' : ''" />
            <span>{{ item.name }}</span>
          </button>
        </router-link>
      </template>

      <div class="w-px h-5 bg-gray-300 dark:bg-gray-700 mx-2"></div>
      <button
        @click="toggleDarkMode"
        class="p-2.5 rounded-full text-[var(--color-text-muted)] hover:text-[var(--color-text-main)] hover:bg-black/5 dark:hover:bg-white/10 transition-all duration-300"
      >
        <IconSun v-if="isDark" class="w-4 h-4" />
        <IconMoon v-else class="w-4 h-4" />
      </button>
    </nav>
  </header>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { useRoute } from 'vue-router';
import IconDashboard from '~icons/lucide/layout-dashboard';
import IconScissors from '~icons/lucide/scissors';
import IconFilm from '~icons/lucide/film';
import IconLibrary from '~icons/lucide/library';
import IconSettings from '~icons/lucide/settings';
import IconInfo from '~icons/lucide/info';
import IconSun from '~icons/lucide/sun';
import IconMoon from '~icons/lucide/moon';
import IconChevronDown from '~icons/lucide/chevron-down';

const route = useRoute();
const studioMenuOpen = ref(false);
const studioMenuRef = ref<HTMLElement | null>(null);

const navItems = [
  { name: 'Dashboard', path: '/', icon: IconDashboard },
  {
    name: 'Studio',
    icon: IconScissors,
    children: [
      { name: 'Clipper', path: '/studio/clipper', icon: IconScissors },
      { name: 'Compilation', path: '/studio/compilation', icon: IconFilm },
    ],
  },
  { name: 'Library', path: '/library', icon: IconLibrary },
  { name: 'Settings', path: '/settings', icon: IconSettings },
  { name: 'About', path: '/about', icon: IconInfo },
];

const isStudioActive = computed(() => route.path.startsWith('/studio'));

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

const onClickOutside = (e: MouseEvent) => {
  const el = studioMenuRef.value as unknown as HTMLElement;
  if (el && !el.contains(e.target as Node)) {
    studioMenuOpen.value = false;
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
  document.addEventListener('click', onClickOutside);
});

onUnmounted(() => {
  document.removeEventListener('click', onClickOutside);
});
</script>

<style scoped>
.fade-scale-enter-active,
.fade-scale-leave-active {
  transition: opacity 0.15s ease, transform 0.15s ease;
}
.fade-scale-enter-from,
.fade-scale-leave-to {
  opacity: 0;
  transform: translate(-50%, -4px) scale(0.98);
}
</style>
