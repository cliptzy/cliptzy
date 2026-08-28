<template>
  <div class="relative w-full group">
    <div
      class="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none text-[var(--color-text-muted)] group-focus-within:text-gray-900 transition-colors"
      v-if="$slots.icon"
    >
      <slot name="icon"></slot>
    </div>
    <input
      :value="modelValue"
      @input="$emit('update:modelValue', ($event.target as HTMLInputElement).value)"
      :type="type"
      :placeholder="placeholder"
      class="w-full bg-white/60 dark:bg-black/30 border border-[var(--color-subtle)] text-[var(--color-text-main)] text-sm rounded-xl focus:ring-1 focus:ring-[var(--color-accent)] focus:border-[var(--color-accent)] focus:outline-none block p-3 transition-all shadow-sm placeholder-[var(--color-text-muted)]"
      :class="[$slots.icon ? 'pl-11' : '']"
    />
  </div>
</template>

<script setup lang="ts">
defineProps({
  modelValue: {
    type: [String, Number],
    default: ''
  },
  type: {
    type: String,
    default: 'text'
  },
  placeholder: {
    type: String,
    default: ''
  }
});

defineEmits(['update:modelValue']);
</script>
