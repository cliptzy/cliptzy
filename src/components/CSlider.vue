<template>
  <div class="w-full relative flex items-center group">
    <input
      type="range"
      :min="min"
      :max="max"
      :step="step"
      :value="modelValue"
      @input="$emit('update:modelValue', Number(($event.target as HTMLInputElement).value))"
      class="w-full h-2 bg-neutral rounded-none appearance-none cursor-pointer outline-none focus:ring-1 focus:ring-primary transition-all"
      :style="`background: linear-gradient(to right, var(--color-primary) ${progress}%, var(--color-neutral) ${progress}%)`"
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
  border-radius: 0px;
    background: var(--color-primary);
  border: 3px solid var(--color-primary);
  box-shadow: none;
  transition: transform 0.1s;
}

input[type="range"]:active::-webkit-slider-thumb {
  transform: scale(0.97);
}
</style>


