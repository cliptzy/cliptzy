<template>
  <BentoCard
    class="col-span-1 md:col-span-2 xl:col-span-2 row-span-2 h-full overflow-y-auto custom-scrollbar p-6 flex flex-col gap-5 !bg-lime-100 dark:!bg-lime-900/40"
  >
    <h2 class="text-lg font-black text-[var(--color-text-main)] tracking-wide flex items-center gap-2 shrink-0">
      <IconType class="w-5 h-5" /> Subtitle & Transkripsi
    </h2>

    <div class="flex items-center justify-between bg-white/50 dark:bg-black/30 p-4 rounded-2xl">
      <div class="flex flex-col">
        <span class="text-sm font-bold text-[var(--color-text-main)]">Burn Subtitle</span>
        <span class="text-[10px] text-[var(--color-text-muted)]">Tampilkan subtitle saat render klip</span>
      </div>
      <ToggleSwitch v-model="settings.config.burn_subtitle" />
    </div>

    <div :class="{ 'opacity-50 pointer-events-none': !settings.config.burn_subtitle }" class="flex flex-col gap-5">
    <div class="flex flex-col gap-1.5">
      <span class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold">Model Whisper</span>
      <select
        v-model="settings.config.subtitle.whisper_model"
        class="w-full bg-white/60 dark:bg-black/30 border-none rounded-2xl p-3 text-sm font-bold text-[var(--color-text-main)] focus:outline-none focus:ring-2 focus:ring-[var(--color-accent)] cursor-pointer shadow-sm"
      >
        <option v-for="m in WHISPER_MODELS" :key="m.value" :value="m.value">{{ m.label }}</option>
      </select>
    </div>

    <SubtitleStyleControls variant="full" />
    </div>

    <p class="text-[10px] text-[var(--color-text-muted)] font-medium leading-relaxed">
      Font, warna, ukuran, dan posisi subtitle dapat di-preview secara real-time di Studio (Quick Settings).
    </p>
  </BentoCard>
</template>

<script setup lang="ts">
import BentoCard from "../BentoCard.vue";
import ToggleSwitch from "../ToggleSwitch.vue";
import SubtitleStyleControls from "../SubtitleStyleControls.vue";
import { useSettingsStore } from "../../stores/settings";
import { WHISPER_MODELS } from "../../constants/aiModels";
import IconType from "~icons/lucide/type";

const settings = useSettingsStore();
</script>
