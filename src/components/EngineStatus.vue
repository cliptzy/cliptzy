<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

interface SystemMetrics {
  cpu_usage: number;
  memory_mb: number;
  system_memory_mb: number;
  system_used_memory_mb: number;
}

const metrics = ref<SystemMetrics | null>(null);
const isOnline = ref(true);
let intervalId: number | null = null;

const fetchMetrics = async () => {
  try {
    const data = await invoke<SystemMetrics>('get_system_metrics');
    metrics.value = data;
    isOnline.value = true;
  } catch (err) {
    console.error('Failed to get metrics:', err);
    isOnline.value = false;
  }
};

onMounted(() => {
  fetchMetrics();
  intervalId = window.setInterval(fetchMetrics, 2000);
});

onUnmounted(() => {
  if (intervalId) window.clearInterval(intervalId);
});
</script>

<template>
  <div class="p-2 bg-white dark:bg-[#1E1E1E] border-t-[3px] border-black dark:border-[#3C4043] text-black dark:text-gray-300 w-full flex flex-col md:flex-row justify-between md:items-center text-xs font-bold transition-colors gap-2">
    <!-- Left: Status Indicator -->
    <div class="flex items-center gap-2">
      <div :class="['w-2 h-2 rounded-full', isOnline ? 'bg-[#34A853]' : 'bg-[#EA4335]']"></div>
      <span>{{ isOnline ? 'System Online' : 'System Offline' }}</span>
    </div>

    <!-- Right: Metrics -->
    <div v-if="metrics" class="flex items-center gap-3 md:gap-4 overflow-x-auto whitespace-nowrap">
      <!-- App CPU Usage -->
      <span class="flex items-center gap-1" title="App CPU Usage">
        <span class="text-gray-500 dark:text-gray-400 font-medium">CPU:</span>
        <span class="bg-gray-100 dark:bg-gray-800 px-2 py-0.5 rounded-full min-w-[3rem] text-center">
          {{ metrics.cpu_usage.toFixed(1) }}%
        </span>
      </span>

      <!-- App RAM Usage -->
      <span class="flex items-center gap-1" title="App RAM Usage">
        <span class="text-gray-500 dark:text-gray-400 font-medium">RAM:</span>
        <span class="bg-gray-100 dark:bg-gray-800 px-2 py-0.5 rounded-full min-w-[4rem] text-center">
          {{ metrics.memory_mb }} MB
        </span>
      </span>

      <!-- System RAM Usage -->
      <span class="flex items-center gap-1" title="System RAM Usage">
        <span class="text-gray-500 dark:text-gray-400 font-medium">SYS:</span>
        <span class="bg-gray-100 dark:bg-gray-800 px-2 py-0.5 rounded-full min-w-[5rem] text-center text-[#FBBC04] dark:text-[#FBBC04]">
          {{ metrics.system_used_memory_mb }} / {{ metrics.system_memory_mb }} MB
        </span>
      </span>
    </div>
  </div>
</template>
