<template>
  <div 
    class="absolute bottom-4 right-4 z-50 flex flex-col gap-2 transition-all duration-500 ease-out"
    :class="appStore.isProcessing ? 'translate-y-0 opacity-100' : 'translate-y-12 opacity-0 pointer-events-none'"
  >
    <div class="bg-[var(--color-surface)]/90 border border-[var(--color-subtle)] rounded-xl shadow-[0_8px_32px_rgba(0,0,0,0.4)] p-4 w-80 backdrop-blur-md">
      <div class="flex justify-between items-center mb-2">
        <span class="text-xs font-bold text-[var(--color-accent)] uppercase tracking-widest flex items-center gap-2">
          <div class="w-2 h-2 rounded-full bg-[var(--color-accent)] animate-pulse shadow-[0_0_8px_var(--color-accent)]"></div>
          {{ appStore.currentProgressEvent?.stage || 'PROCESSING' }}
        </span>
        <span class="text-xs text-white font-mono font-medium">{{ Math.round(appStore.globalProgress) }}%</span>
      </div>
      <div class="w-full h-1.5 bg-[#27272A] rounded-full overflow-hidden mb-3 shadow-inner">
        <div 
          class="h-full bg-[var(--color-accent)] transition-all duration-300 ease-out shadow-[0_0_8px_var(--color-accent)] relative"
          :style="{ width: `${appStore.globalProgress}%` }"
        >
          <div class="absolute inset-0 bg-white/20 w-full animate-[shimmer_2s_infinite]"></div>
        </div>
      </div>
      <div class="flex items-center justify-between">
        <p class="text-xs text-gray-400 truncate flex-1" :title="appStore.progressLabel">
          {{ appStore.progressLabel || 'Initializing task...' }}
        </p>
        <button 
          @click="cancelProcessing" 
          class="ml-2 px-2 py-0.5 bg-red-500/20 text-red-500 border border-red-500/50 rounded text-[10px] font-bold hover:bg-red-500 hover:text-white transition-colors"
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

<style scoped>
@keyframes shimmer {
  0% { transform: translateX(-100%); }
  100% { transform: translateX(100%); }
}
</style>
