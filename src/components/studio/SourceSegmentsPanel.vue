<template>
  <div class="w-full xl:w-[380px] flex flex-col gap-4 overflow-y-auto pr-1 custom-scrollbar shrink-0">
    
    <!-- URL Input -->
    <BentoCard class="p-4 bg-[var(--color-surface)] shrink-0">
      <div class="flex flex-col gap-3">
        <div class="flex items-center bg-black/30 rounded-lg border border-[var(--color-subtle)] focus-within:border-[var(--color-accent)] transition-colors px-3 py-2">
          <IconYoutube class="w-5 h-5 text-red-500 mr-2 shrink-0" />
          <input
            v-model="videoUrl"
            @keydown.enter="handleLoadVideo"
            type="text"
            placeholder="URL YouTube / Path Lokal"
            class="w-full bg-transparent border-none text-white text-sm focus:ring-0 focus:outline-none placeholder-gray-500"
          />
        </div>
        <GlowButton @click="handleLoadVideo" :disabled="videoStore.isLoading || !videoUrl" class="w-full py-2">
          <span v-if="videoStore.isLoading" class="flex items-center justify-center gap-2 text-sm font-bold">
            <IconLoader class="w-4 h-4 animate-spin" /> Memuat Video...
          </span>
          <span v-else class="text-sm font-bold">Load Video</span>
        </GlowButton>
      </div>
    </BentoCard>

    <!-- Video Metadata -->
    <Transition
      enter-active-class="transition-all duration-300 ease-out"
      enter-from-class="opacity-0 -translate-y-4"
      enter-to-class="opacity-100 translate-y-0"
    >
      <BentoCard v-if="videoStore.metadata" class="p-3 flex gap-3 overflow-hidden group shrink-0">
        <div class="w-24 h-14 shrink-0 rounded overflow-hidden relative bg-black">
          <img :src="videoStore.metadata.thumbnail_url" class="w-full h-full object-cover group-hover:scale-110 transition-transform" />
          <div class="absolute bottom-1 right-1 bg-black/80 px-1 rounded text-[9px] font-mono border border-white/10">
            {{ formatDuration(videoStore.metadata.duration) }}
          </div>
        </div>
        <div class="flex flex-col justify-center min-w-0">
          <h4 class="text-sm font-bold truncate text-white" :title="videoStore.metadata.title">{{ videoStore.metadata.title }}</h4>
          <p class="text-[11px] text-gray-400 truncate">{{ videoStore.metadata.uploader || 'YouTube Video' }}</p>
        </div>
      </BentoCard>
    </Transition>

    <!-- Segment List & Scan Controls -->
    <BentoCard class="p-4 flex-1 flex flex-col min-h-[300px]">
      <div class="flex items-center justify-between mb-3 gap-2">
        <h3 class="text-sm font-bold text-gray-400 uppercase tracking-wider flex items-center gap-2 whitespace-nowrap">
          <IconList class="w-4 h-4 text-[var(--color-accent)]" /> Segmen
        </h3>
        
        <!-- Scan Mode Tabs -->
        <div class="flex bg-black/50 p-1 rounded-lg border border-[var(--color-subtle)] shrink-0 overflow-x-auto custom-scrollbar">
          <button 
            @click="scanMode = 'heatmap'" 
            class="px-2 py-1 rounded text-[10px] font-bold transition-colors"
            :class="scanMode === 'heatmap' ? 'bg-white/10 text-white' : 'text-gray-500 hover:text-gray-300'"
          >Heatmap</button>
          <button 
            @click="scanMode = 'ai'" 
            class="px-2 py-1 rounded text-[10px] font-bold transition-colors"
            :class="scanMode === 'ai' ? 'bg-white/10 text-white' : 'text-gray-500 hover:text-gray-300'"
          >AI</button>
          <button 
            @click="scanMode = 'custom'" 
            class="px-2 py-1 rounded text-[10px] font-bold transition-colors"
            :class="scanMode === 'custom' ? 'bg-white/10 text-white' : 'text-gray-500 hover:text-gray-300'"
          >Manual</button>
        </div>
      </div>

      <!-- TAB CUSTOM -->
      <div v-if="scanMode === 'custom'" class="flex-1 flex flex-col gap-3">
        <div class="text-xs text-gray-400 mb-2">Tentukan waktu mulai dan selesai secara manual.</div>
        <div class="flex gap-2">
          <div class="flex-1">
            <label class="text-[10px] uppercase text-gray-500 font-bold ml-1">Mulai</label>
            <input type="text" placeholder="00:00" class="w-full bg-black/30 border border-[var(--color-subtle)] rounded-lg p-2 text-sm text-center focus:border-[var(--color-accent)] focus:outline-none" />
          </div>
          <div class="flex-1">
            <label class="text-[10px] uppercase text-gray-500 font-bold ml-1">Selesai</label>
            <input type="text" placeholder="01:00" class="w-full bg-black/30 border border-[var(--color-subtle)] rounded-lg p-2 text-sm text-center focus:border-[var(--color-accent)] focus:outline-none" />
          </div>
        </div>
        <GlowButton class="w-full py-1.5 mt-2 text-xs">Tambahkan Segmen</GlowButton>
      </div>
      
      <!-- TAB AI -->
      <div v-else-if="scanMode === 'ai'" class="flex-1 flex flex-col relative">
        <div v-if="videoStore.isScanningAI" class="absolute inset-0 z-10 bg-[var(--color-surface)]/80 backdrop-blur-sm flex flex-col items-center justify-center">
          <IconLoader class="w-6 h-6 animate-spin text-[var(--color-accent)] mb-2" />
          <span class="text-xs text-gray-400">Menganalisis AI...</span>
        </div>

        <div v-if="!videoStore.metadata?.ai_segments || videoStore.metadata.ai_segments.length === 0" class="flex-1 flex flex-col items-center justify-center text-center gap-3 opacity-80 py-6">
          <IconSparkles class="w-8 h-8 text-yellow-400" />
          <p class="text-xs text-gray-300 px-4">Klik <b>Scan AI</b> untuk membiarkan LLM mencari momen viral (butuh waktu lebih lama).</p>
          <GlowButton @click="handleScanAI" :disabled="!videoStore.metadata" class="py-1 px-4 text-xs">Jalankan AI Scan</GlowButton>
        </div>

        <div v-else class="flex-1 overflow-y-auto custom-scrollbar flex flex-col gap-2">
          <label v-for="(segment, idx) in videoStore.metadata.ai_segments" :key="idx" class="flex items-start gap-3 p-3 rounded-lg border border-[var(--color-subtle)] bg-black/20 hover:bg-black/40 cursor-pointer group transition-colors">
            <div class="pt-0.5">
              <div class="relative w-4 h-4">
                <input type="checkbox" checked class="peer sr-only" />
                <div class="w-4 h-4 border-2 border-gray-500 rounded peer-checked:bg-[var(--color-accent)] peer-checked:border-[var(--color-accent)] transition-all flex items-center justify-center">
                  <IconCheck class="w-3 h-3 text-black opacity-0 peer-checked:opacity-100" />
                </div>
              </div>
            </div>
            <div class="flex-1 min-w-0">
              <div class="flex justify-between items-center mb-1">
                <span class="text-xs font-bold text-white group-hover:text-yellow-400 transition-colors">AI Klip #{{ idx + 1 }}</span>
                <span class="text-[10px] font-mono text-gray-400 bg-white/5 px-1.5 rounded border border-white/10">
                  {{ formatDuration(segment.start) }} - {{ formatDuration(segment.end) }}
                </span>
              </div>
              <div class="text-[10px] text-gray-500 line-clamp-2">{{ segment.reason || 'Momen menarik' }}</div>
            </div>
          </label>
        </div>
      </div>

      <!-- TAB HEATMAP -->
      <div v-else class="flex-1 flex flex-col relative">
        <div v-if="videoStore.isScanning" class="absolute inset-0 z-10 bg-[var(--color-surface)]/80 backdrop-blur-sm flex flex-col items-center justify-center">
          <IconLoader class="w-6 h-6 animate-spin text-[var(--color-accent)] mb-2" />
          <span class="text-xs text-gray-400">Mencari momen...</span>
        </div>
        
        <div v-if="!videoStore.metadata?.segments || videoStore.metadata.segments.length === 0" class="flex-1 flex flex-col items-center justify-center text-center gap-3 opacity-80 py-6">
          <IconTrending class="w-8 h-8 text-[var(--color-accent)]" />
          <p class="text-xs text-gray-300 px-4">Klik <b>Scan Heatmap</b> untuk menganalisis retensi penonton dan mendapatkan klip terbaik.</p>
          <GlowButton @click="handleScanHeatmap" :disabled="!videoStore.metadata" class="py-1 px-4 text-xs">Jalankan Scan Heatmap</GlowButton>
        </div>

        <div v-else class="flex-1 overflow-y-auto custom-scrollbar flex flex-col gap-2">
          <label v-for="(segment, idx) in videoStore.metadata.segments" :key="idx" class="flex items-start gap-3 p-3 rounded-lg border border-[var(--color-subtle)] bg-black/20 hover:bg-black/40 cursor-pointer group transition-colors">
            <div class="pt-0.5">
              <div class="relative w-4 h-4">
                <input type="checkbox" checked class="peer sr-only" />
                <div class="w-4 h-4 border-2 border-gray-500 rounded peer-checked:bg-[var(--color-accent)] peer-checked:border-[var(--color-accent)] transition-all flex items-center justify-center">
                  <IconCheck class="w-3 h-3 text-black opacity-0 peer-checked:opacity-100" />
                </div>
              </div>
            </div>
            <div class="flex-1 min-w-0">
              <div class="flex justify-between items-center mb-1">
                <span class="text-xs font-bold text-white group-hover:text-[var(--color-accent)] transition-colors">Klip #{{ idx + 1 }}</span>
                <span class="text-[10px] font-mono text-gray-400 bg-white/5 px-1.5 rounded border border-white/10">
                  {{ formatDuration(segment.start) }} - {{ formatDuration(segment.end) }}
                </span>
              </div>
              <div class="flex justify-between items-center">
                <span class="text-[10px] text-gray-500">Durasi: {{ Math.round(segment.end - segment.start) }}s</span>
                <div class="flex items-center gap-1">
                  <div class="w-16 h-1.5 bg-gray-800 rounded-full overflow-hidden">
                    <div class="h-full bg-[var(--color-accent)]" :style="`width: ${Math.min(100, (segment.score || 0.5) * 100)}%`"></div>
                  </div>
                </div>
              </div>
            </div>
          </label>
        </div>
      </div>
    </BentoCard>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { useVideoStore } from '../../stores/video';
import BentoCard from '../BentoCard.vue';
import GlowButton from '../GlowButton.vue';

// Icons
import IconYoutube from '~icons/lucide/youtube';
import IconLoader from '~icons/lucide/loader-2';
import IconList from '~icons/lucide/list';
import IconCheck from '~icons/lucide/check';
import IconSparkles from '~icons/lucide/sparkles';
import IconTrending from '~icons/lucide/trending-up';

const videoStore = useVideoStore();

const videoUrl = defineModel('videoUrl', { type: String, default: '' });
const scanMode = defineModel('scanMode', { type: String, default: 'heatmap' });

const emit = defineEmits(['load-video', 'scan-heatmap', 'scan-ai']);

const handleLoadVideo = () => emit('load-video');
const handleScanHeatmap = () => emit('scan-heatmap');
const handleScanAI = () => emit('scan-ai');

onMounted(async () => {
  try {
    const text = await navigator.clipboard.readText();
    if (text.includes('youtube.com/watch') || text.includes('youtu.be/')) {
      videoUrl.value = text;
    }
  } catch (err) {
    // Ignore clipboard errors
  }
});

const formatDuration = (seconds: number) => {
  if (!seconds) return '0:00';
  const h = Math.floor(seconds / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  const s = Math.floor(seconds % 60);
  if (h > 0) return `${h}:${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`;
  return `${m}:${s.toString().padStart(2, '0')}`;
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
