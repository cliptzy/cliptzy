<template>
    <div class="flex flex-col xl:flex-row gap-4 h-[240px] shrink-0">
        <!-- Timeline Sequence -->
        <BentoCard
            class="flex-1 p-6 flex flex-col justify-between !bg-cyan-200 dark:!bg-cyan-900/40"
        >
            <div class="flex items-center justify-between mb-4">
                <h3
                    class="text-lg font-black text-[var(--color-text-main)] tracking-wide flex items-center gap-2"
                >
                    <IconListVideo class="w-5 h-5" /> Sequence Editor
                </h3>
                <span
                    class="text-xs text-[var(--color-text-main)] font-mono font-bold tracking-widest bg-white/50 dark:bg-black/30 px-3 py-1 rounded-full"
                >
                    {{ formatTime(videoStore.currentTime) }}
                    <span class="text-[var(--color-text-muted)]"
                        >/
                        {{
                            formatTime(
                                videoStore.selectedSegment?.end ||
                                    videoStore.metadata?.duration ||
                                    0,
                            )
                        }}</span
                    >
                </span>
            </div>

            <!-- Video Timeline Track -->
            <div
                ref="timelineTrack"
                @click="handleTimelineClick"
                class="flex-1 bg-white/50 dark:bg-black/30 rounded-2xl border-none relative overflow-hidden flex items-center cursor-pointer shadow-inner"
            >
                <!-- Dynamic Playhead -->
                <div
                    v-show="videoStore.metadata"
                    class="absolute top-0 bottom-0 w-1 bg-gray-600 dark:bg-gray-400 z-30 shadow-[0_0_10px_rgba(8,145,178,0.5)] transition-all duration-75 pointer-events-none"
                    :style="{ left: `${progressPercentage}%` }"
                >
                    <div
                        class="absolute -top-1 -left-1 w-3 h-3 rotate-45 bg-gray-600 dark:bg-gray-400"
                    ></div>
                </div>

                <div
                    v-if="!videoStore.metadata"
                    class="w-full text-center text-xs text-[var(--color-text-muted)] font-black uppercase tracking-widest"
                >
                    TIDAK ADA MEDIA
                </div>

                <div
                    v-else
                    class="w-full h-full flex flex-col justify-center gap-2 relative z-10 pointer-events-none p-2"
                >
                    <!-- Zoomed Segment View -->
                    <template v-if="videoStore.selectedSegment">
                        <!-- Subtitle Track -->
                        <div
                            class="h-8 w-full relative flex items-center justify-center bg-gray-100/50 dark:bg-gray-950/50 rounded-lg"
                        >
                            <span
                                v-if="videoStore.isAnalyzing"
                                class="text-[10px] text-[var(--color-text-muted)] font-bold uppercase animate-pulse flex items-center gap-1"
                            >
                                <IconLoader class="w-4 h-4 animate-spin" />
                                Transcribing Audio...
                            </span>
                            <template v-else-if="segmentTranscript.length">
                                <div
                                    v-for="(word, i) in segmentTranscript"
                                    :key="i"
                                    class="absolute top-0 bottom-0 bg-gray-500/30 border border-gray-400/50 rounded-md flex items-center overflow-hidden"
                                    :style="{
                                        left: `${(word.start / (videoStore.selectedSegment.end - videoStore.selectedSegment.start)) * 100}%`,
                                        width: `${((word.end - word.start) / (videoStore.selectedSegment.end - videoStore.selectedSegment.start)) * 100}%`,
                                    }"
                                >
                                    <span
                                        class="text-[9px] text-[var(--color-text-main)] font-bold px-1 truncate w-full text-center"
                                        >{{ word.text }}</span
                                    >
                                </div>
                            </template>
                        </div>

                        <!-- Main Video Track -->
                        <div
                            class="h-12 w-full bg-gray-500/20 border border-gray-500/30 rounded-xl flex flex-col justify-center px-4 relative overflow-hidden"
                        >
                            <span
                                class="text-[10px] font-black text-[var(--color-text-main)] uppercase px-2 z-10"
                            >
                                KLIP TERPILIH ({{
                                    (
                                        videoStore.selectedSegment.end -
                                        videoStore.selectedSegment.start
                                    ).toFixed(1)
                                }}s)
                            </span>
                            <!-- Inner progress bar for the selected segment -->
                            <div
                                class="absolute bottom-0 left-0 h-1.5 bg-gray-500 z-20 pointer-events-none transition-all duration-75"
                                :style="{ width: `${progressPercentage}%` }"
                            ></div>
                        </div>
                    </template>

                    <!-- Full Video View (All Segments) -->
                    <template v-else-if="videoStore.metadata.segments?.length">
                        <div
                            class="w-full h-12 relative bg-gray-100/50 dark:bg-gray-950/50 rounded-xl"
                        >
                            <div
                                v-for="(seg, idx) in videoStore.metadata
                                    .segments"
                                :key="idx"
                                class="absolute top-0 bottom-0 bg-gray-500/30 border border-gray-500/50 rounded-xl flex flex-col justify-center px-2 group transition-colors"
                                :style="{
                                    left: `${(seg.start / videoStore.metadata.duration) * 100}%`,
                                    width: `${((seg.end - seg.start) / videoStore.metadata.duration) * 100}%`,
                                }"
                            >
                                <span
                                    class="text-[9px] font-black text-[var(--color-text-main)] uppercase truncate"
                                    v-if="
                                        (seg.end - seg.start) /
                                            videoStore.metadata.duration >
                                        0.05
                                    "
                                >
                                    Klip {{ idx + 1 }}
                                </span>
                            </div>
                        </div>
                    </template>

                    <!-- Fallback if no segments -->
                    <template v-else>
                        <div
                            class="h-12 w-full bg-gray-500/20 border border-gray-500/30 rounded-xl flex flex-col justify-center px-4 relative"
                        >
                            <span
                                class="text-[10px] font-black text-[var(--color-text-main)] uppercase"
                                >RAW VIDEO</span
                            >
                        </div>
                    </template>
                </div>
            </div>
        </BentoCard>

        <!-- Action / Generate -->
        <BentoCard
            class="w-full xl:w-[380px] p-8 flex flex-col justify-center items-center text-center gap-4 relative overflow-hidden group shrink-0 !bg-rose-200 dark:!bg-rose-900/40"
        >
            <IconWand2
                class="w-12 h-12 text-[var(--color-text-muted)] group-hover:scale-110 transition-transform duration-500"
            />
            <div class="flex flex-col gap-1 z-10">
                <h3 class="text-2xl font-black text-[var(--color-text-main)]">
                    Generate {{ selectedSegmentsCount }} Shorts
                </h3>
                <p class="text-sm font-bold text-[var(--color-text-muted)]">
                    Total estimasi: ~{{ selectedSegmentsCount * 3 }} menit
                </p>
            </div>

            <button
                class="w-full py-4 text-base font-black mt-2 rounded-full transition-all disabled:opacity-50 disabled:cursor-not-allowed shadow-md hover:shadow-lg z-10 bg-[var(--color-accent)] text-white hover:bg-rose-500"
                :disabled="selectedSegmentsCount === 0 || isRendering"
                @click="handleRender"
            >
                <span
                    v-if="isRendering"
                    class="flex items-center justify-center gap-2"
                >
                    <IconLoader class="w-5 h-5 animate-spin" /> Rendering...
                </span>
                <span v-else>Mulai Rendering</span>
            </button>
        </BentoCard>
    </div>
</template>

<script setup lang="ts">
import { ref, computed } from "vue";
import { useVideoStore } from "../../stores/video";
import { useAppStore } from "../../stores/app";
import { useSettingsStore } from "../../stores/settings";
import { invoke } from "@tauri-apps/api/core";
import BentoCard from "../BentoCard.vue";

import IconListVideo from "~icons/lucide/list-video";
import IconWand2 from "~icons/lucide/wand-2";
import IconLoader from "~icons/lucide/loader-2";

const videoStore = useVideoStore();
const settingsStore = useSettingsStore();
const timelineTrack = ref<HTMLElement | null>(null);
const isRendering = ref(false);

const segmentTranscript = computed(() => {
    if (!videoStore.selectedSegment) return [];
    const key = `${videoStore.selectedSegment.start}-${videoStore.selectedSegment.end}`;
    const analysis = videoStore.analyzedSegments[key];
    return analysis && analysis.transcript ? analysis.transcript : [];
});

const formatTime = (seconds: number) => {
    if (!seconds || isNaN(seconds)) return "00:00:00";
    const h = Math.floor(seconds / 3600);
    const m = Math.floor((seconds % 3600) / 60);
    const s = Math.floor(seconds % 60);
    return `${h.toString().padStart(2, "0")}:${m.toString().padStart(2, "0")}:${s.toString().padStart(2, "0")}`;
};

const progressPercentage = computed(() => {
    const start = videoStore.selectedSegment?.start || 0;
    const end =
        videoStore.selectedSegment?.end || videoStore.metadata?.duration || 1;
    const duration = end - start;
    if (duration <= 0) return 0;
    const p = ((videoStore.currentTime - start) / duration) * 100;
    return Math.max(0, Math.min(100, p));
});

const handleTimelineClick = (e: MouseEvent) => {
    if (!timelineTrack.value || !videoStore.metadata?.duration) return;
    const start = videoStore.selectedSegment?.start || 0;
    const end = videoStore.selectedSegment?.end || videoStore.metadata.duration;
    const duration = end - start;

    const rect = timelineTrack.value.getBoundingClientRect();
    const clickX = e.clientX - rect.left;
    const percentage = Math.max(0, Math.min(1, clickX / rect.width));
    videoStore.currentTime = start + percentage * duration;
};

const selectedSegmentsCount = computed(() => {
    if (!videoStore.metadata) return 0;
    let count = 0;
    if (videoStore.metadata.segments) {
        count += videoStore.metadata.segments.filter(
            (s) => s.selectedForRender,
        ).length;
    }
    if (videoStore.metadata.ai_segments) {
        count += videoStore.metadata.ai_segments.filter(
            (s) => s.selectedForRender,
        ).length;
    }
    return count;
});

const handleRender = async () => {
    if (!videoStore.metadata || !videoStore.currentUrl) return;

    let segmentsToProcess = [];
    if (videoStore.metadata.segments) {
        segmentsToProcess.push(
            ...videoStore.metadata.segments.filter((s) => s.selectedForRender),
        );
    }
    if (videoStore.metadata.ai_segments) {
        segmentsToProcess.push(
            ...videoStore.metadata.ai_segments.filter(
                (s) => s.selectedForRender,
            ),
        );
    }

    if (segmentsToProcess.length === 0) return;

    isRendering.value = true;
    try {
        for (const seg of segmentsToProcess) {
            let originalIndex = -1;
            if (videoStore.metadata.segments) {
                originalIndex = videoStore.metadata.segments.indexOf(seg);
            }
            if (originalIndex === -1 && videoStore.metadata.ai_segments) {
                originalIndex = videoStore.metadata.ai_segments.indexOf(seg);
                if (originalIndex !== -1) {
                    originalIndex += videoStore.metadata.segments?.length || 0;
                }
            }

            const payload = {
                url: videoStore.currentUrl,
                video_id: videoStore.metadata.video_id,
                start: seg.start,
                end: seg.end,
                crop_mode: settingsStore.config.crop_mode,
                use_subtitle: true,
                cookies_path: settingsStore.config.browser || null,
                segment_index: originalIndex !== -1 ? originalIndex + 1 : 1,
            };

            console.log("Invoking clip_video for segment", payload);
            await invoke("clip_video", { payload });
        }
    } catch (err: any) {
        console.error("Render failed", err);
        const appStore = useAppStore();
        appStore.addToast({
            title: "Render Gagal",
            message: err.toString(),
            type: "error",
        });
    } finally {
        isRendering.value = false;
    }
};
</script>
