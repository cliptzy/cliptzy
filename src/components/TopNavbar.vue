<template>
  <header class="w-full flex justify-center py-4 px-6 z-40 transition-all duration-300">
    <nav class="flex items-center gap-2 transition-all">
      <div class="px-4 flex items-center mr-2">
        <span class="font-display font-black tracking-[0.2em] text-sm text-base-content drop-shadow-sm">CLIPTZY</span>
      </div>

      <template v-for="item in navItems" :key="item.path || item.name">
        <!-- Studio dropdown -->
        <div v-if="item.children" class="relative" ref="studioMenuRef">
          <button
            @click="studioMenuOpen = !studioMenuOpen"
            class="relative px-5 py-2 rounded-full flex items-center gap-2 transition-all duration-300 text-sm font-bold"
            :class="isStudioActive
              ? 'bg-base-200 text-base-content shadow-md border border-base-content/5'
              : 'text-base-content/80 hover:text-base-content hover:bg-base-content/5'"
          >
            <component :is="item.icon" class="w-4 h-4 shrink-0" :class="isStudioActive ? 'scale-110' : ''" />
            <span>{{ item.name }}</span>
            <IconChevronDown class="w-3.5 h-3.5 transition-transform" :class="studioMenuOpen ? 'rotate-180' : ''" />
          </button>

          <transition name="fade-scale">
            <div
              v-if="studioMenuOpen"
              class="absolute top-full left-1/2 -translate-x-1/2 mt-2 min-w-[200px] bg-base-200 rounded-[1.5rem] shadow-xl border border-base-content/5 p-1.5 z-50"
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
                    ? 'bg-primary/15 text-primary'
                    : 'text-base-content/80 hover:bg-base-content/5 hover:text-base-content'"
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
              ? 'bg-base-200 text-base-content shadow-md border border-base-content/5'
              : 'text-base-content/80 hover:text-base-content hover:bg-base-content/5'"
          >
            <component :is="item.icon" class="w-4 h-4 shrink-0 transition-transform" :class="isActive ? 'scale-110' : ''" />
            <span>{{ item.name }}</span>
          </button>
        </router-link>
      </template>

      <div class="w-px h-5 bg-base-content/20 mx-2"></div>
      <button
        @click="toggleDarkMode"
        class="p-2.5 rounded-full text-base-content/80 hover:text-base-content hover:bg-base-content/5 transition-all duration-300"
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
    document.documentElement.setAttribute('data-theme', 'dark');
    localStorage.setItem('theme', 'dark');
  } else {
    document.documentElement.classList.remove('dark');
    document.documentElement.setAttribute('data-theme', 'light');
    localStorage.setItem('theme', 'light');
  }
};

const onClickOutside = (e: MouseEvent) => {
  const el = studioMenuRef.value;
  if (el instanceof HTMLElement && !el.contains(e.target as Node)) {
    studioMenuOpen.value = false;
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
