<template>
    <div class="relative inline-block w-full text-left">
        <div
            @click="dropdownOpen = !dropdownOpen"
            class="cursor-pointer"
        >
            <slot name="trigger" />
        </div>

        <transition
            enter-active-class="transition ease-out duration-100"
            enter-from-class="transform opacity-0 scale-95"
            enter-to-class="transform opacity-100 scale-100"
            leave-active-class="transition ease-in duration-75"
            leave-from-class="transform opacity-100 scale-100"
            leave-to-class="transform opacity-0 scale-95"
        >
            <div
                v-show="dropdownOpen"
                class="origin-top-right absolute z-20 mt-2 w-full min-w-[120px] rounded-none bg-base-200 border border-neutral focus:outline-none"
            >
                <div class="py-1">
                    <slot name="default" />
                </div>
            </div>
        </transition>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";

const props = withDefaults(
    defineProps<{
        modelValue?: string | number;
        placeholder?: string;
        options?: Array<{ label: string; value: string | number }>;
    }>(),
    {
        placeholder: "Pilih...",
    },
);

defineEmits(["update:modelValue", "change"]);

const dropdownOpen = ref(false);

const handleClickOutside = (e: MouseEvent) => {
    if (!(e.target as HTMLElement)?.closest(".relative")) {
        dropdownOpen.value = false;
    }
};

onMounted(() => {
    document.addEventListener("click", handleClickOutside);
});

onUnmounted(() => {
    document.removeEventListener("click", handleClickOutside);
});
</script>


