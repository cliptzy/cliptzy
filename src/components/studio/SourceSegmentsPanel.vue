<template>
    <div
        class="w-full xl:w-[380px] flex flex-col gap-4 h-full min-h-0 shrink-0 overflow-y-auto overflow-x-hidden custom-scrollbar"
    >
        <!-- URL Input -->
        <BentoCard class="p-6 shrink-0">
            <div class="flex flex-col gap-4">
                <h3
                    class="text-lg font-black text-[var(--color-text-main)] tracking-wide mb-1 flex items-center gap-2"
                >
                    <IconYoutube class="w-5 h-5 text-red-500" /> {{ mode === 'compilation' ? 'Video Utama (MPL / VOD)' : 'Sumber Video' }}
                </h3>
                <div
                    class="flex items-center bg-white/60 dark:bg-black/30 rounded-2xl focus-within:ring-2 focus-within:ring-gray-500 transition-all px-4 py-3"
                >
                    <input
                        v-model="videoUrl"
                        @keydown.enter="handleLoadVideo"
                        type="text"
                        :placeholder="mode === 'compilation' ? 'URL Video Utama (Misal MPL ID)' : 'URL YouTube / Path Lokal'"
                        class="w-full bg-transparent border-none text-[var(--color-text-main)] font-bold text-sm focus:ring-0 focus:outline-none placeholder-gray-700/60 dark:placeholder-emerald-400/50"
                    />
                </div>
                <div
                    v-if="mode === 'compilation' && !isReactionMode"
                    class="flex items-center bg-white/60 dark:bg-black/30 rounded-2xl focus-within:ring-2 focus-within:ring-gray-500 transition-all px-4 py-3"
                >
                    <input v-model="compilationKeyword" type="text" placeholder="Kata Kunci Pencarian Meme (opsional)"
                        class="w-full bg-transparent border-none text-[var(--color-text-main)] font-bold text-sm focus:ring-0 focus:outline-none placeholder-gray-700/60 dark:placeholder-emerald-400/50"
                    />
                </div>
                <button
                    @click="handleLoadVideo"
                    :disabled="videoStore.isLoading || videoStore.isPreparingCompilation || !videoUrl"
                    class="w-full py-3 rounded-full text-xs font-bold transition-colors disabled:opacity-50 disabled:cursor-not-allowed shadow-sm bg-indigo-600 text-white hover:bg-indigo-700"
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
        </BentoCard>

        <!-- Video Metadata -->
        <Transition
            enter-active-class="transition-all duration-300 ease-out"
            enter-from-class="opacity-0 -translate-y-4"
            enter-to-class="opacity-100 translate-y-0"
        >
            <BentoCard
                v-if="videoStore.metadata"
                class="p-4 shrink-0 !bg-slate-200 dark:!bg-slate-900/40"
            >
                <div class="flex items-start gap-4">
                    <div
                        class="w-32 aspect-video bg-slate-300 dark:bg-slate-800 rounded-xl overflow-hidden shrink-0 relative group"
                    >
                        <img
                            :src="videoStore.metadata.thumbnail_url"
                            class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-500"
                        />
                        <div
                            class="absolute bottom-1 right-1 bg-black/80 text-white text-[10px] font-mono font-bold px-1.5 rounded"
                        >
                            {{ formatDuration(videoStore.metadata.duration) }}
                        </div>
                    </div>
                    <div class="flex flex-col flex-1 min-w-0">
                        <h3
                            class="font-bold text-sm text-[var(--color-text-main)] line-clamp-2 leading-tight mb-1"
                            :title="videoStore.metadata.title"
                        >
                            {{ videoStore.metadata.title }}
                        </h3>
                        <span
                            class="text-xs font-bold text-[var(--color-text-muted)] dark:text-slate-400 mb-2"
                            >{{
                                videoStore.metadata.uploader || "YouTube Video"
                            }}</span
                        >

                        <div class="flex flex-wrap gap-2 mt-auto">
                            <span
                                class="text-[10px] font-bold bg-white/60 dark:bg-black/30 text-[var(--color-text-main)] px-2 py-0.5 rounded-full shadow-sm"
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
            </BentoCard>
        </Transition>

                <!-- Compilation Restreamers -->
        <BentoCard v-if="mode === 'compilation' && isReactionMode && videoStore.compilationData?.epic_moments?.length" class="p-4 shrink-0 !bg-sky-200 dark:!bg-sky-900/40">
            <h3 class="text-sm font-black text-[var(--color-text-main)] tracking-wide mb-3 flex items-center gap-2">
                <IconSparkles class="w-4 h-4" /> Momen Epik Terdeteksi
            </h3>
            <div class="flex flex-col gap-2 max-h-32 overflow-y-auto custom-scrollbar">
                <div
                    v-for="(moment, idx) in videoStore.compilationData.epic_moments"
                    :key="idx"
                    class="flex items-center justify-between p-2 bg-white/60 dark:bg-black/40 rounded-xl text-xs"
                >
                    <span class="font-bold text-[var(--color-text-main)] truncate flex-1 mr-2">{{ moment.description }}</span>
                    <span class="font-mono text-[var(--color-text-muted)] shrink-0">
                        {{ formatTime(moment.start) }}–{{ formatTime(moment.end) }}
                    </span>
                </div>
            </div>
        </BentoCard>

        <BentoCard v-if="mode === 'compilation'" class="p-6 shrink-0 flex flex-col !bg-amber-200 dark:!bg-amber-900/40">
            <div class="flex items-center justify-between mb-1 gap-2">
                <h3 class="text-lg font-black text-[var(--color-text-main)] tracking-wide flex items-center gap-2 whitespace-nowrap">
                    <IconList class="w-5 h-5" /> {{ isReactionMode ? 'Restreamer Tersinkronisasi' : 'Target Restreamer' }}
                </h3>
            </div>
            <p v-if="isReactionMode" class="text-[10px] text-[var(--color-text-muted)] mb-3 leading-tight">
                Ditemukan otomatis via pencocokan audio. Uncheck untuk mengecualikan dari kompilasi.
            </p>
            
            <div class="flex flex-col relative">
                <div v-if="videoStore.isPreparingCompilation" class="absolute inset-0 z-10 bg-[var(--color-surface)]/80 backdrop-blur-sm flex flex-col items-center justify-center min-h-[120px] rounded-xl">
                    <IconLoader class="w-10 h-10 animate-spin text-[var(--color-accent)] mb-3" />
                    <span class="text-xs font-bold uppercase tracking-widest text-[var(--color-text-muted)] animate-pulse">Menyiapkan Kompilasi...</span>
                </div>
                
                <div class="flex flex-col gap-2 relative">
                    <div v-if="videoStore.compilationData?.restreamer_urls?.length" class="flex flex-col gap-2">
                        <label v-for="(url, idx) in videoStore.compilationData.restreamer_urls" :key="idx" class="flex items-center gap-3 p-3 bg-white/60 dark:bg-black/40 rounded-xl cursor-pointer hover:bg-white dark:hover:bg-black/60 transition-colors border border-transparent hover:border-gray-200 dark:hover:border-gray-700">
                            <input type="checkbox" :value="url" v-model="videoStore.selectedRestreamers" class="w-4 h-4 text-indigo-600 rounded focus:ring-indigo-500 bg-gray-100 border-gray-300" />
                            <span class="text-xs font-bold text-[var(--color-text-main)] truncate flex-1">{{ url }}</span>
                        </label>
                    </div>
                    <div v-else-if="videoStore.metadata" class="h-full flex flex-col items-center justify-center text-[var(--color-text-muted)] opacity-50">
                        <IconList class="w-10 h-10 mb-2 opacity-50" />
                        <span class="text-xs font-bold uppercase tracking-widest text-center">
                            {{ isReactionMode ? 'SIAPKAN KOMPILASI UNTUK MENEMUKAN RESTREAMER' : 'TIDAK ADA RESTREAMER DITEMUKAN' }}
                        </span>
                    </div>
                </div>
            </div>
        </BentoCard>

        <!-- Segment List & Scan Controls -->
        <BentoCard v-if="mode === 'clipper'" class="p-6 flex-1 flex flex-col min-h-0 !bg-amber-200 dark:!bg-amber-900/40">
            <div class="flex items-center justify-between mb-4 gap-2">
                <h3
                    class="text-lg font-black text-[var(--color-text-main)] tracking-wide flex items-center gap-2 whitespace-nowrap"
                >
                    <IconList class="w-5 h-5" /> Segmen
                </h3>

                <!-- Scan Mode Tabs -->
                <div
                    class="flex bg-white/50 dark:bg-black/30 p-1 rounded-xl shrink-0 overflow-x-auto custom-scrollbar shadow-sm gap-1"
                >
                    <button
                        @click="scanMode = 'heatmap'"
                        class="px-3 py-1.5 rounded-lg text-xs font-bold transition-all bg-black text-white dark:bg-white dark:text-black hover:bg-gray-800 dark:hover:bg-gray-200"
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
                        class="px-3 py-1.5 rounded-lg text-xs font-bold transition-all bg-black text-white dark:bg-white dark:text-black hover:bg-gray-800 dark:hover:bg-gray-200"
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
                        class="px-3 py-1.5 rounded-lg text-xs font-bold transition-all bg-black text-white dark:bg-white dark:text-black hover:bg-gray-800 dark:hover:bg-gray-200"
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
                <div class="text-xs text-[var(--color-text-muted)] mb-2">
                    Tentukan waktu mulai dan selesai secara manual.
                </div>
                <div class="flex gap-2">
                    <div class="flex-1">
                        <label
                            class="text-[10px] uppercase text-[var(--color-text-muted)] font-bold ml-1"
                            >Mulai</label
                        >
                        <input
                            type="text"
                            placeholder="00:00"
                            class="w-full bg-gray-50 dark:bg-black/30 border border-[var(--color-subtle)] rounded-lg p-2 text-sm text-center focus:border-[var(--color-accent)] focus:outline-none"
                        />
                    </div>
                    <div class="flex-1">
                        <label
                            class="text-[10px] uppercase text-[var(--color-text-muted)] font-bold ml-1"
                            >Selesai</label
                        >
                        <input
                            type="text"
                            placeholder="01:00"
                            class="w-full bg-gray-50 dark:bg-black/30 border border-[var(--color-subtle)] rounded-lg p-2 text-sm text-center focus:border-[var(--color-accent)] focus:outline-none"
                        />
                    </div>
                </div>
                <GlowButton class="w-full py-1.5 mt-2 text-xs"
                    >Tambahkan Segmen</GlowButton
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
                        class="w-6 h-6 animate-spin text-[var(--color-text-main)] mb-2"
                    />
                    <span class="text-xs text-[var(--color-text-muted)]"
                        >Menganalisis AI...</span
                    >
                </div>

                <div
                    v-if="
                        !videoStore.metadata?.ai_segments ||
                        videoStore.metadata.ai_segments.length === 0
                    "
                    class="flex-1 flex flex-col items-center justify-center text-center gap-3 opacity-80 py-6 text-[var(--color-text-muted)]"
                >
                    <IconSparkles class="w-8 h-8" />
                    <p class="text-xs px-4">
                        Klik <b>Scan AI</b> untuk membiarkan LLM mencari momen
                        viral (butuh waktu lebih lama).
                    </p>
                    <GlowButton
                        @click="handleScanAI"
                        :disabled="!videoStore.metadata"
                        class="py-1 px-4 text-xs"
                        >Jalankan AI Scan</GlowButton
                    >
                </div>

                <div
                    v-else
                    class="flex-1 overflow-y-auto custom-scrollbar flex flex-col gap-2"
                >
                    <div class="flex justify-between items-center mb-1 px-1">
                        <span
                            class="text-[10px] text-[var(--color-text-muted)] font-bold uppercase"
                            >{{ videoStore.metadata.ai_segments.length }} Klip
                            Ditemukan</span
                        >
                        <button
                            @click="toggleSelectAll('ai')"
                            class="text-[10px] text-[var(--color-text-main)] hover:text-[var(--color-text-main)] transition-colors"
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
                        class="flex items-start gap-3 p-3 rounded-lg border border-[var(--color-subtle)] bg-black/20 hover:bg-black/40 cursor-pointer group transition-colors"
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
                                    class="w-4 h-4 border-2 border-gray-500 rounded peer-checked:bg-gray-200 dark:bg-gray-800 peer-checked:border-[var(--color-accent)] transition-all flex items-center justify-center"
                                >
                                    <IconCheck
                                        :class="
                                            segment.selectedForRender
                                                ? 'opacity-100'
                                                : 'opacity-0'
                                        "
                                        class="w-3 h-3 text-black"
                                    />
                                </div>
                            </label>
                        </div>
                        <div class="flex-1 min-w-0">
                            <div class="flex justify-between items-center mb-1">
                                <span
                                    class="text-xs font-bold text-[var(--color-text-main)] group-hover:text-[var(--color-text-muted)] transition-colors"
                                    >AI Klip #{{ idx + 1 }}</span
                                >
                                <span
                                    class="text-[10px] font-mono text-[var(--color-text-muted)] bg-white/5 px-1.5 rounded border border-white/10"
                                >
                                    {{ formatDuration(segment.start) }} -
                                    {{ formatDuration(segment.end) }}
                                </span>
                            </div>
                            <div
                                class="text-[10px] text-[var(--color-text-muted)] line-clamp-2"
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
                        class="w-6 h-6 animate-spin text-[var(--color-text-main)] mb-2"
                    />
                    <span class="text-xs text-[var(--color-text-muted)]"
                        >Mencari momen...</span
                    >
                </div>

                <div
                    v-if="
                        !videoStore.metadata?.segments ||
                        videoStore.metadata.segments.length === 0
                    "
                    class="flex-1 flex flex-col items-center justify-center text-center gap-3 opacity-80 py-6 text-[var(--color-text-muted)]"
                >
                    <IconTrending class="w-8 h-8" />
                    <p class="text-xs">
                        Klik <b>Scan Heatmap</b> untuk menganalisis retensi
                        penonton dan mendapatkan klip terbaik.
                    </p>
                    <GlowButton
                        @click="handleScanHeatmap"
                        :disabled="!videoStore.metadata"
                        class="py-1 px-4 text-xs"
                        >Jalankan Scan Heatmap</GlowButton
                    >
                </div>

                <div
                    v-else
                    class="flex-1 overflow-y-auto custom-scrollbar flex flex-col gap-2"
                >
                    <div class="flex justify-between items-center mb-1 px-1">
                        <span
                            class="text-[10px] text-[var(--color-text-muted)] font-bold uppercase"
                            >{{ videoStore.metadata.segments.length }} Klip
                            Ditemukan</span
                        >
                        <button
                            @click="toggleSelectAll('heatmap')"
                            class="text-[10px] text-[var(--color-text-main)] hover:text-[var(--color-text-main)] transition-colors"
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
        </BentoCard>
    </div>
</template>

<script setup lang="ts">
import { onMounted, computed } from "vue";
import { useVideoStore } from "../../stores/video";
import { useSettingsStore } from "../../stores/settings";
import { isReactionCompilation } from "../../constants/compilation";
import BentoCard from "../BentoCard.vue";
import GlowButton from "../GlowButton.vue";
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

    // Check if all are currently selected
    const allSelected = segments.every((s: any) => s.selectedForRender);

    segments.forEach((s: any) => {
        s.selectedForRender = !allSelected;
    });
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


