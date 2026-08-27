<template>
  <div class="flex flex-col gap-6">
    <!-- Dependensi Sistem -->
  <BentoCard class="p-6 flex flex-col gap-5">
      <h2 class="text-lg font-black text-gray-900 dark:text-gray-100 tracking-wide flex items-center gap-2">
        <IconPackage class="w-5 h-5" /> Dependensi Eksternal
      </h2>
      <!-- Dependency Status -->
      <div class="flex flex-col gap-3">
        <div class="flex justify-between items-center bg-white/50 dark:bg-black/30 p-3 rounded-2xl border-none">
          <div class="flex flex-col">
            <span class="text-sm font-bold text-gray-900 dark:text-gray-100">FFmpeg</span>
            <span class="text-xs font-bold" :class="depsStatus.ffmpeg_installed ? 'text-gray-700 dark:text-gray-300' : 'text-red-500'">{{ depsStatus.ffmpeg_version }}</span>
          </div>
          <IconCheckCircle v-if="depsStatus.ffmpeg_installed" class="w-5 h-5 text-gray-600 dark:text-gray-400" />
          <IconXCircle v-else class="w-5 h-5 text-red-500" />
        </div>

        <div class="flex justify-between items-center bg-white/50 dark:bg-black/30 p-3 rounded-2xl border-none">
          <div class="flex flex-col">
            <span class="text-sm font-bold text-gray-900 dark:text-gray-100">Deno</span>
            <span class="text-xs font-bold" :class="depsStatus.deno_installed ? 'text-gray-700 dark:text-gray-300' : 'text-red-500'">{{ depsStatus.deno_version }}</span>
          </div>
          <IconCheckCircle v-if="depsStatus.deno_installed" class="w-5 h-5 text-gray-600 dark:text-gray-400" />
          <IconXCircle v-else class="w-5 h-5 text-red-500" />
        </div>
      </div>

      <div v-if="isInstallingDeps" class="flex flex-col gap-2">
        <div class="flex justify-between text-[10px] text-gray-900 dark:text-gray-300 font-bold">
          <span>{{ installProgressText }}</span>
          <span>{{ Math.round(installProgressPercent) }}%</span>
        </div>
        <div class="w-full h-2 bg-gray-300 dark:bg-gray-800 rounded-full overflow-hidden">
          <div class="h-full bg-gray-600 dark:bg-gray-400 transition-all duration-300" :style="`width: ${installProgressPercent}%`"></div>
        </div>
      </div>

      <button @click="runInstallDeps" :disabled="isInstallingDeps" class="w-full py-3 rounded-full text-xs font-bold transition-colors disabled:opacity-50 disabled:cursor-not-allowed shadow-sm bg-black text-white dark:bg-white dark:text-black hover:bg-gray-800 dark:hover:bg-gray-200">
        <span v-if="isInstallingDeps" class="flex items-center justify-center gap-2">
          <IconLoader class="w-4 h-4 animate-spin" /> Menginstal...
        </span>
        <span v-else>Jalankan Instalasi Otomatis</span>
      </button>
    </BentoCard>

    <!-- Engine & API -->
  <BentoCard class="p-6 flex flex-col gap-5">
      <h2 class="text-lg font-black text-gray-900 dark:text-gray-100 tracking-wide flex items-center gap-2">
        <IconCpu class="w-5 h-5" /> Engine & Hardware
      </h2>

      <!-- Hardware Accel -->
      <div class="flex flex-col gap-3">
        <span class="text-xs font-bold text-gray-900 dark:text-gray-100">Akselerasi Rendering (FFmpeg)</span>
        <div class="grid grid-cols-2 gap-2">
          <button class="py-3 px-2 rounded-2xl transition-all text-xs font-bold text-center disabled:opacity-50 disabled:cursor-not-allowed bg-black text-white dark:bg-white dark:text-black hover:bg-gray-800 dark:hover:bg-gray-200" :disabled="!availableAccels.includes('cpu')" :class="settings.config.hw_accel === 'cpu' ? ' shadow-sm' : 'bg-white/50 hover:bg-white/80 dark:bg-black/30 dark:' bg-black text-white dark:bg-white dark:text-black hover:bg-gray-800 dark:hover:bg-gray-200" @click="settings.config.hw_accel = 'cpu'">
            CPU
          </button>
          <button class="py-3 px-2 rounded-2xl transition-all text-xs font-bold text-center disabled:opacity-50 disabled:cursor-not-allowed bg-black text-white dark:bg-white dark:text-black hover:bg-gray-800 dark:hover:bg-gray-200" :disabled="!availableAccels.includes('mac')" :class="settings.config.hw_accel === 'mac' ? ' shadow-sm' : 'bg-white/50 hover:bg-white/80 dark:bg-black/30 dark:' bg-black text-white dark:bg-white dark:text-black hover:bg-gray-800 dark:hover:bg-gray-200" @click="settings.config.hw_accel = 'mac'">
            Mac (VideoToolbox)
          </button>
          <button class="py-3 px-2 rounded-2xl transition-all text-xs font-bold text-center disabled:opacity-50 disabled:cursor-not-allowed bg-black text-white dark:bg-white dark:text-black hover:bg-gray-800 dark:hover:bg-gray-200" :disabled="!availableAccels.includes('nvidia')" :class="settings.config.hw_accel === 'nvidia' ? ' shadow-sm' : 'bg-white/50 hover:bg-white/80 dark:bg-black/30 dark:' bg-black text-white dark:bg-white dark:text-black hover:bg-gray-800 dark:hover:bg-gray-200" @click="settings.config.hw_accel = 'nvidia'">
            NVIDIA NVENC
          </button>
          <button class="py-3 px-2 rounded-2xl transition-all text-xs font-bold text-center disabled:opacity-50 disabled:cursor-not-allowed bg-black text-white dark:bg-white dark:text-black hover:bg-gray-800 dark:hover:bg-gray-200" :disabled="!availableAccels.includes('amd')" :class="settings.config.hw_accel === 'amd' ? ' shadow-sm' : 'bg-white/50 hover:bg-white/80 dark:bg-black/30 dark:' bg-black text-white dark:bg-white dark:text-black hover:bg-gray-800 dark:hover:bg-gray-200" @click="settings.config.hw_accel = 'amd'">
            AMD AMF
          </button>
        </div>
        <span class="text-[10px] text-gray-700 dark:text-gray-300 font-medium" v-if="isLoadingAccels">Mendeteksi hardware yang tersedia...</span>
      </div>

      <!-- Threads -->
      <div class="flex flex-col gap-2 mt-2">
        <div class="flex justify-between items-center">
          <span class="text-xs font-bold text-gray-900 dark:text-gray-100">Maksimum Worker/Thread</span>
          <span class="text-xs font-black bg-white/60 dark:bg-black/30 px-3 py-1 rounded-full text-gray-900 dark:text-gray-100">{{ settings.config.max_workers }}</span>
        </div>
        <input type="range" min="1" max="16" v-model.number="settings.config.max_workers" class="w-full h-2 bg-gray-300 dark:bg-gray-800 rounded-lg appearance-none cursor-pointer mt-2" />
      </div>

      <!-- API Keys -->
      <div class="flex flex-col gap-3 pt-5 border-t border-gray-300 dark:border-gray-800 mt-2">
        <span class="text-xs font-bold text-gray-900 dark:text-gray-100">API Keys (AI Analytics)</span>
        <div class="relative group">
          <IconKey class="absolute left-4 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-700/60 dark:text-gray-300/60" />
          <input type="password" v-model="settings.config.ai.openai_key" placeholder="OpenAI Key (sk-...)" class="w-full bg-white/60 dark:bg-black/30 border-none rounded-2xl py-3 pl-11 pr-4 text-sm font-bold text-gray-900 dark:text-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-500 placeholder-gray-700/60 dark:placeholder-gray-300/60 transition-all" />
        </div>
        <div class="relative group">
          <IconSparkles class="absolute left-4 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-700/60 dark:text-gray-300/60" />
          <input type="password" v-model="settings.config.ai.gemini_key" placeholder="Gemini API Key" class="w-full bg-white/60 dark:bg-black/30 border-none rounded-2xl py-3 pl-11 pr-4 text-sm font-bold text-gray-900 dark:text-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-500 placeholder-gray-700/60 dark:placeholder-gray-300/60 transition-all" />
        </div>
      </div>
    </BentoCard>

    <!-- Model Whisper -->
  <BentoCard class="p-6 flex flex-col gap-4">
      <h2 class="text-lg font-black text-gray-900 dark:text-gray-100 tracking-wide flex items-center gap-2">
        <IconType class="w-5 h-5" /> Engine Transkripsi
      </h2>
      <div class="flex flex-col gap-3">
        <span class="text-xs text-gray-900 dark:text-gray-100 font-bold">Model Whisper Default</span>
        <select v-model="settings.config.subtitle.whisper_model" class="w-full bg-white/60 dark:bg-black/30 border-none rounded-2xl p-3 text-sm font-bold text-gray-900 dark:text-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-500 transition-all">
          <option value="tiny">Tiny (Cepat, Kurang Akurat)</option>
          <option value="base">Base</option>
          <option value="small">Small (Rekomendasi)</option>
          <option value="medium">Medium</option>
          <option value="large-v3">Large v3 (Paling Akurat)</option>
          <option value="large-v3-turbo">Large v3 Turbo</option>
        </select>
        <p class="text-[10px] text-gray-700 dark:text-gray-300 font-medium mt-1 leading-relaxed">Pengaturan tampilan subtitle (font, warna, ukuran) telah dipindahkan ke menu Studio agar Anda bisa melihat perubahannya secara real-time.</p>
      </div>
    </BentoCard>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useSettingsStore } from '../../stores/settings';
import BentoCard from '../BentoCard.vue';

// Icons
import IconCpu from '~icons/lucide/cpu';
import IconKey from '~icons/lucide/key';
import IconSparkles from '~icons/lucide/sparkles';
import IconType from '~icons/lucide/type';
import IconPackage from '~icons/lucide/package';
import IconCheckCircle from '~icons/lucide/check-circle-2';
import IconXCircle from '~icons/lucide/x-circle';
import IconLoader from '~icons/lucide/loader-2';

const settings = useSettingsStore();
const availableAccels = ref<string[]>(['cpu']);
const isLoadingAccels = ref(true);

const depsStatus = ref({
  ffmpeg_installed: false,
  ffmpeg_version: 'Memeriksa...',
  deno_installed: false,
  deno_version: 'Memeriksa...'
});
const isInstallingDeps = ref(false);
const installProgressText = ref('');
const installProgressPercent = ref(0);

let unlistenDepsProgress: any = null;

const checkDeps = async () => {
  try {
    const status: any = await invoke('check_dependencies');
    depsStatus.value = status;
  } catch (e) {
    console.error("Gagal memeriksa dependensi:", e);
  }
};

const runInstallDeps = async () => {
  isInstallingDeps.value = true;
  installProgressText.value = 'Menyiapkan instalasi...';
  installProgressPercent.value = 0;
  try {
    await invoke('install_dependencies');
    await checkDeps();
  } catch (e) {
    console.error("Instalasi gagal:", e);
    installProgressText.value = 'Instalasi Gagal!';
  } finally {
    isInstallingDeps.value = false;
  }
};

onMounted(async () => {
  checkDeps();

  unlistenDepsProgress = await listen('deps-progress', (event: any) => {
    installProgressText.value = event.payload.step;
    installProgressPercent.value = event.payload.progress;
  });

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

onUnmounted(() => {
  if (unlistenDepsProgress) unlistenDepsProgress();
});
</script>
