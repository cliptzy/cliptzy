<template>
  <button
    :type="type"
    :disabled="disabled || loading"
    class="inline-flex items-center justify-center gap-2 font-bold transition-all duration-300 focus:outline-none disabled:opacity-50 disabled:cursor-not-allowed"
    :class="[
      variantClasses[variant] || variantClasses.primary,
      sizeClasses[size] || sizeClasses.md,
      block ? 'w-full flex' : '',
      roundedClasses[rounded] || roundedClasses.full,
      customClass
    ]"
    @click="$emit('click', $event)"
  >
    <!-- Loading spinner -->
    <svg v-if="loading" class="animate-spin -ml-1 mr-2 h-4 w-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
      <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
      <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
    </svg>
    <slot></slot>
  </button>
</template>

<script setup lang="ts">
const props = defineProps({
  type: { type: String as () => 'button' | 'submit' | 'reset', default: 'button' },
  variant: { type: String, default: 'primary' },
  size: { type: String, default: 'md' },
  rounded: { type: String, default: 'full' },
  disabled: { type: Boolean, default: false },
  loading: { type: Boolean, default: false },
  block: { type: Boolean, default: false },
  customClass: { type: String, default: '' },
});

defineEmits(['click']);

const variantClasses: Record<string, string> = {
  primary: 'bg-blue-600 text-white hover:bg-blue-700 shadow-sm hover:scale-[1.02] active:scale-95',
  accent: 'bg-[var(--color-accent)] text-white hover:brightness-110 shadow-[0_4px_15px_rgba(232,115,137,0.3)] hover:scale-[1.02] active:scale-95',
  secondary: 'bg-white/60 dark:bg-black/30 text-[var(--color-text-main)] hover:bg-white dark:hover:bg-black/50 shadow-sm hover:scale-[1.02] active:scale-95',
  danger: 'bg-red-500 text-white hover:bg-red-600 shadow-sm hover:scale-[1.02] active:scale-95',
  'danger-soft': 'bg-red-100 dark:bg-red-900/30 text-red-600 dark:text-red-400 hover:bg-red-500 hover:text-white dark:hover:bg-red-500 shadow-sm hover:scale-[1.02] active:scale-95',
  ghost: 'bg-transparent text-[var(--color-text-muted)] hover:text-[var(--color-text-main)] hover:bg-black/5 dark:hover:bg-white/10',
  link: 'bg-transparent text-blue-500 hover:underline p-0',
};

const sizeClasses: Record<string, string> = {
  xs: 'py-1.5 px-3 text-[10px]',
  sm: 'py-2 px-4 text-xs',
  md: 'py-3 px-6 text-sm',
  lg: 'py-4 px-8 text-base',
  icon: 'p-3',
  'icon-sm': 'p-2',
  'icon-xs': 'p-1.5 text-[10px]',
};

const roundedClasses: Record<string, string> = {
  none: 'rounded-none',
  sm: 'rounded-sm',
  md: 'rounded-md',
  lg: 'rounded-lg',
  xl: 'rounded-xl',
  '2xl': 'rounded-2xl',
  full: 'rounded-full',
};
</script>
