<template>
  <div class="fixed top-4 right-4 z-[9999] flex flex-col gap-3 pointer-events-none w-80 max-w-full">
    <TransitionGroup name="toast" tag="div" class="flex flex-col gap-3 relative">
      <div 
        v-for="toast in appStore.toasts" 
        :key="toast.id"
        class="bg-[#09090b]/95 border border-[var(--color-subtle)] rounded-lg p-3 shadow-[0_8px_30px_rgba(0,0,0,0.5)] backdrop-blur-md pointer-events-auto flex items-start gap-3 overflow-hidden group"
      >
        <!-- Border highlight based on type -->
        <div class="absolute left-0 top-0 bottom-0 w-1" :class="{
          'bg-[var(--color-accent)]': toast.type === 'success',
          'bg-red-500': toast.type === 'error',
          'bg-yellow-500': toast.type === 'warning',
          'bg-blue-500': toast.type === 'info'
        }"></div>
        
        <div class="mt-0.5 shrink-0" :class="{
          'text-[var(--color-accent)]': toast.type === 'success',
          'text-red-500': toast.type === 'error',
          'text-yellow-500': toast.type === 'warning',
          'text-blue-500': toast.type === 'info'
        }">
          <IconCheckCircle v-if="toast.type === 'success'" class="w-4 h-4" />
          <IconXCircle v-else-if="toast.type === 'error'" class="w-4 h-4" />
          <IconAlertTriangle v-else-if="toast.type === 'warning'" class="w-4 h-4" />
          <IconInfo v-else class="w-4 h-4" />
        </div>
        
        <div class="flex-1 flex flex-col">
          <span class="font-bold text-white text-xs font-mono uppercase tracking-widest">{{ toast.title }}</span>
          <span v-if="toast.message" class="text-[10px] text-gray-400 font-mono mt-1 leading-relaxed break-words">{{ toast.message }}</span>
        </div>
        
        <button @click="appStore.removeToast(toast.id)" class="text-gray-500 hover:text-white shrink-0 opacity-0 group-hover:opacity-100 transition-opacity">
          <IconX class="w-4 h-4" />
        </button>
      </div>
    </TransitionGroup>
  </div>
</template>

<script setup lang="ts">
import { useAppStore } from '../stores/app';

// Icons
import IconCheckCircle from '~icons/lucide/check-circle';
import IconXCircle from '~icons/lucide/x-circle';
import IconAlertTriangle from '~icons/lucide/alert-triangle';
import IconInfo from '~icons/lucide/info';
import IconX from '~icons/lucide/x';

const appStore = useAppStore();
</script>

<style scoped>
.toast-move,
.toast-enter-active,
.toast-leave-active {
  transition: all 0.3s cubic-bezier(0.55, 0, 0.1, 1);
}

.toast-enter-from {
  opacity: 0;
  transform: translateX(30px) scale(0.95);
}

.toast-leave-to {
  opacity: 0;
  transform: translateY(-20px) scale(0.95);
}

.toast-leave-active {
  position: absolute;
  width: 100%;
}
</style>
