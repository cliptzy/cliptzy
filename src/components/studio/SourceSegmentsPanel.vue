<template>
    <div class="w-full xl:w-[380px] flex flex-col gap-0 h-full min-h-0 shrink-0 overflow-y-auto overflow-x-hidden custom-scrollbar border-l border-neutral border-r">
        <!-- URL Input -->
        <div class="p-6 shrink-0 bg-base-100 border-b border-neutral">
            <div class="flex flex-col gap-4">
                <h3
                    class="text-lg font-black text-base-content tracking-wide mb-1 flex items-center gap-2"
                >
                    <IconYoutube class="w-5 h-5 text-red-500" /> {{ mode === 'compilation' ? 'Video Utama (MPL / VOD)' : 'Sumber Video' }}
                </h3>
                <div
                    class="flex items-center bg-base-200 rounded-none focus-within:ring-2 focus-within:ring-primary border border-neutral transition-all px-4 py-3"
                >
                    <input
                        v-model="videoUrl"
                        @keydown.enter="handleLoadVideo"
                        type="text"
                        :placeholder="mode === 'compilation' ? 'URL Video Utama (Misal MPL ID)' : 'URL YouTube / Path Lokal'"
                        class="w-full bg-transparent border-none text-base-content font-bold text-sm focus:ring-0 focus:outline-none placeholder-neutral-700/60 dark:placeholder-emerald-400/50"
                    />
                </div>
                <div
                    v-if="mode === 'compilation'"
                    class="flex items-center bg-base-200 rounded-none focus-within:ring-2 focus-within:ring-primary border border-neutral transition-all px-4 py-3"
                >
                    <input
                        v-model="compilationKeyword"
                        type="text"
                        :placeholder="isReactionMode
                            ? 'Kata kunci restreamer (misal: AE RRQ MPL S18)'
                            : 'Kata Kunci Pencarian Meme (opsional)'"
                        class="w-full bg-transparent border-none text-base-content font-bold text-sm focus:ring-0 focus:outline-none placeholder-neutral-700/60 dark:placeholder-emerald-400/50"
                    />
                </div>
                <button
                    @click="handleLoadVideo"
                    :disabled="videoStore.isLoading || videoStore.isPreparingCompilation || !videoUrl"
                    class="w-full py-3 rounded-none text-xs font-bold transition-colors disabled:opacity-50 disabled:cursor-not-allowed shadow-sm bg-primary text-primary-content hover:bg-primary/90"
                >
                    <span
                        v-if="videoStore.isLoading || videoStore.isPreparingCompilation"
                        class="flex items-center justify-center gap-2"
                    >
                        <IconLoader class="w-4 h-4 animate-spin" /> Memuat
                        Video...
                    </span>
                    <span v-else>{{ mode === 'compilation' ? 'Siapkan Kompilasi' : 'Load Video' }}</span>
                </button>
            </div>
        </div>

        <!-- Video Metadata -->
        <Transition
            enter-active-class="transition-all duration-300 ease-out"
            enter-from-class="opacity-0 -translate-y-4"
            enter-to-class="opacity-100 translate-y-0"
        >
            <div
                v-if="videoStore.metadata"
                class="p-4 shrink-0 bg-base-100 border-b border-neutral"
            >
                <div class="flex items-start gap-4">
                    <div
                        class="w-32 aspect-video bg-base-200 rounded-none overflow-hidden shrink-0 relative group"
                    >
                        <img
                            :src="videoStore.metadata.thumbnail_url"
                            class="w-full h-full object-cover  transition-transform duration-500"
                        />
                        <div
                            class="absolute bottom-1 right-1 bg-base-100/70 text-base-content text-[10px] font-mono font-bold px-1.5 rounded"
                        >
                            {{ formatDuration(videoStore.metadata.duration) }}
                        </div>
                    </div>
                    <div class="flex flex-col flex-1 min-w-0">
                        <h3
                            class="font-bold text-sm text-base-content line-clamp-2 leading-tight mb-1"
                            :title="videoStore.metadata.title"
                        >
                            {{ videoStore.metadata.title }}
                        </h3>
                        <span
                            class="text-xs font-bold text-secondary dark:text-slate-400 mb-2"
                            >{{
                                videoStore.metadata.uploader || "YouTube Video"
                            }}</span
                        >

                        <div class="flex flex-wrap gap-2 mt-auto">
                            <span
                                class="text-[10px] font-bold bg-base-200/60 text-base-content px-2 py-0.5 rounded-none shadow-sm"
                                >{{
                                    (
                                        videoStore.metadata.view_count || 0
                                    ).toLocaleString()
                                }}
                                Views</span
                            >
                        </div>
                    </div>
                </div>
            </div>
        </Transition>

                <!-- Compilation Restreamers -->
        <div v-if="mode === 'compilation' && isReactionMode && videoStore.compilationData?.epic_moments?.length" class="p-4 shrink-0 bg-base-100 border-b border-neutral">
            <h3 class="text-sm font-black text-base-content tracking-wide mb-3 flex items-center gap-2">
                <IconSparkles class="w-4 h-4" /> Momen Epik Terdeteksi
            </h3>
            <div class="flex flex-col gap-2 max-h-32 overflow-y-auto custom-scrollbar">
                <div
                    v-for="(moment, idx) in videoStore.compilationData.epic_moments"
                    :key="idx"
                    class="flex items-center justify-between p-2 bg-base-200 rounded-none text-xs"
                >
                    <span class="font-bold text-base-content truncate flex-1 mr-2">{{ moment.description }}</span>
                    <span class="font-mono text-secondary shrink-0">
                        {{ formatTime(moment.start) }}–{{ formatTime(moment.end) }}
                    </span>
                </div>
            </div>
        </div>

        <div v-if="mode === 'compilation'" class="p-6 shrink-0 flex flex-col bg-base-100 border-b border-neutral">
            <div class="flex items-center justify-between mb-1 gap-2">
                <h3 class="text-lg font-black text-base-content tracking-wide flex items-center gap-2 whitespace-nowrap">
                    <IconList class="w-5 h-5" /> {{ isReactionMode ? 'Restreamer Tersinkronisasi' : 'Target Restreamer' }}
                </h3>
                <button
                    v-if="videoStore.compilationData?.restreamers?.length"
                    @click="toggleSelectAllRestreamers"
                    class="text-[10px] text-base-content hover:opacity-80 transition-opacity whitespace-nowrap shrink-0"
                >
                    Toggle Select All
                </button>
            </div>
            <p v-if="isReactionMode" class="text-[10px] text-secondary mb-3 leading-tight">
                Ditemukan otomatis via pencocokan audio. Uncheck untuk mengecualikan dari kompilasi.
            </p>
            
            <div class="flex flex-col relative">
                <div v-if="videoStore.isPreparingCompilation" class="absolute inset-0 z-10 bg-[var(--color-surface)]/80 backdrop-blur-sm flex flex-col items-center justify-center min-h-[120px] rounded-none">
                    <IconLoader class="w-10 h-10 animate-spin text-[var(--color-accent)] mb-3" />
                    <span class="text-xs font-bold uppercase tracking-widest text-secondary animate-pulse">Menyiapkan Kompilasi...</span>
                </div>
                
                <div class="flex flex-col gap-2 relative max-h-80 overflow-y-auto custom-scrollbar">
                    <div v-if="videoStore.compilationData?.restreamers?.length" class="flex flex-col gap-2">
                        <div class="flex justify-between items-center px-1 mb-1">
                            <span class="text-[10px] text-secondary font-bold uppercase">
                                {{ videoStore.compilationData.restreamers.length }} Restreamer Ditemukan
                            </span>
                            <span class="text-[10px] text-secondary font-bold">
                                {{ videoStore.selectedRestreamers.length }} dipilih
                            </span>
                        </div>
                        <label
                            v-for="restreamer in videoStore.compilationData.restreamers"
                            :key="restreamer.video_id"
                            class="flex items-start gap-3 p-3 bg-base-200 rounded-none cursor-pointer hover:bg-base-300 transition-colors border border-transparent hover:border-neutral"
                        >
                            <input
                                type="checkbox"
                                :value="restreamer.video_url"
                                v-model="videoStore.selectedRestreamers"
                                class="w-4 h-4 mt-1 text-primary rounded focus:ring-primary bg-base-200 border-neutral shrink-0"
                            />
                            <div class="w-24 aspect-video bg-base-200 rounded-none overflow-hidden shrink-0 relative">
                                <img
                                    :src="restreamer.thumbnail"
                                    :alt="restreamer.title"
                                    class="w-full h-full object-cover"
                                />
                                <div
                                    v-if="restreamer.duration"
                                    class="absolute bottom-0.5 right-0.5 bg-base-100/70 text-base-content text-[9px] font-mono font-bold px-1 rounded"
                                >
                                    {{ formatDuration(restreamer.duration) }}
                                </div>
                            </div>
                            <div class="flex flex-col flex-1 min-w-0">
                                <h4
                                    class="font-bold text-xs text-base-content line-clamp-2 leading-tight mb-0.5"
                                    :title="restreamer.title"
                                >
                                    {{ restreamer.title }}
                                </h4>
                                <span class="text-[10px] font-bold text-secondary dark:text-slate-400 truncate mb-1">
                                    {{ restreamer.uploader || 'YouTube Channel' }}
                                </span>
                                <div class="flex flex-wrap gap-1.5">
                                    <span
                                        v-if="restreamer.view_count"
                                        class="text-[9px] font-bold bg-base-200/60 text-base-content px-1.5 py-0.5 rounded-none"
                                    >
                                        {{ restreamer.view_count.toLocaleString() }} views
                                    </span>
                                    <span
                                        v-if="restreamer.upload_date"
                                        class="text-[9px] font-bold bg-base-200/60 text-secondary px-1.5 py-0.5 rounded-none"
                                    >
                                        {{ formatUploadDate(restreamer.upload_date) }}
                                    </span>
                                </div>
                            </div>
                        </label>
                    </div>
                    <div v-else-if="videoStore.metadata" class="h-full flex flex-col items-center justify-center text-secondary opacity-50 py-6">
                        <IconList class="w-10 h-10 mb-2 opacity-50" />
                        <span class="text-xs font-bold uppercase tracking-widest text-center">
                            {{ isReactionMode ? 'SIAPKAN KOMPILASI UNTUK MENEMUKAN RESTREAMER' : 'TIDAK ADA RESTREAMER DITEMUKAN' }}
                        </span>
                    </div>
                </div>
            </div>
        </div>

        <!-- Segment List & Scan Controls -->
        <div v-if="mode === 'clipper'" class="p-6 flex-1 flex flex-col min-h-0 bg-base-100 border-b border-neutral">
            <div class="flex items-center justify-between mb-4 gap-2">
                <h3
                    class="text-lg font-black text-base-content tracking-wide flex items-center gap-2 whitespace-nowrap"
                >
                    <IconList class="w-5 h-5" /> Segmen
                </h3>

                <!-- Scan Mode Tabs -->
                <div
                    class="flex bg-base-200 p-1 rounded-none border border-neutral shrink-0 overflow-x-auto custom-scrollbar shadow-sm gap-1"
                >
                    <button
                        @click="scanMode = 'heatmap'"
                        class="px-3 py-1.5 rounded-none text-xs font-bold transition-all bg-base-200 border border-neutral text-base-content hover:bg-base-300"
                        :class="
                            scanMode === 'heatmap'
                                ? 'shadow-sm'
                                : 'opacity-50 hover:opacity-100'
                        "
                    >
                        Heatmap
                    </button>
                    <button
                        @click="scanMode = 'ai'"
                        class="px-3 py-1.5 rounded-none text-xs font-bold transition-all bg-base-200 border border-neutral text-base-content hover:bg-base-300"
                        :class="
                            scanMode === 'ai'
                                ? 'shadow-sm'
                                : 'opacity-50 hover:opacity-100'
                        "
                    >
                        AI
                    </button>
                    <button
                        @click="scanMode = 'custom'"
                        class="px-3 py-1.5 rounded-none text-xs font-bold transition-all bg-base-200 border border-neutral text-base-content hover:bg-base-300"
                        :class="
                            scanMode === 'custom'
                                ? 'shadow-sm'
                                : 'opacity-50 hover:opacity-100'
                        "
                    >
                        Manual
                    </button>
                </div>
            </div>

            <!-- TAB CUSTOM -->
            <div
                v-if="scanMode === 'custom'"
                class="flex-1 flex flex-col gap-3"
            >
                <div class="text-xs text-secondary mb-2">
                    Tentukan waktu mulai dan selesai secara manual.
                </div>
                <div class="flex gap-2">
                    <div class="flex-1">
                        <label
                            class="text-[10px] uppercase text-secondary font-bold ml-1"
                            >Mulai</label
                        >
                        <input
                            type="text"
                            placeholder="00:00"
                            class="w-full bg-base-200 border border-neutral rounded-none p-2 text-sm text-center focus:border-[var(--color-accent)] focus:outline-none"
                        />
                    </div>
                    <div class="flex-1">
                        <label
                            class="text-[10px] uppercase text-secondary font-bold ml-1"
                            >Selesai</label
                        >
                        <input
                            type="text"
                            placeholder="01:00"
                            class="w-full bg-base-200 border border-neutral rounded-none p-2 text-sm text-center focus:border-[var(--color-accent)] focus:outline-none"
                        />
                    </div>
                </div>
                <CButton variant="primary" class="w-full py-1.5 mt-2 text-xs"
                    >Tambahkan Segmen</CButton
                >
            </div>

            <!-- TAB AI -->
            <div
                v-else-if="scanMode === 'ai'"
                class="flex-1 flex flex-col relative min-h-0"
            >
                <div
                    v-if="videoStore.isScanningAI"
                    class="absolute inset-0 z-10 bg-[var(--color-surface)]/80 backdrop-blur-sm flex flex-col items-center justify-center"
                >
                    <IconLoader
                        class="w-6 h-6 animate-spin text-base-content mb-2"
                    />
                    <span class="text-xs text-secondary"
                        >Menganalisis AI...</span
                    >
                </div>

                <div
                    v-if="
                        !videoStore.metadata?.ai_segments ||
                        videoStore.metadata.ai_segments.length === 0
                    "
                    class="flex-1 flex flex-col items-center justify-center text-center gap-3 opacity-80 py-6 text-secondary"
                >
                    <IconSparkles class="w-8 h-8" />
                    <p class="text-xs px-4">
                        Klik <b>Scan AI</b> untuk membiarkan LLM mencari momen
                        viral (butuh waktu lebih lama).
                    </p>
                    <CButton variant="primary" 
                        @click="handleScanAI"
                        :disabled="!videoStore.metadata"
                        class="py-1 px-4 text-xs"
                        >Jalankan AI Scan</CButton
                    >
                </div>

                <div
                    v-else
                    class="flex-1 overflow-y-auto custom-scrollbar flex flex-col gap-2"
                >
                    <div class="flex justify-between items-center mb-1 px-1">
                        <span
                            class="text-[10px] text-secondary font-bold uppercase"
                            >{{ videoStore.metadata.ai_segments.length }} Klip
                            Ditemukan</span
                        >
                        <button
                            @click="toggleSelectAll('ai')"
                            class="text-[10px] text-base-content hover:text-base-content transition-colors"
                        >
                            Toggle Select All
                        </button>
                    </div>
                    <div
                        v-for="(segment, idx) in videoStore.metadata
                            .ai_segments"
                        :key="idx"
                        @click="
                            videoStore.currentTime = segment.start;
                            videoStore.selectedSegment = segment;
                        "
                        class="flex items-start gap-3 p-3 rounded-none border border-[var(--color-subtle)] bg-base-300/20 hover:bg-base-300/40 cursor-pointer group transition-colors"
                    >
                        <div class="pt-0.5">
                            <label
                                class="relative w-4 h-4 block cursor-pointer"
                                @click.stop
                            >
                                <input
                                    type="checkbox"
                                    v-model="segment.selectedForRender"
                                    class="peer sr-only"
                                />
                                <div
                                    class="w-4 h-4 border-2 border-gray-500 rounded peer-checked:bg-base-300 peer-checked:border-[var(--color-accent)] transition-all flex items-center justify-center"
                                >
                                    <IconCheck
                                        :class="
                                            segment.selectedForRender
                                                ? 'opacity-100'
                                                : 'opacity-0'
                                        "
                                        class="w-3 h-3 text-base-content"
                                    />
                                </div>
                            </label>
                        </div>
                        <div class="flex-1 min-w-0">
                            <div class="flex justify-between items-center mb-1">
                                <span
                                    class="text-xs font-bold text-base-content group-hover:text-secondary transition-colors"
                                    >AI Klip #{{ idx + 1 }}</span
                                >
                                <span
                                    class="text-[10px] font-mono text-secondary bg-base-100/10 px-1.5 rounded border border-neutral/20"
                                >
                                    {{ formatDuration(segment.start) }} -
                                    {{ formatDuration(segment.end) }}
                                </span>
                            </div>
                            <div
                                class="text-[10px] text-secondary line-clamp-2"
                            >
                                {{ segment.reason || "Momen menarik" }}
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <!-- TAB HEATMAP -->
            <div v-else class="flex-1 flex flex-col relative min-h-0">
                <div
                    v-if="videoStore.isScanning"
                    class="absolute inset-0 z-10 bg-[var(--color-surface)]/80 backdrop-blur-sm flex flex-col items-center justify-center"
                >
                    <IconLoader
                        class="w-6 h-6 animate-spin text-base-content mb-2"
                    />
                    <span class="text-xs text-secondary"
                        >Mencari momen...</span
                    >
                </div>

                <div
                    v-if="
                        !videoStore.metadata?.segments ||
                        videoStore.metadata.segments.length === 0
                    "
                    class="flex-1 flex flex-col items-center justify-center text-center gap-3 opacity-80 py-6 text-secondary"
                >
                    <IconTrending class="w-8 h-8" />
                    <p class="text-xs">
                        Klik <b>Scan Heatmap</b> untuk menganalisis retensi
                        penonton dan mendapatkan klip terbaik.
                    </p>
                    <CButton variant="primary" 
                        @click="handleScanHeatmap"
                        :disabled="!videoStore.metadata"
                        class="py-1 px-4 text-xs"
                        >Jalankan Scan Heatmap</CButton
                    >
                </div>

                <div
                    v-else
                    class="flex-1 overflow-y-auto custom-scrollbar flex flex-col gap-2"
                >
                    <div class="flex justify-between items-center mb-1 px-1">
                        <span
                            class="text-[10px] text-secondary font-bold uppercase"
                            >{{ videoStore.metadata.segments.length }} Klip
                            Ditemukan</span
                        >
                        <button
                            @click="toggleSelectAll('heatmap')"
                            class="text-[10px] text-base-content hover:text-base-content transition-colors"
                        >
                            Toggle Select All
                        </button>
                    </div>
                    <ScanResultCard
                        v-for="(segment, idx) in videoStore.metadata.segments"
                        :key="idx"
                        :segment="segment"
                        :index="idx"
                        type="heatmap"
                        :active="videoStore.selectedSegment === segment"
                        @select="
                            videoStore.currentTime = segment.start;
                            videoStore.selectedSegment = segment;
                        "
                    />
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { onMounted, computed } from "vue";
import { useVideoStore } from "../../stores/video";
import { useSettingsStore } from "../../stores/settings";
import { isReactionCompilation } from "../../constants/compilation";

import ScanResultCard from "./ScanResultCard.vue";

defineProps<{
    mode?: 'clipper' | 'compilation'
}>();

// Icons
import IconYoutube from "~icons/lucide/youtube";
import IconLoader from "~icons/lucide/loader-2";
import IconList from "~icons/lucide/list";
import IconCheck from "~icons/lucide/check";
import IconSparkles from "~icons/lucide/sparkles";
import IconTrending from "~icons/lucide/trending-up";

const videoStore = useVideoStore();
const settingsStore = useSettingsStore();

const isReactionMode = computed(() =>
    isReactionCompilation(settingsStore.config.compilation.compilation_type),
);

const videoUrl = defineModel("videoUrl", { type: String, default: "" });
const scanMode = defineModel("scanMode", { type: String, default: "heatmap" });
const compilationKeyword = defineModel("compilationKeyword", { type: String, default: "" });

const emit = defineEmits(["load-video", "scan-heatmap", "scan-ai"]);

const handleLoadVideo = () => emit("load-video");
const handleScanHeatmap = () => emit("scan-heatmap");
const handleScanAI = () => emit("scan-ai");

onMounted(async () => {
    try {
        const text = await navigator.clipboard.readText();
        if (text.includes("youtube.com/watch") || text.includes("youtu.be/")) {
            videoUrl.value = text;
        }
    } catch (err) {
        // Ignore clipboard errors
    }
});

const formatDuration = (seconds: number) => {
    if (!seconds) return "0:00";
    const h = Math.floor(seconds / 3600);
    const m = Math.floor((seconds % 3600) / 60);
    const s = Math.floor(seconds % 60);
    if (h > 0)
        return `${h}:${m.toString().padStart(2, "0")}:${s.toString().padStart(2, "0")}`;
    return `${m}:${s.toString().padStart(2, "0")}`;
};

const formatTime = (seconds: number) => {
    const m = Math.floor(seconds / 60);
    const s = Math.floor(seconds % 60);
    return `${m}:${s.toString().padStart(2, "0")}`;
};

const toggleSelectAll = (tab: "heatmap" | "ai") => {
    if (!videoStore.metadata) return;
    const segments =
        tab === "heatmap"
            ? videoStore.metadata.segments
            : videoStore.metadata.ai_segments;
    if (!segments || segments.length === 0) return;

    const allSelected = segments.every((s: any) => s.selectedForRender);

    segments.forEach((s: any) => {
        s.selectedForRender = !allSelected;
    });
};

const toggleSelectAllRestreamers = () => {
    const restreamers = videoStore.compilationData?.restreamers;
    if (!restreamers?.length) return;

    const allSelected = restreamers.every((r) =>
        videoStore.selectedRestreamers.includes(r.video_url),
    );

    if (allSelected) {
        videoStore.selectedRestreamers = [];
    } else {
        videoStore.selectedRestreamers = restreamers.map((r) => r.video_url);
    }
};

const formatUploadDate = (yyyymmdd: string) => {
    if (!yyyymmdd || yyyymmdd.length !== 8) return yyyymmdd;
    const y = yyyymmdd.slice(0, 4);
    const m = yyyymmdd.slice(4, 6);
    const d = yyyymmdd.slice(6, 8);
    return `${d}/${m}/${y}`;
};
</script>

<style scoped>
.custom-scrollbar::-webkit-scrollbar {
    width: 4px;
}
.custom-scrollbar::-webkit-scrollbar-track {
    background: rgba(255, 255, 255, 0.02);
    border-radius: 4px;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.1);
    border-radius: 4px;
}
.custom-scrollbar::-webkit-scrollbar-thumb:hover {
    background: rgba(255, 255, 255, 0.2);
}
</style>




