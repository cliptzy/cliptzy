<template>
    <div
        class="bg-base-200 transition-colors duration-200 ease-out overflow-hidden"
        :class="[
            roundedClass,
            paddingClass,
            bordered ? 'border border-neutral' : '',
        ]"
    >
        <slot></slot>
    </div>
</template>

<script setup lang="ts">
import { computed } from "vue";

const props = withDefaults(
    defineProps<{
        bordered?: boolean;
        padding?: "sm" | "md" | "lg" | "none";
        rounded?: string;
    }>(),
    {
        bordered: true,
        padding: "md",
        rounded: "none",
    },
);

const roundedClass = computed(() => {
    const map: Record<string, string> = {
        none: "rounded-none",
        sm: "rounded-none",
        md: "rounded-none",
        lg: "rounded-none",
        xl: "rounded-none",
        "2xl": "rounded-none",
    };
    return map[props.rounded] || "rounded-none";
});

const paddingClass = computed(() => {
    const map: Record<string, string> = {
        none: "p-0",
        sm: "p-4",
        md: "p-6",
        lg: "p-8",
    };
    return map[props.padding] || "p-6";
});
</script>


