<template>
 <div
 class="bg-base-100 "
 >
 <h2 class="text-lg font-black text-base-content tracking-wide flex items-center gap-2 shrink-0">
 <IconFilm class="w-5 h-5" /> Mode Kompilasi
 </h2>

 <div class="flex flex-col gap-2">
 <span class="text-[10px] text-secondary uppercase font-bold">Tipe Kompilasi</span>
 <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
 <button
 v-for="opt in COMPILATION_TYPES"
 :key="opt.value"
 type="button"
 class="p-3 rounded-none text-left transition-all border-2"
 :class="settings.config.compilation.compilation_type === opt.value
 ? 'border-primary bg-base-200  '
 : 'border-transparent bg-base-200/50 hover:bg-base-200/70 dark:hover:bg-base-300/40'"
 @click="onCompilationTypeChange(opt.value)"
 >
 <span class="text-sm font-bold block">{{ opt.label }}</span>
 <span class="text-[10px] text-secondary leading-tight">{{ opt.description }}</span>
 </button>
 </div>
 </div>

 <!-- Mode Reaksi Restreamer -->
 <template v-if="isReactionMode">
 <div class="flex flex-col gap-3 p-4 bg-base-200/50 rounded-none">
 <h3 class="text-sm font-black text-base-content flex items-center gap-2">
 <IconAudioLines class="w-4 h-4" /> Alur Sinkronisasi Audio
 </h3>
 <p class="text-xs text-secondary leading-relaxed">
 Video utama ditranskripsi, momen epik dikurasi AI, lalu restreamer dicocokkan otomatis via cross-correlation audio.
 Output horizontal 16:9 tanpa crop, durasi segmen tidak dibatasi.
 </p>
 <div class="grid grid-cols-1 sm:grid-cols-3 gap-2 text-xs">
 <div class="bg-base-200/60 dark:bg-base-300/40 rounded-none px-3 py-2 text-center">
 <span class="block font-black text-base-content">16:9</span>
 <span class="text-secondary">Horizontal</span>
 </div>
 <div class="bg-base-200/60 dark:bg-base-300/40 rounded-none px-3 py-2 text-center">
 <span class="block font-black text-base-content">Tanpa Crop</span>
 <span class="text-secondary">Resolusi asli</span>
 </div>
 <div class="bg-base-200/60 dark:bg-base-300/40 rounded-none px-3 py-2 text-center">
 <span class="block font-black text-base-content">∞ Durasi</span>
 <span class="text-secondary">Segmen panjang OK</span>
 </div>
 </div>
 </div>

 <label class="flex items-center justify-between bg-base-200/50 p-3 rounded-none cursor-pointer">
 <div class="flex flex-col">
 <span class="text-sm font-bold">Subtitle Kompilasi</span>
 <span class="text-[10px] text-secondary">Burn subtitle pada output kompilasi</span>
 </div>
 <CToggle v-model="settings.config.compilation.use_subtitle" />
 </label>
 </template>

 <!-- Mode Meme Shorts -->
 <template v-else>
 <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
 <div class="flex flex-col gap-1.5">
 <span class="text-[10px] text-secondary uppercase font-bold">Urutan Klip</span>
 <select v-model="settings.config.compilation.ordering" class="w-full bg-base-200 border border-neutral rounded-none p-3 text-sm font-bold focus:outline-none focus:ring-2 focus:ring-primary cursor-pointer ">
 <option value="countdown">Countdown (Terbaru dulu)</option>
 <option value="chronological">Kronologis</option>
 <option value="score">Berdasarkan Skor</option>
 </select>
 </div>
 <div class="flex flex-col gap-1.5">
 <span class="text-[10px] text-secondary uppercase font-bold">Crop Mode Kompilasi</span>
 <select
 v-model="settings.config.compilation.crop_mode"
 class="w-full bg-base-200 border border-neutral rounded-none p-3 text-sm font-bold focus:outline-none focus:ring-2 focus:ring-primary cursor-pointer "
 >
 <option v-for="opt in COMPILATION_CROP_MODES" :key="opt.value" :value="opt.value">
 {{ opt.label }}
 </option>
 </select>
 </div>
 <div class="flex flex-col gap-1.5 md:">
 <span class="text-[10px] text-secondary uppercase font-bold">
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
 class="w-full h-2 bg-neutral rounded-none appearance-none cursor-pointer mt-2 accent-primary"
 />
 </div>
 <div class="flex flex-col gap-1.5 md:">
 <span class="text-[10px] text-secondary uppercase font-bold">Durasi Penomoran ({{ settings.config.compilation.numbering_duration }}s)</span>
 <input type="range" min="1" max="10" step="0.5" v-model.number="settings.config.compilation.numbering_duration" class="w-full h-2 bg-neutral rounded-none appearance-none cursor-pointer mt-2 accent-primary" />
 </div>
 <div class="flex flex-col gap-1.5 md:">
 <span class="text-[10px] text-secondary uppercase font-bold">Template TTS Penomoran</span>
 <input v-model="settings.config.compilation.tts_template" type="text" placeholder="Nomor {n}! {name}!" class="w-full bg-base-200 border border-neutral rounded-none p-3 text-sm font-bold focus:outline-none focus:ring-2 focus:ring-primary " />
 </div>
 </div>

 <div class="flex flex-col gap-3 pt-2 border-t border-neutral dark:border-neutral">
 <label class="flex items-center justify-between bg-base-200/50 p-3 rounded-none cursor-pointer">
 <div class="flex flex-col">
 <span class="text-sm font-bold">TTS Penomoran</span>
 <span class="text-[10px] text-secondary">Bacakan nomor urut via AI voice</span>
 </div>
 <CToggle v-model="settings.config.compilation.use_tts" />
 </label>
 <label class="flex items-center justify-between bg-base-200/50 p-3 rounded-none cursor-pointer">
 <div class="flex flex-col">
 <span class="text-sm font-bold">Subtitle Kompilasi</span>
 <span class="text-[10px] text-secondary">Burn subtitle pada output kompilasi</span>
 </div>
 <CToggle v-model="settings.config.compilation.use_subtitle" />
 </label>
 </div>
 </template>
 </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
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


