<template>
  <div class="h-full flex flex-col gap-4 max-w-[1600px] mx-auto w-full">
    <!-- Top Area: 3 Spatial Panels -->
    <div class="flex-1 flex flex-col xl:flex-row gap-4 min-h-0">
      
      <!-- LEFT PANEL: Inspector -->
      <InspectorPanel 
        v-model:aiWhisper="aiWhisper"
        v-model:aiBRoll="aiBRoll"
      />

      <!-- CENTER PANEL: Stage / Preview -->
      <PreviewPanel />

      <!-- RIGHT PANEL: Source & Segments -->
      <SourceSegmentsPanel 
        v-model:videoUrl="videoUrl"
        v-model:scanMode="scanMode"
        @load-video="handleLoadVideo"
        @scan-heatmap="handleScanHeatmap"
        @scan-ai="handleScanAI"
      />
    </div>

    <!-- Bottom Area: Timeline & Action -->
    <TimelinePanel />
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useVideoStore } from '../stores/video';

// Import refactored components
import InspectorPanel from '../components/studio/InspectorPanel.vue';
import PreviewPanel from '../components/studio/PreviewPanel.vue';
import SourceSegmentsPanel from '../components/studio/SourceSegmentsPanel.vue';
import TimelinePanel from '../components/studio/TimelinePanel.vue';

const videoStore = useVideoStore();
const videoUrl = ref('');

const aiWhisper = ref(true);
const aiBRoll = ref(false);
const scanMode = ref('heatmap');

const handleLoadVideo = async () => {
  if (!videoUrl.value) return;
  await videoStore.previewVideo(videoUrl.value);
  // Auto-switch to heatmap if metadata loaded
  if (videoStore.metadata) {
    scanMode.value = 'heatmap';
  }
};

const handleScanHeatmap = async () => {
  if (!videoStore.metadata) return;
  await videoStore.processHeatmap(videoUrl.value);
};

const handleScanAI = async () => {
  if (!videoStore.metadata) return;
  videoStore.isScanningAI = true;
  
  try {
    const settingsStore = (await import('../stores/settings')).useSettingsStore();
    const browserName = settingsStore.config?.browser || null;
    
    // Panggil real AI backend endpoint
    const result: any = await invoke('scan_video', { 
      url: videoUrl.value, 
      cookiesPath: browserName,
      // parameter mode 'ai' diperlukan oleh Rust jika itu opsi scan yang berbeda
      // namun fungsi ini diasumsikan tetap memanggil logic scanning (ditambahkan di backend nanti)
    });
    
    if (result && result.segments) {
      videoStore.metadata.ai_segments = result.segments.map((s: any) => ({ ...s, selectedForRender: true }));
    }
  } catch (err: any) {
    const appStore = (await import('../stores/app')).useAppStore();
    appStore.addToast({
      title: 'AI Scan Gagal',
      message: String(err),
      type: 'error'
    });
  } finally {
    videoStore.isScanningAI = false;
  }
};
</script>
