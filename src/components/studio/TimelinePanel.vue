<template>
  <div class="flex flex-col xl:flex-row gap-4 h-[240px] shrink-0">
    <!-- Timeline Sequence -->
    <BentoCard class="flex-1 p-4 flex flex-col justify-between">
      <div class="flex items-center justify-between mb-4">
        <h3 class="text-sm font-bold text-gray-400 uppercase tracking-wider flex items-center gap-2">
          <IconListVideo class="w-4 h-4 text-[var(--color-accent)]" /> Sequence Editor
        </h3>
        <span class="text-xs text-[var(--color-accent)] font-mono font-bold tracking-widest bg-[var(--color-accent)]/10 px-3 py-1 rounded-full">
            {{ formatTime(videoStore.currentTime) }} <span class="text-gray-500">/ {{ formatTime(videoStore.selectedSegment?.end || videoStore.metadata?.duration || 0) }}</span>
        </span>
      </div>
      
      <!-- Video Timeline Track -->
      <div 
        ref="timelineTrack"
        @click="handleTimelineClick"
        class="flex-1 bg-black/40 rounded-lg border border-[var(--color-subtle)] relative overflow-hidden flex items-center cursor-pointer"
      >
        <!-- Dynamic Playhead -->
        <div 
            v-show="videoStore.metadata"
            class="absolute top-0 bottom-0 w-px bg-[var(--color-accent)] z-30 shadow-[0_0_10px_var(--color-accent)] transition-all duration-75 pointer-events-none"
            :style="{ left: `${progressPercentage}%` }"
        >
          <div class="absolute -top-1 -left-1.5 w-3 h-3 rotate-45 bg-[var(--color-accent)]"></div>
        </div>
        
        <div v-if="!videoStore.metadata" class="w-full text-center text-xs text-gray-600 font-bold uppercase tracking-widest">
          TIDAK ADA MEDIA
        </div>
        
        <div v-else class="w-full h-full flex flex-col justify-center gap-2 relative z-10 pointer-events-none">
          <!-- Zoomed Segment View -->
          <template v-if="videoStore.selectedSegment">
              <!-- Subtitle Track -->
              <div class="h-6 w-full relative flex items-center justify-center">
                  <span v-if="videoStore.isAnalyzing" class="text-[9px] text-gray-400 font-bold uppercase animate-pulse flex items-center gap-1">
                      <IconLoader class="w-3 h-3 animate-spin" /> Transcribing Audio...
                  </span>
                  <template v-else-if="segmentTranscript.length">
                      <div 
                          v-for="(word, i) in segmentTranscript" 
                          :key="i"
                          class="absolute top-0 bottom-0 bg-blue-500/30 border border-blue-400/50 rounded flex items-center overflow-hidden"
                          :style="{
                              left: `${(word.start / (videoStore.selectedSegment.end - videoStore.selectedSegment.start)) * 100}%`,
                              width: `${((word.end - word.start) / (videoStore.selectedSegment.end - videoStore.selectedSegment.start)) * 100}%`
                          }"
                      >
                          <span class="text-[8px] text-blue-200 font-bold px-1 truncate w-full">{{ word.text }}</span>
                      </div>
                  </template>
              </div>

              <!-- Main Video Track -->
              <div class="h-10 w-full bg-[var(--color-accent)]/10 border border-[var(--color-accent)]/30 rounded flex flex-col justify-center px-4 relative overflow-hidden">
                <span class="text-[9px] font-bold text-[var(--color-accent)] uppercase px-2 z-10 drop-shadow-md">
                    KLIP TERPILIH ({{ (videoStore.selectedSegment.end - videoStore.selectedSegment.start).toFixed(1) }}s)
                </span>
                <!-- Inner progress bar for the selected segment -->
                <div class="absolute bottom-0 left-0 h-1 bg-[var(--color-accent)]/80 z-20 pointer-events-none" :style="{ width: `${progressPercentage}%` }"></div>
              </div>
          </template>
          
          <!-- Full Video View (All Segments) -->
          <template v-else-if="videoStore.metadata.segments?.length">
              <div class="w-full h-10 relative">
                  <div 
                      v-for="(seg, idx) in videoStore.metadata.segments" 
                      :key="idx"
                      class="absolute top-0 bottom-0 bg-[var(--color-accent)]/20 border border-[var(--color-accent)]/50 rounded flex flex-col justify-center px-2 group transition-colors"
                      :style="{
                          left: `${(seg.start / videoStore.metadata.duration) * 100}%`,
                          width: `${((seg.end - seg.start) / videoStore.metadata.duration) * 100}%`
                      }"
                  >
                      <span class="text-[9px] font-bold text-[var(--color-accent)] uppercase truncate" v-if="((seg.end - seg.start) / videoStore.metadata.duration) > 0.05">
                          Klip {{ idx + 1 }}
                      </span>
                  </div>
              </div>
          </template>
          
          <!-- Fallback if no segments -->
          <template v-else>
              <div class="h-10 w-full bg-[var(--color-accent)]/20 border border-[var(--color-accent)]/50 rounded flex flex-col justify-center px-4 relative">
                <span class="text-[9px] font-bold text-[var(--color-accent)] uppercase">RAW VIDEO</span>
              </div>
          </template>
        </div>
      </div>
    </BentoCard>

    <!-- Action / Generate -->
    <BentoCard class="w-full xl:w-[380px] p-6 flex flex-col justify-center items-center text-center gap-4 relative overflow-hidden group shrink-0">
      <!-- Background Effect -->
      <div class="absolute -right-20 -bottom-20 w-64 h-64 bg-[var(--color-accent)]/10 blur-[80px] rounded-full group-hover:bg-[var(--color-accent)]/20 transition-colors duration-700"></div>
      
      <IconWand2 class="w-10 h-10 text-[var(--color-accent)]" />
      <div class="flex flex-col">
        <h3 class="text-xl font-black text-white">Generate 5 Shorts</h3>
        <p class="text-xs text-gray-400">Total estimasi: ~3 menit</p>
      </div>
      
      <GlowButton class="w-full py-3 text-lg mt-2">
        Mulai Rendering
      </GlowButton>
    </BentoCard>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { useVideoStore } from '../../stores/video';
import BentoCard from '../BentoCard.vue';
import GlowButton from '../GlowButton.vue';

import IconListVideo from '~icons/lucide/list-video';
import IconWand2 from '~icons/lucide/wand-2';
import IconLoader from '~icons/lucide/loader-2';

const videoStore = useVideoStore();
const timelineTrack = ref<HTMLElement | null>(null);

const segmentTranscript = computed(() => {
    if (!videoStore.selectedSegment) return [];
    const key = `${videoStore.selectedSegment.start}-${videoStore.selectedSegment.end}`;
    const analysis = videoStore.analyzedSegments[key];
    return (analysis && analysis.transcript) ? analysis.transcript : [];
});

const formatTime = (seconds: number) => {
    if (!seconds || isNaN(seconds)) return '00:00:00';
    const h = Math.floor(seconds / 3600);
    const m = Math.floor((seconds % 3600) / 60);
    const s = Math.floor(seconds % 60);
    return `${h.toString().padStart(2, '0')}:${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`;
};

const progressPercentage = computed(() => {
    const start = videoStore.selectedSegment?.start || 0;
    const end = videoStore.selectedSegment?.end || videoStore.metadata?.duration || 1;
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
    videoStore.currentTime = start + (percentage * duration);
};
</script>
