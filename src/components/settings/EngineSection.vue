<template>
  <div class="flex flex-col gap-6">
    <!-- Engine & API -->
    <section class="flex flex-col gap-3">
      <h2 class="text-sm font-bold text-gray-400 uppercase tracking-widest flex items-center gap-2">
        <IconCpu class="w-4 h-4 text-[var(--color-accent)]" /> Engine & Hardware
      </h2>
      <BentoCard class="p-5 flex flex-col gap-5">
        <!-- Hardware Accel -->
        <div class="flex flex-col gap-2">
          <span class="text-xs font-semibold text-white">Akselerasi Rendering (FFmpeg)</span>
          <div class="grid grid-cols-2 gap-2">
            <button 
              class="p-2 rounded border transition-all text-xs text-center disabled:opacity-50 disabled:cursor-not-allowed" 
              :disabled="!availableAccels.includes('cpu')"
              :class="settings.config.hw_accel === 'cpu' ? 'border-[var(--color-accent)] text-[var(--color-accent)] bg-[var(--color-accent)]/10' : 'border-[var(--color-subtle)] text-gray-400 hover:text-white'" 
              @click="settings.config.hw_accel = 'cpu'">
              CPU
            </button>
            <button 
              class="p-2 rounded border transition-all text-xs text-center disabled:opacity-50 disabled:cursor-not-allowed" 
              :disabled="!availableAccels.includes('mac')"
              :class="settings.config.hw_accel === 'mac' ? 'border-[var(--color-accent)] text-[var(--color-accent)] bg-[var(--color-accent)]/10' : 'border-[var(--color-subtle)] text-gray-400 hover:text-white'" 
              @click="settings.config.hw_accel = 'mac'">
              Mac (VideoToolbox)
            </button>
            <button 
              class="p-2 rounded border transition-all text-xs text-center disabled:opacity-50 disabled:cursor-not-allowed" 
              :disabled="!availableAccels.includes('nvidia')"
              :class="settings.config.hw_accel === 'nvidia' ? 'border-[var(--color-accent)] text-[var(--color-accent)] bg-[var(--color-accent)]/10' : 'border-[var(--color-subtle)] text-gray-400 hover:text-white'" 
              @click="settings.config.hw_accel = 'nvidia'">
              NVIDIA NVENC
            </button>
            <button 
              class="p-2 rounded border transition-all text-xs text-center disabled:opacity-50 disabled:cursor-not-allowed" 
              :disabled="!availableAccels.includes('amd')"
              :class="settings.config.hw_accel === 'amd' ? 'border-[var(--color-accent)] text-[var(--color-accent)] bg-[var(--color-accent)]/10' : 'border-[var(--color-subtle)] text-gray-400 hover:text-white'" 
              @click="settings.config.hw_accel = 'amd'">
              AMD AMF
            </button>
          </div>
          <span class="text-[9px] text-gray-500 mt-1" v-if="isLoadingAccels">Mendeteksi hardware yang tersedia...</span>
        </div>
        <!-- Threads -->
        <div class="flex flex-col gap-2">
          <div class="flex justify-between items-center">
            <span class="text-xs font-semibold text-white">Maksimum Worker/Thread</span>
            <span class="text-[10px] font-mono text-[var(--color-accent)]">{{ settings.config.max_workers }}</span>
          </div>
          <input type="range" min="1" max="16" v-model.number="settings.config.max_workers" class="w-full h-1 bg-[var(--color-subtle)] rounded-lg appearance-none cursor-pointer accent-[var(--color-accent)]" />
        </div>
        <!-- API Keys -->
        <div class="flex flex-col gap-2 pt-3 border-t border-[var(--color-subtle)]">
          <span class="text-xs font-semibold text-white">API Keys (AI Analytics)</span>
          <div class="relative group">
            <IconKey class="absolute left-3 top-1/2 -translate-y-1/2 w-3 h-3 text-gray-500" />
            <input type="password" v-model="settings.config.ai.openai_key" placeholder="OpenAI Key (sk-...)" class="w-full bg-black/30 border border-[var(--color-subtle)] rounded py-1.5 pl-8 pr-3 text-xs text-white focus:outline-none focus:border-[var(--color-accent)]" />
          </div>
          <div class="relative group">
            <IconSparkles class="absolute left-3 top-1/2 -translate-y-1/2 w-3 h-3 text-gray-500" />
            <input type="password" v-model="settings.config.ai.gemini_key" placeholder="Gemini API Key" class="w-full bg-black/30 border border-[var(--color-subtle)] rounded py-1.5 pl-8 pr-3 text-xs text-white focus:outline-none focus:border-[var(--color-accent)]" />
          </div>
        </div>
      </BentoCard>
    </section>

    <!-- Model Whisper & AI Core Defaults -->
    <section class="flex flex-col gap-3">
      <h2 class="text-sm font-bold text-gray-400 uppercase tracking-widest flex items-center gap-2">
        <IconType class="w-4 h-4 text-[var(--color-accent)]" /> Engine Transkripsi
      </h2>
      <BentoCard class="p-5 flex flex-col gap-4">
        <!-- Model Whisper -->
        <div class="flex flex-col gap-1">
          <span class="text-[10px] text-gray-400 uppercase font-bold">Model Whisper Default</span>
          <select v-model="settings.config.subtitle.whisper_model" class="w-full bg-black/50 border border-[var(--color-subtle)] rounded p-2 text-xs text-white focus:outline-none focus:border-[var(--color-accent)]">
            <option value="tiny">Tiny (Cepat, Kurang Akurat)</option>
            <option value="base">Base</option>
            <option value="small">Small (Rekomendasi)</option>
            <option value="medium">Medium</option>
            <option value="large-v3">Large v3 (Paling Akurat)</option>
            <option value="large-v3-turbo">Large v3 Turbo</option>
          </select>
          <p class="text-[9px] text-gray-500 mt-1">Pengaturan tampilan subtitle (font, warna, ukuran) telah dipindahkan ke menu Studio agar Anda bisa melihat perubahannya secara real-time.</p>
        </div>
      </BentoCard>
    </section>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useSettingsStore } from '../../stores/settings';
import BentoCard from '../BentoCard.vue';

// Icons
import IconCpu from '~icons/lucide/cpu';
import IconKey from '~icons/lucide/key';
import IconSparkles from '~icons/lucide/sparkles';
import IconType from '~icons/lucide/type';

const settings = useSettingsStore();
const availableAccels = ref<string[]>(['cpu']);
const isLoadingAccels = ref(true);

onMounted(async () => {
  try {
    const accels = await invoke<string[]>('get_available_hwaccels');
    availableAccels.value = accels;
    
    // Auto fallback to CPU if current settings is not available
    if (!accels.includes(settings.config.hw_accel)) {
      settings.config.hw_accel = 'cpu';
    }
  } catch (e) {
    console.error("Gagal memeriksa hardware accel:", e);
  } finally {
    isLoadingAccels.value = false;
  }
});
</script>
