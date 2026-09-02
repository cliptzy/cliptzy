<template>
  <div class="fixed bottom-20 right-4 z-[9999] flex flex-col gap-3 pointer-events-none w-80 max-w-full">
    <TransitionGroup name="toast" tag="div" class="flex flex-col gap-3 relative">
      <div 
        v-for="toast in appStore.toasts" 
        :key="toast.id"
        class="bg-base-200 border rounded-none p-3 backdrop-blur-md pointer-events-auto flex items-start gap-3 overflow-hidden group"
        :style="{ borderColor: getBorderColor(toast.type) }"
      >
        <!-- Left accent bar based on type -->
        <div class="absolute left-0 top-0 bottom-0 w-1" :style="{ backgroundColor: getAccentColor(toast.type) }"></div>
        
        <div class="mt-0.5 shrink-0" :style="{ color: getAccentColor(toast.type) }">
          <IconCheckCircle v-if="toast.type === 'success'" class="w-4 h-4" />
          <IconXCircle v-else-if="toast.type === 'error'" class="w-4 h-4" />
          <IconAlertTriangle v-else-if="toast.type === 'warning'" class="w-4 h-4" />
          <IconInfo v-else class="w-4 h-4" />
        </div>
        
        <div class="flex-1 flex flex-col">
          <span class="font-bold text-base-content text-xs font-mono uppercase tracking-widest">{{ toast.title }}</span>
          <span v-if="toast.message" class="text-[10px] text-secondary font-mono mt-1 leading-relaxed break-words">{{ toast.message }}</span>
        </div>
        
        <button 
          @click="appStore.removeToast(toast.id)" 
          class="text-secondary hover:text-base-content shrink-0 opacity-0 group-hover:opacity-100 transition-opacity duration-150"
          :aria-label="'Tutup notifikasi'"
        >
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

// Color mapping using CSS variables for theme consistency
const getAccentColor = (type: string): string => {
  switch (type) {
    case 'success': return 'var(--color-success)';
    case 'error': return 'var(--color-error)';
    case 'warning': return 'var(--color-warning)';
    case 'info': return 'var(--color-accent)';
    default: return 'var(--color-accent)';
  }
};

const getBorderColor = (type: string): string => {
  switch (type) {
    case 'success': return 'color-mix(in srgb, var(--color-success) 20%, transparent)';
    case 'error': return 'color-mix(in srgb, var(--color-error) 20%, transparent)';
    case 'warning': return 'color-mix(in srgb, var(--color-warning) 20%, transparent)';
    case 'info': return 'color-mix(in srgb, var(--color-accent) 20%, transparent)';
    default: return 'color-mix(in srgb, var(--color-base-content) 8%, transparent)';
  }
};
</script>

<style scoped>
/* Slide-up from bottom animation - 200ms as per UI_DESIGN.md */
.toast-move,
.toast-enter-active,
.toast-leave-active {
  transition: all 0.2s cubic-bezier(0.55, 0, 0.1, 1);
}

.toast-enter-from {
  opacity: 0;
  transform: translateY(20px) scale(0.95);
}

.toast-leave-to {
  opacity: 0;
  transform: translateY(-10px) scale(0.98);
}

.toast-leave-active {
  position: absolute;
  width: 100%;
}

/* Respect reduced motion preference */
@media (prefers-reduced-motion: reduce) {
  .toast-move,
  .toast-enter-active,
  .toast-leave-active {
    transition: opacity 0.1s ease;
  }
  
  .toast-enter-from,
  .toast-leave-to {
    transform: none;
  }
}
</style>


