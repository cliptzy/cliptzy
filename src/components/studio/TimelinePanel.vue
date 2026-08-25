<template>
  <div class="flex flex-col xl:flex-row gap-4 h-[240px] shrink-0">
    <!-- Timeline Sequence -->
    <BentoCard class="flex-1 p-4 flex flex-col justify-between">
      <div class="flex items-center justify-between mb-4">
        <h3 class="text-sm font-bold text-gray-400 uppercase tracking-wider flex items-center gap-2">
          <IconListVideo class="w-4 h-4 text-[var(--color-accent)]" /> Sequence Editor
        </h3>
        <span class="text-xs text-gray-500 font-mono">00:00:00 / 00:00:00</span>
      </div>
      
      <!-- Video Timeline Track (Mock) -->
      <div class="flex-1 bg-black/40 rounded-lg border border-[var(--color-subtle)] relative overflow-hidden flex items-center px-4">
        <div class="absolute left-1/3 top-0 bottom-0 w-px bg-[var(--color-accent)] z-10 shadow-[0_0_10px_var(--color-accent)]">
          <div class="absolute -top-1 -left-1.5 w-3 h-3 rotate-45 bg-[var(--color-accent)]"></div>
        </div>
        
        <div v-if="!videoStore.metadata" class="w-full text-center text-xs text-gray-600 font-bold uppercase tracking-widest">
          TIDAK ADA MEDIA
        </div>
        
        <div v-else class="w-full h-12 flex gap-1 items-center">
          <!-- Mock Segments -->
          <div class="h-10 bg-[var(--color-accent)]/20 border border-[var(--color-accent)]/50 rounded flex-1 flex flex-col justify-center px-2 group cursor-pointer hover:bg-[var(--color-accent)]/30 transition-colors">
            <span class="text-[9px] font-bold text-[var(--color-accent)] uppercase">Klip Utama</span>
            <div class="w-full h-1 bg-[var(--color-accent)]/50 rounded-full mt-1"></div>
          </div>
          <div class="h-10 bg-blue-500/20 border border-blue-500/50 rounded w-1/4 flex flex-col justify-center px-2 cursor-pointer hover:bg-blue-500/30 transition-colors">
            <span class="text-[9px] font-bold text-blue-400 uppercase">B-Roll</span>
          </div>
          <div class="h-10 bg-[var(--color-accent)]/20 border border-[var(--color-accent)]/50 rounded w-1/3 flex flex-col justify-center px-2 cursor-pointer hover:bg-[var(--color-accent)]/30 transition-colors">
            <span class="text-[9px] font-bold text-[var(--color-accent)] uppercase">Klip Utama 2</span>
          </div>
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
import { useVideoStore } from '../../stores/video';
import BentoCard from '../BentoCard.vue';
import GlowButton from '../GlowButton.vue';

import IconListVideo from '~icons/lucide/list-video';
import IconWand2 from '~icons/lucide/wand-2';

const videoStore = useVideoStore();
</script>
