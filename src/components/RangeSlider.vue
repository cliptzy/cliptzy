<template>
  <div class="w-full relative flex items-center group">
    <input
      type="range"
      :min="min"
      :max="max"
      :step="step"
      :value="modelValue"
      @input="$emit('update:modelValue', Number(($event.target as HTMLInputElement).value))"
      class="w-full h-2 bg-[#3f3f46] rounded-lg appearance-none cursor-pointer outline-none focus:ring-1 focus:ring-[var(--color-accent)] shadow-[inset_0_1px_2px_rgba(0,0,0,0.5)] transition-all"
      :style="`background: linear-gradient(to right, var(--color-accent) ${progress}%, #3f3f46 ${progress}%)`"
    />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps({
  modelValue: {
    type: Number,
    default: 50
  },
  min: {
    type: Number,
    default: 0
  },
  max: {
    type: Number,
    default: 100
  },
  step: {
    type: Number,
    default: 1
  }
});

defineEmits(['update:modelValue']);

const progress = computed(() => {
  return ((props.modelValue - props.min) / (props.max - props.min)) * 100;
});
</script>

<style scoped>
input[type="range"]::-webkit-slider-thumb {
  appearance: none;
  width: 16px;
  height: 16px;
  border-radius: 50%;
  background: white;
  cursor: pointer;
  box-shadow: 0 0 5px rgba(0,0,0,0.5);
  border: 3px solid var(--color-accent);
  transition: transform 0.1s;
}

input[type="range"]:active::-webkit-slider-thumb {
  transform: scale(1.2);
}
</style>
