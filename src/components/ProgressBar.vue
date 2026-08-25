<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { listen, UnlistenFn } from '@tauri-apps/api/event';

interface ProgressPayload {
  percentage: number;
  message?: string;
}

const progress = ref(0);
const message = ref('');
let unlisten: UnlistenFn | undefined;

onMounted(async () => {
  unlisten = await listen<ProgressPayload>('progress', (event) => {
    progress.value = event.payload.percentage;
    if (event.payload.message) {
      message.value = event.payload.message;
    }
  });
});

onUnmounted(() => {
  if (unlisten) unlisten();
});
</script>
<template>
  <div class="w-full">
    <div v-if="message" class="text-xs text-gray-500 mb-1 flex justify-between">
      <span>{{ message }}</span>
      <span>{{ Math.round(progress) }}%</span>
    </div>
    <div class="w-full bg-gray-200 h-2 rounded overflow-hidden">
      <div 
        class="bg-blue-600 h-2 rounded transition-all duration-300 ease-out" 
        :style="{ width: `${progress}%` }"
      ></div>
    </div>
  </div>
</template>
