<template>
  <footer class="h-6 w-full bg-white dark:bg-[#09090B] border-t border-[var(--color-subtle)] flex items-center justify-between px-2 text-[10px] text-[var(--color-text-muted)] font-mono z-50 shrink-0">
    <div class="flex items-center gap-4 h-full">
      <!-- Auth Status -->
      <div class="flex items-center gap-1.5 hover:text-[var(--color-text-main)] cursor-pointer transition-colors h-full px-1" title="Authentication Status">
        <div class="w-1.5 h-1.5 rounded-full" :class="auth.isLoggedIn ? 'bg-green-500' : 'bg-red-500'"></div>
        <span>{{ auth.isLoggedIn ? auth.email || 'Authenticated' : 'Offline' }}</span>
      </div>

      <!-- App Status -->
      <div class="flex items-center gap-1.5 hover:text-[var(--color-text-main)] cursor-pointer transition-colors h-full px-1" title="Application Status">
        <template v-if="appStore.isProcessing">
          <svg class="w-3 h-3 animate-spin" :class="stageColorClass" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
          <span :class="stageColorClass">{{ appStore.progressLabel || 'Processing...' }} ({{ Math.round(appStore.globalProgress) }}%)</span>
        </template>
        <template v-else>
          <svg class="w-3 h-3 text-[var(--color-text-muted)]" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg>
          <span>Ready</span>
        </template>
      </div>
    </div>

    <div class="flex items-center gap-4 h-full">
      <!-- System Metrics -->
      <div class="flex items-center gap-3 h-full">
        <!-- CPU Usage -->
        <div class="flex items-center gap-1 hover:text-[var(--color-text-main)] cursor-default px-1" title="CPU Usage">
          <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z"></path></svg>
          <span>{{ cpuUsage }}%</span>
        </div>
        
        <!-- GPU Usage -->
        <div class="flex items-center gap-1 hover:text-[var(--color-text-main)] cursor-default px-1" title="GPU Usage">
          <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17V7m0 10a2 2 0 01-2 2H5a2 2 0 01-2-2V7a2 2 0 012-2h2a2 2 0 012 2m0 10a2 2 0 002 2h2a2 2 0 002-2M9 7a2 2 0 012-2h2a2 2 0 012 2m0 10V7m0 10a2 2 0 002 2h2a2 2 0 002-2V7a2 2 0 00-2-2h-2a2 2 0 00-2 2"></path></svg>
          <span v-if="hasGpu">{{ gpuUsage }}%</span>
          <span v-else>N/A</span>
        </div>

        <!-- RAM Usage -->
        <div class="flex items-center gap-1 hover:text-[var(--color-text-main)] cursor-default px-1" title="RAM Usage">
          <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"></path></svg>
          <span>{{ memoryUsage }} MB</span>
        </div>

        <!-- Network -->
        <div class="flex items-center gap-1 hover:text-[var(--color-text-main)] cursor-default px-1" title="Network (Download / Upload)">
          <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4"></path></svg>
          <span>↓{{ networkRx }} KB/s ↑{{ networkTx }} KB/s</span>
        </div>
      </div>
      
      <!-- Version / Backend -->
      <div class="flex items-center gap-1.5 hover:text-[var(--color-text-main)] cursor-pointer transition-colors h-full px-1" title="Native Rust Backend">
        <svg class="w-3 h-3" viewBox="0 0 32 32" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
          <path d="M26.4 17.6c.1-1.3-.1-2.5-.5-3.7-.1-.3-.2-.5-.4-.7-.2-.2-.5-.3-.7-.4-.3-.1-.6-.2-.8-.2-.3 0-.6 0-.8.1-.3.1-.5.3-.7.5-.2.2-.4.4-.5.7-.1.3-.2.6-.2 1 0 .4.1.7.2 1 .1.3.3.5.5.7.2.2.5.4.7.5.3.1.6.2.8.2.3 0 .6-.1.8-.2.2-.1.5-.3.7-.5.2-.2.3-.5.4-.7zm-14.8-8c-.3-.2-.6-.3-1-.3s-.7.1-1 .3c-.3.2-.5.5-.6.8-.1.3-.2.7-.2 1 0 .4.1.7.2 1 .1.3.3.6.6.8.3.2.6.3 1 .3s.7-.1 1-.3c.3-.2.5-.5.6-.8.1-.3.2-.7.2-1 0-.4-.1-.7-.2-1-.1-.3-.3-.6-.6-.8zm15 15.6c-.3-.2-.6-.3-1-.3s-.7.1-1 .3c-.3.2-.5.5-.6.8-.1.3-.2.7-.2 1 0 .4.1.7.2 1 .1.3.3.6.6.8.3.2.6.3 1 .3s.7-.1 1-.3c.3-.2.5-.5.6-.8.1-.3.2-.7.2-1 0-.4-.1-.7-.2-1-.1-.3-.3-.6-.6-.8zM14.6 1.4c-.1.3-.2.7-.2 1 0 .4.1.7.2 1 .1.3.3.6.6.8.3.2.6.3 1 .3s.7-.1 1-.3c.3-.2.5-.5.6-.8.1-.3.2-.7.2-1 0-.4-.1-.7-.2-1-.1-.3-.3-.6-.6-.8-.3-.2-.6-.3-1-.3s-.7.1-1 .3c-.3.2-.5.5-.6.8zM29 6.2c-.3-.2-.6-.3-1-.3s-.7.1-1 .3c-.3.2-.5.5-.6.8-.1.3-.2.7-.2 1 0 .4.1.7.2 1 .1.3.3.6.6.8.3.2.6.3 1 .3s.7-.1 1-.3c.3-.2.5-.5.6-.8.1-.3.2-.7.2-1 0-.4-.1-.7-.2-1-.1-.3-.3-.6-.6-.8zM11.6 28c-.3-.2-.6-.3-1-.3s-.7.1-1 .3c-.3.2-.5.5-.6.8-.1.3-.2.7-.2 1 0 .4.1.7.2 1 .1.3.3.6.6.8.3.2.6.3 1 .3s.7-.1 1-.3c.3-.2.5-.5.6-.8.1-.3.2-.7.2-1 0-.4-.1-.7-.2-1-.1-.3-.3-.6-.6-.8zm-4.7-4.6c-.1.3-.2.7-.2 1 0 .4.1.7.2 1 .1.3.3.6.6.8.3.2.6.3 1 .3s.7-.1 1-.3c.3-.2.5-.5.6-.8.1-.3.2-.7.2-1 0-.4-.1-.7-.2-1-.1-.3-.3-.6-.6-.8-.3-.2-.6-.3-1-.3s-.7.1-1 .3c-.3.2-.5.5-.6.8zm-2.8-5.3c-.3-.2-.6-.3-1-.3s-.7.1-1 .3c-.3.2-.5.5-.6.8-.1.3-.2.7-.2 1 0 .4.1.7.2 1 .1.3.3.6.6.8.3.2.6.3 1 .3s.7-.1 1-.3c.3-.2.5-.5.6-.8.1-.3.2-.7.2-1 0-.4-.1-.7-.2-1-.1-.3-.3-.6-.6-.8z"/>
        </svg>
        <span>Tauri v2</span>
      </div>
      
      <!-- Notifications (Bell) -->
      <div class="flex items-center gap-1.5 hover:text-[var(--color-text-main)] cursor-pointer transition-colors h-full px-1" title="Notifications">
        <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9"></path></svg>
      </div>
    </div>
  </footer>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from 'vue';
import { useAuthStore } from '../stores/auth';
import { useAppStore } from '../stores/app';
import { invoke } from '@tauri-apps/api/core';

const auth = useAuthStore();
const appStore = useAppStore();

const stageColorClass = computed(() => {
  const stage = appStore.currentProgressEvent?.stage?.toLowerCase();
  switch (stage) {
    case 'download': return 'text-[var(--color-text-muted)]';
    case 'crop': return 'text-[var(--color-text-muted)]';
    case 'transcribe': return 'text-[var(--color-text-muted)]';
    case 'subtitle': return 'text-[var(--color-text-muted)]';
    case 'finalize': return 'text-[var(--color-text-muted)]';
    default: return 'text-[var(--color-text-main)] ';
  }
});

const cpuUsage = ref('0.0');
const memoryUsage = ref('0');
const hasGpu = ref(false);
const gpuUsage = ref('0.0');
const networkRx = ref('0.0');
const networkTx = ref('0.0');

let statsInterval: number | null = null;

const fetchMetrics = async () => {
  try {
    const metrics = await invoke<any>('get_system_metrics');
    if (metrics) {
      cpuUsage.value = metrics.cpu_usage.toFixed(1);
      memoryUsage.value = metrics.memory_mb.toString();
      hasGpu.value = metrics.has_gpu;
      if (metrics.gpu_usage !== null && metrics.gpu_usage !== undefined) {
        gpuUsage.value = metrics.gpu_usage.toFixed(1);
      }
      networkRx.value = (metrics.network_rx_kbps || 0).toFixed(1);
      networkTx.value = (metrics.network_tx_kbps || 0).toFixed(1);
    }
  } catch (e) {
    console.error("Failed to fetch system metrics", e);
  }
};

onMounted(() => {
  fetchMetrics();
  // Update every 2 seconds
  statsInterval = window.setInterval(fetchMetrics, 2000);
});

onUnmounted(() => {
  if (statsInterval) clearInterval(statsInterval);
});
</script>
