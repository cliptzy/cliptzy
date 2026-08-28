<script setup lang="ts">
// AuthLayout: minimal layout with no sidebar/status bar
</script>

<template>
  <div class="h-screen w-full bg-[#F8F9FA] dark:bg-[#121212] text-[var(--color-text-main)] dark:text-[var(--color-text-main)] flex flex-col transition-colors overflow-hidden">
    <!-- Router View for login pages -->
    <router-view class="flex-1 overflow-y-auto" />
  </div>
</template>
