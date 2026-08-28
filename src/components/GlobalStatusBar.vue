<template>
  <div 
    class="absolute bottom-4 right-4 z-50 flex flex-col gap-2 transition-all duration-500 ease-out"
    :class="appStore.isProcessing ? 'translate-y-0 opacity-100' : 'translate-y-12 opacity-0 pointer-events-none'"
  >
    <div class="bg-[var(--color-surface)]/90 border border-[var(--color-subtle)] rounded-xl shadow-[0_8px_32px_rgba(0,0,0,0.4)] p-4 w-80 backdrop-blur-md">
      <div class="flex justify-between items-center mb-2">
        <span class="text-xs font-bold text-[var(--color-text-main)] uppercase tracking-widest flex items-center gap-2">
          <div class="w-2 h-2 rounded-full bg-gray-200 dark:bg-gray-800 animate-pulse shadow-[0_0_8px_var(--color-accent)]"></div>
          {{ appStore.currentProgressEvent?.stage || 'PROCESSING' }}
        </span>
        <span class="text-xs text-[var(--color-text-main)] font-mono font-medium">{{ Math.round(appStore.globalProgress) }}%</span>
      </div>
      <ProgressBar
        class="w-full mb-3"
        :progress="appStore.globalProgress"
        animated
      />
      <div class="flex items-center justify-between">
        <p class="text-xs text-[var(--color-text-muted)] truncate flex-1" :title="appStore.progressLabel">
          {{ appStore.progressLabel || 'Initializing task...' }}
        </p>
        <button 
          @click="cancelProcessing" 
          class="ml-2 px-2 py-0.5 bg-red-500/20 text-red-500 border border-red-500/50 rounded text-[10px] font-bold hover:bg-red-500 hover:text-[var(--color-text-main)] transition-colors"
        >
          BATAL
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useAppStore } from '../stores/app';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { onMounted, onUnmounted } from 'vue';
import type { ProgressEvent } from '../stores/app';
import ProgressBar from './ProgressBar.vue';

const appStore = useAppStore();
let unlisten: (() => void) | null = null;

const cancelProcessing = async () => {
  try {
    await invoke('cancel_processing');
    appStore.isProcessing = false;
    appStore.progressLabel = 'Dibatalkan...';
  } catch (err) {
    console.error("Gagal membatalkan proses", err);
  }
};

onMounted(async () => {
  try {
    unlisten = await listen<ProgressEvent>('clip-progress', (event) => {
      appStore.setProgress(event.payload);
    });
  } catch (err) {
    console.error("Failed to listen to clip-progress event", err);
  }
});

onUnmounted(() => {
  if (unlisten) unlisten();
});
</script>
