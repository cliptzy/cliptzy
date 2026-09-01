<template>
    <BentoCard
        class="flex-1 flex flex-col items-center justify-center !bg-black relative overflow-hidden group p-4 h-full min-h-0 xl:min-h-[400px]"
    >
        <h3
            class="absolute top-4 left-4 text-xs font-bold text-[var(--color-text-muted)] uppercase tracking-wider z-10 flex items-center gap-2"
        >
            <IconMonitorPlay class="w-4 h-4" /> Preview
        </h3>

        <div
            class="relative w-full bg-gray-900 rounded-lg overflow-hidden border border-gray-800 shadow-2xl transition-all duration-300"
            :class="props.mode === 'compilation' ? 'aspect-video max-w-[640px]' : { 'max-w-[320px]': true, 'aspect-[9/16]': settings.config.output_ratio === '9:16', 'aspect-square': settings.config.output_ratio === '1:1', 'aspect-video': settings.config.output_ratio === '16:9', 'aspect-auto': settings.config.output_ratio === 'original' }"
        >
            <!-- Iframe Container to crop YouTube to output ratio -->
            <div
                v-show="isYoutube"
                class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 h-full aspect-video pointer-events-none opacity-90"
            >
                <div id="youtube-player" class="w-full h-full"></div>
            </div>

            <video
                v-if="!isYoutube && videoStore.metadata?.stream_url"
                ref="videoPlayer"
                :src="videoStore.metadata.stream_url"
                class="w-full h-full object-cover opacity-90"
                loop
                @timeupdate="
                    localTime = ($event.target as HTMLVideoElement).currentTime;
                    if (isPlaying) {
                        videoStore.currentTime = localTime;
                        if (videoStore.selectedSegment && localTime >= videoStore.selectedSegment.end) {
                            videoPlayer?.pause();
                            isPlaying = false;
                            videoStore.currentTime = videoStore.selectedSegment.end;
                        }
                    }
                "
            />

            <img
                v-if="!videoStore.metadata"
                src="https://images.unsplash.com/photo-1611162617474-5b21e879e113?q=80&w=1000&auto=format&fit=crop"
                class="w-full h-full object-cover opacity-50"
            />
            <img
                v-else-if="!isYoutube && !videoStore.metadata?.stream_url"
                :src="videoStore.metadata?.thumbnail_url"
                class="w-full h-full object-cover opacity-50"
            />

            <!-- Safe Zones Overlay -->
            <div
                v-show="props.mode !== 'compilation' && showSafeZone && settings.config.output_ratio === '9:16'"
                class="absolute inset-0 pointer-events-none"
            >
                <!-- Bottom Vignette (Title & description area) -->
                <div
                    class="absolute left-0 right-0 bottom-0 h-40 bg-gradient-to-t from-black/80 to-transparent"
                ></div>

                <!-- Right Interaction Icons (Like, Comment, Share) -->
                <div
                    class="absolute right-2 bottom-32 flex flex-col gap-4 opacity-70"
                >
                    <div class="w-10 h-10 rounded-full bg-white/30 backdrop-blur-sm border border-white/20"></div>
                    <div class="w-10 h-10 rounded-full bg-white/30 backdrop-blur-sm border border-white/20"></div>
                    <div class="w-10 h-10 rounded-full bg-white/30 backdrop-blur-sm border border-white/20"></div>
                    <div class="w-10 h-10 rounded-full bg-white/30 backdrop-blur-sm border border-white/20"></div>
                </div>

                <!-- Safe Zone Rectangle -->
                <div
                    class="absolute left-2 right-14 top-16 bottom-32 border border-gray-400/40 rounded-lg shadow-[inset_0_0_20px_rgba(250,204,21,0.1)] transition-opacity duration-300 border-dashed"
                >
                    <span
                        class="absolute top-2 left-2 text-[8px] font-mono text-gray-400/90 font-bold"
                        >SAFE ZONE</span
                    >
                </div>
            </div>

            <!-- Watermark Overlay -->
            <template v-if="settings.config.burn_watermark">
              <img
                  v-if="watermarkUrl"
                  :src="watermarkUrl"
                  class="absolute left-1/2 -translate-x-1/2 pointer-events-none opacity-50 object-contain w-24 h-24"
                  :class="{ 'top-8': settings.config.watermark_position === 'top', 'top-1/2 -translate-y-1/2': settings.config.watermark_position === 'center', 'bottom-32': settings.config.watermark_position === 'bottom' }"
              />
              <!-- Watermark Overlay Placeholder -->
              <div
                  v-else
                  class="absolute left-1/2 -translate-x-1/2 pointer-events-none opacity-50 text-[var(--color-text-main)] font-bold text-sm bg-gray-50 dark:bg-black/30 px-2 py-1 rounded"
                  :class="{ 'top-8': settings.config.watermark_position === 'top', 'top-1/2 -translate-y-1/2': settings.config.watermark_position === 'center', 'bottom-32': settings.config.watermark_position === 'bottom' }"
              >
                  @cliptzy
              </div>
            </template>

            <!-- Subtitle Overlay -->
            <div
                v-if="settings.config.burn_subtitle"
                class="absolute left-0 w-full text-center px-4 pointer-events-none flex flex-col items-center justify-center"
                :class="{ 'top-24': settings.config.subtitle.location === 'top', 'top-1/2 -translate-y-1/2': settings.config.subtitle.location === 'center', 'bottom-24': settings.config.subtitle.location === 'bottom' }"
                v-show="currentSubtitle || !videoStore.selectedSegment"
            >
                <span
                    v-if="settings.config.subtitle.animation === 'hormozi'"
                    key="hormozi"
                    class="font-black uppercase drop-shadow-[0_4px_4px_rgba(0,0,0,0.8)]"
                    style="-webkit-text-stroke: 1px black"
                    :style="subtitleStyle"
                >
                    {{ currentSubtitle?.text || 'INI SANGAT PENTING!' }}
                </span>
                <span
                    v-else-if="settings.config.subtitle.animation === 'karaoke'"
                    key="karaoke"
                    class="font-bold drop-shadow-md flex justify-center flex-wrap gap-x-1.5"
                    :style="{ fontFamily: subtitleStyle.fontFamily, fontSize: subtitleStyle.fontSize }"
                >
                    <span
                        v-if="currentSubtitle"
                        v-for="(w, idx) in currentSubtitle.words"
                        :key="idx"
                        class="transition-transform"
                        :class="w.active ? 'scale-110' : ''"
                        :style="w.active ? { color: subtitleStyle.color } : { color: '#D1D5DB' }"
                    >
                        {{ w.text.trim() }}
                    </span>
                    <span v-else class="text-gray-300">
                        <span class="scale-110 inline-block transition-transform" :style="{ color: subtitleStyle.color }">Ini</span>
                        sangat penting!
                    </span>
                </span>
                <span
                    v-else-if="settings.config.subtitle.border_style === 3"
                    key="brutalist"
                    class="font-mono uppercase px-2 py-0.5 shadow-[4px_4px_0px_#000]"
                    :style="{
                        fontFamily: subtitleStyle.fontFamily,
                        fontSize: subtitleStyle.fontSize,
                        color: subtitleStyle.color,
                        backgroundColor: brutalistBgColor
                    }"
                >
                    {{ currentSubtitle?.text || 'INI SANGAT PENTING' }}
                </span>
                <span
                    v-else
                    key="plain"
                    class="font-bold drop-shadow-md"
                    :style="subtitleStyle"
                >
                    {{ currentSubtitle?.text || 'Ini sangat penting!' }}
                </span>
            </div>
        </div>

        <!-- Floating Play Controls -->
        <div
            class="absolute bottom-6 left-1/2 -translate-x-1/2 flex items-center gap-4 bg-black/60 backdrop-blur-xl px-6 py-2 rounded-full border border-white/10 opacity-0 group-hover:opacity-100 transition-opacity duration-300 z-10"
        >
            <button class="text-[var(--color-text-main)] hover:text-[var(--color-text-main)] transition-colors" @click="seekRelative(-5)" >
                <IconSkipBack class="w-5 h-5" />
            </button>
            <button @click="togglePlay" class="w-10 h-10 bg-white text-[var(--color-text-main)] rounded-full flex items-center justify-center hover:scale-105 transition-transform shadow-[0_0_15px_rgba(255,255,255,0.3)]" >
                <IconPause v-if="isPlaying" class="w-5 h-5" />
                <IconPlay v-else class="w-5 h-5 ml-1" />
            </button>
            <button class="text-[var(--color-text-main)] hover:text-[var(--color-text-main)] transition-colors" @click="seekRelative(5)" >
                <IconSkipForward class="w-5 h-5" />
            </button>

            <div class="w-px h-6 bg-white/20 mx-2"></div>

            <!-- Volume Control -->
            <div class="flex items-center gap-2 group/volume w-24">
                <button @click="toggleMute" class="text-[var(--color-text-main)] hover:text-[var(--color-text-main)] transition-colors shrink-0">
                    <IconVolumeX v-if="isMuted || volume === 0" class="w-5 h-5" />
                    <IconVolume2 v-else class="w-5 h-5" />
                </button>
                <input
                    type="range"
                    v-model="volume"
                    min="0"
                    max="100"
                    class="w-full h-1 bg-white/30 rounded-lg appearance-none cursor-pointer accent-[var(--color-accent)] outline-none"
                    @input="handleVolumeChange"
                />
            </div>

            <div class="w-px h-6 bg-white/20 mx-2"></div>

            <button class="text-[var(--color-text-main)] hover:text-[var(--color-text-main)] transition-colors" @click="showSafeZone = !showSafeZone" :class="{ 'text-[var(--color-text-main)] ': showSafeZone }" title="Toggle UI Safe Zones" >
                <IconLayoutTemplate class="w-5 h-5" />
            </button>
        </div>
    </BentoCard>
</template>

<script setup lang="ts">
import { ref, watch, computed, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useVideoStore } from "../../stores/video";
import { useSettingsStore } from "../../stores/settings";
import { assToHex, assToOpacity } from "../../constants/subtitle";
import BentoCard from "../BentoCard.vue";

const props = defineProps<{
    mode?: 'clipper' | 'compilation'
}>();

import IconMonitorPlay from "~icons/lucide/monitor-play";
import IconPlay from "~icons/lucide/play";
import IconPause from "~icons/lucide/pause";
import IconSkipBack from "~icons/lucide/skip-back";
import IconSkipForward from "~icons/lucide/skip-forward";
import IconLayoutTemplate from "~icons/lucide/layout-template";
import IconVolume2 from "~icons/lucide/volume-2";
import IconVolumeX from "~icons/lucide/volume-x";

const videoStore = useVideoStore();
const settings = useSettingsStore();

const watermarkUrl = ref<string | null>(null);

watch(() => settings.config.watermark_image, async (newPath) => {
    if (newPath) {
        try {
            watermarkUrl.value = await invoke('read_image_base64', { path: newPath });
        } catch (e) {
            console.error("Failed to load watermark:", e);
            watermarkUrl.value = null;
        }
    } else {
        watermarkUrl.value = null;
    }
}, { immediate: true });

const isPlaying = ref(false);
const localTime = ref(0);
const showSafeZone = ref(true);
const videoPlayer = ref<HTMLVideoElement | null>(null);

const volume = ref(100);
const isMuted = ref(false);

let ytPlayer: any = null;
let ytInterval: any = null;

const isYoutube = computed(() => {
    return videoStore.metadata?.video_id && videoStore.metadata.video_id !== 'local';
});

const subtitleStyle = computed(() => {
    const colorHex = assToHex(settings.config.subtitle.color);
    const fontFamily = settings.config.subtitle.font || 'sans-serif';
    const scale = 568 / 1280;
    const fontSizePx = (settings.config.subtitle.font_size || 60) * scale;

    return {
        color: colorHex,
        fontFamily: fontFamily,
        fontSize: `${fontSizePx}px`,
        lineHeight: '1.2'
    };
});

const brutalistBgColor = computed(() => {
    let colorHex = '#DC2626';
    const assColor = settings.config.subtitle.bg_color;
    if (assColor && assColor.length === 10 && assColor.startsWith('&H')) {
        colorHex = assToHex(assColor);
        const opacity = assToOpacity(assColor);
        if (opacity < 100) {
            return `color-mix(in srgb, ${colorHex} ${opacity}%, transparent)`;
        }
    }
    return colorHex;
});

const currentSubtitle = computed(() => {
    if (!videoStore.selectedSegment) return null;
    const key = `${videoStore.selectedSegment.start}-${videoStore.selectedSegment.end}`;
    const analysis = videoStore.analyzedSegments[key];
    if (!analysis || !analysis.transcript) return null;

    // Find the segment that matches localTime
    const segment = analysis.transcript.find((t: any) => {
        const absStart = t.start + videoStore.selectedSegment!.start;
        const absEnd = t.end + videoStore.selectedSegment!.start;
        return localTime.value >= absStart && localTime.value <= absEnd;
    });

    if (!segment) return null;

    // If no word-level timestamps, return full text
    if (!segment.words || segment.words.length === 0) {
        return {
            text: segment.text,
            words: [{ text: segment.text, active: true }]
        };
    }

    const maxWords = settings.config.subtitle.max_words || 3;
    const words = segment.words;

    // Group words into chunks of `maxWords`
    const chunks: any[][] = [];
    for (let i = 0; i < words.length; i += maxWords) {
        chunks.push(words.slice(i, i + maxWords));
    }

    // Find which chunk we are currently in based on localTime
    let activeChunk: any[] | null = null;

    for (const chunk of chunks) {
        const firstWordStart = chunk[0].start + videoStore.selectedSegment!.start;
        const lastWordEnd = chunk[chunk.length - 1].end + videoStore.selectedSegment!.start;

        // If current time is within this chunk's time boundaries
        if (localTime.value >= firstWordStart && localTime.value <= lastWordEnd) {
            activeChunk = chunk;
            break;
        }

        // If we are in a gap BEFORE this chunk starts
        if (localTime.value < firstWordStart) {
            // Only show the chunk precisely when it starts (0.0 threshold)
            // This prevents subtitles appearing before spoken during gaps
            if (firstWordStart - localTime.value <= 0.05) {
                activeChunk = chunk;
            }
            break;
        }
    }

    if (!activeChunk) return null;

    // Find the most recently active word index inside the chunk
    let activeWordIndex = -1;
    for (let i = 0; i < activeChunk.length; i++) {
        const absStart = activeChunk[i].start + videoStore.selectedSegment!.start;
        if (localTime.value >= absStart) {
            activeWordIndex = i;
        }
    }
    if (activeWordIndex === -1) activeWordIndex = 0; // fallback if it hasn't started yet

    const displayWords = activeChunk.map((w: any, index: number) => {
        return {
            text: w.word,
            active: index === activeWordIndex
        };
    });

    return {
        text: displayWords.map((w: any) => w.text).join(''), // `w.word` already has leading spaces from whisper
        words: displayWords
    };
});

const applyVolume = () => {
    const vol = isMuted.value ? 0 : volume.value;

    if (isYoutube.value && ytPlayer && ytPlayer.setVolume) {
        if (isMuted.value) {
            ytPlayer.mute();
        } else {
            ytPlayer.unMute();
            ytPlayer.setVolume(vol);
        }
    } else if (videoPlayer.value) {
        videoPlayer.value.muted = isMuted.value;
        videoPlayer.value.volume = vol / 100;
    }
};

const handleVolumeChange = () => {
    if (volume.value > 0 && isMuted.value) {
        isMuted.value = false;
    } else if (volume.value == 0 && !isMuted.value) {
        isMuted.value = true;
    }
    applyVolume();
};

const toggleMute = () => {
    isMuted.value = !isMuted.value;
    applyVolume();
};

const initPlayer = () => {
    if (!isYoutube.value || !(window as any).YT || ytPlayer) return;
    ytPlayer = new (window as any).YT.Player('youtube-player', {
        videoId: videoStore.metadata?.video_id,
        width: '100%',
        height: '100%',
        playerVars: {
            autoplay: 0,
            controls: 0,
            disablekb: 1,
            fs: 0,
            modestbranding: 1,
            rel: 0,
            showinfo: 0,
            mute: 0,
            playsinline: 1
        },
        events: {
            onReady: (_event: any) => {
                applyVolume();
                isPlaying.value = false;

                // Start polling time and sync to store
                ytInterval = setInterval(() => {
                    if (ytPlayer && ytPlayer.getCurrentTime) {
                        const t = ytPlayer.getCurrentTime();
                        localTime.value = t;
                        if (isPlaying.value) {
                            videoStore.currentTime = t;

                            // Auto-pause if we hit the end of selected segment
                            if (videoStore.selectedSegment && t >= videoStore.selectedSegment.end) {
                                ytPlayer.pauseVideo();
                                isPlaying.value = false;
                                videoStore.currentTime = videoStore.selectedSegment.end;
                            }
                        }
                    }
                }, 100);
            }
        }
    });
};

watch(videoPlayer, (player) => {
    if (player) {
        applyVolume();
    }
});

onMounted(() => {
    if (!(window as any).YT) {
        const tag = document.createElement('script');
        tag.src = "https://www.youtube.com/iframe_api";
        document.head.appendChild(tag);
        (window as any).onYouTubeIframeAPIReady = () => {
            if (isYoutube.value) {
                setTimeout(initPlayer, 100);
            }
        };
    } else if (isYoutube.value) {
        setTimeout(initPlayer, 100);
    }
});

onUnmounted(() => {
    if (ytInterval) clearInterval(ytInterval);
    if (ytPlayer) ytPlayer.destroy();
});

watch(() => videoStore.metadata?.video_id, (newId) => {
    if (isYoutube.value) {
        if (ytPlayer && ytPlayer.loadVideoById) {
            ytPlayer.loadVideoById(newId);
        } else {
            setTimeout(initPlayer, 100);
        }
    }
});

const togglePlay = () => {
    // Auto-rewind if we try to play at the end of the segment
    if (!isPlaying.value && videoStore.selectedSegment) {
        if (videoStore.currentTime >= videoStore.selectedSegment.end - 0.2) {
            videoStore.currentTime = videoStore.selectedSegment.start;
        }
    }

    if (isYoutube.value) {
        if (!ytPlayer) return;
        if (isPlaying.value) {
            ytPlayer.pauseVideo();
        } else {
            ytPlayer.playVideo();
        }
        isPlaying.value = !isPlaying.value;
    } else {
        if (!videoPlayer.value) return;
        if (isPlaying.value) {
            videoPlayer.value.pause();
        } else {
            videoPlayer.value.play();
        }
        isPlaying.value = !isPlaying.value;
    }
};

const seekRelative = (delta: number) => {
    videoStore.currentTime = Math.max(0, videoStore.currentTime + delta);
};

// Expose a way for other components (like Timeline) to control the player
// Add a threshold check so we don't infinitely seek when store updates from normal playback
watch(() => videoStore.currentTime, (time) => {
    if (time === undefined) return;

    if (isYoutube.value) {
        if (ytPlayer && ytPlayer.seekTo) {
            const current = ytPlayer.getCurrentTime() || 0;
            // Only seek if difference is large enough (meaning it was a manual seek, not normal playback)
            if (Math.abs(current - time) > 0.5) {
                ytPlayer.seekTo(time, true);
                localTime.value = time;
                if (isPlaying.value) ytPlayer.playVideo();
            }
        } else {
            localTime.value = time;
        }
    } else {
        if (videoPlayer.value) {
            const current = videoPlayer.value.currentTime;
            if (Math.abs(current - time) > 0.5) {
                videoPlayer.value.currentTime = time;
                localTime.value = time;
                // if (isPlaying.value) videoPlayer.value.play().catch(e => console.log('Autoplay prevented', e));
            }
        }
    }
});
</script>
