<template>
  <div class="grid grid-cols-1 md:grid-cols-3 border-b border-neutral bg-base-100">
    <!-- System Monitor -->
    <div class="p-6 border-r border-neutral flex flex-col justify-center gap-3">
      <h3 class="text-sm font-black text-base-content uppercase tracking-wider mb-1 flex items-center gap-2">
        <IconActivity class="w-4 h-4 text-secondary" /> System Monitor
      </h3>
      
      <div class="flex items-center gap-3">
        <span class="text-xs font-mono font-bold text-secondary w-8">CPU</span>
        <CProgress class="flex-1" :progress="Number(cpuUsage)" thin />
        <span class="text-xs font-mono text-base-content w-8 text-right">{{ cpuUsage }}%</span>
      </div>
      
      <div class="flex items-center gap-3">
        <span class="text-xs font-mono font-bold text-secondary w-8">GPU</span>
        <CProgress class="flex-1" :progress="hasGpu ? Number(gpuUsage) : 0" thin />
        <span class="text-xs font-mono text-base-content w-8 text-right">{{ hasGpu ? gpuUsage + '%' : 'N/A' }}</span>
      </div>

      <div class="flex items-center gap-3">
        <span class="text-xs font-mono font-bold text-secondary w-8">RAM</span>
        <CProgress class="flex-1" :progress="ramUsagePercent" thin />
        <span class="text-xs font-mono text-base-content w-10 text-right">{{ Math.round(ramUsagePercent) }}%</span>
      </div>
    </div>

    <!-- Storage / App RAM Usage -->
    <div class="p-6 border-r border-neutral flex flex-col justify-center">
      <h3 class="text-sm font-black text-base-content uppercase tracking-wider mb-2 flex items-center gap-2">
        <IconHardDrive class="w-4 h-4 text-secondary" /> App RAM Usage
      </h3>
      <div class="flex items-baseline gap-2">
        <p class="text-4xl font-black text-base-content">{{ appMemoryMb }}</p>
        <p class="text-sm font-mono text-secondary">MB</p>
      </div>
      <CProgress class="mt-3 w-full" :progress="appRamPercent" thin />
    </div>

    <!-- Active Jobs -->
    <div class="p-6 flex flex-col justify-center">
      <h3 class="text-sm font-black text-base-content uppercase tracking-wider mb-2 flex items-center gap-2">
        <IconLink class="w-4 h-4 text-secondary" /> Active Jobs
      </h3>
      <div class="flex flex-col gap-2 mt-1">
        <div class="flex items-center justify-between p-2 bg-base-200 border border-neutral">
          <span class="text-xs font-bold text-base-content flex items-center gap-2">
            <div class="w-2 h-2 bg-accent" :class="{ 'animate-pulse': processingJobsCount > 0 }"></div> Processing
          </span>
          <span class="text-xs font-mono text-secondary">{{ processingJobsCount }} active</span>
        </div>
        <div class="flex items-center justify-between p-2 bg-base-200 border border-neutral opacity-70">
          <span class="text-xs font-bold text-base-content flex items-center gap-2">
            <div class="w-2 h-2 bg-neutral"></div> Draft
          </span>
          <span class="text-xs font-mono text-secondary">{{ draftJobsCount }} waiting</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue';
import IconActivity from '~icons/lucide/activity';
import IconHardDrive from '~icons/lucide/hard-drive';
import IconLink from '~icons/lucide/layers';

import { useSystemMetrics } from '../../composables/useSystemMetrics';
import { useJobHistory } from '../../composables/useJobHistory';

const { metrics } = useSystemMetrics();
const { jobs, loadHistory } = useJobHistory();

const cpuUsage = computed(() => metrics.value?.cpu_usage?.toFixed(0) || '0');
const hasGpu = computed(() => metrics.value?.has_gpu || false);
const gpuUsage = computed(() => {
  if (metrics.value?.gpu_usage !== null && metrics.value?.gpu_usage !== undefined) {
    return metrics.value.gpu_usage.toFixed(0);
  }
  return '0';
});

const ramUsagePercent = computed(() => {
  if (!metrics.value || metrics.value.system_memory_mb === 0) return 0;
  return (metrics.value.system_used_memory_mb / metrics.value.system_memory_mb) * 100;
});

const appMemoryMb = computed(() => metrics.value?.memory_mb || 0);

const appRamPercent = computed(() => {
  if (!metrics.value || metrics.value.system_memory_mb === 0) return 0;
  return (metrics.value.memory_mb / metrics.value.system_memory_mb) * 100;
});

const processingJobsCount = computed(() => {
  return jobs.value.filter(j => j.status.toLowerCase() === 'processing' || j.status.toLowerCase() === 'memproses').length;
});

const draftJobsCount = computed(() => {
  return jobs.value.filter(j => j.status.toLowerCase() === 'draft' || j.status.toLowerCase() === 'queued').length;
});

onMounted(() => {
  loadHistory();
});
</script>
