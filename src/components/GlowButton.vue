<template>
  <button
    class="relative inline-flex items-center justify-center px-6 py-3 font-semibold text-black bg-[var(--color-accent)] rounded-full group focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-[var(--color-accent)] focus:ring-offset-[var(--color-base)] transition-all duration-300"
    :class="[disabled ? 'opacity-50 cursor-not-allowed' : 'hover:scale-[1.02] active:scale-95']"
    :disabled="disabled"
    @click="$emit('click', $event)"
  >
    <div
      v-if="glow && !disabled"
      class="absolute inset-0 rounded-full blur-md bg-[var(--color-accent)] opacity-40 group-hover:opacity-70 transition-opacity duration-300 -z-10"
    ></div>
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
