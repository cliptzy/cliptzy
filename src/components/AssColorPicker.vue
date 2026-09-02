<template>
  <div class="flex flex-col gap-1.5 min-w-0 w-full">
    <span v-if="label" class="text-[10px] text-secondary uppercase font-bold">{{ label }}</span>
    <div class="flex items-center gap-2 min-w-0">
      <label
        class="relative shrink-0 w-10 h-10 rounded-none overflow-hidden cursor-pointer border transition-colors duration-150"
        :class="['border-neutral', 'hover:border-primary']"
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
        class="flex-1 min-w-0 bg-base-200 border border-neutral rounded-none px-3 py-2 text-xs font-mono font-bold text-base-content focus:outline-none focus:ring-2 focus:ring-primary shadow-sm uppercase transition-shadow duration-150"
      />
    </div>
    <div v-if="showOpacity" class="flex flex-col gap-1 mt-0.5 min-w-0 w-full">
      <div class="flex items-center justify-between gap-2">
        <span class="text-[9px] text-secondary uppercase font-bold">Opasitas</span>
        <span class="text-[9px] font-bold text-secondary font-mono">{{ opacity }}%</span>
      </div>
      <input
        type="range"
        min="0"
        max="100"
        :value="opacity"
        class="w-full min-w-0 h-1.5 bg-neutral rounded-none appearance-none cursor-pointer accent-primary"
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


