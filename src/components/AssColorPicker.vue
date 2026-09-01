<template>
  <div class="flex flex-col gap-1.5 min-w-0 w-full">
    <span v-if="label" class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold">{{ label }}</span>
    <div class="flex items-center gap-2 min-w-0">
      <label
        class="relative shrink-0 w-10 h-10 rounded-xl overflow-hidden cursor-pointer ring-1 ring-black/10 dark:ring-white/10 shadow-sm"
        :title="label || 'Pilih warna'"
      >
        <input
          type="color"
          :value="hexValue"
          class="absolute inset-0 w-[150%] h-[150%] -translate-x-1/4 -translate-y-1/4 cursor-pointer border-none p-0"
          @input="onColorInput"
        />
        <span
          class="absolute inset-0 pointer-events-none"
          :style="{ backgroundColor: previewColor }"
        />
      </label>
      <input
        type="text"
        :value="hexValue"
        readonly
        class="flex-1 min-w-0 bg-white/60 dark:bg-black/30 border-none rounded-xl px-3 py-2 text-xs font-mono font-bold text-[var(--color-text-main)] focus:outline-none focus:ring-2 focus:ring-[var(--color-accent)] shadow-sm uppercase"
      />
    </div>
    <div v-if="showOpacity" class="flex flex-col gap-1 mt-0.5 min-w-0 w-full">
      <div class="flex items-center justify-between gap-2">
        <span class="text-[9px] text-[var(--color-text-muted)] uppercase font-bold">Opasitas</span>
        <span class="text-[9px] font-bold text-[var(--color-text-muted)]">{{ opacity }}%</span>
      </div>
      <input
        type="range"
        min="0"
        max="100"
        :value="opacity"
        class="w-full min-w-0 h-1.5 bg-gray-300 dark:bg-gray-800 rounded-lg appearance-none cursor-pointer accent-[var(--color-accent)]"
        @input="onOpacityInput"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { assToHex, assToOpacity, hexToAss } from "../constants/subtitle";

const props = withDefaults(
  defineProps<{
    modelValue: string;
    label?: string;
    showOpacity?: boolean;
  }>(),
  {
    label: "",
    showOpacity: false,
  },
);

const emit = defineEmits<{
  "update:modelValue": [value: string];
}>();

const hexValue = computed(() => assToHex(props.modelValue));
const opacity = computed(() => assToOpacity(props.modelValue));

const previewColor = computed(() => {
  if (!props.showOpacity || opacity.value >= 100) return hexValue.value;
  const pct = opacity.value / 100;
  return `color-mix(in srgb, ${hexValue.value} ${pct * 100}%, transparent)`;
});

const emitAss = (hex: string, op: number) => {
  emit("update:modelValue", hexToAss(hex, props.showOpacity ? op : 100));
};

const onColorInput = (e: Event) => {
  const hex = (e.target as HTMLInputElement).value;
  emitAss(hex, opacity.value);
};

const onOpacityInput = (e: Event) => {
  const op = Number((e.target as HTMLInputElement).value);
  emitAss(hexValue.value, op);
};
</script>
