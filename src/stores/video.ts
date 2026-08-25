import { defineStore } from 'pinia';
import { ref } from 'vue';
import { useAppStore } from './app';
// import { invoke } from '@tauri-apps/api/core';

export interface VideoSegment {
  start: number;
  end: number;
  score?: number;
  reason?: string;
}

export interface VideoMetadata {
  title: string;
  duration: number;
  thumbnail_url: string;
  uploader?: string;
  heatmap?: any[];
  segments?: VideoSegment[];
  ai_segments?: VideoSegment[];
}

export const useVideoStore = defineStore('video', () => {
  const currentUrl = ref('');
  const isAnalyzing = ref(false);
  const isLoading = ref(false);
  const isScanning = ref(false);
  const isScanningAI = ref(false);
  const metadata = ref<VideoMetadata | null>(null);
  const error = ref('');
  
  const analyzeVideo = async (url: string) => {
    if (!url) return;
    
    currentUrl.value = url;
    isAnalyzing.value = true;
    isLoading.value = true;
    error.value = '';
    
    try {
      const appStore = useAppStore();
      appStore.setProgress({ stage: 'ANALYSIS', label: 'Fetching metadata...', current: 1, total: 100 });
      await new Promise(resolve => setTimeout(resolve, 500));
      appStore.setProgress({ stage: 'ANALYSIS', label: 'Parsing heatmaps...', current: 50, total: 100 });
      await new Promise(resolve => setTimeout(resolve, 500));
      appStore.setProgress({ stage: 'ANALYSIS', label: 'Done', current: 100, total: 100 });
      
      metadata.value = {
        title: 'Podcast 10 Jam tentang Coding',
        uploader: 'Tech Channel',
        duration: 3540,
        thumbnail_url: 'https://images.unsplash.com/photo-1611162617474-5b21e879e113?q=80&w=1000&auto=format&fit=crop'
      };
    } catch (err: any) {
      console.error(err);
      error.value = err.toString();
    } finally {
      isAnalyzing.value = false;
      isLoading.value = false;
    }
  };

  const previewVideo = async (url: string) => {
    return analyzeVideo(url);
  };

  const processHeatmap = async (_url: string) => {
    if (!metadata.value) return;
    isScanning.value = true;
    try {
      await new Promise(resolve => setTimeout(resolve, 1500));
      metadata.value.segments = [
        { start: 0, end: 60, score: 0.8 },
        { start: 120, end: 180, score: 0.9 },
        { start: 300, end: 360, score: 0.7 },
      ];
    } finally {
      isScanning.value = false;
    }
  };
  
  return {
    currentUrl,
    isAnalyzing,
    isLoading,
    isScanning,
    isScanningAI,
    metadata,
    error,
    analyzeVideo,
    previewVideo,
    processHeatmap
  };
});
