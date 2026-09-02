<template>
    <div
        class="fixed z-50 px-2.5 py-1.5 text-xs font-medium text-base-content bg-base-300 rounded-none pointer-events-none opacity-0 transition-opacity duration-150"
        :class="positionClasses[position]"
        v-show="visible"
    >
        <slot></slot>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";

const props = withDefaults(
    defineProps<{
        text?: string;
        position?: "top" | "bottom" | "left" | "right";
    }>(),
    {
        position: "top",
    },
);

const visible = ref(false);

const positionClasses: Record<string, string> = {
    top: "mb-2 translate-y-[-100%] bottom-full left-1/2 -translate-x-1/2",
    bottom: "mt-2 translate-y-[100%] top-full left-1/2 -translate-x-1/2",
    left: "mr-2 translate-x-[-100%] right-full top-1/2 -translate-y-1/2",
    right: "ml-2 translate-x-[100%] left-full top-1/2 -translate-y-1/2",
};

onMounted(() => {
    visible.value = true;
});

onUnmounted(() => {
    visible.value = false;
});
</script>


