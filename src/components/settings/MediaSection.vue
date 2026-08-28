<template>
    <!-- Pengaturan Editing -->
  <BentoCard class="col-span-1 md:col-span-2 xl:col-span-2 row-span-1 h-full overflow-y-auto custom-scrollbar p-6 flex flex-col gap-5 !bg-cyan-100 dark:!bg-cyan-900/40">
      <h2 class="text-lg font-black text-[var(--color-text-main)] tracking-wide flex items-center gap-2 shrink-0">
        <IconScissors class="w-5 h-5" /> Standar Pemotongan
      </h2>
      <div class="flex flex-col gap-4 mt-auto">
        <div class="flex flex-col gap-2">
           <div class="flex justify-between items-center">
             <span class="text-xs font-bold text-[var(--color-text-main)]">Durasi Minimal Klip</span>
             <span class="text-xs font-black bg-[var(--color-accent)] text-white px-3 py-1 rounded-full shadow-sm">{{ settings.config.min_duration }} Detik</span>
           </div>
           <input type="range" min="10" max="600" step="10" v-model.number="settings.config.min_duration" class="w-full h-2 bg-gray-300 dark:bg-gray-800 rounded-lg appearance-none cursor-pointer mt-1 accent-[var(--color-accent)]" />
        </div>
        <div class="flex flex-col gap-2 pt-2 border-t border-gray-300 dark:border-gray-800">
           <div class="flex justify-between items-center">
             <span class="text-xs font-bold text-[var(--color-text-main)]">Padding Waktu Klip</span>
             <span class="text-xs font-black bg-[var(--color-accent)] text-white px-3 py-1 rounded-full shadow-sm">{{ settings.config.padding }} Detik</span>
           </div>
           <input type="range" min="-10" max="30" step="1" v-model.number="settings.config.padding" class="w-full h-2 bg-gray-300 dark:bg-gray-800 rounded-lg appearance-none cursor-pointer mt-1 accent-[var(--color-accent)]" />
        </div>
      </div>
    </BentoCard>

    <!-- TTS Voice -->
  <BentoCard class="col-span-1 md:col-span-1 xl:col-span-1 row-span-1 h-full overflow-y-auto custom-scrollbar p-6 flex flex-col gap-5 !bg-violet-100 dark:!bg-violet-900/40">
      <h2 class="text-lg font-black text-[var(--color-text-main)] tracking-wide flex items-center gap-2 shrink-0">
        <IconMic class="w-5 h-5" /> Text-to-Speech (AI Voice)
      </h2>
      <div class="flex flex-col gap-3 mt-auto">
        <div class="flex flex-col gap-1">
          <span class="text-[10px] text-[var(--color-text-main)] uppercase font-bold">Bahasa Utama</span>
          <select v-model="settings.config.tts_language" class="w-full bg-white/60 dark:bg-black/30 border-none rounded-2xl p-2.5 text-sm font-bold text-[var(--color-text-main)] focus:outline-none focus:ring-2 focus:ring-[var(--color-accent)] transition-colors cursor-pointer shadow-sm">
            <option value="default">Deteksi Otomatis</option>
            <option value="id">Indonesia</option>
            <option value="en">English</option>
          </select>
        </div>
        <div class="flex flex-col gap-1">
          <span class="text-[10px] text-[var(--color-text-main)] uppercase font-bold">Karakter Suara</span>
          <select v-model="settings.config.tts_voice" class="w-full bg-white/60 dark:bg-black/30 border-none rounded-2xl p-2.5 text-sm font-bold text-[var(--color-text-main)] focus:outline-none focus:ring-2 focus:ring-[var(--color-accent)] transition-colors cursor-pointer shadow-sm">
            <option value="female">Wanita</option>
            <option value="male">Pria</option>
          </select>
        </div>
      </div>
    </BentoCard>

    <!-- Aset Media (Intro/Outro/Watermark) -->
  <BentoCard class="col-span-1 md:col-span-2 xl:col-span-4 row-span-1 h-full overflow-y-auto custom-scrollbar p-6 flex flex-col gap-5 !bg-emerald-100 dark:!bg-emerald-900/40">
      <h2 class="text-lg font-black text-[var(--color-text-main)] tracking-wide flex items-center gap-2 shrink-0">
        <IconImage class="w-5 h-5" /> Branding & Aset Dasar
      </h2>
      <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-4 gap-3 mt-auto">
        <!-- Asset Pickers -->
        <div class="flex items-center gap-2">
          <div @click="selectAsset('intro_video')" class="flex-1 flex items-center justify-between p-3 bg-white/50 dark:bg-black/30 rounded-2xl hover:bg-white dark:hover:bg-black/50 transition-colors cursor-pointer group shadow-sm">
            <div class="flex flex-col">
              <span class="text-sm font-bold text-[var(--color-text-main)]">Video Intro</span>
              <span class="text-[10px] font-bold text-[var(--color-text-muted)] truncate max-w-[120px]" :title="settings.config.intro_video || ''">{{ settings.config.intro_video || 'Belum di-set' }}</span>
            </div>
            <IconUpload class="w-5 h-5 text-[var(--color-text-muted)] group-hover:scale-110 transition-transform" />
          </div>
          <button v-if="settings.config.intro_video" @click="clearAsset('intro_video')" class="p-3 bg-white/50 dark:bg-black/30 rounded-2xl hover:bg-red-500 hover:text-white dark:hover:bg-red-500 transition-colors group shadow-sm shrink-0" title="Hapus Video Intro">
            <IconTrash class="w-5 h-5 text-red-500 group-hover:text-white" />
          </button>
        </div>

        <div class="flex items-center gap-2">
          <div @click="selectAsset('outro_video')" class="flex-1 flex items-center justify-between p-3 bg-white/50 dark:bg-black/30 rounded-2xl hover:bg-white dark:hover:bg-black/50 transition-colors cursor-pointer group shadow-sm">
            <div class="flex flex-col">
              <span class="text-sm font-bold text-[var(--color-text-main)]">Video Outro</span>
              <span class="text-[10px] font-bold text-[var(--color-text-muted)] truncate max-w-[120px]" :title="settings.config.outro_video || ''">{{ settings.config.outro_video || 'Belum di-set' }}</span>
            </div>
            <IconUpload class="w-5 h-5 text-[var(--color-text-muted)] group-hover:scale-110 transition-transform" />
          </div>
          <button v-if="settings.config.outro_video" @click="clearAsset('outro_video')" class="p-3 bg-white/50 dark:bg-black/30 rounded-2xl hover:bg-red-500 hover:text-white dark:hover:bg-red-500 transition-colors group shadow-sm shrink-0" title="Hapus Video Outro">
            <IconTrash class="w-5 h-5 text-red-500 group-hover:text-white" />
          </button>
        </div>

        <div class="flex items-center gap-2">
          <div @click="selectAsset('watermark_image')" class="flex-1 flex items-center justify-between p-3 bg-white/50 dark:bg-black/30 rounded-2xl hover:bg-white dark:hover:bg-black/50 transition-colors cursor-pointer group shadow-sm">
            <div class="flex flex-col">
              <span class="text-sm font-bold text-[var(--color-text-main)]">Watermark</span>
              <span class="text-[10px] font-bold text-[var(--color-text-muted)] truncate max-w-[120px]" :title="settings.config.watermark_image || ''">{{ settings.config.watermark_image || 'Belum di-set' }}</span>
            </div>
            <IconUpload class="w-5 h-5 text-[var(--color-text-muted)] group-hover:scale-110 transition-transform" />
          </div>
          <button v-if="settings.config.watermark_image" @click="clearAsset('watermark_image')" class="p-3 bg-white/50 dark:bg-black/30 rounded-2xl hover:bg-red-500 hover:text-white dark:hover:bg-red-500 transition-colors group shadow-sm shrink-0" title="Hapus Gambar Watermark">
            <IconTrash class="w-5 h-5 text-red-500 group-hover:text-white" />
          </button>
        </div>

        <div class="flex items-center gap-2">
          <div @click="selectAsset('video_frame')" class="flex-1 flex items-center justify-between p-3 bg-white/50 dark:bg-black/30 rounded-2xl hover:bg-white dark:hover:bg-black/50 transition-colors cursor-pointer group shadow-sm">
            <div class="flex flex-col">
              <span class="text-sm font-bold text-[var(--color-text-main)]">BG Frame</span>
              <span class="text-[10px] font-bold text-[var(--color-text-muted)] truncate max-w-[120px]" :title="settings.config.video_frame || ''">{{ settings.config.video_frame || 'Belum di-set' }}</span>
            </div>
            <IconUpload class="w-5 h-5 text-[var(--color-text-muted)] group-hover:scale-110 transition-transform" />
          </div>
          <button v-if="settings.config.video_frame" @click="clearAsset('video_frame')" class="p-3 bg-white/50 dark:bg-black/30 rounded-2xl hover:bg-red-500 hover:text-white dark:hover:bg-red-500 transition-colors group shadow-sm shrink-0" title="Hapus Background Frame">
            <IconTrash class="w-5 h-5 text-red-500 group-hover:text-white" />
          </button>
        </div>
      </div>
      <p class="text-[10px] text-[var(--color-text-muted)] mt-1 font-bold shrink-0">Posisi watermark kini dapat diatur secara real-time di halaman Studio.</p>
    </BentoCard>
</template>

<script setup lang="ts">
import { useSettingsStore } from '../../stores/settings';
import { useAppStore } from '../../stores/app';
import BentoCard from '../BentoCard.vue';
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';

// Icons
import IconScissors from '~icons/lucide/scissors';
import IconMic from '~icons/lucide/mic';
import IconImage from '~icons/lucide/image';
import IconUpload from '~icons/lucide/upload';
import IconTrash from '~icons/lucide/trash';

const settings = useSettingsStore();
const appStore = useAppStore();

function clearAsset(type: 'intro_video' | 'outro_video' | 'watermark_image' | 'video_frame') {
  // @ts-ignore - dynamic key assignment
  settings.config[type] = "";
}

async function selectAsset(type: 'intro_video' | 'outro_video' | 'watermark_image' | 'video_frame') {
  let title = '';
  let filters = [];

  if (type === 'intro_video' || type === 'outro_video') {
    title = 'Pilih Video';
    filters = [{ name: 'Video', extensions: ['mp4', 'mov', 'mkv', 'avi', 'webm'] }];
  } else {
    title = 'Pilih Gambar';
    filters = [{ name: 'Gambar', extensions: ['png', 'jpg', 'jpeg', 'webp'] }];
  }

  try {
    const selected = await open({
      multiple: false,
      title,
      filters
    });

    if (selected && typeof selected === 'string') {
      const filename = selected.split(/[/\\]/).pop() || 'asset';
      const safeFilename = `${type}_${Date.now()}_${filename}`;

      // Salin file ke app_data_dir/assets menggunakan Rust invoke
      const relPath = await invoke<string>('copy_asset_file', {
        sourcePath: selected,
        filename: safeFilename
      });

      if (relPath) {
        settings.config[type] = relPath;

        // Simpan konfigurasi (jika sudah didukung via store/watcher, jika belum ini bisa berguna)
        appStore.addToast({
          type: 'success',
          title: 'Aset Berhasil Diunggah',
          message: `${filename} berhasil disalin dan siap digunakan.`
        });
      }
    }
  } catch (e: any) {
    console.error("Gagal menyalin aset:", e);
    appStore.addToast({
      type: 'error',
      title: 'Upload Gagal',
      message: e.toString() || 'Gagal menyalin aset'
    });
  }
}
</script>
