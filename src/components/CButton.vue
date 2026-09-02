<template>
    <button
        :type="type"
        :disabled="disabled || loading"
        class="inline-flex items-center justify-center gap-2 font-bold transition-all duration-150 active:scale-[0.97] focus:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed disabled:active:scale-100"
        :class="[
            variantClasses[variant] || variantClasses.primary,
            sizeClasses[size] || sizeClasses.md,
            block ? 'w-full flex' : '',
            roundedClasses[rounded] || roundedClasses.none,
            ghost ? variantClasses.ghost : '',
            customClass,
        ]"
        @click="$emit('click', $event)"
    >
        <!-- Loading spinner -->
        <svg
            v-if="loading"
            class="animate-spin -ml-1 mr-2 h-4 w-4"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
        >
            <circle
                class="opacity-25"
                cx="12"
                cy="12"
                r="10"
                stroke="currentColor"
                stroke-width="4"
            ></circle>
            <path
                class="opacity-75"
                fill="currentColor"
                d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
            ></path>
        </svg>
        <slot></slot>
    </button>
</template>

<script setup lang="ts">
const props = defineProps({
    type: {
        type: String as () => "button" | "submit" | "reset",
        default: "button",
    },
    variant: { type: String, default: "primary" },
    size: { type: String, default: "md" },
    rounded: { type: String, default: "none" },
    ghost: { type: Boolean, default: false },
    disabled: { type: Boolean, default: false },
    loading: { type: Boolean, default: false },
    block: { type: Boolean, default: false },
    customClass: { type: String, default: "" },
});

defineEmits(["click"]);

const variantClasses: Record<string, string> = {
    primary: "bg-primary text-primary-content",
    accent: "bg-accent text-primary-content",
    secondary:
        "bg-base-200/50 dark:bg-base-200/30 text-base-content",
    danger: "bg-error text-error-content",
    "danger-soft":
        "bg-error/20 text-error hover:bg-error hover:text-error-content",
    ghost: "bg-transparent text-secondary hover:text-base-content hover:bg-base-300/50",
    link: "bg-transparent text-primary hover:underline p-0",
};

const sizeClasses: Record<string, string> = {
    xs: "py-1.5 px-3 text-[10px]",
    sm: "py-2 px-4 text-xs",
    md: "py-3 px-6 text-sm",
    lg: "py-4 px-8 text-base",
    icon: "p-3",
    "icon-sm": "p-2",
    "icon-xs": "p-1.5 text-[10px]",
};

const roundedClasses: Record<string, string> = {
    none: "rounded-none",
    sm: "rounded-none",
    md: "rounded-none",
    lg: "rounded-none",
    xl: "rounded-none",
    "2xl": "rounded-none",
};
</script>


