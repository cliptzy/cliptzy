<template>
  <div class="relative w-full group">
    <div
      class="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none text-gray-400 group-focus-within:text-[var(--color-accent)] transition-colors"
      v-if="$slots.icon"
    >
      <slot name="icon"></slot>
    </div>
    <input
      :value="modelValue"
      @input="$emit('update:modelValue', ($event.target as HTMLInputElement).value)"
      :type="type"
      :placeholder="placeholder"
      class="w-full bg-[#131316] border border-[var(--color-subtle)] text-white text-sm rounded-xl focus:ring-1 focus:ring-[var(--color-accent)] focus:border-[var(--color-accent)] focus:outline-none block p-3 transition-colors placeholder-gray-600 shadow-[inset_0_2px_4px_rgba(0,0,0,0.3)]"
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
