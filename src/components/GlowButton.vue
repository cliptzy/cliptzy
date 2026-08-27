<template>
  <button
    class="relative inline-flex items-center justify-center px-6 py-3 font-semibold text-white bg-[var(--color-accent)] rounded-full group focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-[var(--color-accent)] focus:ring-offset-[var(--color-base)] transition-all duration-300"
    :class="[disabled ? 'opacity-50 cursor-not-allowed' : 'active:scale-95 hover:bg-rose-500']"
    :disabled="disabled"
    @click="$emit('click', $event)"
  >
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
