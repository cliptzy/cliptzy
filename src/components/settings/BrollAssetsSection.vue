<template>
  <BentoCard class="p-6 shrink-0 !bg-purple-100 dark:!bg-purple-900/40">
    <div class="flex items-center justify-between mb-4">
      <h3 class="text-lg font-black text-[var(--color-text-main)] tracking-wide flex items-center gap-2">
        <IconFolderTree class="w-5 h-5" /> Aset B-roll
      </h3>
      <button
        @click="refreshBrollList"
        class="p-1 rounded-lg hover:bg-purple-200 dark:hover:bg-purple-800 transition-colors"
        :title="'Refresh'"
      >
        <IconRefreshCw class="w-4 h-4 text-[var(--color-text-main)]" :class="{ 'animate-spin': isLoading }" />
      </button>
    </div>

    <!-- B-roll Directory Setting -->
    <div class="flex flex-col gap-2 mb-4">
      <span class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold">
        Folder B-roll
      </span>
      <div class="flex gap-2">
        <input
          v-model="settings.config.broll_dir"
          class="flex-1 bg-white/50 dark:bg-black/30 border-none rounded-xl px-3 py-2 text-xs text-[var(--color-text-main)] focus:outline-none focus:ring-2 focus:ring-purple-500"
          placeholder="assets/broll"
        />
      </div>
      <span class="text-[9px] text-[var(--color-text-muted)]">
        Path relatif dari AppData. Video di sini akan dipilih secara acak untuk mode Split B-roll.
      </span>
    </div>

    <!-- Import Button -->
    <div class="flex gap-2 mb-4">
      <button
        @click="importBrollFile"
        class="flex-1 py-2.5 rounded-xl text-xs font-bold transition-colors bg-indigo-600 text-white hover:bg-indigo-700 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
        :disabled="isProcessing"
      >
        <IconUpload class="w-4 h-4" />
        Import Video B-roll
      </button>
    </div>

    <!-- B-roll Files List -->
    <div v-if="brollFiles.length > 0" class="space-y-1">
      <div
        v-for="file in brollFiles"
        :key="file"
        class="flex items-center justify-between p-2 bg-white/30 dark:bg-black/20 rounded-xl group"
      >
        <div class="flex items-center gap-2 overflow-hidden">
          <IconVideo class="w-4 h-4 text-[var(--color-text-main)] shrink-0" />
          <span class="text-xs text-[var(--color-text-main)] truncate">{{ file }}</span>
        </div>
        <button
          @click="deleteBrollFile(file)"
          class="p-1 rounded-lg opacity-0 group-hover:opacity-100 hover:bg-red-500/20 text-red-500 transition-colors"
          :disabled="isProcessing"
          :title="'Hapus ' + file"
        >
          <IconTrash2 class="w-3.5 h-3.5" />
        </button>
      </div>
    </div>

    <!-- Empty State -->
    <div v-else class="flex flex-col items-center justify-center py-8 gap-2">
      <IconFolderOpen class="w-10 h-10 text-[var(--color-text-muted)] opacity-40" />
      <span class="text-xs text-[var(--color-text-muted)] text-center">
        Belum ada aset B-roll.<br />
        Import video untuk memulai.
      </span>
    </div>

    <!-- Info Footer -->
    <div v-if="brollFiles.length > 0" class="mt-3 pt-2 border-t border-purple-300 dark:border-purple-800">
      <span class="text-[9px] text-[var(--color-text-muted)]">
        {{ brollFiles.length }} file tersedia untuk mode Split B-roll.
      </span>
    </div>
  </BentoCard>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { useSettingsStore } from '../../stores/settings';
import { useAppStore } from '../../stores/app';
import BentoCard from '../BentoCard.vue';
import IconFolderTree from '~icons/lucide/folder-tree';
import IconFolderOpen from '~icons/lucide/folder-open';
import IconUpload from '~icons/lucide/upload';
import IconVideo from '~icons/lucide/video';
import IconTrash2 from '~icons/lucide/trash-2';
import IconRefreshCw from '~icons/lucide/refresh-cw';

const settings = useSettingsStore();
const appStore = useAppStore();

const brollFiles = ref<string[]>([]);
const isLoading = ref(false);
const isProcessing = ref(false);

const refreshBrollList = async () => {
  isLoading.value = true;
  try {
    const files: string[] = await invoke('list_broll_assets');
    brollFiles.value = files;
  } catch (err) {
    console.error('Failed to list B-roll assets:', err);
    brollFiles.value = [];
  } finally {
    isLoading.value = false;
  }
};

const importBrollFile = async () => {
  isProcessing.value = true;
  try {
    const selected = await open({
      title: 'Pilih B-roll Video',
      multiple: false,
      filters: [{ name: 'Video Files', extensions: ['mp4', 'mov', 'avi', 'mkv', 'webm'] }],
    });

    if (selected && typeof selected === 'string') {
      await invoke('import_broll_file', { sourcePath: selected });
      await refreshBrollList();
      appStore.addToast({
        title: 'B-roll Berhasil Diimport',
        message: 'File telah ditambahkan ke folder B-roll.',
        type: 'success',
      });
    }
  } catch (err: any) {
    appStore.addToast({ title: 'Gagal Import', message: String(err), type: 'error' });
  } finally {
    isProcessing.value = false;
  }
};

const deleteBrollFile = async (filename: string) => {
  isProcessing.value = true;
  try {
    await invoke('delete_broll_file', { filename });
    await refreshBrollList();
    appStore.addToast({ title: 'B-roll Dihapus', message: `${filename} telah dihapus.`, type: 'info' });
  } catch (err: any) {
    appStore.addToast({ title: 'Gagal Hapus', message: String(err), type: 'error' });
  } finally {
    isProcessing.value = false;
  }
};

onMounted(() => {
  refreshBrollList();
});
</script>
