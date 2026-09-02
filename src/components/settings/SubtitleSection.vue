<template>
 <div
 class="bg-base-100 "
 >
 <h2 class="text-lg font-black text-base-content tracking-wide flex items-center gap-2 shrink-0">
 <IconType class="w-5 h-5" /> Subtitle & Transkripsi
 </h2>

 <div class="flex items-center justify-between bg-base-200/50 p-4 rounded-none">
 <div class="flex flex-col">
 <span class="text-sm font-bold text-base-content">Burn Subtitle</span>
 <span class="text-[10px] text-secondary">Tampilkan subtitle saat render klip</span>
 </div>
 <CToggle v-model="settings.config.burn_subtitle" />
 </div>

 <div :class="{ 'opacity-50 pointer-events-none': !settings.config.burn_subtitle }" class="flex flex-col gap-5">
 <div class="flex flex-col gap-1.5">
 <span class="text-[10px] text-secondary uppercase font-bold">Model Whisper</span>
 <select
 v-model="settings.config.subtitle.whisper_model"
 class="w-full bg-base-200 border border-neutral rounded-none p-3 text-sm font-bold text-base-content focus:outline-none focus:ring-2 focus:ring-primary cursor-pointer "
 >
 <option v-for="m in WHISPER_MODELS" :key="m.value" :value="m.value">{{ m.label }}</option>
 </select>
 </div>

 <SubtitleStyleControls variant="full" />
 </div>

 <p class="text-[10px] text-secondary font-medium leading-relaxed">
 Font, warna, ukuran, dan posisi subtitle dapat di-preview secara real-time di Studio (Quick Settings).
 </p>
 </div>
</template>

<script setup lang="ts">
import SubtitleStyleControls from "../SubtitleStyleControls.vue";
import { useSettingsStore } from "../../stores/settings";
import { WHISPER_MODELS } from "../../constants/aiModels";
import IconType from "~icons/lucide/type";

const settings = useSettingsStore();
</script>


