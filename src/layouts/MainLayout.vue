<template>
  <div class="h-screen w-full bg-[var(--color-base)] text-[var(--color-text-main)] flex flex-col font-sans overflow-hidden selection:bg-gray-200 dark:bg-gray-800 selection:text-black">
    <MacOsWindowsTitleBar />
    <TopNavbar />
    
    <div class="flex-1 flex flex-col overflow-hidden relative">
      <!-- Main Content Area -->
      <main class="flex-1 overflow-y-auto px-4 md:px-8 pb-4 relative scroll-smooth">
        <router-view v-slot="{ Component }">
          <transition name="fade-scale" mode="out-in">
            <component :is="Component" />
          </transition>
        </router-view>
      </main>

      <!-- Global Status Monitor -->
      <GlobalStatusBar />
      
      <!-- Toast Notifications -->
      <TerminalToast />
    </div>
    
    <!-- Footer Status Bar -->
    <AppFooter />
  </div>
</template>

<script setup lang="ts">
import MacOsWindowsTitleBar from '../components/MacOsWindowsTitleBar.vue';
import TopNavbar from '../components/TopNavbar.vue';
import GlobalStatusBar from '../components/GlobalStatusBar.vue';
import TerminalToast from '../components/TerminalToast.vue';
import AppFooter from '../components/AppFooter.vue';
</script>

<style>
.fade-scale-enter-active,
.fade-scale-leave-active {
  transition: opacity 0.2s ease, transform 0.2s ease;
}

.fade-scale-enter-from,
.fade-scale-leave-to {
  opacity: 0;
  transform: scale(0.99);
}
</style>
