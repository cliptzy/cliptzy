<template>
  <div class="rounded-full overflow-hidden shadow-inner" :class="[heightClass, trackClass]">
    <div 
      class="h-full transition-all duration-300 ease-out relative"
      :class="fillClass"
      :style="{ width: `${clampedProgress}%` }"
    >
      <div v-if="animated" class="absolute inset-0 bg-white/20 w-full animate-[shimmer_2s_infinite]"></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';

const props = withDefaults(defineProps<{
  progress: number;
  heightClass?: string;
  trackClass?: string;
  fillClass?: string;
  animated?: boolean;
}>(), {
  progress: 0,
  heightClass: 'h-1.5',
  trackClass: 'bg-gray-200 dark:bg-gray-800/60',
  fillClass: 'bg-[var(--color-accent)]',
  animated: false,
});

const clampedProgress = computed(() => Math.max(0, Math.min(100, props.progress)));
</script>

<style scoped>
@keyframes shimmer {
  0% { transform: translateX(-100%); }
  100% { transform: translateX(100%); }
}
</style>
