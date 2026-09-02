<template>
    <hr
        v-if="orientation === 'horizontal'"
        class="w-full border-0 border-t"
        :class="variantClasses[variant]"
    />
    <div
        v-else
        class="h-full w-0 border-0 border-l"
        :class="variantClasses[variant]"
    />
</template>

<script setup lang="ts">
const props = withDefaults(
    defineProps<{
        orientation?: "horizontal" | "vertical";
        variant?: "default" | "muted" | "strong";
    }>(),
    {
        orientation: "horizontal",
        variant: "default",
    },
);

const variantClasses: Record<string, string> = {
    default: "border-neutral",
    muted: "border-base-300",
    strong: "border-neutral opacity-50",
};
</script>


