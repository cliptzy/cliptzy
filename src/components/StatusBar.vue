<template>
  <footer 
    class="h-6 w-full flex items-center justify-between px-2 text-[11px] font-mono z-50 shrink-0 border-t"
    :style="{ 
      backgroundColor: 'color-mix(in srgb, var(--color-base-100) 95%, black)',
      borderColor: 'color-mix(in srgb, var(--color-base-content) 8%, transparent)',
      color: 'color-mix(in srgb, var(--color-base-content) 70%, transparent)'
    }"
  >
    <div class="flex items-center gap-3 h-full">
          <!-- Auth Status -->
      <div class="flex items-center gap-1.5 hover:opacity-100 cursor-pointer transition-opacity h-full px-1" :title="auth.isLoggedIn ? `Logged in as ${auth.email}` : 'Not authenticated'">
        <div class="w-1.5 h-1.5 rounded-none" :class="auth.isLoggedIn ? 'bg-success' : 'bg-error'"></div>
        <span class="text-secondary">{{ auth.isLoggedIn ? (auth.email || 'Connected') : 'Offline' }}</span>
      </div>

      <!-- Processing Status -->
      <div class="flex items-center gap-1.5 h-full px-1" v-if="appStore.isProcessing">
        <svg class="w-3 h-3 animate-spin text-primary" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
        </svg>
        <span class="text-primary">{{ appStore.progressLabel || 'Processing' }}</span>
        <!-- Inline thin progress bar -->
        <CProgress class="w-16 h-0.5 mb-0" :progress="appStore.globalProgress" thin />
        <span class="text-primary">{{ Math.round(appStore.globalProgress) }}%</span>
      </div>
    </div>

    <div class="flex items-center gap-3 h-full">
      <!-- System Metrics -->
      <div class="flex items-center gap-2.5 h-full">
        <!-- CPU -->
        <div class="flex items-center gap-1 hover:opacity-100 px-1" :title="`CPU Usage: ${cpuUsage}%`">
          <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z"></path>
          </svg>
          <span class="text-secondary">CPU {{ cpuUsage }}%</span>
        </div>
        
        <!-- GPU -->
        <div class="flex items-center gap-1 hover:opacity-100 px-1" :title="hasGpu ? `GPU Usage: ${gpuUsage}%` : 'No GPU detected'">
          <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17V7m0 10a2 2 0 01-2 2H5a2 2 0 01-2-2V7a2 2 0 012-2h2a2 2 0 012 2m0 10a2 2 0 002 2h2a2 2 0 002-2M9 7a2 2 0 012-2h2a2 2 0 012 2m0 10V7m0 10a2 2 0 002 2h2a2 2 0 002-2V7a2 2 0 00-2-2h-2a2 2 0 00-2 2"></path>
          </svg>
          <span v-if="hasGpu" class="text-secondary">GPU {{ gpuUsage }}%</span>
          <span v-else class="text-secondary">GPU N/A</span>
        </div>

        <!-- RAM -->
        <div class="flex items-center gap-1 hover:opacity-100 px-1" :title="`Memory Usage: ${memoryUsage} MB`">
          <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"></path>
          </svg>
          <span class="text-secondary">{{ memoryUsage }}MB</span>
        </div>
      </div>

      <!-- Divider -->
      <div class="w-px h-3 bg-neutral"></div>

      <!-- Version -->
      <div class="flex items-center gap-1 px-1" title="Tauri v2">
        <span class="text-secondary">v1.0.0</span>
      </div>

      <!-- Divider -->
      <div class="w-px h-3 bg-neutral"></div>

      <!-- Keyboard Shortcut Hints (Context-aware) -->
      <div class="flex items-center gap-2.5 h-full text-secondary" v-if="shortcutHints.length">
        <span class="text-[9px] uppercase font-mono opacity-60">Shortcuts</span>
        <div class="flex items-center gap-1.5">
          <span
            v-for="hint in shortcutHints"
            :key="hint.key"
            :title="hint.label"
            class="inline-flex items-center gap-0.5 text-[9px] font-mono bg-base-300 px-1 py-0.25 rounded-none"
          >
            <kbd class="px-1 py-0.25 text-[8px] font-bold bg-base-200 border border-neutral rounded-none">{{ hint.key }}</kbd>
            <span>{{ hint.label }}</span>
          </span>
        </div>
      </div>
    </div>
  </footer>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useRoute } from 'vue-router';
import { useAuthStore } from '../stores/auth';
import { useAppStore } from '../stores/app';
import { useSystemMetrics } from '../composables/useSystemMetrics';

const auth = useAuthStore();
const appStore = useAppStore();
const route = useRoute();

const { metrics } = useSystemMetrics();

const cpuUsage = computed(() => metrics.value?.cpu_usage?.toFixed(0) || '0');
const memoryUsage = computed(() => metrics.value?.memory_mb?.toString() || '0');
const hasGpu = computed(() => metrics.value?.has_gpu || false);
const gpuUsage = computed(() => {
  if (metrics.value?.gpu_usage !== null && metrics.value?.gpu_usage !== undefined) {
    return metrics.value.gpu_usage.toFixed(0);
  }
  return '0';
});

// Context-aware keyboard shortcut hints
interface ShortcutHint {
  key: string;
  label: string;
}

const routeShortcuts: Record<string, ShortcutHint[]> = {
  studio: [
    { key: 'Space', label: 'Preview' },
    { key: 'Ctrl+S', label: 'Render' },
  ],
  library: [
    { key: 'Ctrl+F', label: 'Cari' },
  ],
  settings: [
    { key: 'Ctrl+S', label: 'Simpan' },
  ],
};

const shortcutHints = computed(() => {
  const name = route.name as string;
  return routeShortcuts[name] || [];
});
</script>
