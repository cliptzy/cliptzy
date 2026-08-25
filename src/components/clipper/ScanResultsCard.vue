<script setup lang="ts">
import { computed } from "vue";
import IconCheckCircle from "~icons/lucide/check-circle";
import IconCheck from "~icons/lucide/check";

const props = defineProps<{
    segments: { start: number; end: number; score?: number }[];
    selectedIndices: number[];
    analyzeStatus: string;
}>();

const emit = defineEmits<{
    (e: "update:selectedIndices", value: number[]): void;
}>();

const isAllSelected = computed(() => {
    return (
        props.segments.length > 0 &&
        props.selectedIndices.length === props.segments.length
    );
});

const toggleSelectAll = () => {
    if (isAllSelected.value) {
        emit("update:selectedIndices", []);
    } else {
        emit(
            "update:selectedIndices",
            props.segments.map((_, i) => i),
        );
    }
};

const toggleSegment = (index: number) => {
    const current = [...props.selectedIndices];
    const pos = current.indexOf(index);
    if (pos > -1) {
        current.splice(pos, 1);
    } else {
        current.push(index);
    }
    emit("update:selectedIndices", current);
};

const formatTime = (seconds: number) => {
    const m = Math.floor(seconds / 60);
    const s = Math.floor(seconds % 60);
    return `${m}:${s.toString().padStart(2, "0")}`;
};
</script>

<template>
    <div
        class="border-[3px] border-black dark:border-[#3C4043] rounded-[32px] bg-white dark:bg-[#1E1E1E] p-8 transition-colors animate-in fade-in slide-in-from-bottom-4"
    >
        <div
            class="flex flex-col md:flex-row md:items-center justify-between mb-6 gap-4"
        >
            <h3 class="text-xl font-black flex items-center gap-2">
                <IconCheckCircle
                    v-if="analyzeStatus === 'done'"
                    class="w-6 h-6 text-[#34A853]"
                />
                <span v-if="analyzeStatus === 'done'"
                    >Video Dipindai ({{ segments.length }} Klip)</span
                >
                <span v-else>Hasil Pindaian</span>
            </h3>
        </div>
        <div class="my-2">
            <button
                v-if="segments.length > 0"
                @click="toggleSelectAll"
                class="border-[2px] border-black dark:border-[#5F6368] rounded-full px-4 py-2 font-bold text-sm flex items-center justify-center gap-2 transition-colors hover:bg-gray-100 dark:hover:bg-[#3C4043]"
            >
                <div
                    :class="[
                        'w-4 h-4 border-2 border-black dark:border-white rounded-sm flex items-center justify-center transition-colors',
                        isAllSelected ? 'bg-black dark:bg-white' : '',
                    ]"
                >
                    <IconCheck
                        v-if="isAllSelected"
                        class="w-3 h-3 text-white dark:text-black"
                    />
                </div>
                Pilih Semua
            </button>
        </div>

        <div
            class="p-4 bg-[#F8F9FA] dark:bg-[#1E1E1E] border-[3px] border-black dark:border-[#5F6368] rounded-3xl flex flex-col gap-3 max-h-[600px] overflow-y-auto"
        >
            <div
                v-for="(seg, idx) in segments"
                :key="idx"
                @click="toggleSegment(idx)"
                :class="[
                    'border-[3px] rounded-2xl p-3 cursor-pointer transition-all flex flex-col gap-2 relative overflow-hidden group',
                    selectedIndices.includes(idx)
                        ? 'border-[#4285F4] bg-[#E8F0FE] dark:bg-[#1A233A]'
                        : 'border-black dark:border-[#3C4043] bg-white dark:bg-[#28292C] hover:-translate-y-1',
                ]"
            >
                <div class="flex items-center justify-between z-10">
                    <span class="font-black text-md">Klip {{ idx + 1 }}</span>
                    <div
                        :class="[
                            'w-5 h-5 border-[2px] border-black rounded-full flex items-center justify-center transition-colors',
                            selectedIndices.includes(idx)
                                ? 'bg-[#4285F4] border-[#4285F4]'
                                : 'bg-white dark:bg-transparent dark:border-gray-500',
                        ]"
                    >
                        <IconCheck
                            v-if="selectedIndices.includes(idx)"
                            class="w-3 h-3 text-white"
                        />
                    </div>
                </div>

                <div
                    class="flex items-center gap-2 text-sm font-bold text-gray-600 dark:text-gray-400 z-10"
                >
                    <span
                        class="bg-gray-200 dark:bg-gray-700 px-2 py-1 rounded-md"
                        >{{ formatTime(seg.start) }}</span
                    >
                    <span>-</span>
                    <span
                        class="bg-gray-200 dark:bg-gray-700 px-2 py-1 rounded-md"
                        >{{ formatTime(seg.end) }}</span
                    >

                    <span
                        v-if="seg.score"
                        class="ml-auto text-xs font-bold text-[#EA4335]"
                    >
                        Skor: {{ Math.round(seg.score * 100) }}% 🔥
                    </span>
                </div>
            </div>

            <div
                v-if="analyzeStatus === 'done' && segments.length === 0"
                class="col-span-full text-center text-gray-500 font-bold py-8"
            >
                Tidak ada segmen yang ditemukan. Silakan gunakan metode lain.
            </div>

            <div
                v-if="analyzeStatus !== 'done' && analyzeStatus !== 'scanning'"
                class="col-span-full text-center text-gray-400 font-bold py-12 flex flex-col items-center justify-center gap-2"
            >
                <div
                    class="w-16 h-16 border-[3px] border-gray-300 dark:border-gray-600 rounded-2xl border-dashed"
                ></div>
                <span class="mt-2 text-sm">Belum ada hasil pindai</span>
            </div>
        </div>
    </div>
</template>
