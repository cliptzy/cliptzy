import { defineStore } from 'pinia';
import { ref } from 'vue';

export const useClipperStore = defineStore('clipper', () => {
  // Input State
  const urlInput = ref('');
  const scanMethod = ref('heatmap');

  // UI State
  const analyzeStatus = ref<"idle" | "scanning" | "done" | "error">("idle");
  const scanMessage = ref('');
  const scanProgress = ref(0);

  // Data State
  const videoTitle = ref<string>('');
  const videoThumbnail = ref<string>('');
  const segments = ref<{ start: number; end: number; score?: number }[]>([]);
  const selectedIndices = ref<number[]>([]);
  const jobs = ref<any[]>([]);

  return { 
    urlInput, 
    scanMethod, 
    analyzeStatus, 
    scanMessage, 
    scanProgress, 
    videoTitle,
    videoThumbnail,
    segments, 
    selectedIndices, 
    jobs 
  };
});
