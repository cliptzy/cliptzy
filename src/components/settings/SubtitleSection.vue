<template>
  <BentoCard
    class="col-span-1 md:col-span-2 xl:col-span-2 row-span-2 h-full overflow-y-auto custom-scrollbar p-6 flex flex-col gap-5 !bg-lime-100 dark:!bg-lime-900/40"
  >
    <h2 class="text-lg font-black text-[var(--color-text-main)] tracking-wide flex items-center gap-2 shrink-0">
      <IconType class="w-5 h-5" /> Subtitle & Transkripsi
    </h2>

    <div class="flex items-center justify-between bg-white/50 dark:bg-black/30 p-4 rounded-2xl">
      <div class="flex flex-col">
        <span class="text-sm font-bold text-[var(--color-text-main)]">Subtitle Aktif</span>
        <span class="text-[10px] text-[var(--color-text-muted)]">Burn subtitle saat render klip</span>
      </div>
      <ToggleSwitch v-model="settings.config.subtitle.enabled" />
    </div>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <div class="flex flex-col gap-1.5">
        <span class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold">Model Whisper</span>
        <select
          v-model="settings.config.subtitle.whisper_model"
          class="w-full bg-white/60 dark:bg-black/30 border-none rounded-2xl p-3 text-sm font-bold text-[var(--color-text-main)] focus:outline-none focus:ring-2 focus:ring-[var(--color-accent)] cursor-pointer shadow-sm"
        >
          <option v-for="m in WHISPER_MODELS" :key="m.value" :value="m.value">{{ m.label }}</option>
        </select>
      </div>
      <div class="flex flex-col gap-1.5">
        <span class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold">Gaya Subtitle</span>
        <select v-model="settings.config.subtitle.style" class="w-full bg-white/60 dark:bg-black/30 border-none rounded-2xl p-3 text-sm font-bold focus:outline-none focus:ring-2 focus:ring-[var(--color-accent)] cursor-pointer shadow-sm">
          <option value="plain">Plain</option>
          <option value="boxed">Boxed</option>
          <option value="karaoke">Karaoke</option>
        </select>
      </div>
      <div class="flex flex-col gap-1.5">
        <span class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold">Animasi</span>
        <select v-model="settings.config.subtitle.animation" class="w-full bg-white/60 dark:bg-black/30 border-none rounded-2xl p-3 text-sm font-bold focus:outline-none focus:ring-2 focus:ring-[var(--color-accent)] cursor-pointer shadow-sm">
          <option value="none">Tanpa Animasi</option>
          <option value="hormozi">Hormozi</option>
          <option value="karaoke">Karaoke</option>
        </select>
      </div>
      <div class="flex flex-col gap-1.5">
        <span class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold">Border Style ({{ settings.config.subtitle.border_style }})</span>
        <input type="range" min="1" max="4" v-model.number="settings.config.subtitle.border_style" class="w-full h-2 bg-gray-300 dark:bg-gray-800 rounded-lg appearance-none cursor-pointer mt-2 accent-[var(--color-accent)]" />
      </div>
      <div class="flex flex-col gap-1.5">
        <span class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold">Delay ({{ settings.config.subtitle.delay }}s)</span>
        <input type="range" min="0" max="5" step="0.1" v-model.number="settings.config.subtitle.delay" class="w-full h-2 bg-gray-300 dark:bg-gray-800 rounded-lg appearance-none cursor-pointer mt-2 accent-[var(--color-accent)]" />
      </div>
      <div class="flex flex-col gap-1.5">
        <span class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold">Maks Kata / Baris</span>
        <input type="range" min="1" max="10" v-model.number="settings.config.subtitle.max_words" class="w-full h-2 bg-gray-300 dark:bg-gray-800 rounded-lg appearance-none cursor-pointer mt-2 accent-[var(--color-accent)]" />
      </div>
      <div class="flex flex-col gap-1.5">
        <span class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold">Warna Teks (ASS)</span>
        <select v-model="settings.config.subtitle.color" class="w-full bg-white/60 dark:bg-black/30 border-none rounded-2xl p-3 text-sm font-bold focus:outline-none focus:ring-2 focus:ring-[var(--color-accent)] cursor-pointer shadow-sm">
          <option value="&H0000FFFF">Kuning</option>
          <option value="&H00FFFFFF">Putih</option>
          <option value="&H0000FF00">Hijau</option>
        </select>
      </div>
      <div class="flex flex-col gap-1.5">
        <span class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold">Warna Background (ASS)</span>
        <input v-model="settings.config.subtitle.bg_color" type="text" placeholder="&H80000000" class="w-full bg-white/60 dark:bg-black/30 border-none rounded-2xl p-3 text-sm font-bold focus:outline-none focus:ring-2 focus:ring-[var(--color-accent)] shadow-sm" />
      </div>
      <div class="flex flex-col gap-1.5">
        <span class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold">Ukuran Font ({{ settings.config.subtitle.font_size }})</span>
        <input type="range" min="20" max="150" v-model.number="settings.config.subtitle.font_size" class="w-full h-2 bg-gray-300 dark:bg-gray-800 rounded-lg appearance-none cursor-pointer mt-2 accent-[var(--color-accent)]" />
      </div>
      <div class="flex flex-col gap-1.5">
        <span class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold">Font Default</span>
        <input v-model="settings.config.subtitle.font" type="text" class="w-full bg-white/60 dark:bg-black/30 border-none rounded-2xl p-3 text-sm font-bold focus:outline-none focus:ring-2 focus:ring-[var(--color-accent)] shadow-sm" />
      </div>
      <div class="flex flex-col gap-1.5">
        <span class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold">Posisi Default</span>
        <select v-model="settings.config.subtitle.location" class="w-full bg-white/60 dark:bg-black/30 border-none rounded-2xl p-3 text-sm font-bold focus:outline-none focus:ring-2 focus:ring-[var(--color-accent)] cursor-pointer shadow-sm">
          <option value="bottom">Bawah</option>
          <option value="center">Tengah</option>
          <option value="top">Atas</option>
        </select>
      </div>
      <div class="flex flex-col gap-1.5">
        <span class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold">Direktori Font (opsional)</span>
        <input v-model="settings.config.subtitle.fonts_dir" type="text" placeholder="Kosongkan untuk default" class="w-full bg-white/60 dark:bg-black/30 border-none rounded-2xl p-3 text-sm font-bold focus:outline-none focus:ring-2 focus:ring-[var(--color-accent)] shadow-sm" />
      </div>
    </div>

    <p class="text-[10px] text-[var(--color-text-muted)] font-medium leading-relaxed">
      Font, warna, ukuran, dan posisi subtitle dapat di-preview secara real-time di Studio (Quick Settings).
    </p>
  </BentoCard>
</template>

<script setup lang="ts">
import BentoCard from "../BentoCard.vue";
import ToggleSwitch from "../ToggleSwitch.vue";
import { useSettingsStore } from "../../stores/settings";
import { WHISPER_MODELS } from "../../constants/aiModels";
import IconType from "~icons/lucide/type";

const settings = useSettingsStore();
</script>
