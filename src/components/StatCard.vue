<script setup lang="ts">
defineProps<{
  title: string
  value: string | number
  accentClass?: string
}>()
</script>

<template>
  <div class="relative overflow-hidden border-[3px] border-black dark:border-[#3C4043] rounded-[32px] p-6 bg-white dark:bg-[#1E1E1E] text-black dark:text-white transition-colors group">
    <!-- Decorative geometric accent -->
    <div v-if="accentClass" :class="['absolute top-0 right-0 w-24 h-24 rounded-bl-full -z-0 opacity-20 dark:opacity-10 group-hover:scale-110 transition-transform origin-top-right', accentClass]"></div>
    
    <h3 class="text-sm font-bold uppercase tracking-wider mb-2 relative z-10 text-gray-600 dark:text-gray-400">{{ title }}</h3>
    <div class="text-5xl font-black relative z-10 tracking-tight">{{ value }}</div>
    <div class="relative z-10">
      <slot name="footer"></slot>
    </div>
  </div>
</template>
