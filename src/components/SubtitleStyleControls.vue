<template>
  <div class="flex flex-col gap-4">
    <!-- Preset Gaya -->
    <div class="flex flex-col gap-2">
      <span :class="labelClass">Gaya Subtitle</span>
      <div class="grid grid-cols-2 gap-2">
        <button
          v-for="preset in SUBTITLE_PRESETS"
          :key="preset.id"
          type="button"
          @click="selectPreset(preset.id)"
          class="p-3 rounded-2xl transition-all text-left flex flex-col gap-0.5 bg-white/50 dark:bg-black/30 hover:bg-white dark:hover:bg-black/50"
          :class="activePreset === preset.id ? 'ring-2 ring-[var(--color-accent)] shadow-sm' : ''"
        >
          <span
            class="font-black text-xs text-[var(--color-text-main)]"
            :class="preset.id === 'brutalist' ? 'font-mono uppercase tracking-wider' : ''"
          >
            {{ preset.label }}
          </span>
          <span class="text-[9px] text-[var(--color-text-muted)] leading-tight font-medium">
            {{ preset.description }}
          </span>
        </button>
      </div>
      <p
        v-if="activePreset === 'brutalist'"
        class="text-[9px] text-[var(--color-text-muted)] font-medium leading-tight"
      >
        Preset Brutalist meng-override font & warna saat render (sesuai engine ASS).
      </p>
    </div>

    <!-- Font, Warna, Posisi -->
    <div :class="gridClass">
      <div class="flex flex-col gap-1">
        <span :class="labelClass">Font</span>
        <select
          v-model="settings.config.subtitle.font"
          :class="selectClass"
        >
          <option v-for="f in SUBTITLE_FONTS" :key="f.value" :value="f.value">
            {{ f.label }}
          </option>
        </select>
      </div>

      <div class="flex flex-col gap-1">
        <span :class="labelClass">Posisi</span>
        <select
          v-model="settings.config.subtitle.location"
          :class="selectClass"
        >
          <option v-for="loc in SUBTITLE_LOCATIONS" :key="loc.value" :value="loc.value">
            {{ loc.label }}
          </option>
        </select>
      </div>

      <AssColorPicker
        v-model="settings.config.subtitle.color"
        label="Warna Teks"
        :class="variant === 'compact' ? 'col-span-2' : ''"
      />

      <AssColorPicker
        v-model="settings.config.subtitle.bg_color"
        label="Warna Background"
        show-opacity
        :class="variant === 'compact' ? 'col-span-2' : ''"
      />

      <div class="flex flex-col gap-1" :class="variant === 'compact' ? '' : ''">
        <span :class="labelClass">
          Maks Kata
          <template v-if="variant === 'compact'"> ({{ settings.config.subtitle.max_words }})</template>
        </span>
        <input
          type="range"
          min="1"
          max="10"
          v-model.number="settings.config.subtitle.max_words"
          class="w-full h-2 bg-gray-300 dark:bg-gray-800 rounded-lg appearance-none cursor-pointer accent-[var(--color-accent)]"
          :class="variant === 'compact' ? 'mt-1.5' : 'mt-2'"
        />
      </div>

      <div v-if="variant === 'full'" class="flex flex-col gap-1">
        <span :class="labelClass">Delay ({{ settings.config.subtitle.delay }}s)</span>
        <input
          type="range"
          min="0"
          max="5"
          step="0.1"
          v-model.number="settings.config.subtitle.delay"
          class="w-full h-2 bg-gray-300 dark:bg-gray-800 rounded-lg appearance-none cursor-pointer mt-2 accent-[var(--color-accent)]"
        />
      </div>
    </div>

    <div class="flex flex-col gap-1">
      <span :class="labelClass">
        Ukuran Font ({{ settings.config.subtitle.font_size }})
      </span>
      <input
        type="range"
        min="20"
        max="150"
        v-model.number="settings.config.subtitle.font_size"
        class="w-full h-2 bg-gray-300 dark:bg-gray-800 rounded-lg appearance-none cursor-pointer accent-[var(--color-accent)]"
        :class="variant === 'compact' ? 'mt-1' : 'mt-2'"
      />
    </div>

    <div v-if="variant === 'full'" class="flex flex-col gap-1.5">
      <span :class="labelClass">Direktori Font (opsional)</span>
      <input
        v-model="settings.config.subtitle.fonts_dir"
        type="text"
        placeholder="Kosongkan untuk default"
        class="w-full bg-white/60 dark:bg-black/30 border-none rounded-2xl p-3 text-sm font-bold focus:outline-none focus:ring-2 focus:ring-[var(--color-accent)] shadow-sm text-[var(--color-text-main)]"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useSettingsStore } from "../stores/settings";
import AssColorPicker from "./AssColorPicker.vue";
import {
  SUBTITLE_PRESETS,
  SUBTITLE_FONTS,
  SUBTITLE_LOCATIONS,
  detectSubtitlePreset,
  applySubtitlePreset,
  type SubtitlePresetId,
} from "../constants/subtitle";

const props = withDefaults(
  defineProps<{
    variant?: "compact" | "full";
  }>(),
  {
    variant: "full",
  },
);

const settings = useSettingsStore();

const activePreset = computed(() =>
  detectSubtitlePreset(
    settings.config.subtitle.animation,
    settings.config.subtitle.border_style,
  ),
);

const selectPreset = (preset: SubtitlePresetId) => {
  applySubtitlePreset(preset, settings.config.subtitle);
};

const labelClass = computed(() =>
  props.variant === "compact"
    ? "text-[9px] text-[var(--color-text-muted)] uppercase font-bold"
    : "text-[10px] text-[var(--color-text-muted)] uppercase font-bold",
);

const selectClass = computed(() =>
  props.variant === "compact"
    ? "w-full bg-white/50 dark:bg-black/30 border-none rounded-xl p-2 text-[10px] font-bold text-[var(--color-text-main)] focus:outline-none focus:ring-2 focus:ring-gray-500 cursor-pointer"
    : "w-full bg-white/60 dark:bg-black/30 border-none rounded-2xl p-3 text-sm font-bold text-[var(--color-text-main)] focus:outline-none focus:ring-2 focus:ring-[var(--color-accent)] cursor-pointer shadow-sm",
);

const gridClass = computed(() =>
  props.variant === "compact"
    ? "grid grid-cols-2 gap-3"
    : "grid grid-cols-1 md:grid-cols-2 gap-4",
);
</script>
