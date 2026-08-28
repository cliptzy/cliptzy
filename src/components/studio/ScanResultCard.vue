<template>
    <div
        @click="$emit('select')"
        class="flex items-start gap-3 p-3 rounded-lg border transition-colors cursor-pointer group"
        :class="[
            active
                ? 'border-[var(--color-accent)] bg-black/40 shadow-sm'
                : 'border-[var(--color-subtle)] bg-black/20 hover:bg-black/40'
        ]"
    >
        <div class="pt-0.5">
            <label class="relative w-4 h-4 block cursor-pointer" @click.stop>
                <input
                    type="checkbox"
                    v-model="segment.selectedForRender"
                    class="sr-only"
                />
                <div
                    class="w-4 h-4 border-2 rounded transition-all flex items-center justify-center"
                    :class="[
                        segment.selectedForRender
                            ? 'bg-gray-200 dark:bg-gray-800 border-[var(--color-accent)]'
                            : 'border-gray-500'
                    ]"
                >
                    <IconCheck
                        class="w-3 h-3 text-[var(--color-text-main)] transition-opacity"
                        :class="segment.selectedForRender ? 'opacity-100' : 'opacity-0'"
                    />
                </div>
            </label>
        </div>
        <div class="flex-1 min-w-0">
            <div class="flex justify-between items-center mb-1">
                <span
                    class="text-xs font-bold transition-colors"
                    :class="active ? 'text-[var(--color-text-main)]' : 'text-[var(--color-text-main)] group-hover:text-[var(--color-text-muted)]'"
                >
                    {{ type === 'ai' ? 'AI Klip' : 'Klip' }} #{{ index + 1 }}
                </span>
                <span class="text-[10px] font-mono text-[var(--color-text-muted)] bg-white/5 px-1.5 rounded border border-white/10">
                    {{ formatDuration(segment.start) }} - {{ formatDuration(segment.end) }}
                </span>
            </div>
            
            <div v-if="type === 'ai'" class="text-[10px] text-[var(--color-text-muted)] line-clamp-2">
                {{ segment.reason || "Momen menarik" }}
            </div>
            <div v-else class="flex justify-between items-center">
                <span class="text-[10px] text-[var(--color-text-muted)]">
                    Durasi: {{ Math.round(segment.end - segment.start) }}s
                </span>
                <div class="flex items-center gap-1">
                    <ProgressBar
                        class="w-16"
                        :progress="(segment.score || 0.5) * 100"
                    />
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import IconCheck from "~icons/lucide/check";
import ProgressBar from "../ProgressBar.vue";

const props = defineProps({
    segment: {
        type: Object,
        required: true
    },
    index: {
        type: Number,
        required: true
    },
    type: {
        type: String as () => 'ai' | 'heatmap',
        default: 'heatmap'
    },
    active: {
        type: Boolean,
        default: false
    }
});

defineEmits(["select"]);

const formatDuration = (seconds: number) => {
    if (!seconds) return "0:00";
    const h = Math.floor(seconds / 3600);
    const m = Math.floor((seconds % 3600) / 60);
    const s = Math.floor(seconds % 60);
    if (h > 0)
        return `${h}:${m.toString().padStart(2, "0")}:${s.toString().padStart(2, "0")}`;
    return `${m}:${s.toString().padStart(2, "0")}`;
};
</script>
