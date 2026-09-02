<template>
  <div class="w-full xl:w-[280px] flex flex-col gap-0 h-full min-h-0 overflow-y-auto pr-0 custom-scrollbar shrink-0 border-r border-neutral">
    <template v-if="mode === 'clipper'">
      <!-- Visual Crop & Tracking Controller -->
      <div class="p-6 shrink-0 bg-base-100 border-b border-neutral">
        <h3 class="text-lg font-black text-base-content tracking-wide mb-4 flex items-center gap-2">
          <IconCrop class="w-5 h-5" /> Mode Crop & Rasio
        </h3>
        <div class="flex flex-col gap-4">
          
          <!-- Output Ratio -->
          <div class="flex flex-col gap-1">
            <span class="text-[10px] text-secondary uppercase font-bold">Rasio Output</span>
            <select v-model="settings.config.output_ratio" class="w-full bg-base-200/50 border-none rounded-none p-2 text-xs font-bold text-base-content focus:outline-none focus:ring-2 focus:ring-gray-500 cursor-pointer">
              <option value="9:16">9:16 (Shorts / Reels)</option>
              <option value="1:1">1:1 (Square)</option>
              <option value="16:9">16:9 (Landscape)</option>
              <option value="original">Original</option>
            </select>
          </div>

          <!-- Crop Mode -->
          <div class="flex flex-col gap-3">
            <span class="text-[10px] text-secondary uppercase font-bold">Tipe Tampilan (Layout)</span>
            <label v-for="mode in CROP_MODES" :key="mode.value" class="flex items-center gap-3 cursor-pointer group" :title="mode.description">
              <input type="radio" name="cropMode" :value="mode.value" v-model="settings.config.crop_mode" class="hidden" />
              <div class="w-4 h-4 rounded-none border-2 flex items-center justify-center transition-colors" :class="settings.config.crop_mode === mode.value ? 'border-primary' : 'border-neutral'">
                <div v-show="settings.config.crop_mode === mode.value" class="w-2 h-2 rounded-none bg-primary"></div>
              </div>
              <span class="text-sm font-bold transition-colors flex items-center gap-2" :class="settings.config.crop_mode === mode.value ? 'text-base-content' : 'text-secondary'">
                <span>{{ mode.icon }}</span>
                <span>{{ mode.label }}</span>
                <span v-if="mode.isBeta" class="text-[9px] px-1.5 py-0.5 rounded bg-orange-500 text-primary-content font-black uppercase">Beta</span>
              </span>
            </label>
          </div>

          <!-- Conditional UI Hints -->
          <div v-if="selectedCropModeInfo?.requiresFaces" class="flex items-start gap-2 bg-base-200 rounded-none p-3 border border-accent/50">
            <IconSparkles class="w-4 h-4 text-accent shrink-0 mt-0.5" />
            <span class="text-[10px] text-base-content font-medium leading-tight">
              Mode ini membutuhkan face detection. Proses akan lebih lama karena AI tracking.
            </span>
          </div>

          <div v-if="selectedCropModeInfo?.requiresBroll" class="flex items-start gap-2 bg-amber-50 dark:bg-amber-900/30 rounded-none p-3 border border-amber-200 dark:border-amber-800">
            <span class="text-amber-600 dark:text-amber-400 text-lg">ℹ️</span>
            <div class="flex flex-col gap-1">
              <span class="text-[10px] text-amber-700 dark:text-amber-300 font-medium leading-tight">
                Pastikan folder B-roll memiliki minimal 1 video. Kelola di Settings > Assets.
              </span>
              <span class="text-[9px] text-amber-600 dark:text-amber-400 font-bold">
                Path: {{ settings.config.broll_dir || 'assets/broll' }}
              </span>
            </div>
          </div>

          <!-- Face Tracking Mode -->
          <div class="flex flex-col gap-1 border-t border-neutral pt-4 mt-1">
            <span class="text-[10px] text-secondary uppercase font-bold">Metode Pelacakan Wajah</span>
            <select v-model="settings.config.face_tracking_mode" class="w-full bg-base-200/50 border-none rounded-none p-2 text-xs font-bold text-base-content focus:outline-none focus:ring-2 focus:ring-gray-500 cursor-pointer">
              <option value="cinematic">Sinematik (Mulus & Lambat)</option>
              <option value="fast">Dinamis (Standard AI)</option>
              <option value="static">Statis (Kunci Posisi Awal)</option>
            </select>
            <span class="text-[9px] text-secondary mt-1 font-medium leading-tight">
              Menentukan bagaimana kamera mengikuti wajah saat di-crop.
            </span>
          </div>
        </div>
      </div>

      <!-- AI & Subtitle Command Center -->
      <div class="p-6 shrink-0 bg-base-100 border-b border-neutral">
        <h3 class="text-lg font-black text-base-content tracking-wide mb-4 flex items-center gap-2">
          <IconSparkles class="w-5 h-5" /> AI Processing
        </h3>
        <div class="flex flex-col gap-5">
          <div class="flex items-center justify-between">
            <div class="flex flex-col">
              <span class="text-sm font-bold text-base-content">Whisper AI</span>
              <span class="text-[10px] text-secondary font-medium">Subtitle otomatis</span>
            </div>
            <CToggle v-model="settings.config.subtitle.enabled" />
          </div>
          <div class="flex items-center justify-between">
            <div class="flex flex-col">
              <span class="text-sm font-bold text-base-content">Auto B-Roll</span>
              <span class="text-[10px] text-secondary font-medium">Sisipkan overlay meme/b-roll</span>
            </div>
            <CToggle v-model="settings.config.ai.use_add_meme" />
          </div>

          <!-- Pemisah -->
          <div class="w-full h-px bg-neutral my-1"></div>

          <div class="flex items-center justify-between">
            <div class="flex flex-col">
              <span class="text-sm font-bold text-base-content">Visual Emotion</span>
              <span class="text-[10px] text-secondary font-medium">Deteksi emosi wajah (ONNX)</span>
            </div>
            <CToggle v-model="settings.config.ai.use_emotion_detection" />
          </div>
          
          <div class="flex items-center justify-between">
            <div class="flex flex-col">
              <span class="text-sm font-bold text-base-content">Audio Event</span>
              <span class="text-[10px] text-secondary font-medium">Deteksi tawa, teriak, dll (AST)</span>
            </div>
            <CToggle v-model="settings.config.ai.use_audio_analysis" />
          </div>

          <div class="flex items-center justify-between">
            <div class="flex flex-col">
              <span class="text-sm font-bold text-base-content">Voice Tone</span>
              <span class="text-[10px] text-secondary font-medium">Deteksi intonasi vokal (SER)</span>
            </div>
            <CToggle v-model="settings.config.ai.use_voice_analysis" />
          </div>

          <div class="flex items-center justify-between">
            <div class="flex flex-col">
              <span class="text-sm font-bold text-base-content">Text Sentiment</span>
              <span class="text-[10px] text-secondary font-medium">Klasifikasi NLP transkrip</span>
            </div>
            <CToggle v-model="settings.config.ai.use_text_analysis" />
          </div>
        </div>
      </div>

      <!-- Subtitle Styling & Customization -->
      <div class="p-6 flex flex-col gap-4 shrink-0 bg-base-100 border-b border-neutral">
        <h3 class="text-lg font-black text-base-content tracking-wide mb-2 flex items-center gap-2">
          <IconType class="w-5 h-5" /> Kustomisasi Subtitle
        </h3>

        <div class="flex items-center justify-between">
          <div class="flex flex-col">
            <span class="text-sm font-bold text-base-content">Burn Subtitle</span>
            <span class="text-[10px] text-secondary font-medium">Tampilkan subtitle saat render</span>
          </div>
          <CToggle v-model="settings.config.burn_subtitle" />
        </div>

        <div :class="{ 'opacity-50 pointer-events-none': !settings.config.burn_subtitle }">
          <SubtitleStyleControls variant="compact" />
        </div>

        <!-- Action Generate Subtitle -->
        <div v-if="settings.config.subtitle.enabled" class="mt-1 border-t border-neutral pt-4">
          <button @click="handleGenerateSubtitle" :disabled="videoStore.isAnalyzing || !videoStore.metadata?.stream_url" class="w-full py-3 rounded-none text-xs font-bold transition-colors disabled:opacity-50 disabled:cursor-not-allowed shadow-sm bg-primary text-primary-content hover:bg-primary/90">
            <span v-if="videoStore.isAnalyzing" class="flex items-center justify-center gap-2">
              <IconLoader class="w-4 h-4 animate-spin" /> Sedang Generate...
            </span>
            <span v-else>Generate Subtitle</span>
          </button>
        </div>
      </div>

      <!-- Branding / Watermark -->
      <div class="p-6 shrink-0 bg-base-100 border-b border-neutral">
        <h3 class="text-lg font-black text-base-content tracking-wide mb-3 flex items-center gap-2">
          <IconImage class="w-5 h-5" /> Branding
        </h3>
        <div class="flex flex-col gap-4">
          <div class="flex items-center justify-between">
            <div class="flex flex-col">
              <span class="text-sm font-bold text-base-content">Burn Watermark</span>
              <span class="text-[10px] text-secondary font-medium">Tampilkan watermark saat render</span>
            </div>
            <CToggle v-model="settings.config.burn_watermark" />
          </div>
          <div class="flex flex-col gap-2" :class="{ 'opacity-50 pointer-events-none': !settings.config.burn_watermark }">
            <span class="text-[10px] text-secondary uppercase font-bold">Posisi Watermark</span>
            <select v-model="settings.config.watermark_position" class="w-full bg-base-200/50 border-none rounded-none p-3 text-xs font-bold text-base-content focus:outline-none focus:ring-2 focus:ring-gray-500 cursor-pointer">
              <option value="top">Atas (Top)</option>
              <option value="center">Tengah (Center)</option>
              <option value="bottom">Bawah (Bottom)</option>
            </select>
          </div>
        </div>
      </div>
    </template>

    <template v-else-if="mode === 'compilation'">
      <div class="p-6 shrink-0 bg-base-100 border-b border-neutral">
        <h3 class="text-lg font-black text-base-content tracking-wide mb-4 flex items-center gap-2">
          <IconFilm class="w-5 h-5" /> Tipe Kompilasi
        </h3>
        <div class="flex flex-col gap-3">
          <div
            v-for="opt in COMPILATION_TYPES"
            :key="opt.value"
            class="p-3 rounded-none cursor-pointer transition-all border-2"
            :class="settings.config.compilation.compilation_type === opt.value
              ? 'border-indigo-500 bg-base-200/70 dark:bg-base-300/50'
              : 'border-transparent bg-base-200/40 dark:bg-base-300/30 hover:bg-base-200/60 dark:hover:bg-base-300/40'"
            @click="onCompilationTypeChange(opt.value)"
          >
            <span class="text-sm font-bold text-base-content">{{ opt.label }}</span>
            <p class="text-[10px] text-secondary mt-0.5 leading-tight">{{ opt.description }}</p>
          </div>
        </div>
      </div>

      <div v-if="isReactionMode" class="p-6 shrink-0 bg-base-100 border-b border-neutral">
        <h3 class="text-lg font-black text-base-content tracking-wide mb-4 flex items-center gap-2">
          <IconAudioLines class="w-5 h-5" /> Sinkronisasi Audio
        </h3>
        <div class="flex flex-col gap-4">
          <p class="text-xs text-secondary leading-relaxed">
            Restreamer ditemukan dan diselaraskan otomatis via pencocokan audio terhadap momen epik dari video utama.
            Label nama restreamer ditambahkan otomatis di sudut video.
          </p>
          <div class="flex items-center justify-between">
            <div class="flex flex-col">
              <span class="text-sm font-bold text-base-content">Subtitle Kompilasi</span>
              <span class="text-[10px] text-secondary font-medium">Burn subtitle pada output</span>
            </div>
            <CToggle v-model="settings.config.compilation.use_subtitle" />
          </div>
        </div>
      </div>

      <div v-else class="p-6 shrink-0 bg-base-100 border-b border-neutral">
        <h3 class="text-lg font-black text-base-content tracking-wide mb-4 flex items-center gap-2">
          <IconListOrdered class="w-5 h-5" /> Urutan & Penomoran
        </h3>
        <div class="flex flex-col gap-4">
          <div class="flex flex-col gap-1">
            <span class="text-[10px] text-secondary uppercase font-bold">Urutan Kompilasi</span>
            <select
              v-model="settings.config.compilation.ordering"
              class="w-full bg-base-200/50 border-none rounded-none p-2 text-xs font-bold text-base-content focus:outline-none focus:ring-2 focus:ring-gray-500 cursor-pointer"
            >
              <option value="countdown">Countdown</option>
              <option value="chronological">Kronologis</option>
              <option value="score">Berdasarkan Skor</option>
            </select>
          </div>
          <div class="flex items-center justify-between">
            <div class="flex flex-col">
              <span class="text-sm font-bold text-base-content">TTS Penomoran</span>
              <span class="text-[10px] text-secondary font-medium">Bacakan nomor urut</span>
            </div>
            <CToggle v-model="settings.config.compilation.use_tts" />
          </div>
          <div class="flex items-center justify-between">
            <div class="flex flex-col">
              <span class="text-sm font-bold text-base-content">Subtitle Kompilasi</span>
              <span class="text-[10px] text-secondary font-medium">Burn subtitle pada output</span>
            </div>
            <CToggle v-model="settings.config.compilation.use_subtitle" />
          </div>
        </div>
      </div>

      <div v-if="isReactionMode" class="p-6 shrink-0 bg-base-100 border-b border-neutral">
        <h3 class="text-lg font-black text-base-content tracking-wide mb-4 flex items-center gap-2">
          <IconMonitor class="w-5 h-5" /> Output Reaksi
        </h3>
        <div class="flex flex-col gap-3 text-xs text-secondary">
          <div class="flex justify-between items-center bg-base-200/40 dark:bg-base-300/30 rounded-none px-3 py-2">
            <span class="font-bold">Rasio</span>
            <span class="font-black text-base-content">16:9 Horizontal</span>
          </div>
          <div class="flex justify-between items-center bg-base-200/40 dark:bg-base-300/30 rounded-none px-3 py-2">
            <span class="font-bold">Crop</span>
            <span class="font-black text-base-content">Tanpa crop (resolusi asli)</span>
          </div>
          <div class="flex justify-between items-center bg-base-200/40 dark:bg-base-300/30 rounded-none px-3 py-2">
            <span class="font-bold">Durasi segmen</span>
            <span class="font-black text-base-content">Tidak terbatas</span>
          </div>
        </div>
      </div>

      <div v-else class="p-6 shrink-0 bg-base-100 border-b border-neutral">
        <h3 class="text-lg font-black text-base-content tracking-wide mb-4 flex items-center gap-2">
          <IconType class="w-5 h-5" /> Layout Kompilasi
        </h3>
        <div class="flex flex-col gap-4">
                    <div class="flex flex-col gap-1">
            <span class="text-[10px] text-secondary uppercase font-bold">Crop Mode</span>
            <select
              v-model="settings.config.compilation.crop_mode"
              class="w-full bg-base-200/50 border-none rounded-none p-2 text-xs font-bold text-base-content focus:outline-none focus:ring-2 focus:ring-gray-500 cursor-pointer"
            >
              <option v-for="opt in COMPILATION_CROP_MODES" :key="opt.value" :value="opt.value">
                {{ opt.label }}
              </option>
            </select>
            <span v-if="settings.config.compilation.crop_mode === 'none'" class="text-[9px] text-secondary mt-0.5">
              Resolusi horizontal asli dipertahankan — hanya trim durasi.
            </span>
            <span v-else-if="settings.config.compilation.crop_mode === 'full_face' || settings.config.compilation.crop_mode === 'center_face' || settings.config.compilation.crop_mode === 'split_face' || settings.config.compilation.crop_mode === 'multi_face'" class="text-[9px] text-secondary mt-0.5">
              ℹ️ Membutuhkan face detection. Proses akan lebih lama.
            </span>
            <span v-else-if="settings.config.compilation.crop_mode === 'split_broll'" class="text-[9px] text-secondary mt-0.5">
              ℹ️ Pastikan folder B-roll memiliki minimal 1 video. Kelola di Settings > Assets.
            </span>
          </div>
          <div class="flex flex-col gap-1">
            <span class="text-[10px] text-secondary uppercase font-bold">
              Batas Durasi Segmen
              <template v-if="settings.config.compilation.max_segment_duration === 0">(Tidak terbatas)</template>
              <template v-else>({{ settings.config.compilation.max_segment_duration }}s)</template>
            </span>
            <input
              type="range"
              min="0"
              max="600"
              step="15"
              v-model.number="settings.config.compilation.max_segment_duration"
              class="w-full h-2 bg-neutral rounded-none appearance-none cursor-pointer mt-1 accent-primary"
            />
            <span class="text-[9px] text-secondary">
              Batas maksimum durasi per segmen meme shorts.
            </span>
          </div>
          <div class="flex flex-col gap-1">
            <span class="text-[10px] text-secondary uppercase font-bold">Durasi Penomoran ({{ settings.config.compilation.numbering_duration }}s)</span>
            <input
              type="range"
              min="1"
              max="10"
              step="0.5"
              v-model.number="settings.config.compilation.numbering_duration"
              class="w-full h-2 bg-neutral rounded-none appearance-none cursor-pointer mt-1 accent-primary"
            />
          </div>
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">

import SubtitleStyleControls from '../SubtitleStyleControls.vue';
import { useSettingsStore } from '../../stores/settings';
import { useVideoStore } from '../../stores/video';

import IconCrop from '~icons/lucide/crop';
import IconSparkles from '~icons/lucide/sparkles';
import IconType from '~icons/lucide/type';
import IconImage from '~icons/lucide/image';
import IconLoader from '~icons/lucide/loader-2';
import IconFilm from '~icons/lucide/film';
import IconAudioLines from '~icons/lucide/audio-lines';
import IconListOrdered from '~icons/lucide/list-ordered';
import IconMonitor from '~icons/lucide/monitor';
import { watch, computed } from 'vue';
import {
  COMPILATION_TYPES,
  COMPILATION_CROP_MODES,
  applyCompilationTypeDefaults,
  isReactionCompilation,
  type CompilationType,
} from '../../constants/compilation';
import { CROP_MODES, getCropModeInfo } from '../../constants/cropModes';

const props = defineProps<{
  mode?: 'clipper' | 'compilation'
}>();

const settings = useSettingsStore();
const videoStore = useVideoStore();

const isReactionMode = computed(() =>
  isReactionCompilation(settings.config.compilation.compilation_type),
);

const selectedCropModeInfo = computed(() =>
  getCropModeInfo(settings.config.crop_mode)
);

const onCompilationTypeChange = (type: CompilationType) => {
  applyCompilationTypeDefaults(type, settings.config.compilation);
  if (type === 'reaction') {
    settings.setRatioPreset('16:9');
  }
};

const handleGenerateSubtitle = async () => {
  if (!videoStore.currentUrl) return;
  // Use selected segment
  const start = videoStore.selectedSegment?.start || videoStore.currentTime || 0;
  const end = videoStore.selectedSegment?.end || start + 60; 
  await videoStore.analyzeSegmentAudio(videoStore.currentUrl, start, end, videoStore.metadata?.stream_url);
};

// Auto-generate if a new segment is selected and aiWhisper is enabled
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

watch(() => settings.config.subtitle.enabled, (newVal) => {
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


