<template>
  <div class="h-screen w-full bg-[var(--color-base)] text-white flex flex-col font-sans overflow-hidden selection:bg-[var(--color-accent)] selection:text-black">
    <MacOsWindowsTitleBar />
    
    <div class="flex-1 flex flex-col md:flex-row overflow-hidden relative">
      <!-- Sidebar / Bottom Dock -->
      <Sidebar />
      
      <!-- Main Content Area -->
      <main class="flex-1 overflow-y-auto p-4 md:p-8 relative scroll-smooth">
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
  </div>
</template>

<script setup lang="ts">
import MacOsWindowsTitleBar from '../components/MacOsWindowsTitleBar.vue';
import Sidebar from '../components/Sidebar.vue';
import GlobalStatusBar from '../components/GlobalStatusBar.vue';
import TerminalToast from '../components/TerminalToast.vue';
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
