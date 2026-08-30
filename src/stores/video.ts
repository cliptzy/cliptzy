import { defineStore } from 'pinia';
import { ref, watch } from 'vue';
import { useAppStore } from './app';
import { invoke } from '@tauri-apps/api/core';

export interface VideoSegment {
  start: number;
  end: number;
  score?: number;
  reason?: string;
  selectedForRender?: boolean;
}

export interface VideoMetadata {
  video_id?: string;
  video_url?: string;
  title: string;
  duration: number;
  thumbnail_url: string;
  thumbnail?: string;
  uploader?: string;
  view_count?: number;
  upload_date?: string | null;
  heatmap?: any[];
  segments?: VideoSegment[];
  ai_segments?: VideoSegment[];
  stream_url?: string;
}

export interface RestreamerInfo {
  video_id: string;
  video_url: string;
  title: string;
  uploader: string;
  thumbnail: string;
  duration: number;
  upload_date?: string | null;
  view_count?: number | null;
}

export interface CompilationData {
  video_info: VideoMetadata;
  main_audio_16k_path: string;
  epic_moments: { start: number; end: number; description: string }[];
  restreamers: RestreamerInfo[];
}

export const useVideoStore = defineStore('video', () => {
  const currentUrl = ref('');
  const isAnalyzing = ref(false);
  const isLoading = ref(false);
  const isScanning = ref(false);
  const isScanningAI = ref(false);
  const metadata = ref<VideoMetadata | null>(null);
  const currentTime = ref<number>(0);
  const selectedSegment = ref<VideoSegment | null>(null);
  const analyzedSegments = ref<Record<string, any>>({});
  const error = ref('');
  const compilationData = ref<CompilationData | null>(null);
  const selectedRestreamers = ref<string[]>([]);
  const isPreparingCompilation = ref(false);
  
  const prepareCompilation = async (url: string, searchKeywords?: string) => {
    isPreparingCompilation.value = true;
    const appStore = useAppStore();
    appStore.setProgress({
      stage: 'COMPILATION',
      label: 'Memulai persiapan kompilasi...',
      current: 1,
      total: 100,
    });

    try {
      const video_id = extractVideoId(url) || "unknown";
      const res = await invoke('prepare_compilation', { 
        videoUrl: url, 
        videoId: video_id, 
        searchKeywords: searchKeywords || null,
      });
      compilationData.value = res as CompilationData;
      if (res && (res as CompilationData).video_info) {
        const info = (res as CompilationData).video_info;
        metadata.value = {
          video_id: info.video_id,
          video_url: info.video_url,
          title: info.title,
          duration: info.duration,
          thumbnail_url: info.thumbnail || info.thumbnail_url || '',
          uploader: info.uploader,
          upload_date: info.upload_date,
          stream_url: info.stream_url,
        };
      }
      if (res && (res as CompilationData).restreamers?.length) {
        selectedRestreamers.value = (res as CompilationData).restreamers.map((r) => r.video_url);
      } else {
        selectedRestreamers.value = [];
      }
      appStore.addToast({
        title: 'Persiapan Selesai',
        message: `Ditemukan ${(res as CompilationData)?.epic_moments?.length ?? 0} momen epik dan ${(res as CompilationData)?.restreamers?.length ?? 0} restreamer.`,
        type: 'success',
      });
    } catch (e: any) {
      const message = typeof e === 'string' ? e : (e?.message || String(e));
      console.error('[prepare_compilation] Gagal:', message);
      error.value = message;
      appStore.setProgress({
        stage: 'COMPILATION',
        label: `Gagal: ${message}`,
        current: 100,
        total: 100,
      });
      appStore.isProcessing = false;
      appStore.addToast({
        title: 'Persiapan Kompilasi Gagal',
        message,
        type: 'error',
        duration: 8000,
      });
    } finally {
      isPreparingCompilation.value = false;
    }
  };

  
  const extractVideoId = (url: string) => {
    const match = url.match(/(?:youtu\.be\/|youtube\.com\/(?:embed\/|v\/|watch\?v=|watch\?.+&v=))([^&?]+)/);
    return match ? match[1] : null;
  };

  const analyzeVideo = async (url: string) => {
    if (!url) return;
    
    currentUrl.value = url;
    isAnalyzing.value = true;
    isLoading.value = true;
    error.value = '';
    
    try {
      const appStore = useAppStore();
      const settingsStore = (await import('./settings')).useSettingsStore();
      
      appStore.setProgress({ stage: 'ANALYSIS', label: 'Fetching metadata...', current: 1, total: 100 });
      
      const browserName = settingsStore.config?.browser || null;
      const result: any = await invoke('scan_video', { url, cookiesPath: browserName });
      console.log('SCAN_VIDEO_RESULT:', result);
      
      appStore.setProgress({ stage: 'ANALYSIS', label: 'Done', current: 100, total: 100 });
      
      const parsedVideoId = result.video_id || result.videoId || extractVideoId(url) || 'local';
      
      metadata.value = {
        video_id: parsedVideoId,
        title: result.title || 'Video',
        uploader: 'YouTube Channel', // TODO: Get uploader from rust
        duration: result.duration || 0,
        thumbnail_url: result.thumbnail || result.thumbnailUrl || result.thumbnail_url || '',
        segments: (result.segments || []).map((s: any) => ({ ...s, selectedForRender: true })),
        ai_segments: [],
        stream_url: result.stream_url || result.streamUrl
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
      // Heatmap is already fetched during scan_video, so we just wait a bit for UX
      // or we can use this function to re-fetch if needed.
      await new Promise(resolve => setTimeout(resolve, 500));
      // segments are already in metadata.value.segments
    } finally {
      isScanning.value = false;
    }
  };
  
  const analyzeSegmentAudio = async (url: string, start: number, end: number, streamUrl?: string) => {
    isAnalyzing.value = true;
    try {
      const result: any = await invoke('analyze_segment_audio', { 
        url, 
        start, 
        end,
        streamUrl: streamUrl || null
      });
      console.log('Pre-analysis result:', result);
      
      const key = `${start}-${end}`;
      analyzedSegments.value[key] = result;
      
      return result;
    } catch (err: any) {
      console.error(err);
      error.value = err.toString();
    } finally {
      isAnalyzing.value = false;
    }
  };

  watch(selectedSegment, async (newSeg) => {
    if (newSeg && currentUrl.value) {
      // Avoid re-analyzing if we already have it
      const key = `${newSeg.start}-${newSeg.end}`;
      if (!analyzedSegments.value[key]) {
        // Disabled auto-transcribe on click
        // await analyzeSegmentAudio(currentUrl.value, newSeg.start, newSeg.end, metadata.value?.stream_url);
      }
    }
  });

  return { compilationData, selectedRestreamers, isPreparingCompilation, prepareCompilation,
    currentUrl,
    isAnalyzing,
    isLoading,
    isScanning,
    isScanningAI,
    metadata,
    currentTime,
    selectedSegment,
    analyzedSegments,
    error,
    analyzeVideo,
    previewVideo,
    processHeatmap,
    analyzeSegmentAudio
  };
});


