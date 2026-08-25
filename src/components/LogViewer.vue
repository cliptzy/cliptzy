<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { listen } from '@tauri-apps/api/event';

const logs = ref<string[]>([]);
let unlisten: any;

onMounted(async () => {
  unlisten = await listen<string>('log', (event) => {
    logs.value.push(event.payload);
    if (logs.value.length > 500) logs.value.shift();
  });
});

onUnmounted(() => {
  if (unlisten) unlisten();
});
</script>
<template>
  <div class="h-full overflow-y-auto bg-black text-green-400 p-3 font-mono text-xs shadow-inner">
    <div v-for="(log, i) in logs" :key="i" class="mb-1 opacity-90 hover:opacity-100">{{ log }}</div>
    <div v-if="logs.length === 0" class="text-gray-500 italic">Waiting for engine logs...</div>
  </div>
</template>
