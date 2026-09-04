import { ref, onMounted, onUnmounted } from 'vue';
import { load } from '@tauri-apps/plugin-store';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';

export interface JobHistory {
  video_id: string;
  title: string;
  url: string;
  thumbnail: string;
  mode: string;
  status: string;
  updated_at: number;
}

export function useJobHistory() {
  const jobs = ref<JobHistory[]>([]);
  const isLoading = ref(false);
  const error = ref<string | null>(null);
  let unlistenFn: UnlistenFn | null = null;

  const loadHistory = async () => {
    isLoading.value = true;
    error.value = null;
    try {
      const store = await load('history.json', { autoSave: false });
      const entries = await store.entries();
      
      jobs.value = entries
        .map(([_, value]) => value as JobHistory)
        .sort((a, b) => (b.updated_at || 0) - (a.updated_at || 0));
    } catch (err: any) {
      console.error("Gagal memuat history:", err);
      error.value = err.message || "Failed to load history";
    } finally {
      isLoading.value = false;
    }
  };

  // Auto-reload when backend emits update event
  onMounted(async () => {
    await loadHistory();
    unlistenFn = await listen('job-history-updated', () => {
      loadHistory();
    });
  });

  onUnmounted(() => {
    if (unlistenFn) {
      unlistenFn();
    }
  });

  const formatTime = (timestamp: number) => {
    if (!timestamp) return 'Tidak diketahui';
    const rtf = new Intl.RelativeTimeFormat('id', { numeric: 'auto' });
    const diffInSeconds = Math.floor(Date.now() / 1000 - timestamp);
    
    if (diffInSeconds < 60) return rtf.format(-diffInSeconds, 'second');
    if (diffInSeconds < 3600) return rtf.format(-Math.floor(diffInSeconds / 60), 'minute');
    if (diffInSeconds < 86400) return rtf.format(-Math.floor(diffInSeconds / 3600), 'hour');
    return rtf.format(-Math.floor(diffInSeconds / 86400), 'day');
  };

  const formatDate = (timestamp: number) => {
    if (!timestamp) return 'Tidak diketahui';
    return new Date(timestamp * 1000).toLocaleDateString('id-ID', {
      year: 'numeric',
      month: 'short',
      day: 'numeric'
    });
  };

  return {
    jobs,
    isLoading,
    error,
    loadHistory,
    formatTime,
    formatDate
  };
}
