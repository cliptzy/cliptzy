<template>
    <button
        :type="type"
        :disabled="disabled"
        class="inline-flex items-center justify-center transition-all duration-150 active:scale-[0.97] focus:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed disabled:active:scale-100"
        :class="[
            variantClasses[variant],
            sizeClasses[size],
            roundedClasses[rounded] || roundedClasses.none,
            customClass,
        ]"
        @click="$emit('click', $event)"
    >
        <slot></slot>
    </button>
</template>

<script setup lang="ts">
defineProps({
    type: {
        type: String as () => "button" | "submit" | "reset",
        default: "button",
    },
    variant: { type: String, default: "secondary" },
    size: { type: String, default: "md" },
    rounded: { type: String, default: "none" },
    disabled: { type: Boolean, default: false },
    customClass: { type: String, default: "" },
});

defineEmits(["click"]);

const variantClasses: Record<string, string> = {
    primary: "bg-primary text-primary-content",
    secondary:
        "bg-base-200/50 dark:bg-base-200/30 text-base-content hover:bg-base-300/50",
    ghost:
        "bg-transparent text-secondary hover:text-base-content hover:bg-base-300/50",
    danger: "bg-error text-error-content",
};

const sizeClasses: Record<string, string> = {
    sm: "w-8 h-8 p-2",
    md: "w-10 h-10 p-2.5",
    lg: "w-12 h-12 p-3",
};

const roundedClasses: Record<string, string> = {
    none: "rounded-none",
    sm: "rounded-none",
    md: "rounded-none",
    lg: "rounded-none",
    xl: "rounded-none",
};
</script>
