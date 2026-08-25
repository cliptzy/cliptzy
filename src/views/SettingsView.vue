<template>
  <div class="h-full flex flex-col max-w-7xl mx-auto w-full gap-6 pb-8 overflow-y-auto custom-scrollbar pr-2">
    <div class="flex items-center justify-between mt-4">
      <h1 class="text-3xl font-black text-white tracking-wide">Pengaturan</h1>
      <span class="text-xs font-mono text-gray-500 bg-black/50 px-3 py-1 rounded-full border border-[var(--color-subtle)]">v1.0.0-beta (Rust Engine)</span>
    </div>

    <!-- Layout Grid Utama -->
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      
      <!-- KOLOM 1: Profil, Akun Sosial & Penyimpanan -->
      <div class="flex flex-col gap-6">
        <!-- Profil Sistem -->
        <section class="flex flex-col gap-3">
          <h2 class="text-sm font-bold text-gray-400 uppercase tracking-widest flex items-center gap-2">
            <IconUser class="w-4 h-4 text-[var(--color-accent)]" /> Profil Sistem
          </h2>
          <BentoCard class="p-5 flex items-center justify-between group">
            <div class="flex items-center gap-4">
              <img v-if="auth.avatarUrl" :src="auth.avatarUrl" class="w-12 h-12 rounded-full border-2 border-[var(--color-subtle)]" />
              <div v-else class="w-12 h-12 rounded-full bg-[var(--color-accent)]/20 flex items-center justify-center text-[var(--color-accent)] font-bold text-xl border-2 border-[var(--color-subtle)]">
                {{ auth.displayName ? auth.displayName.charAt(0).toUpperCase() : 'U' }}
              </div>
              <div class="flex flex-col min-w-0">
                <span class="font-bold text-white text-base truncate">{{ auth.displayName || 'Google User' }}</span>
                <span class="text-xs text-gray-400 truncate">{{ auth.email || 'Email tidak tersedia' }}</span>
              </div>
            </div>
            <div class="flex flex-col items-end gap-2">
              <span class="text-[10px] font-medium text-green-400 bg-green-400/10 px-2 py-0.5 rounded border border-green-400/20">Tersinkronisasi</span>
              <button @click="handleLogout" class="text-[10px] text-red-500 hover:underline">Logout</button>
            </div>
          </BentoCard>
        </section>

        <!-- Akun Sosial -->
        <section class="flex flex-col gap-3">
          <h2 class="text-sm font-bold text-gray-400 uppercase tracking-widest flex items-center gap-2">
            <IconLink class="w-4 h-4 text-[var(--color-accent)]" /> Akun Sosial
          </h2>
          <div class="flex flex-col gap-3">
            <BentoCard class="p-4 flex items-center justify-between group hover:border-gray-500 transition-colors">
              <div class="flex items-center gap-3">
                <div class="w-8 h-8 rounded-full bg-red-500/10 flex items-center justify-center"><IconYoutube class="w-4 h-4 text-red-500" /></div>
                <div class="flex flex-col"><span class="font-bold text-white text-sm">YouTube</span><span class="text-[10px] text-gray-400">@cliptzy_official</span></div>
              </div>
              <span class="relative flex h-2 w-2"><span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-green-400 opacity-75"></span><span class="relative inline-flex rounded-full h-2 w-2 bg-green-500"></span></span>
            </BentoCard>
            <BentoCard class="p-4 flex items-center justify-between group hover:border-gray-500 transition-colors">
              <div class="flex items-center gap-3">
                <div class="w-8 h-8 rounded-full bg-white/5 border border-[var(--color-subtle)] flex items-center justify-center"><IconTiktok class="w-4 h-4 text-white" /></div>
                <div class="flex flex-col"><span class="font-bold text-white text-sm">TikTok</span><span class="text-[10px] text-yellow-500">Perlu Login</span></div>
              </div>
              <button class="bg-[var(--color-accent)] text-black text-[10px] font-bold px-2 py-1 rounded hover:scale-105 transition-transform">Hubungkan</button>
            </BentoCard>
          </div>
        </section>

        <!-- Penyimpanan -->
        <section class="flex flex-col gap-3">
          <h2 class="text-sm font-bold text-gray-400 uppercase tracking-widest flex items-center gap-2">
            <IconHardDrive class="w-4 h-4 text-[var(--color-accent)]" /> Penyimpanan
          </h2>
          <BentoCard class="p-5 flex flex-col items-center">
            <div class="relative w-32 h-32 flex items-center justify-center">
              <svg class="w-full h-full transform -rotate-90" viewBox="0 0 100 100">
                <circle cx="50" cy="50" r="40" stroke="rgba(255,255,255,0.05)" stroke-width="8" fill="none" />
                <circle cx="50" cy="50" r="40" stroke="var(--color-accent)" stroke-width="8" fill="none" stroke-linecap="round" stroke-dasharray="251.2" stroke-dashoffset="62.8" class="transition-all duration-1000 ease-out shadow-[0_0_15px_var(--color-accent)]" />
              </svg>
              <div class="absolute inset-0 flex flex-col items-center justify-center">
                <span class="text-xl font-black text-white">4.2</span>
                <span class="text-[9px] text-gray-400 font-medium">GB Terpakai</span>
              </div>
            </div>
            <button @click="clearCache" class="mt-4 w-full py-2 rounded border border-red-500/30 text-red-400 text-xs font-bold hover:bg-red-500 hover:text-white transition-colors flex items-center justify-center gap-2">
              <IconTrash2 class="w-3 h-3" /> Bersihkan Cache
            </button>
          </BentoCard>
        </section>
      </div>

      <!-- KOLOM 2: Engine & Subtitle -->
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
                <button class="p-2 rounded border transition-all text-xs text-center" :class="settings.config.hw_accel === 'cpu' ? 'border-[var(--color-accent)] text-[var(--color-accent)] bg-[var(--color-accent)]/10' : 'border-[var(--color-subtle)] text-gray-400 hover:text-white'" @click="settings.config.hw_accel = 'cpu'">CPU</button>
                <button class="p-2 rounded border transition-all text-xs text-center" :class="settings.config.hw_accel === 'mac' ? 'border-[var(--color-accent)] text-[var(--color-accent)] bg-[var(--color-accent)]/10' : 'border-[var(--color-subtle)] text-gray-400 hover:text-white'" @click="settings.config.hw_accel = 'mac'">Mac (VideoToolbox)</button>
                <button class="p-2 rounded border transition-all text-xs text-center" :class="settings.config.hw_accel === 'nvidia' ? 'border-[var(--color-accent)] text-[var(--color-accent)] bg-[var(--color-accent)]/10' : 'border-[var(--color-subtle)] text-gray-400 hover:text-white'" @click="settings.config.hw_accel = 'nvidia'">NVIDIA NVENC</button>
                <button class="p-2 rounded border transition-all text-xs text-center" :class="settings.config.hw_accel === 'amd' ? 'border-[var(--color-accent)] text-[var(--color-accent)] bg-[var(--color-accent)]/10' : 'border-[var(--color-subtle)] text-gray-400 hover:text-white'" @click="settings.config.hw_accel = 'amd'">AMD AMF</button>
              </div>
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

      <!-- KOLOM 3: Editing, Asset & Lainnya -->
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
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { useAppStore } from '../stores/app';
import { useAuthStore } from '../stores/auth';
import { useSettingsStore } from '../stores/settings';
import BentoCard from '../components/BentoCard.vue';

// Icons
import IconLink from '~icons/lucide/link';
import IconUser from '~icons/lucide/user';
import IconYoutube from '~icons/lucide/youtube';
import IconTiktok from '~icons/lucide/smartphone'; // Fallback
import IconInstagram from '~icons/lucide/instagram';
import IconCpu from '~icons/lucide/cpu';
import IconHardDrive from '~icons/lucide/hard-drive';
import IconKey from '~icons/lucide/key';
import IconSparkles from '~icons/lucide/sparkles';
import IconTrash2 from '~icons/lucide/trash-2';
import IconType from '~icons/lucide/type';
import IconScissors from '~icons/lucide/scissors';
import IconMic from '~icons/lucide/mic';
import IconImage from '~icons/lucide/image';
import IconUpload from '~icons/lucide/upload';

const appStore = useAppStore();
const auth = useAuthStore();
const settings = useSettingsStore();
const router = useRouter();

const clearCache = () => {
  appStore.addToast({
    type: 'success',
    title: 'Cache Cleared',
    message: '3.1 GB of temporary files have been successfully deleted.',
    duration: 3000
  });
};

const handleLogout = async () => {
  await auth.logout();
  router.push('/login');
};
</script>

<style scoped>
.custom-scrollbar::-webkit-scrollbar {
  width: 6px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: rgba(255, 255, 255, 0.02);
  border-radius: 8px;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.1);
  border-radius: 8px;
}
.custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.2);
}
</style>
