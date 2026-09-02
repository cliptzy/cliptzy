<template>
 <!-- Pengaturan Editing -->
 <div class="bg-base-100 ">
 <h2 class="text-lg font-black text-base-content tracking-wide flex items-center gap-2 shrink-0">
 <IconScissors class="w-5 h-5" /> Standar Pemotongan
 </h2>
 <div class="flex flex-col gap-4 mt-auto">
 <div class="flex flex-col gap-2">
 <div class="flex justify-between items-center">
 <span class="text-xs font-bold text-base-content">Durasi Minimal Klip</span>
 <span class="text-xs font-black bg-primary text-primary-content px-3 py-1 rounded-none ">{{ settings.config.min_duration }} Detik</span>
 </div>
 <input type="range" min="10" max="600" step="10" v-model.number="settings.config.min_duration" class="w-full h-2 bg-neutral rounded-none appearance-none cursor-pointer mt-1 accent-primary" />
 </div>
 <div class="flex flex-col gap-2 pt-2 border-t border-neutral ">
 <div class="flex justify-between items-center">
 <span class="text-xs font-bold text-base-content">Padding Waktu Klip</span>
 <span class="text-xs font-black bg-primary text-primary-content px-3 py-1 rounded-none ">{{ settings.config.padding }} Detik</span>
 </div>
 <input type="range" min="-10" max="30" step="1" v-model.number="settings.config.padding" class="w-full h-2 bg-neutral rounded-none appearance-none cursor-pointer mt-1 accent-primary" />
 </div>
 </div>
 </div>

 <!-- TTS Voice -->
 <div class="bg-base-100 ">
 <h2 class="text-lg font-black text-base-content tracking-wide flex items-center gap-2 shrink-0">
 <IconMic class="w-5 h-5" /> Text-to-Speech (AI Voice)
 </h2>
 <div class="flex flex-col gap-3 mt-auto">
 <div class="flex flex-col gap-1">
 <span class="text-[10px] text-base-content uppercase font-bold">Bahasa Utama</span>
 <select v-model="settings.config.tts_language" class="w-full bg-base-200 border border-neutral rounded-none p-2.5 text-sm font-bold text-base-content focus:outline-none focus:ring-2 focus:ring-primary transition-colors cursor-pointer ">
 <option value="default">Deteksi Otomatis</option>
 <option value="id">Indonesia</option>
 <option value="en">English</option>
 </select>
 </div>
 <div class="flex flex-col gap-1">
 <span class="text-[10px] text-base-content uppercase font-bold">Karakter Suara</span>
 <select v-model="settings.config.tts_voice" class="w-full bg-base-200 border border-neutral rounded-none p-2.5 text-sm font-bold text-base-content focus:outline-none focus:ring-2 focus:ring-primary transition-colors cursor-pointer ">
 <option value="female">Wanita</option>
 <option value="male">Pria</option>
 </select>
 </div>
 </div>
 </div>

 <!-- Aset Media (Intro/Outro/Watermark) -->
 <div class="bg-base-100 ">
 <h2 class="text-lg font-black text-base-content tracking-wide flex items-center gap-2 shrink-0">
 <IconImage class="w-5 h-5" /> Branding & Aset Dasar
 </h2>
 <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-4 gap-3 mt-auto">
 <!-- Asset Pickers -->
 <div class="flex items-center gap-2">
 <div @click="selectAsset('intro_video')" class="flex-1 flex items-center justify-between p-3 bg-base-200/50 rounded-none hover:bg-base-200 dark:hover:bg-base-300/50 transition-colors cursor-pointer group ">
 <div class="flex flex-col">
 <span class="text-sm font-bold text-base-content">Video Intro</span>
 <span class="text-[10px] font-bold text-secondary truncate max-w-[120px]" :title="settings.config.intro_video || ''">{{ settings.config.intro_video || 'Belum di-set' }}</span>
 </div>
 <IconUpload class="w-5 h-5 text-secondary transition-transform" />
 </div>
 <button v-if="settings.config.intro_video" @click="clearAsset('intro_video')" class="p-3 bg-base-100/50 rounded-none hover:bg-error hover:text-[var(--color-error)] dark:hover:bg-error transition-colors group  shrink-0" title="Hapus Video Intro">
 <IconTrash class="w-5 h-5 text-error group-hover:text-[var(--color-error)]" />
 </button>
 </div>

 <div class="flex items-center gap-2">
 <div @click="selectAsset('outro_video')" class="flex-1 flex items-center justify-between p-3 bg-base-200/50 rounded-none hover:bg-base-200 dark:hover:bg-base-300/50 transition-colors cursor-pointer group ">
 <div class="flex flex-col">
 <span class="text-sm font-bold text-base-content">Video Outro</span>
 <span class="text-[10px] font-bold text-secondary truncate max-w-[120px]" :title="settings.config.outro_video || ''">{{ settings.config.outro_video || 'Belum di-set' }}</span>
 </div>
 <IconUpload class="w-5 h-5 text-secondary transition-transform" />
 </div>
 <button v-if="settings.config.outro_video" @click="clearAsset('outro_video')" class="p-3 bg-base-100/50 rounded-none hover:bg-error hover:text-[var(--color-error)] dark:hover:bg-error transition-colors group  shrink-0" title="Hapus Video Outro">
 <IconTrash class="w-5 h-5 text-error group-hover:text-[var(--color-error)]" />
 </button>
 </div>

 <div class="flex items-center gap-2">
 <div @click="selectAsset('watermark_image')" class="flex-1 flex items-center justify-between p-3 bg-base-200/50 rounded-none hover:bg-base-200 dark:hover:bg-base-300/50 transition-colors cursor-pointer group ">
 <div class="flex flex-col">
 <span class="text-sm font-bold text-base-content">Watermark</span>
 <span class="text-[10px] font-bold text-secondary truncate max-w-[120px]" :title="settings.config.watermark_image || ''">{{ settings.config.watermark_image || 'Belum di-set' }}</span>
 </div>
 <IconUpload class="w-5 h-5 text-secondary transition-transform" />
 </div>
 <button v-if="settings.config.watermark_image" @click="clearAsset('watermark_image')" class="p-3 bg-base-100/50 rounded-none hover:bg-error hover:text-[var(--color-error)] dark:hover:bg-error transition-colors group  shrink-0" title="Hapus Gambar Watermark">
 <IconTrash class="w-5 h-5 text-error group-hover:text-[var(--color-error)]" />
 </button>
 </div>

 <div class="flex items-center gap-2">
 <div @click="selectAsset('video_frame')" class="flex-1 flex items-center justify-between p-3 bg-base-200/50 rounded-none hover:bg-base-200 dark:hover:bg-base-300/50 transition-colors cursor-pointer group ">
 <div class="flex flex-col">
 <span class="text-sm font-bold text-base-content">BG Frame</span>
 <span class="text-[10px] font-bold text-secondary truncate max-w-[120px]" :title="settings.config.video_frame || ''">{{ settings.config.video_frame || 'Belum di-set' }}</span>
 </div>
 <IconUpload class="w-5 h-5 text-secondary transition-transform" />
 </div>
 <button v-if="settings.config.video_frame" @click="clearAsset('video_frame')" class="p-3 bg-base-100/50 rounded-none hover:bg-error hover:text-[var(--color-error)] dark:hover:bg-error transition-colors group  shrink-0" title="Hapus Background Frame">
 <IconTrash class="w-5 h-5 text-error group-hover:text-[var(--color-error)]" />
 </button>
 </div>
 </div>
 <p class="text-[10px] text-secondary mt-1 font-bold shrink-0">Posisi watermark kini dapat diatur secara real-time di halaman Studio.</p>
 </div>
</template>

<script setup lang="ts">
import { useSettingsStore } from '../../stores/settings';
import { useAppStore } from '../../stores/app';
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


