<template>
  <button class="relative inline-flex items-center justify-center px-6 py-3 font-semibold rounded-full group focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-900 dark:focus:ring-white transition-all duration-300 bg-white/60 text-gray-900 dark:bg-black/30 dark:text-gray-100 hover:bg-white dark:hover:bg-black/50" :class="[disabled ? 'opacity-50 cursor-not-allowed' : 'active:scale-95']" :disabled="disabled" @click="$emit('click', $event)" >
    <slot></slot>
  </button>
</template>

<script setup lang="ts">
defineProps({
  disabled: {
    type: Boolean,
    default: false
  },
  glow: {
    type: Boolean,
    default: true
  }
});

defineEmits(['click']);
</script>
