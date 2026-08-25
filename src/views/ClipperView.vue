<script setup lang="ts">
import { watch, onMounted } from "vue";
import { storeToRefs } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import ClipperHeader from "../components/clipper/ClipperHeader.vue";
import VideoSourceCard from "../components/clipper/VideoSourceCard.vue";
import ScanResultsCard from "../components/clipper/ScanResultsCard.vue";
import ProcessingConfigCard from "../components/clipper/ProcessingConfigCard.vue";
import OutputConfigCard from "../components/clipper/OutputConfigCard.vue";
import { useSettingsStore } from "../stores/settings";
import { useClipperStore } from "../stores/clipper";

const settingsStore = useSettingsStore();
const clipperStore = useClipperStore();

// UI States from Store (preserved across navigation)
const { urlInput, scanMethod, analyzeStatus, scanMessage, scanProgress, segments, selectedIndices, videoTitle, videoThumbnail } = storeToRefs(clipperStore);

const saveConfig = async () => {
    try {
        await invoke('save_config_file', { configJson: JSON.stringify(settingsStore.toDict(), null, 2) });
    } catch (e) {
        console.error('Failed to save config', e);
    }
};

onMounted(() => {
    saveConfig();
});

watch(() => settingsStore.config, () => {
    saveConfig();
}, { deep: true });

const analyzeVideo = async () => {
    if (!urlInput.value) return;
    analyzeStatus.value = "scanning";
    scanProgress.value = 50;
    scanMessage.value = 'Menganalisa video dengan yt-dlp...';
    segments.value = [];
    selectedIndices.value = [];
    videoTitle.value = '';
    videoThumbnail.value = '';

    try {
        const result = await invoke<any>('analyze_video', { 
            url: urlInput.value,
            cookiesPath: settingsStore.config.youtube.session 
        });
        analyzeStatus.value = 'done';
        if (result) {
            if (result.title) videoTitle.value = result.title;
            if (result.thumbnail) videoThumbnail.value = result.thumbnail;
            if (result.segments) {
                segments.value = result.segments;
                // Select all by default
                selectedIndices.value = result.segments.map((_: any, i: number) => i);
            }
        }
    } catch (e: any) {
        analyzeStatus.value = 'error';
        scanMessage.value = e || 'Gagal memindai video';
    }
};

const submitJob = async () => {
    if (analyzeStatus.value !== "done") return;
    if (selectedIndices.value.length === 0) {
        alert("Pilih minimal satu segmen klip!");
        return;
    }

    console.log("Submitting video for processing:", urlInput.value);
    console.log("Selected clips:", selectedIndices.value);
};
</script>

<template>
    <div class="max-w-7xl mx-auto p-4 md:p-8 text-black dark:text-white transition-colors">
        <ClipperHeader />

        <div class="flex flex-col lg:flex-row gap-8 mt-8">
            <!-- Kiri: Input & Konfigurasi -->
            <div class="flex-1 space-y-8">
                <VideoSourceCard
                    v-model:urlInput="urlInput"
                    v-model:cookiesPath="settingsStore.config.youtube.session"
                    v-model:scanMethod="scanMethod"
                    :analyzeStatus="analyzeStatus === 'error' ? 'idle' : analyzeStatus"
                    @analyze="analyzeVideo"
                />
                
                <div class="grid grid-cols-1 md:grid-cols-2 gap-6" :class="{'opacity-50 pointer-events-none': analyzeStatus !== 'done'}">
                    <ProcessingConfigCard />

                    <OutputConfigCard
                        v-model:aspectRatio="settingsStore.config.output_ratio"
                        @submit="submitJob"
                    />
                </div>
            </div>

            <!-- Kanan: Sidebar Preview & Segments -->
            <div class="w-full lg:w-[400px] flex flex-col space-y-6">
                <!-- Video Preview -->
                <div v-if="videoThumbnail" class="border-[3px] border-black rounded-[32px] overflow-hidden bg-white dark:bg-[#28292C]">
                    <img :src="videoThumbnail" alt="Thumbnail" class="w-full h-auto object-cover border-b-[3px] border-black" />
                    <div class="p-4">
                        <h3 class="font-bold text-lg line-clamp-2 leading-tight">{{ videoTitle }}</h3>
                    </div>
                </div>
                <div v-else class="border-[3px] border-gray-300 dark:border-[#3C4043] rounded-[32px] border-dashed overflow-hidden bg-transparent flex flex-col items-center justify-center p-8 aspect-video text-gray-400">
                    <span class="font-bold">Belum ada video</span>
                </div>

                <!-- Loading State -->
                <div v-if="analyzeStatus === 'scanning'" class="border-[3px] border-black rounded-[32px] bg-[#F8F9FA] dark:bg-[#28292C] p-8 text-center animate-pulse">
                    <h3 class="font-bold text-lg mb-4">{{ scanMessage }}</h3>
                    <div class="w-full bg-gray-200 rounded-full h-4 dark:bg-gray-700 border-2 border-black overflow-hidden">
                      <div class="bg-[#4285F4] h-full rounded-full transition-all duration-300" :style="{ width: `${scanProgress}%` }"></div>
                    </div>
                </div>
                
                <!-- Error State -->
                <div v-else-if="analyzeStatus === 'error'" class="border-[3px] border-[#EA4335] rounded-[32px] bg-[#FCE8E6] dark:bg-[#3C1E1E] p-8 text-center">
                    <h3 class="font-bold text-lg text-[#EA4335]">{{ scanMessage }}</h3>
                </div>

                <!-- Scan Results -->
                <ScanResultsCard 
                    :segments="segments" 
                    v-model:selectedIndices="selectedIndices"
                    :analyzeStatus="analyzeStatus"
                />
            </div>
        </div>
    </div>
</template>
