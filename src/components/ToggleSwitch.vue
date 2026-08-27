<template>
  <button type="button" class="relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-[var(--color-accent)] focus:ring-offset-2 focus:ring-offset-[var(--color-base)] shadow-[inset_0_1px_2px_rgba(0,0,0,0.3)]" :class="[modelValue ? 'bg-gray-200 dark:bg-gray-800' : 'bg-[#3f3f46]']" role="switch" :aria-checked="modelValue" @click="$emit('update:modelValue', !modelValue)" >
    <span
      aria-hidden="true"
      class="pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out"
      :class="[modelValue ? 'translate-x-5' : 'translate-x-0']"
    ></span>
  </button>
</template>

<script setup lang="ts">
defineProps({
  modelValue: {
    type: Boolean,
    default: false
  }
});

defineEmits(['update:modelValue']);
</script>
