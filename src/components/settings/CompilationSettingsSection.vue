<template>
  <BentoCard
    class="col-span-1 md:col-span-2 xl:col-span-2 row-span-2 h-full overflow-y-auto custom-scrollbar p-6 flex flex-col gap-5 !bg-purple-100 dark:!bg-purple-900/40"
  >
    <h2 class="text-lg font-black text-[var(--color-text-main)] tracking-wide flex items-center gap-2 shrink-0">
      <IconFilm class="w-5 h-5" /> Mode Kompilasi
    </h2>

    <div class="flex flex-col gap-2">
      <span class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold">Tipe Kompilasi</span>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
        <button
          v-for="opt in COMPILATION_TYPES"
          :key="opt.value"
          type="button"
          class="p-3 rounded-2xl text-left transition-all border-2"
          :class="settings.config.compilation.compilation_type === opt.value
            ? 'border-purple-500 bg-white/80 dark:bg-black/50 shadow-sm'
            : 'border-transparent bg-white/50 dark:bg-black/30 hover:bg-white/70 dark:hover:bg-black/40'"
          @click="onCompilationTypeChange(opt.value)"
        >
          <span class="text-sm font-bold block">{{ opt.label }}</span>
          <span class="text-[10px] text-[var(--color-text-muted)] leading-tight">{{ opt.description }}</span>
        </button>
      </div>
    </div>

    <!-- Mode Reaksi Restreamer -->
    <template v-if="isReactionMode">
      <div class="flex flex-col gap-3 p-4 bg-white/50 dark:bg-black/30 rounded-2xl">
        <h3 class="text-sm font-black text-[var(--color-text-main)] flex items-center gap-2">
          <IconAudioLines class="w-4 h-4" /> Alur Sinkronisasi Audio
        </h3>
        <p class="text-xs text-[var(--color-text-muted)] leading-relaxed">
          Video utama ditranskripsi, momen epik dikurasi AI, lalu restreamer dicocokkan otomatis via cross-correlation audio.
          Output horizontal 16:9 tanpa crop, durasi segmen tidak dibatasi.
        </p>
        <div class="grid grid-cols-1 sm:grid-cols-3 gap-2 text-xs">
          <div class="bg-white/60 dark:bg-black/40 rounded-xl px-3 py-2 text-center">
            <span class="block font-black text-[var(--color-text-main)]">16:9</span>
            <span class="text-[var(--color-text-muted)]">Horizontal</span>
          </div>
          <div class="bg-white/60 dark:bg-black/40 rounded-xl px-3 py-2 text-center">
            <span class="block font-black text-[var(--color-text-main)]">Tanpa Crop</span>
            <span class="text-[var(--color-text-muted)]">Resolusi asli</span>
          </div>
          <div class="bg-white/60 dark:bg-black/40 rounded-xl px-3 py-2 text-center">
            <span class="block font-black text-[var(--color-text-main)]">∞ Durasi</span>
            <span class="text-[var(--color-text-muted)]">Segmen panjang OK</span>
          </div>
        </div>
      </div>

      <label class="flex items-center justify-between bg-white/50 dark:bg-black/30 p-3 rounded-2xl cursor-pointer">
        <div class="flex flex-col">
          <span class="text-sm font-bold">Subtitle Kompilasi</span>
          <span class="text-[10px] text-[var(--color-text-muted)]">Burn subtitle pada output kompilasi</span>
        </div>
        <ToggleSwitch v-model="settings.config.compilation.use_subtitle" />
      </label>
    </template>

    <!-- Mode Meme Shorts -->
    <template v-else>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div class="flex flex-col gap-1.5">
          <span class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold">Urutan Klip</span>
          <select v-model="settings.config.compilation.ordering" class="w-full bg-white/60 dark:bg-black/30 border-none rounded-2xl p-3 text-sm font-bold focus:outline-none focus:ring-2 focus:ring-[var(--color-accent)] cursor-pointer shadow-sm">
            <option value="countdown">Countdown (Terbaru dulu)</option>
            <option value="chronological">Kronologis</option>
            <option value="score">Berdasarkan Skor</option>
          </select>
        </div>
        <div class="flex flex-col gap-1.5">
          <span class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold">Crop Mode Kompilasi</span>
          <select
            v-model="settings.config.compilation.crop_mode"
            class="w-full bg-white/60 dark:bg-black/30 border-none rounded-2xl p-3 text-sm font-bold focus:outline-none focus:ring-2 focus:ring-[var(--color-accent)] cursor-pointer shadow-sm"
          >
            <option v-for="opt in COMPILATION_CROP_MODES" :key="opt.value" :value="opt.value">
              {{ opt.label }}
            </option>
          </select>
        </div>
        <div class="flex flex-col gap-1.5 md:col-span-2">
          <span class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold">
            Batas Durasi Segmen
            <template v-if="settings.config.compilation.max_segment_duration === 0"> (Tidak terbatas)</template>
            <template v-else> ({{ settings.config.compilation.max_segment_duration }} detik)</template>
          </span>
          <input
            type="range"
            min="0"
            max="600"
            step="15"
            v-model.number="settings.config.compilation.max_segment_duration"
            class="w-full h-2 bg-gray-300 dark:bg-gray-800 rounded-lg appearance-none cursor-pointer mt-2 accent-[var(--color-accent)]"
          />
        </div>
        <div class="flex flex-col gap-1.5 md:col-span-2">
          <span class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold">Durasi Penomoran ({{ settings.config.compilation.numbering_duration }}s)</span>
          <input type="range" min="1" max="10" step="0.5" v-model.number="settings.config.compilation.numbering_duration" class="w-full h-2 bg-gray-300 dark:bg-gray-800 rounded-lg appearance-none cursor-pointer mt-2 accent-[var(--color-accent)]" />
        </div>
        <div class="flex flex-col gap-1.5 md:col-span-2">
          <span class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold">Template TTS Penomoran</span>
          <input v-model="settings.config.compilation.tts_template" type="text" placeholder="Nomor {n}! {name}!" class="w-full bg-white/60 dark:bg-black/30 border-none rounded-2xl p-3 text-sm font-bold focus:outline-none focus:ring-2 focus:ring-[var(--color-accent)] shadow-sm" />
        </div>
      </div>

      <div class="flex flex-col gap-3 pt-2 border-t border-purple-200 dark:border-purple-800/50">
        <label class="flex items-center justify-between bg-white/50 dark:bg-black/30 p-3 rounded-2xl cursor-pointer">
          <div class="flex flex-col">
            <span class="text-sm font-bold">TTS Penomoran</span>
            <span class="text-[10px] text-[var(--color-text-muted)]">Bacakan nomor urut via AI voice</span>
          </div>
          <ToggleSwitch v-model="settings.config.compilation.use_tts" />
        </label>
        <label class="flex items-center justify-between bg-white/50 dark:bg-black/30 p-3 rounded-2xl cursor-pointer">
          <div class="flex flex-col">
            <span class="text-sm font-bold">Subtitle Kompilasi</span>
            <span class="text-[10px] text-[var(--color-text-muted)]">Burn subtitle pada output kompilasi</span>
          </div>
          <ToggleSwitch v-model="settings.config.compilation.use_subtitle" />
        </label>
      </div>
    </template>
  </BentoCard>
</template>

<script setup lang="ts">
import { computed } from "vue";
import BentoCard from "../BentoCard.vue";
import ToggleSwitch from "../ToggleSwitch.vue";
import { useSettingsStore } from "../../stores/settings";
import IconFilm from "~icons/lucide/film";
import IconAudioLines from "~icons/lucide/audio-lines";
import {
  COMPILATION_TYPES,
  COMPILATION_CROP_MODES,
  applyCompilationTypeDefaults,
  isReactionCompilation,
  type CompilationType,
} from "../../constants/compilation";

const settings = useSettingsStore();

const isReactionMode = computed(() =>
  isReactionCompilation(settings.config.compilation.compilation_type),
);

const onCompilationTypeChange = (type: CompilationType) => {
  applyCompilationTypeDefaults(type, settings.config.compilation);
  if (type === "reaction") {
    settings.setRatioPreset("16:9");
  }
};
</script>
