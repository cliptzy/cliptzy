<template>
  <div class="h-full min-h-0 flex flex-col gap-4 max-w-[1600px] mx-auto w-full">
    <div class="flex-1 min-h-0 flex flex-col xl:flex-row gap-4 overflow-y-auto xl:overflow-hidden transition-all duration-300 ease-out">
      <InspectorPanel :mode="currentMode" />

      <PreviewPanel :mode="currentMode" />

      <SourceSegmentsPanel
        :mode="currentMode"
        v-model:videoUrl="videoUrl"
        v-model:scanMode="scanMode"
        v-model:compilationKeyword="compilationKeyword"
        @load-video="handleLoadVideo"
        @scan-heatmap="handleScanHeatmap"
        @scan-ai="handleScanAI"
      />
    </div>

    <TimelinePanel :mode="currentMode" class="transition-all duration-300 ease-out" />
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useVideoStore } from '../stores/video';
import { useSettingsStore } from '../stores/settings';

import InspectorPanel from '../components/studio/InspectorPanel.vue';
import PreviewPanel from '../components/studio/PreviewPanel.vue';
import SourceSegmentsPanel from '../components/studio/SourceSegmentsPanel.vue';
import TimelinePanel from '../components/studio/TimelinePanel.vue';

const props = defineProps<{
  mode?: string;
}>();

export type StudioMode = 'clipper' | 'compilation';

const currentMode = computed<StudioMode>(() =>
  props.mode === 'compilation' ? 'compilation' : 'clipper'
);

const videoStore = useVideoStore();
const settingsStore = useSettingsStore();
const videoUrl = ref('');
const scanMode = ref('heatmap');
const compilationKeyword = ref('');

const handleLoadVideo = async () => {
  if (!videoUrl.value) return;
  if (currentMode.value === 'compilation') {
    const keywords = compilationKeyword.value.trim() || undefined;
    await videoStore.prepareCompilation(videoUrl.value, keywords);
  } else {
    await videoStore.previewVideo(videoUrl.value);
    if (videoStore.metadata) {
      scanMode.value = 'heatmap';
    }
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
    const browserName = settingsStore.config?.browser || null;

    const result: any = await invoke('scan_video', {
      url: videoUrl.value,
      cookiesPath: browserName,
    });

    if (result && result.segments) {
      videoStore.metadata.ai_segments = result.segments.map((s: any) => ({ ...s, selectedForRender: true }));
    }
  } catch (err: any) {
    const appStore = (await import('../stores/app')).useAppStore();
    appStore.addToast({
      title: 'AI Scan Gagal',
      message: String(err),
      type: 'error',
    });
  } finally {
    videoStore.isScanningAI = false;
  }
};
</script>
