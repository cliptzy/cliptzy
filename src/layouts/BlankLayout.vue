<template>
  <div class="h-screen w-full bg-base-100 text-base-content flex flex-col font-sans overflow-hidden selection:bg-primary/20 selection:text-primary">
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


