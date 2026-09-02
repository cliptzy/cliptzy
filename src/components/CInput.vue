<template>
    <div class="relative w-full group">
        <div
            class="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none text-secondary group-focus-within:text-base-content transition-colors"
            v-if="$slots.icon"
        >
            <slot name="icon"></slot>
        </div>
        <input
            :value="modelValue"
            @input="
                $emit(
                    'update:modelValue',
                    ($event.target as HTMLInputElement).value,
                )
            "
            :type="type"
            :placeholder="placeholder"
            class="w-full bg-base-200 border border-neutral text-base-content text-sm rounded-none focus:ring-1 focus:ring-primary focus:border-primary focus:outline-none block p-3 transition-all placeholder-secondary"
            :class="[$slots.icon ? 'pl-11' : '']"
        />
    </div>
</template>

<script setup lang="ts">
defineProps({
    modelValue: {
        type: [String, Number],
        default: "",
    },
    type: {
        type: String,
        default: "text",
    },
    placeholder: {
        type: String,
        default: "",
    },
});

defineEmits(["update:modelValue"]);
</script>


