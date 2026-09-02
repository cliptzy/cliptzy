<template>
    <button
        type="button"
        role="switch"
        :aria-checked="modelValue"
        :disabled="disabled"
        @click="!disabled && $emit('update:modelValue', !modelValue)"
        class="relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-none border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2 focus:ring-offset-base-100 disabled:opacity-50 disabled:cursor-not-allowed"
        :class="modelValue ? 'bg-primary' : 'bg-base-300'"
    >
        <span
            aria-hidden="true"
            class="pointer-events-none inline-block h-5 w-5 transform rounded-none ring-0 transition duration-200 ease-in-out"
            :class="modelValue ? 'translate-x-5 bg-primary-content' : 'translate-x-0 bg-base-100'"
        ></span>
    </button>
</template>

<script setup lang="ts">
defineProps({
    modelValue: {
        type: Boolean,
        default: false,
    },
    disabled: {
        type: Boolean,
        default: false,
    },
});

defineEmits(["update:modelValue"]);
</script>


