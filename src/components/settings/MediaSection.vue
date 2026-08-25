<template>
  <div class="flex flex-col gap-6">
    <!-- Pengaturan Editing -->
    <section class="flex flex-col gap-3">
      <h2 class="text-sm font-bold text-gray-400 uppercase tracking-widest flex items-center gap-2">
        <IconScissors class="w-4 h-4 text-[var(--color-accent)]" /> Standar Pemotongan
      </h2>
      <BentoCard class="p-5 flex flex-col gap-4">
        <div class="flex flex-col gap-1">
           <div class="flex justify-between">
             <span class="text-[10px] text-gray-400 uppercase font-bold">Durasi Minimal Klip</span>
             <span class="text-[10px] text-white font-mono">{{ settings.config.min_duration }} Detik</span>
           </div>
           <input type="range" min="10" max="600" step="10" v-model.number="settings.config.min_duration" class="w-full h-1 bg-[var(--color-subtle)] rounded-lg appearance-none cursor-pointer accent-[var(--color-accent)] mt-1" />
        </div>
        <div class="flex flex-col gap-1">
           <div class="flex justify-between">
             <span class="text-[10px] text-gray-400 uppercase font-bold">Padding Waktu Klip</span>
             <span class="text-[10px] text-white font-mono">{{ settings.config.padding }} Detik</span>
           </div>
           <input type="range" min="-10" max="30" step="1" v-model.number="settings.config.padding" class="w-full h-1 bg-[var(--color-subtle)] rounded-lg appearance-none cursor-pointer accent-[var(--color-accent)] mt-1" />
        </div>
      </BentoCard>
    </section>

    <!-- TTS Voice -->
    <section class="flex flex-col gap-3">
      <h2 class="text-sm font-bold text-gray-400 uppercase tracking-widest flex items-center gap-2">
        <IconMic class="w-4 h-4 text-[var(--color-accent)]" /> Text-to-Speech (AI Voice)
      </h2>
      <BentoCard class="p-5 flex flex-col gap-3">
        <div class="grid grid-cols-2 gap-3">
          <div class="flex flex-col gap-1">
            <span class="text-[10px] text-gray-400 uppercase font-bold">Bahasa Utama</span>
            <select v-model="settings.config.tts_language" class="w-full bg-black/50 border border-[var(--color-subtle)] rounded p-2 text-xs text-white focus:outline-none focus:border-[var(--color-accent)]">
              <option value="default">Deteksi Otomatis</option>
              <option value="id">Indonesia</option>
              <option value="en">English</option>
            </select>
          </div>
          <div class="flex flex-col gap-1">
            <span class="text-[10px] text-gray-400 uppercase font-bold">Karakter Suara</span>
            <select v-model="settings.config.tts_voice" class="w-full bg-black/50 border border-[var(--color-subtle)] rounded p-2 text-xs text-white focus:outline-none focus:border-[var(--color-accent)]">
              <option value="female">Wanita</option>
              <option value="male">Pria</option>
            </select>
          </div>
        </div>
      </BentoCard>
    </section>

    <!-- Aset Media (Intro/Outro/Watermark) -->
    <section class="flex flex-col gap-3">
      <h2 class="text-sm font-bold text-gray-400 uppercase tracking-widest flex items-center gap-2">
        <IconImage class="w-4 h-4 text-[var(--color-accent)]" /> Branding & Aset Dasar
      </h2>
      <BentoCard class="p-5 flex flex-col gap-3">
        <!-- Asset Pickers (UI Mock) -->
        <div class="flex items-center justify-between p-2 bg-black/30 border border-[var(--color-subtle)] rounded hover:border-gray-500 transition-colors cursor-pointer group">
          <div class="flex flex-col">
            <span class="text-xs font-bold text-white group-hover:text-[var(--color-accent)] transition-colors">Video Intro</span>
            <span class="text-[9px] text-gray-500">{{ settings.config.intro_video || 'Belum di-set' }}</span>
          </div>
          <IconUpload class="w-4 h-4 text-gray-400" />
        </div>
        
        <div class="flex items-center justify-between p-2 bg-black/30 border border-[var(--color-subtle)] rounded hover:border-gray-500 transition-colors cursor-pointer group">
          <div class="flex flex-col">
            <span class="text-xs font-bold text-white group-hover:text-[var(--color-accent)] transition-colors">Video Outro</span>
            <span class="text-[9px] text-gray-500">{{ settings.config.outro_video || 'Belum di-set' }}</span>
          </div>
          <IconUpload class="w-4 h-4 text-gray-400" />
        </div>

        <div class="flex items-center justify-between p-2 bg-black/30 border border-[var(--color-subtle)] rounded hover:border-gray-500 transition-colors cursor-pointer group">
          <div class="flex flex-col">
            <span class="text-xs font-bold text-white group-hover:text-[var(--color-accent)] transition-colors">Gambar Watermark</span>
            <span class="text-[9px] text-gray-500">{{ settings.config.watermark_image || 'Belum di-set' }}</span>
          </div>
          <IconUpload class="w-4 h-4 text-gray-400" />
        </div>

        <div class="flex items-center justify-between p-2 bg-black/30 border border-[var(--color-subtle)] rounded hover:border-gray-500 transition-colors cursor-pointer group">
          <div class="flex flex-col">
            <span class="text-xs font-bold text-white group-hover:text-[var(--color-accent)] transition-colors">Background Frame</span>
            <span class="text-[9px] text-gray-500">{{ settings.config.video_frame || 'Belum di-set' }}</span>
          </div>
          <IconUpload class="w-4 h-4 text-gray-400" />
        </div>
        <p class="text-[9px] text-gray-500 mt-1">Posisi watermark kini dapat diatur secara real-time di halaman Studio.</p>
      </BentoCard>
    </section>
  </div>
</template>

<script setup lang="ts">
import { useSettingsStore } from '../../stores/settings';
import BentoCard from '../BentoCard.vue';

// Icons
import IconScissors from '~icons/lucide/scissors';
import IconMic from '~icons/lucide/mic';
import IconImage from '~icons/lucide/image';
import IconUpload from '~icons/lucide/upload';

const settings = useSettingsStore();
</script>
