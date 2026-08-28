<template>
  <div class="w-full xl:w-[280px] flex flex-col gap-4 overflow-y-auto pr-1 custom-scrollbar shrink-0">
    <!-- Visual Crop & Tracking Controller -->
  <BentoCard class="p-6 shrink-0 !bg-sky-100 dark:!bg-sky-900/40">
      <h3 class="text-lg font-black text-[var(--color-text-main)] tracking-wide mb-4 flex items-center gap-2">
        <IconCrop class="w-5 h-5" /> Mode Crop & Rasio
      </h3>
      <div class="flex flex-col gap-4">
        
        <!-- Output Ratio -->
        <div class="flex flex-col gap-1">
          <span class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold">Rasio Output</span>
          <select v-model="settings.config.output_ratio" class="w-full bg-white/50 dark:bg-black/30 border-none rounded-xl p-2 text-xs font-bold text-[var(--color-text-main)] focus:outline-none focus:ring-2 focus:ring-gray-500 cursor-pointer">
            <option value="9:16">9:16 (Shorts / Reels)</option>
            <option value="1:1">1:1 (Square)</option>
            <option value="16:9">16:9 (Landscape)</option>
            <option value="original">Original</option>
          </select>
        </div>

        <!-- Crop Mode -->
        <div class="flex flex-col gap-3">
          <span class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold">Tipe Tampilan (Layout)</span>
          <label class="flex items-center gap-3 cursor-pointer group">
            <input type="radio" name="cropMode" value="default" v-model="settings.config.crop_mode" class="hidden" />
            <div class="w-4 h-4 rounded-full border-2 flex items-center justify-center transition-colors" :class="settings.config.crop_mode === 'default' ? 'border-gray-700 dark:border-gray-400' : 'border-gray-400 dark:border-gray-700'">
              <div v-show="settings.config.crop_mode === 'default'" class="w-2 h-2 rounded-full bg-gray-700 dark:bg-gray-400"></div>
            </div>
            <span class="text-sm font-bold transition-colors" :class="settings.config.crop_mode === 'default' ? 'text-[var(--color-text-main)] ' : 'text-[var(--color-text-muted)] '">Center Crop (Default)</span>
          </label>
          <label class="flex items-center gap-3 cursor-pointer group">
            <input type="radio" name="cropMode" value="center_face" v-model="settings.config.crop_mode" class="hidden" />
            <div class="w-4 h-4 rounded-full border-2 flex items-center justify-center transition-colors" :class="settings.config.crop_mode === 'center_face' ? 'border-gray-700 dark:border-gray-400' : 'border-gray-400 dark:border-gray-700'">
              <div v-show="settings.config.crop_mode === 'center_face'" class="w-2 h-2 rounded-full bg-gray-700 dark:bg-gray-400"></div>
            </div>
            <span class="text-sm font-bold transition-colors" :class="settings.config.crop_mode === 'center_face' ? 'text-[var(--color-text-main)] ' : 'text-[var(--color-text-muted)] '">Center Face (Track)</span>
          </label>
          <label class="flex items-center gap-3 cursor-pointer group">
            <input type="radio" name="cropMode" value="full" v-model="settings.config.crop_mode" class="hidden" />
            <div class="w-4 h-4 rounded-full border-2 flex items-center justify-center transition-colors" :class="settings.config.crop_mode === 'full' ? 'border-gray-700 dark:border-gray-400' : 'border-gray-400 dark:border-gray-700'">
              <div v-show="settings.config.crop_mode === 'full'" class="w-2 h-2 rounded-full bg-gray-700 dark:bg-gray-400"></div>
            </div>
            <span class="text-sm font-bold transition-colors" :class="settings.config.crop_mode === 'full' ? 'text-[var(--color-text-main)] ' : 'text-[var(--color-text-muted)] '">Full + Blur Background</span>
          </label>
          <label class="flex items-center gap-3 cursor-pointer group">
            <input type="radio" name="cropMode" value="full_face" v-model="settings.config.crop_mode" class="hidden" />
            <div class="w-4 h-4 rounded-full border-2 flex items-center justify-center transition-colors" :class="settings.config.crop_mode === 'full_face' ? 'border-gray-700 dark:border-gray-400' : 'border-gray-400 dark:border-gray-700'">
              <div v-show="settings.config.crop_mode === 'full_face'" class="w-2 h-2 rounded-full bg-gray-700 dark:bg-gray-400"></div>
            </div>
            <span class="text-sm font-bold transition-colors" :class="settings.config.crop_mode === 'full_face' ? 'text-[var(--color-text-main)] ' : 'text-[var(--color-text-muted)] '">Face Track + Full (Split)</span>
          </label>
        </div>

        <!-- Face Tracking Mode -->
        <div class="flex flex-col gap-1 border-t border-gray-300 dark:border-gray-800 pt-4 mt-1">
          <span class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold">Metode Pelacakan Wajah</span>
          <select v-model="settings.config.face_tracking_mode" class="w-full bg-white/50 dark:bg-black/30 border-none rounded-xl p-2 text-xs font-bold text-[var(--color-text-main)] focus:outline-none focus:ring-2 focus:ring-gray-500 cursor-pointer">
            <option value="cinematic">Sinematik (Mulus & Lambat)</option>
            <option value="fast">Dinamis (Standard AI)</option>
            <option value="static">Statis (Kunci Posisi Awal)</option>
          </select>
          <span class="text-[9px] text-[var(--color-text-muted)] mt-1 font-medium leading-tight">
            Menentukan bagaimana kamera mengikuti wajah saat di-crop.
          </span>
        </div>
      </div>
    </BentoCard>

    <!-- AI & Subtitle Command Center -->
  <BentoCard class="p-6 shrink-0 !bg-fuchsia-100 dark:!bg-fuchsia-900/40">
      <h3 class="text-lg font-black text-[var(--color-text-main)] tracking-wide mb-4 flex items-center gap-2">
        <IconSparkles class="w-5 h-5" /> AI Processing
      </h3>
      <div class="flex flex-col gap-5">
        <div class="flex items-center justify-between">
          <div class="flex flex-col">
            <span class="text-sm font-bold text-[var(--color-text-main)]">Whisper AI</span>
            <span class="text-[10px] text-[var(--color-text-muted)] font-medium">Subtitle otomatis</span>
          </div>
          <ToggleSwitch v-model="aiWhisper" />
        </div>
        <div class="flex items-center justify-between">
          <div class="flex flex-col">
            <span class="text-sm font-bold text-[var(--color-text-main)]">Auto B-Roll</span>
            <span class="text-[10px] text-[var(--color-text-muted)] font-medium">Sisipkan overlay</span>
          </div>
          <ToggleSwitch v-model="aiBRoll" />
        </div>
      </div>
    </BentoCard>

    <!-- Subtitle Styling & Customization -->
  <BentoCard class="p-6 flex flex-col gap-4 shrink-0 !bg-lime-100 dark:!bg-lime-900/40">
      <h3 class="text-lg font-black text-[var(--color-text-main)] tracking-wide mb-2 flex items-center gap-2">
        <IconType class="w-5 h-5" /> Kustomisasi Subtitle
      </h3>
      
      <!-- Presets -->
      <div class="grid grid-cols-2 gap-2">
        <button @click="settings.config.subtitle.animation = 'hormozi'; settings.config.subtitle.border_style = 1" class="p-3 rounded-2xl transition-all text-center flex flex-col items-center justify-center gap-1 bg-white/50 dark:bg-black/30 hover:bg-white dark:hover:bg-black/50" :class="settings.config.subtitle.animation === 'hormozi' ? 'ring-2 ring-gray-500 shadow-sm' : ''">
          <span class="font-black text-xs uppercase text-[var(--color-text-muted)]">Hormozi</span>
        </button>
        <button @click="settings.config.subtitle.animation = 'karaoke'; settings.config.subtitle.border_style = 1" class="p-3 rounded-2xl transition-all text-center flex flex-col items-center justify-center gap-1 bg-white/50 dark:bg-black/30 hover:bg-white dark:hover:bg-black/50" :class="settings.config.subtitle.animation === 'karaoke' ? 'ring-2 ring-gray-500 shadow-sm' : ''">
          <span class="font-black text-xs text-[var(--color-text-muted)]">Karaoke</span>
        </button>
        <button @click="settings.config.subtitle.border_style = 3; settings.config.subtitle.animation = 'none'" class="p-3 rounded-2xl transition-all text-center flex flex-col items-center justify-center gap-1 col-span-2 bg-white/50 dark:bg-black/30 hover:bg-white dark:hover:bg-black/50" :class="settings.config.subtitle.border_style === 3 ? 'ring-2 ring-gray-500 shadow-sm' : ''">
          <span class="font-mono text-xs uppercase tracking-widest text-white bg-red-600 px-2 py-0.5 font-bold">BRUTALIST BOX</span>
        </button>
      </div>

      <!-- Action Generate Subtitle -->
      <div v-if="aiWhisper" class="mt-2">
        <button @click="handleGenerateSubtitle" :disabled="videoStore.isAnalyzing || !videoStore.metadata?.stream_url" class="w-full py-3 rounded-full text-xs font-bold transition-colors disabled:opacity-50 disabled:cursor-not-allowed shadow-sm bg-indigo-600 text-white hover:bg-indigo-700">
          <span v-if="videoStore.isAnalyzing" class="flex items-center justify-center gap-2">
            <IconLoader class="w-4 h-4 animate-spin" /> Sedang Generate...
          </span>
          <span v-else>Generate Subtitle</span>
        </button>
      </div>

      <!-- Deep Subtitle Settings -->
      <div class="flex flex-col gap-4 mt-2 border-t border-gray-300 dark:border-gray-800 pt-4">
        <div class="grid grid-cols-2 gap-3">
          <div class="flex flex-col gap-1">
            <span class="text-[9px] text-[var(--color-text-muted)] uppercase font-bold">Font</span>
            <select v-model="settings.config.subtitle.font" class="w-full bg-white/50 dark:bg-black/30 border-none rounded-xl p-2 text-[10px] font-bold text-[var(--color-text-main)] focus:outline-none focus:ring-2 focus:ring-gray-500 cursor-pointer">
              <option value="Arial">Arial</option>
              <option value="Impact">Impact</option>
              <option value="TheBoldFont">TheBoldFont</option>
            </select>
          </div>
          <div class="flex flex-col gap-1">
            <span class="text-[9px] text-[var(--color-text-muted)] uppercase font-bold">Warna</span>
            <select v-model="settings.config.subtitle.color" class="w-full bg-white/50 dark:bg-black/30 border-none rounded-xl p-2 text-[10px] font-bold text-[var(--color-text-main)] focus:outline-none focus:ring-2 focus:ring-gray-500 cursor-pointer">
              <option value="&H0000FFFF">Kuning</option>
              <option value="&H00FFFFFF">Putih</option>
              <option value="&H0000FF00">Hijau</option>
            </select>
          </div>
          <div class="flex flex-col gap-1">
            <span class="text-[9px] text-[var(--color-text-muted)] uppercase font-bold">Posisi Y</span>
            <select v-model="settings.config.subtitle.location" class="w-full bg-white/50 dark:bg-black/30 border-none rounded-xl p-2 text-[10px] font-bold text-[var(--color-text-main)] focus:outline-none focus:ring-2 focus:ring-gray-500 cursor-pointer">
              <option value="bottom">Bawah</option>
              <option value="center">Tengah</option>
              <option value="top">Atas</option>
            </select>
          </div>
          <div class="flex flex-col gap-1">
            <span class="text-[9px] text-[var(--color-text-muted)] uppercase font-bold">Maks Kata ({{ settings.config.subtitle.max_words }})</span>
            <input type="range" min="1" max="10" v-model.number="settings.config.subtitle.max_words" class="w-full h-2 bg-gray-300 dark:bg-gray-800 rounded-lg appearance-none cursor-pointer mt-1.5 accent-[var(--color-accent)]" />
          </div>
        </div>
        <div class="flex flex-col gap-1">
          <span class="text-[9px] text-[var(--color-text-muted)] uppercase font-bold">Ukuran Font ({{ settings.config.subtitle.font_size }})</span>
          <input type="range" min="20" max="150" v-model.number="settings.config.subtitle.font_size" class="w-full h-2 bg-gray-300 dark:bg-gray-800 rounded-lg appearance-none cursor-pointer mt-1 accent-[var(--color-accent)]" />
        </div>
      </div>
    </BentoCard>

    <!-- Branding / Watermark -->
  <BentoCard class="p-6 shrink-0 !bg-teal-100 dark:!bg-teal-900/40">
      <h3 class="text-lg font-black text-[var(--color-text-main)] tracking-wide mb-3 flex items-center gap-2">
        <IconImage class="w-5 h-5" /> Branding
      </h3>
      <div class="flex flex-col gap-2">
        <span class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold">Posisi Watermark</span>
        <select v-model="settings.config.watermark_position" class="w-full bg-white/50 dark:bg-black/30 border-none rounded-xl p-3 text-xs font-bold text-[var(--color-text-main)] focus:outline-none focus:ring-2 focus:ring-gray-500 cursor-pointer">
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

// Icons
import IconCrop from '~icons/lucide/crop';
import IconSparkles from '~icons/lucide/sparkles';
import IconType from '~icons/lucide/type';
import IconImage from '~icons/lucide/image';
import IconLoader from '~icons/lucide/loader-2';

const settings = useSettingsStore();
const videoStore = useVideoStore();

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
watch(() => videoStore.selectedSegment, (_newSegment, _oldSegment) => {
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
