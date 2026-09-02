<template>
    <div
        class="overflow-hidden bg-base-300"
        :class="[
            roundedClass,
            heightClassResolved,
            trackClassResolved,
        ]"
    >
        <div
            class="h-full transition-all duration-300 ease-out relative"
            :class="fillClassResolved"
            :style="{ width: `${clampedProgress}%` }"
        >
        </div>
    </div>
</template>

<script setup lang="ts">
import { computed } from "vue";

const props = withDefaults(
    defineProps<{
        progress: number;
        heightClass?: string;
        trackClass?: string;
        fillClass?: string;
        thin?: boolean;
    }>(),
    {
        progress: 0,
        thin: true,
    },
);

const clampedProgress = computed(() =>
    Math.max(0, Math.min(100, props.progress)),
);

const roundedClass = computed(() => "rounded-none");

const heightClassResolved = computed(() =>
    props.heightClass || (props.thin ? "h-1" : "h-2"),
);

const trackClassResolved = computed(() =>
    props.trackClass || "bg-base-300",
);

const fillClassResolved = computed(() =>
    props.fillClass || "bg-primary",
);
</script>




