<template>
    <span
        class="inline-flex items-center px-2.5 py-0.5 rounded-none text-[10px] font-bold leading-4"
        :class="[variantClasses[variant], sizeClasses[size]]"
    >
        <slot></slot>
    </span>
</template>

<script setup lang="ts">
const props = withDefaults(
    defineProps<{
        variant?: "info" | "success" | "warning" | "error" | "neutral";
        size?: "sm" | "md";
    }>(),
    {
        variant: "neutral",
        size: "md",
    },
);

const variantClasses: Record<string, string> = {
    info: "bg-info/20 text-info",
    success: "bg-success/20 text-success",
    warning: "bg-warning/20 text-warning",
    error: "bg-error/20 text-error",
    neutral: "bg-base-300/50 text-secondary",
};

const sizeClasses: Record<string, string> = {
    sm: "px-2 py-0.5 text-[9px]",
    md: "px-2.5 py-0.5 text-[10px]",
};
</script>


