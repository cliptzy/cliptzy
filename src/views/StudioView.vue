<template>
  <div class="h-full min-h-0 flex flex-col gap-0 w-full border-l border-neutral">
    <!-- Top Header & Mode Switcher -->
    <header class="h-12 bg-base-100 border-b border-neutral flex items-center justify-between px-6 shrink-0">
      <div class="flex items-center gap-3">
        <IconScissors class="w-4 h-4 text-primary" v-if="currentMode === 'clipper'" />
        <IconLayers class="w-4 h-4 text-primary" v-else />
        <h2 class="text-sm font-black tracking-widest uppercase text-base-content">
          Studio <span class="text-secondary opacity-50 mx-1">/</span> {{ currentMode === 'clipper' ? 'Clipper' : 'Compilation' }}
        </h2>
      </div>

      <!-- Segmented Control -->
      <div class="flex items-center p-1 bg-base-200 border border-neutral">
        <router-link
          to="/studio/clipper"
          class="text-[10px] font-bold uppercase tracking-widest px-4 py-1 transition-colors flex items-center gap-2"
          :class="currentMode === 'clipper' ? 'bg-base-100 text-base-content border border-neutral' : 'text-secondary hover:text-base-content border border-transparent'"
        >
          <IconScissors class="w-3 h-3" /> Clipper
        </router-link>
        <router-link
          to="/studio/compilation"
          class="text-[10px] font-bold uppercase tracking-widest px-4 py-1 transition-colors flex items-center gap-2"
          :class="currentMode === 'compilation' ? 'bg-base-100 text-base-content border border-neutral' : 'text-secondary hover:text-base-content border border-transparent'"
        >
          <IconLayers class="w-3 h-3" /> Kompilasi
        </router-link>
      </div>
    </header>
    <div class="flex-1 min-h-0 flex flex-col xl:flex-row gap-0 overflow-y-auto xl:overflow-hidden border-b border-neutral">
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

    <TimelinePanel :mode="currentMode" />
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useVideoStore } from '../stores/video';
import { useSettingsStore } from '../stores/settings';

import IconScissors from '~icons/lucide/scissors';
import IconLayers from '~icons/lucide/layers';

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


