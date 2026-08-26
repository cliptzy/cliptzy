<template>
  <div class="w-full xl:w-[280px] flex flex-col gap-4 overflow-y-auto pr-1 custom-scrollbar shrink-0">
    <!-- Visual Crop & Tracking Controller -->
    <BentoCard class="p-4 shrink-0">
      <h3 class="text-sm font-bold text-gray-400 uppercase tracking-wider mb-4 flex items-center gap-2">
        <IconCrop class="w-4 h-4 text-[var(--color-accent)]" /> Mode Crop & Rasio
      </h3>
      <div class="flex flex-col gap-4">
        
        <!-- Output Ratio -->
        <div class="flex flex-col gap-1">
          <span class="text-[10px] text-gray-400 uppercase font-bold">Rasio Output</span>
          <select v-model="settings.config.output_ratio" class="w-full bg-black/50 border border-[var(--color-subtle)] rounded p-1.5 text-xs text-white focus:outline-none focus:border-[var(--color-accent)]">
            <option value="9:16">9:16 (Shorts / Reels)</option>
            <option value="1:1">1:1 (Square)</option>
            <option value="16:9">16:9 (Landscape)</option>
            <option value="original">Original</option>
          </select>
        </div>

        <!-- Crop Mode -->
        <div class="flex flex-col gap-3">
          <span class="text-[10px] text-gray-400 uppercase font-bold">Tipe Tracking</span>
          <label class="flex items-center gap-3 cursor-pointer group">
            <input type="radio" name="cropMode" value="auto" v-model="cropMode" class="hidden" />
            <div class="w-4 h-4 rounded-full border flex items-center justify-center group-hover:border-[var(--color-accent)] transition-colors" :class="cropMode === 'auto' ? 'border-[var(--color-accent)]' : 'border-gray-500'">
              <div v-show="cropMode === 'auto'" class="w-2 h-2 rounded-full bg-[var(--color-accent)]"></div>
            </div>
            <span class="text-sm font-medium transition-colors" :class="cropMode === 'auto' ? 'text-white' : 'text-gray-400 group-hover:text-gray-200'">AI Auto-Tracking</span>
          </label>
          <label class="flex items-center gap-3 cursor-pointer group">
            <input type="radio" name="cropMode" value="static" v-model="cropMode" class="hidden" />
            <div class="w-4 h-4 rounded-full border flex items-center justify-center group-hover:border-[var(--color-accent)] transition-colors" :class="cropMode === 'static' ? 'border-[var(--color-accent)]' : 'border-gray-500'">
              <div v-show="cropMode === 'static'" class="w-2 h-2 rounded-full bg-[var(--color-accent)]"></div>
            </div>
            <span class="text-sm font-medium transition-colors" :class="cropMode === 'static' ? 'text-white' : 'text-gray-400 group-hover:text-gray-200'">Static Center</span>
          </label>
          <label class="flex items-center gap-3 cursor-pointer group">
            <input type="radio" name="cropMode" value="split" v-model="cropMode" class="hidden" />
            <div class="w-4 h-4 rounded-full border flex items-center justify-center group-hover:border-[var(--color-accent)] transition-colors" :class="cropMode === 'split' ? 'border-[var(--color-accent)]' : 'border-gray-500'">
              <div v-show="cropMode === 'split'" class="w-2 h-2 rounded-full bg-[var(--color-accent)]"></div>
            </div>
            <span class="text-sm font-medium transition-colors" :class="cropMode === 'split' ? 'text-white' : 'text-gray-400 group-hover:text-gray-200'">Split Screen</span>
          </label>
        </div>
      </div>
    </BentoCard>

    <!-- AI & Subtitle Command Center -->
    <BentoCard class="p-4 shrink-0">
      <h3 class="text-sm font-bold text-gray-400 uppercase tracking-wider mb-4 flex items-center gap-2">
        <IconSparkles class="w-4 h-4 text-[var(--color-accent)]" /> AI Processing
      </h3>
      <div class="flex flex-col gap-4">
        <div class="flex items-center justify-between">
          <div class="flex flex-col">
            <span class="text-sm font-semibold text-white">Whisper AI</span>
            <span class="text-[10px] text-gray-500">Subtitle otomatis</span>
          </div>
          <ToggleSwitch v-model="aiWhisper" />
        </div>
        <div class="flex items-center justify-between">
          <div class="flex flex-col">
            <span class="text-sm font-semibold text-white">Auto B-Roll</span>
            <span class="text-[10px] text-gray-500">Sisipkan overlay</span>
          </div>
          <ToggleSwitch v-model="aiBRoll" />
        </div>
      </div>
    </BentoCard>

    <!-- Subtitle Styling & Customization -->
    <BentoCard class="p-4 flex flex-col gap-4 shrink-0">
      <h3 class="text-sm font-bold text-gray-400 uppercase tracking-wider mb-2 flex items-center gap-2">
        <IconType class="w-4 h-4 text-[var(--color-accent)]" /> Kustomisasi Subtitle
      </h3>
      
      <!-- Presets -->
      <div class="grid grid-cols-2 gap-2">
        <button @click="settings.config.subtitle.animation = 'hormozi'; settings.config.subtitle.border_style = 1" class="p-2 rounded border transition-all text-left flex flex-col gap-1 group" :class="settings.config.subtitle.animation === 'hormozi' ? 'border-[var(--color-accent)] bg-[var(--color-accent)]/10' : 'border-[var(--color-subtle)] bg-white/5 hover:border-gray-500'">
          <span class="font-black text-xs uppercase text-yellow-400">Hormozi</span>
        </button>
        <button @click="settings.config.subtitle.animation = 'karaoke'; settings.config.subtitle.border_style = 1" class="p-2 rounded border transition-all text-left flex flex-col gap-1 group" :class="settings.config.subtitle.animation === 'karaoke' ? 'border-[var(--color-accent)] bg-[var(--color-accent)]/10' : 'border-[var(--color-subtle)] bg-white/5 hover:border-gray-500'">
          <span class="font-bold text-xs text-green-400">Karaoke</span>
        </button>
        <button @click="settings.config.subtitle.border_style = 3; settings.config.subtitle.animation = 'none'" class="p-2 rounded border transition-all text-left flex flex-col gap-1 col-span-2 group" :class="settings.config.subtitle.border_style === 3 ? 'border-[var(--color-accent)] bg-[var(--color-accent)]/10' : 'border-[var(--color-subtle)] bg-white/5 hover:border-gray-500'">
          <span class="font-mono text-xs uppercase tracking-widest text-white bg-red-600 px-1 w-fit">BRUTALIST BOX</span>
        </button>
      </div>

      <!-- Action Generate Subtitle -->
      <div v-if="aiWhisper" class="mt-1">
        <GlowButton @click="handleGenerateSubtitle" :disabled="videoStore.isAnalyzing || !videoStore.metadata?.stream_url" class="w-full py-2 text-xs">
          <span v-if="videoStore.isAnalyzing" class="flex items-center justify-center gap-2">
            <IconLoader class="w-3 h-3 animate-spin" /> Sedang Generate...
          </span>
          <span v-else>Generate Subtitle (Pre-Analysis)</span>
        </GlowButton>
      </div>

      <!-- Deep Subtitle Settings -->
      <div class="flex flex-col gap-3 mt-2 border-t border-[var(--color-subtle)] pt-4">
        <div class="grid grid-cols-2 gap-3">
          <div class="flex flex-col gap-1">
            <span class="text-[9px] text-gray-400 uppercase font-bold">Font</span>
            <select v-model="settings.config.subtitle.font" class="w-full bg-black/50 border border-[var(--color-subtle)] rounded p-1.5 text-[10px] text-white focus:outline-none focus:border-[var(--color-accent)]">
              <option value="Arial">Arial</option>
              <option value="Impact">Impact</option>
              <option value="TheBoldFont">TheBoldFont</option>
            </select>
          </div>
          <div class="flex flex-col gap-1">
            <span class="text-[9px] text-gray-400 uppercase font-bold">Warna</span>
            <select v-model="settings.config.subtitle.color" class="w-full bg-black/50 border border-[var(--color-subtle)] rounded p-1.5 text-[10px] text-white focus:outline-none focus:border-[var(--color-accent)]">
              <option value="&H0000FFFF">Kuning</option>
              <option value="&H00FFFFFF">Putih</option>
              <option value="&H0000FF00">Hijau</option>
            </select>
          </div>
          <div class="flex flex-col gap-1">
            <span class="text-[9px] text-gray-400 uppercase font-bold">Posisi Y</span>
            <select v-model="settings.config.subtitle.location" class="w-full bg-black/50 border border-[var(--color-subtle)] rounded p-1.5 text-[10px] text-white focus:outline-none focus:border-[var(--color-accent)]">
              <option value="bottom">Bawah</option>
              <option value="center">Tengah</option>
              <option value="top">Atas</option>
            </select>
          </div>
          <div class="flex flex-col gap-1">
            <span class="text-[9px] text-gray-400 uppercase font-bold">Maks Kata ({{ settings.config.subtitle.max_words }})</span>
            <input type="range" min="1" max="10" v-model.number="settings.config.subtitle.max_words" class="w-full h-1 bg-[var(--color-subtle)] rounded-lg appearance-none cursor-pointer accent-[var(--color-accent)] mt-1.5" />
          </div>
        </div>
        <div class="flex flex-col gap-1">
          <span class="text-[9px] text-gray-400 uppercase font-bold">Ukuran Font ({{ settings.config.subtitle.font_size }})</span>
          <input type="range" min="20" max="150" v-model.number="settings.config.subtitle.font_size" class="w-full h-1 bg-[var(--color-subtle)] rounded-lg appearance-none cursor-pointer accent-[var(--color-accent)] mt-1" />
        </div>
      </div>
    </BentoCard>

    <!-- Branding / Watermark -->
    <BentoCard class="p-4 shrink-0">
      <h3 class="text-sm font-bold text-gray-400 uppercase tracking-wider mb-3 flex items-center gap-2">
        <IconImage class="w-4 h-4 text-[var(--color-accent)]" /> Branding
      </h3>
      <div class="flex flex-col gap-1">
        <span class="text-[10px] text-gray-400 uppercase font-bold">Posisi Watermark</span>
        <select v-model="settings.config.watermark_position" class="w-full bg-black/50 border border-[var(--color-subtle)] rounded p-2 text-xs text-white focus:outline-none focus:border-[var(--color-accent)]">
          <option value="top">Atas (Top)</option>
          <option value="center">Tengah (Center)</option>
          <option value="bottom">Bawah (Bottom)</option>
        </select>
      </div>
    </BentoCard>
  </div>
</template>

<script setup lang="ts">
import { useSettingsStore } from '../../stores/settings';
import { useVideoStore } from '../../stores/video';
import BentoCard from '../BentoCard.vue';
import ToggleSwitch from '../ToggleSwitch.vue';
import GlowButton from '../GlowButton.vue';

// Icons
import IconCrop from '~icons/lucide/crop';
import IconSparkles from '~icons/lucide/sparkles';
import IconType from '~icons/lucide/type';
import IconImage from '~icons/lucide/image';
import IconLoader from '~icons/lucide/loader-2';

const settings = useSettingsStore();
const videoStore = useVideoStore();

const cropMode = defineModel('cropMode', { type: String, default: 'auto' });
const aiWhisper = defineModel('aiWhisper', { type: Boolean, default: true });
const aiBRoll = defineModel('aiBRoll', { type: Boolean, default: false });

const handleGenerateSubtitle = async () => {
  if (!videoStore.currentUrl) return;
  // Use selected segment
  const start = videoStore.selectedSegment?.start || videoStore.currentTime || 0;
  const end = videoStore.selectedSegment?.end || start + 60; 
  await videoStore.analyzeSegmentAudio(videoStore.currentUrl, start, end, videoStore.metadata?.stream_url);
};

// Auto-generate if a new segment is selected and aiWhisper is enabled
import { watch } from 'vue';
watch(() => videoStore.selectedSegment, (newSegment, oldSegment) => {
  // Disabled auto-transcribe on segment click
  /*
  if (aiWhisper.value && newSegment && newSegment.start !== oldSegment?.start) {
    const key = `${newSegment.start}-${newSegment.end}`;
    if (!videoStore.analyzedSegments[key] && !videoStore.isAnalyzing) {
      handleGenerateSubtitle();
    }
  }
  */
});

watch(aiWhisper, (newVal) => {
  if (newVal && videoStore.selectedSegment) {
    const key = `${videoStore.selectedSegment.start}-${videoStore.selectedSegment.end}`;
    if (!videoStore.analyzedSegments[key] && !videoStore.isAnalyzing) {
      handleGenerateSubtitle();
    }
  }
});
</script>

<style scoped>
.custom-scrollbar::-webkit-scrollbar {
  width: 4px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: rgba(255, 255, 255, 0.02);
  border-radius: 4px;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.1);
  border-radius: 4px;
}
.custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.2);
}
</style>
