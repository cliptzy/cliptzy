<template>
  <div class="h-screen w-full bg-[var(--color-base)] text-[var(--color-text-main)] flex flex-col font-sans overflow-hidden selection:bg-gray-200 dark:bg-gray-800 selection:text-black">
    <MacOsWindowsTitleBar />
    
    <div class="flex-1 overflow-y-auto relative flex flex-col">
      <router-view />
    </div>
    
    <TerminalToast />
  </div>
</template>

<script setup lang="ts">
import MacOsWindowsTitleBar from '../components/MacOsWindowsTitleBar.vue';
import TerminalToast from '../components/TerminalToast.vue';
</script>
