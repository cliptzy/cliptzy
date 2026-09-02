import { ref, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

export interface SystemMetrics {
  cpu_usage: number;
  memory_mb: number;
  system_memory_mb: number;
  system_used_memory_mb: number;
  network_rx_kbps: number;
  network_tx_kbps: number;
  has_gpu: boolean;
  gpu_usage: number | null;
}

const metrics = ref<SystemMetrics | null>(null);
let interval: number | null = null;
let subscribers = 0;

export function useSystemMetrics() {
  const fetchMetrics = async () => {
    try {
      const result = await invoke<SystemMetrics>('get_system_metrics');
      if (result) {
        metrics.value = result;
      }
    } catch (e) {
      console.error("Failed to fetch system metrics", e);
    }
  };

  onMounted(() => {
    subscribers++;
    if (subscribers === 1) {
      fetchMetrics();
      interval = window.setInterval(fetchMetrics, 2000);
    }
  });

  onUnmounted(() => {
    subscribers--;
    if (subscribers === 0 && interval) {
      clearInterval(interval);
      interval = null;
    }
  });

  return {
    metrics
  };
}
