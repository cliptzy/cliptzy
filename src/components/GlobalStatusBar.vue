<template>
  <div 
    class="absolute bottom-4 right-4 z-50 flex flex-col gap-2 transition-all duration-500 ease-out"
    :class="appStore.isProcessing ? 'translate-y-0 opacity-100' : 'translate-y-12 opacity-0 pointer-events-none'"
  >
    <div class="bg-base-200/90 border border-neutral rounded-none p-4 w-80 backdrop-blur-md">
      <div class="flex justify-between items-center mb-2">
        <span class="text-xs font-bold text-base-content uppercase tracking-widest flex items-center gap-2">
          <div class="w-2 h-2 rounded-none bg-accent animate-pulse shadow-[0_0_8px_var(--color-accent)]"></div>
          {{ appStore.currentProgressEvent?.stage || 'PROCESSING' }}
        </span>
        <span class="text-xs text-base-content font-mono font-medium">{{ Math.round(appStore.globalProgress) }}%</span>
      </div>
            <CProgress
        class="w-full mb-3"
        :progress="appStore.globalProgress"
        thin
        :striped="appStore.isProcessing"
      />
      <div class="flex items-center justify-between">
        <p class="text-xs text-secondary truncate flex-1" :title="appStore.progressLabel">
          {{ appStore.progressLabel || 'Initializing task...' }}
        </p>
        <button 
          @click="cancelProcessing" 
          class="ml-2 px-2 py-0.5 bg-error/20 text-error border border-error/50 rounded-none text-[10px] font-bold hover:bg-error hover:text-[var(--color-error-content)] transition-colors duration-150"
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


