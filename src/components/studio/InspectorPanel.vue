<template>
  <div class="w-full xl:w-[280px] flex flex-col gap-4 h-full min-h-0 overflow-y-auto pr-1 custom-scrollbar shrink-0">
    <template v-if="mode === 'clipper'">
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
            <ToggleSwitch v-model="settings.config.subtitle.enabled" />
          </div>
          <div class="flex items-center justify-between">
            <div class="flex flex-col">
              <span class="text-sm font-bold text-[var(--color-text-main)]">Auto B-Roll</span>
              <span class="text-[10px] text-[var(--color-text-muted)] font-medium">Sisipkan overlay meme/b-roll</span>
            </div>
            <ToggleSwitch v-model="settings.config.ai.use_add_meme" />
          </div>

          <!-- Pemisah -->
          <div class="w-full h-px bg-fuchsia-200 dark:bg-fuchsia-800/50 my-1"></div>

          <div class="flex items-center justify-between">
            <div class="flex flex-col">
              <span class="text-sm font-bold text-[var(--color-text-main)]">Visual Emotion</span>
              <span class="text-[10px] text-[var(--color-text-muted)] font-medium">Deteksi emosi wajah (ONNX)</span>
            </div>
            <ToggleSwitch v-model="settings.config.ai.use_emotion_detection" />
          </div>
          
          <div class="flex items-center justify-between">
            <div class="flex flex-col">
              <span class="text-sm font-bold text-[var(--color-text-main)]">Audio Event</span>
              <span class="text-[10px] text-[var(--color-text-muted)] font-medium">Deteksi tawa, teriak, dll (AST)</span>
            </div>
            <ToggleSwitch v-model="settings.config.ai.use_audio_analysis" />
          </div>

          <div class="flex items-center justify-between">
            <div class="flex flex-col">
              <span class="text-sm font-bold text-[var(--color-text-main)]">Voice Tone</span>
              <span class="text-[10px] text-[var(--color-text-muted)] font-medium">Deteksi intonasi vokal (SER)</span>
            </div>
            <ToggleSwitch v-model="settings.config.ai.use_voice_analysis" />
          </div>

          <div class="flex items-center justify-between">
            <div class="flex flex-col">
              <span class="text-sm font-bold text-[var(--color-text-main)]">Text Sentiment</span>
              <span class="text-[10px] text-[var(--color-text-muted)] font-medium">Klasifikasi NLP transkrip</span>
            </div>
            <ToggleSwitch v-model="settings.config.ai.use_text_analysis" />
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
        <div v-if="settings.config.subtitle.enabled" class="mt-2">
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
    </template>

    <template v-else-if="mode === 'compilation'">
      <BentoCard class="p-6 shrink-0 !bg-indigo-100 dark:!bg-indigo-900/40">
        <h3 class="text-lg font-black text-[var(--color-text-main)] tracking-wide mb-4 flex items-center gap-2">
          <IconFilm class="w-5 h-5" /> Tipe Kompilasi
        </h3>
        <div class="flex flex-col gap-3">
          <div
            v-for="opt in COMPILATION_TYPES"
            :key="opt.value"
            class="p-3 rounded-xl cursor-pointer transition-all border-2"
            :class="settings.config.compilation.compilation_type === opt.value
              ? 'border-indigo-500 bg-white/70 dark:bg-black/50'
              : 'border-transparent bg-white/40 dark:bg-black/30 hover:bg-white/60 dark:hover:bg-black/40'"
            @click="onCompilationTypeChange(opt.value)"
          >
            <span class="text-sm font-bold text-[var(--color-text-main)]">{{ opt.label }}</span>
            <p class="text-[10px] text-[var(--color-text-muted)] mt-0.5 leading-tight">{{ opt.description }}</p>
          </div>
        </div>
      </BentoCard>

      <BentoCard v-if="isReactionMode" class="p-6 shrink-0 !bg-purple-100 dark:!bg-purple-900/40">
        <h3 class="text-lg font-black text-[var(--color-text-main)] tracking-wide mb-4 flex items-center gap-2">
          <IconAudioLines class="w-5 h-5" /> Sinkronisasi Audio
        </h3>
        <div class="flex flex-col gap-4">
          <p class="text-xs text-[var(--color-text-muted)] leading-relaxed">
            Restreamer ditemukan dan diselaraskan otomatis via pencocokan audio terhadap momen epik dari video utama.
            Label nama restreamer ditambahkan otomatis di sudut video.
          </p>
          <div class="flex items-center justify-between">
            <div class="flex flex-col">
              <span class="text-sm font-bold text-[var(--color-text-main)]">Subtitle Kompilasi</span>
              <span class="text-[10px] text-[var(--color-text-muted)] font-medium">Burn subtitle pada output</span>
            </div>
            <ToggleSwitch v-model="settings.config.compilation.use_subtitle" />
          </div>
        </div>
      </BentoCard>

      <BentoCard v-else class="p-6 shrink-0 !bg-purple-100 dark:!bg-purple-900/40">
        <h3 class="text-lg font-black text-[var(--color-text-main)] tracking-wide mb-4 flex items-center gap-2">
          <IconListOrdered class="w-5 h-5" /> Urutan & Penomoran
        </h3>
        <div class="flex flex-col gap-4">
          <div class="flex flex-col gap-1">
            <span class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold">Urutan Kompilasi</span>
            <select
              v-model="settings.config.compilation.ordering"
              class="w-full bg-white/50 dark:bg-black/30 border-none rounded-xl p-2 text-xs font-bold text-[var(--color-text-main)] focus:outline-none focus:ring-2 focus:ring-gray-500 cursor-pointer"
            >
              <option value="countdown">Countdown</option>
              <option value="chronological">Kronologis</option>
              <option value="score">Berdasarkan Skor</option>
            </select>
          </div>
          <div class="flex items-center justify-between">
            <div class="flex flex-col">
              <span class="text-sm font-bold text-[var(--color-text-main)]">TTS Penomoran</span>
              <span class="text-[10px] text-[var(--color-text-muted)] font-medium">Bacakan nomor urut</span>
            </div>
            <ToggleSwitch v-model="settings.config.compilation.use_tts" />
          </div>
          <div class="flex items-center justify-between">
            <div class="flex flex-col">
              <span class="text-sm font-bold text-[var(--color-text-main)]">Subtitle Kompilasi</span>
              <span class="text-[10px] text-[var(--color-text-muted)] font-medium">Burn subtitle pada output</span>
            </div>
            <ToggleSwitch v-model="settings.config.compilation.use_subtitle" />
          </div>
        </div>
      </BentoCard>

      <BentoCard v-if="isReactionMode" class="p-6 shrink-0 !bg-orange-100 dark:!bg-orange-900/40">
        <h3 class="text-lg font-black text-[var(--color-text-main)] tracking-wide mb-4 flex items-center gap-2">
          <IconMonitor class="w-5 h-5" /> Output Reaksi
        </h3>
        <div class="flex flex-col gap-3 text-xs text-[var(--color-text-muted)]">
          <div class="flex justify-between items-center bg-white/40 dark:bg-black/30 rounded-xl px-3 py-2">
            <span class="font-bold">Rasio</span>
            <span class="font-black text-[var(--color-text-main)]">16:9 Horizontal</span>
          </div>
          <div class="flex justify-between items-center bg-white/40 dark:bg-black/30 rounded-xl px-3 py-2">
            <span class="font-bold">Crop</span>
            <span class="font-black text-[var(--color-text-main)]">Tanpa crop (resolusi asli)</span>
          </div>
          <div class="flex justify-between items-center bg-white/40 dark:bg-black/30 rounded-xl px-3 py-2">
            <span class="font-bold">Durasi segmen</span>
            <span class="font-black text-[var(--color-text-main)]">Tidak terbatas</span>
          </div>
        </div>
      </BentoCard>

      <BentoCard v-else class="p-6 shrink-0 !bg-orange-100 dark:!bg-orange-900/40">
        <h3 class="text-lg font-black text-[var(--color-text-main)] tracking-wide mb-4 flex items-center gap-2">
          <IconType class="w-5 h-5" /> Layout Kompilasi
        </h3>
        <div class="flex flex-col gap-4">
          <div class="flex flex-col gap-1">
            <span class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold">Crop Mode</span>
            <select
              v-model="settings.config.compilation.crop_mode"
              class="w-full bg-white/50 dark:bg-black/30 border-none rounded-xl p-2 text-xs font-bold text-[var(--color-text-main)] focus:outline-none focus:ring-2 focus:ring-gray-500 cursor-pointer"
            >
              <option v-for="opt in COMPILATION_CROP_MODES" :key="opt.value" :value="opt.value">
                {{ opt.label }}
              </option>
            </select>
            <span v-if="settings.config.compilation.crop_mode === 'none'" class="text-[9px] text-[var(--color-text-muted)] mt-0.5">
              Resolusi horizontal asli dipertahankan — hanya trim durasi.
            </span>
          </div>
          <div class="flex flex-col gap-1">
            <span class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold">
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
              class="w-full h-2 bg-gray-300 dark:bg-gray-800 rounded-lg appearance-none cursor-pointer mt-1 accent-[var(--color-accent)]"
            />
            <span class="text-[9px] text-[var(--color-text-muted)]">
              Batas maksimum durasi per segmen meme shorts.
            </span>
          </div>
          <div class="flex flex-col gap-1">
            <span class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold">Durasi Penomoran ({{ settings.config.compilation.numbering_duration }}s)</span>
            <input
              type="range"
              min="1"
              max="10"
              step="0.5"
              v-model.number="settings.config.compilation.numbering_duration"
              class="w-full h-2 bg-gray-300 dark:bg-gray-800 rounded-lg appearance-none cursor-pointer mt-1 accent-[var(--color-accent)]"
            />
          </div>
        </div>
      </BentoCard>
    </template>
  </div>
</template>

<script setup lang="ts">

import BentoCard from '../BentoCard.vue';
import ToggleSwitch from '../ToggleSwitch.vue';
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

const props = defineProps<{
  mode?: 'clipper' | 'compilation'
}>();

const settings = useSettingsStore();
const videoStore = useVideoStore();

const isReactionMode = computed(() =>
  isReactionCompilation(settings.config.compilation.compilation_type),
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
