import { defineStore } from 'pinia';
import { ref } from 'vue';

export interface ProgressEvent {
  stage: string;
  label: string;
  current: number;
  total: number;
  detail?: string;
}

export interface Toast {
  id: string;
  type: 'info' | 'success' | 'error' | 'warning';
  title: string;
  message?: string;
  duration?: number;
}

export const useAppStore = defineStore('app', () => {
  const isSidebarCollapsed = ref(false);
  const globalProgress = ref(0);
  const progressLabel = ref('');
  const isProcessing = ref(false);
  const currentProgressEvent = ref<ProgressEvent | null>(null);
  
  const toasts = ref<Toast[]>([]);
  
  const toggleSidebar = () => {
    isSidebarCollapsed.value = !isSidebarCollapsed.value;
  };
  
  const setProgress = (event: ProgressEvent) => {
    currentProgressEvent.value = event;
    if (event.total > 0) {
      globalProgress.value = (event.current / event.total) * 100;
    } else {
      globalProgress.value = 0;
    }
    progressLabel.value = event.label;
    isProcessing.value = event.current < event.total;
  };

  const addToast = (toast: Omit<Toast, 'id'>) => {
    const id = Math.random().toString(36).substring(2, 9);
    const newToast = { ...toast, id };
    toasts.value.push(newToast);
    
    if (toast.duration !== 0) {
      setTimeout(() => {
        removeToast(id);
      }, toast.duration || 3000);
    }
  };

  const removeToast = (id: string) => {
    toasts.value = toasts.value.filter(t => t.id !== id);
  };
  
  return {
    isSidebarCollapsed,
    globalProgress,
    progressLabel,
    isProcessing,
    currentProgressEvent,
    toasts,
    toggleSidebar,
    setProgress,
    addToast,
    removeToast
  };
});
