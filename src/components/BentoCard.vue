<template>
  <div
    class="rounded-[32px] transition-colors duration-300 ease-out overflow-hidden"
    :class="disableRandomBg ? '' : randomBg"
  >
    <slot></slot>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';

defineProps({
  disableRandomBg: {
    type: Boolean,
    default: false
  }
});

const pastelColors = [
  'bg-gray-100 dark:bg-gray-900',
  'bg-slate-100 dark:bg-slate-900',
  'bg-zinc-100 dark:bg-zinc-900',
  'bg-neutral-100 dark:bg-neutral-900',
  'bg-stone-100 dark:bg-stone-900'
];

const randomBg = ref(pastelColors[Math.floor(Math.random() * pastelColors.length)]);
</script>
