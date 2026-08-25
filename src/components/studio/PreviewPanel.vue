<template>
  <BentoCard class="flex-1 flex flex-col items-center justify-center bg-black relative overflow-hidden group p-4 border border-[var(--color-subtle)] h-full min-h-[400px]">
    <h3 class="absolute top-4 left-4 text-xs font-bold text-gray-500 uppercase tracking-wider z-10 flex items-center gap-2">
      <IconMonitorPlay class="w-4 h-4" /> Preview
    </h3>
    
    <!-- Video Player Mock (Portrait 9:16) -->
    <div class="relative w-full max-w-[320px] aspect-[9/16] bg-gray-900 rounded-lg overflow-hidden border border-gray-800 shadow-2xl">
      <img 
        :src="videoStore.metadata?.thumbnail_url || 'https://images.unsplash.com/photo-1611162617474-5b21e879e113?q=80&w=1000&auto=format&fit=crop'" 
        class="w-full h-full object-cover opacity-50" 
      />
      
      <!-- Safe Zones Overlay -->
      <div class="absolute inset-0 pointer-events-none border border-dashed border-red-500/30">
        <div class="absolute right-2 bottom-32 flex flex-col gap-4 opacity-40">
          <div class="w-8 h-8 rounded-full bg-white/20"></div>
          <div class="w-8 h-8 rounded-full bg-white/20"></div>
          <div class="w-8 h-8 rounded-full bg-white/20"></div>
        </div>
        <div class="absolute left-2 right-12 bottom-4 h-16 bg-gradient-to-t from-black/80 to-transparent"></div>
        <div class="absolute inset-x-4 top-16 bottom-48 border border-[var(--color-accent)]/30 rounded shadow-[inset_0_0_20px_rgba(217,249,157,0.1)] transition-opacity duration-300 opacity-50 group-hover:opacity-100">
          <span class="absolute top-2 left-2 text-[8px] font-mono text-[var(--color-accent)]/80">SAFE ZONE</span>
        </div>
      </div>

      <!-- Dummy Subtitle -->
      <div class="absolute bottom-24 left-0 w-full text-center px-4">
        <span v-if="settings.config.subtitle.animation === 'hormozi'" class="text-2xl font-black uppercase text-yellow-400 drop-shadow-[0_4px_4px_rgba(0,0,0,0.8)]" style="-webkit-text-stroke: 1px black;">
          INI SANGAT <span class="text-white">PENTING!</span>
        </span>
        <span v-else-if="settings.config.subtitle.animation === 'karaoke'" class="text-xl font-bold text-gray-300 drop-shadow-md">
          <span class="text-green-400">Ini</span> sangat penting!
        </span>
        <span v-else-if="settings.config.subtitle.border_style === 3" class="text-xl font-mono uppercase bg-red-600 text-white px-2 py-0.5 shadow-[4px_4px_0px_#000]">
          INI SANGAT PENTING
        </span>
        <span v-else class="text-xl font-bold text-white drop-shadow-md">
          Ini sangat penting!
        </span>
      </div>
    </div>

    <!-- Floating Play Controls -->
    <div class="absolute bottom-6 left-1/2 -translate-x-1/2 flex items-center gap-4 bg-black/60 backdrop-blur-xl px-6 py-2 rounded-full border border-white/10 opacity-0 group-hover:opacity-100 transition-opacity duration-300">
      <button class="text-white hover:text-[var(--color-accent)] transition-colors"><IconSkipBack class="w-5 h-5" /></button>
      <button class="w-10 h-10 bg-white text-black rounded-full flex items-center justify-center hover:scale-105 transition-transform shadow-[0_0_15px_rgba(255,255,255,0.3)]"><IconPlay class="w-5 h-5 ml-1" /></button>
      <button class="text-white hover:text-[var(--color-accent)] transition-colors"><IconSkipForward class="w-5 h-5" /></button>
    </div>
  </BentoCard>
</template>

<script setup lang="ts">
import { useVideoStore } from '../../stores/video';
import { useSettingsStore } from '../../stores/settings';
import BentoCard from '../BentoCard.vue';

import IconMonitorPlay from '~icons/lucide/monitor-play';
import IconPlay from '~icons/lucide/play';
import IconSkipBack from '~icons/lucide/skip-back';
import IconSkipForward from '~icons/lucide/skip-forward';

const videoStore = useVideoStore();
const settings = useSettingsStore();
</script>
