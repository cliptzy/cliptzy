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
  // This would call the AI endpoint in backend
  videoStore.isScanningAI = true;
  // Simulate AI scan delay
  setTimeout(() => {
    videoStore.metadata!.ai_segments = [
      { start: 10.5, end: 45.2, reason: 'Pengenalan topik utama dengan nada emosional tinggi', selectedForRender: true },
      { start: 120.0, end: 180.5, reason: 'Klimaks perdebatan atau poin paling kontroversial', selectedForRender: true },
      { start: 300.0, end: 345.0, reason: 'Kesimpulan dan CTA yang kuat', selectedForRender: true }
    ];
    videoStore.isScanningAI = false;
  }, 3000);
};
</script>
