<template>
  <div class="p-6 bg-base-100">
    <!-- Header -->
    <div class="flex items-center justify-between mb-4">
      <h3 class="text-lg font-black text-base-content tracking-wide flex items-center gap-2">
        <IconBoxes class="w-5 h-5" /> Model ONNX Lokal
      </h3>
      <button
        @click="refreshModels"
        class="p-1 rounded-none hover:bg-base-300 transition-colors"
        :title="'Refresh'"
      >
        <IconRefreshCw class="w-4 h-4 text-base-content" :class="{ 'animate-spin': isLoading }" />
      </button>
    </div>

    <!-- Summary strip (terminal-style) -->
    <div
      class="flex items-center gap-4 mb-4 px-3 py-2 bg-base-200 border border-neutral rounded-none font-mono text-[10px] text-secondary"
    >
      <span class="flex items-center gap-1.5">
        <span class="w-1.5 h-1.5 rounded-none" :class="summaryDotClass"></span>
        {{ installedCount }} / {{ models.length }} terpasang
      </span>
      <span class="hidden sm:inline">·</span>
      <span class="hidden sm:inline">AppData/models/</span>
      <span class="ml-auto">
        <button
          @click="downloadAll"
          :disabled="isDownloadingAll || models.length === 0 || installedCount === models.length"
          class="px-2.5 py-1 rounded-none text-[10px] font-bold transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          :class="installedCount > 0 ? 'bg-base-300 text-base-content hover:bg-neutral' : 'bg-primary text-primary-content hover:bg-primary/90'"
        >
          Unduh Semua
        </button>
      </span>
    </div>

    <!-- Model rows -->
    <div class="flex flex-col divide-y divide-neutral border border-neutral rounded-none">
      <div
        v-for="model in models"
        :key="model.id"
        class="flex flex-col gap-3 p-4 bg-base-100 hover:bg-base-200/40 transition-colors"
      >
        <!-- Top line: status dot + name + actions -->
        <div class="flex items-start gap-3">
          <div class="flex items-center gap-2.5 min-w-0">
            <span
              class="w-2 h-2 rounded-none shrink-0 mt-1"
              :class="statusDotClass(model)"
              :title="statusLabel(model)"
            ></span>
            <div class="flex flex-col min-w-0">
              <div class="flex items-center gap-2">
                <span class="text-sm font-bold text-base-content truncate">
                  {{ model.display_name }}
                </span>
                <CBadge variant="neutral" size="sm">{{ model.category }}</CBadge>
              </div>
              <span class="text-[10px] font-mono text-secondary truncate mt-0.5">
                {{ model.file }}
              </span>
            </div>
          </div>

          <div class="flex items-center gap-2 ml-auto shrink-0">
            <span class="text-[10px] font-mono text-secondary mr-1 hidden md:inline">
              {{ currentSize(model) }}
            </span>
            <button
              v-if="model.exists"
              @click="deleteModel(model)"
              :disabled="busy.has(model.id)"
              class="px-2.5 py-1.5 rounded-none text-[10px] font-bold transition-colors bg-transparent text-error hover:bg-error/15 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              Hapus
            </button>
            <button
              @click="downloadModel(model)"
              :disabled="busy.has(model.id)"
              class="px-2.5 py-1.5 rounded-none text-[10px] font-bold transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-1.5"
              :class="model.exists ? 'bg-base-300 text-base-content hover:bg-neutral' : 'bg-primary text-primary-content hover:bg-primary/90'"
            >
              <IconLoader v-if="busy.has(model.id)" class="w-3 h-3 animate-spin" />
              <IconDownload v-else class="w-3 h-3" />
              {{ model.exists ? 'Unduh Ulang' : 'Unduh' }}
            </button>
          </div>
        </div>

        <!-- Description -->
        <p class="text-xs text-secondary leading-relaxed pr-2">
          {{ model.description }}
        </p>

        <!-- Tags -->
        <div v-if="model.tags && model.tags.length" class="flex items-center gap-2 flex-wrap">
          <span
            v-for="tag in model.tags"
            :key="tag"
            class="px-2 py-0.5 rounded-none bg-base-200 border border-neutral text-[9px] font-mono text-secondary"
          >
            {{ tag }}
          </span>
        </div>

        <!-- Inline progress -->
        <div v-if="busy.has(model.id)" class="flex flex-col gap-1.5">
          <div class="flex justify-between text-[10px] font-mono text-base-content">
            <span>{{ progressText }}</span>
            <span>{{ Math.round(progressPercent) }}%</span>
          </div>
          <CProgress :progress="progressPercent" heightClass="h-1.5" />
        </div>

        <!-- Installed path -->
        <div
          v-if="model.exists && !busy.has(model.id)"
          class="text-[9px] font-mono text-secondary/70 truncate"
        >
          {{ model.path }}
        </div>
      </div>
    </div>

    <!-- Empty / not-connected state -->
    <div
      v-if="backendConnected === false"
      class="flex flex-col items-center justify-center py-8 gap-2 border border-neutral border-t-0 rounded-none"
    >
      <IconPlugZap class="w-8 h-8 text-secondary opacity-40" />
      <span class="text-xs text-secondary text-center">
        Backend Manajemen Model belum tersedia.<br />
        Panggilan <span class="font-mono">list_onnx_models</span> belum diimplementasi.
      </span>
    </div>
  </div>
</template>
<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { useAppStore } from "../../stores/app";
import CProgress from "../CProgress.vue";
import CBadge from "../CBadge.vue";

// Icons
import IconBoxes from "~icons/lucide/boxes";
import IconRefreshCw from "~icons/lucide/refresh-cw";
import IconDownload from "~icons/lucide/download";
import IconLoader from "~icons/lucide/loader-2";
import IconPlugZap from "~icons/lucide/plug-zap";

export interface OnnxModelItem {
  id: string;
  file: string;
  url: string;
  display_name: string;
  category: string;
  description: string;
  approx_size: string;
  tags: string[];
  exists: boolean;
  size_bytes?: number;
  path?: string;
}

const appStore = useAppStore();

const models = ref<OnnxModelItem[]>([]);
const busy = ref<Set<string>>(new Set());
const isLoading = ref(false);
const isDownloadingAll = ref(false);
const backendConnected = ref<boolean | null>(null);

const progressText = ref("");
const progressPercent = ref(0);

let unlistenProgress: UnlistenFn | null = null;

const installedCount = computed(() => models.value.filter((m) => m.exists).length);

const summaryDotClass = computed(() => {
  if (models.value.length === 0) return "bg-neutral";
  if (installedCount.value === 0) return "bg-error";
  if (installedCount.value === models.value.length) return "bg-success";
  return "bg-warning";
});

const statusLabel = (model: OnnxModelItem) => {
  if (busy.value.has(model.id)) return "Memproses...";
  return model.exists ? "Terpasang" : "Belum diunduh";
};

const statusDotClass = (model: OnnxModelItem) => {
  if (busy.value.has(model.id)) return "bg-accent";
  return model.exists ? "bg-success" : "bg-error";
};

const currentSize = (model: OnnxModelItem) => {
  if (model.size_bytes != null) return formatBytes(model.size_bytes);
  return model.approx_size || "";
};

function formatBytes(bytes: number): string {
  if (bytes <= 0) return "0 B";
  const units = ["B", "KB", "MB", "GB"];
  const i = Math.floor(Math.log(bytes) / Math.log(1024));
  const val = bytes / Math.pow(1024, i);
  return `${val.toFixed(i === 0 ? 0 : 1)} ${units[i]}`;
}

/**
 * Ambil daftar model beserta status nyata dari backend.
 */
const refreshModels = async () => {
  isLoading.value = true;
  try {
    const res = await invoke<OnnxModelItem[]>("list_onnx_models");
    models.value = res;
    backendConnected.value = true;
  } catch (e) {
    backendConnected.value = false;
    console.warn("list_onnx_models gagal:", e);
  } finally {
    isLoading.value = false;
  }
};

/**
 * Unduh / unduh ulang satu model via backend streaming.
 */
const downloadModel = async (model: OnnxModelItem) => {
  if (busy.value.has(model.id)) return;
  busy.value = new Set(busy.value).add(model.id);
  progressText.value = `Mengunduh ${model.file}...`;
  progressPercent.value = 0;
  try {
    await invoke("download_onnx_model", { id: model.id });
    appStore.addToast({
      title: "Model Diunduh",
      message: `${model.display_name} siap digunakan.`,
      type: "success",
    });
  } catch (e: any) {
    appStore.addToast({
      title: "Gagal Mengunduh",
      message: String(e),
      type: "error",
    });
  } finally {
    const next = new Set(busy.value);
    next.delete(model.id);
    busy.value = next;
    await refreshModels();
  }
};

/**
 * Hapus satu model dari disk.
 */
const deleteModel = async (model: OnnxModelItem) => {
  if (busy.value.has(model.id)) return;
  busy.value = new Set(busy.value).add(model.id);
  try {
    await invoke("delete_onnx_model", { id: model.id });
    appStore.addToast({
      title: "Model Dihapus",
      message: `${model.file} telah dihapus dari disk.`,
      type: "info",
    });
  } catch (e: any) {
    appStore.addToast({
      title: "Gagal Menghapus",
      message: String(e),
      type: "error",
    });
  } finally {
    const next = new Set(busy.value);
    next.delete(model.id);
    busy.value = next;
    await refreshModels();
  }
};

/**
 * Unduh semua model yang belum terpasang secara berurutan.
 */
const downloadAll = async () => {
  if (isDownloadingAll.value) return;
  isDownloadingAll.value = true;
  try {
    for (const model of models.value) {
      if (model.exists) continue;
      await downloadModel(model);
    }
  } finally {
    isDownloadingAll.value = false;
  }
};

onMounted(async () => {
  await refreshModels();

  // Dengarkan progres unduhan real-time dari backend (`onnx-download-progress`).
  try {
    unlistenProgress = await listen<{ current: number; total: number; label?: string }>(
      "onnx-download-progress",
      (event) => {
        const total = event.payload.total || 100;
        progressPercent.value = (event.payload.current / total) * 100;
        if (event.payload.label) progressText.value = event.payload.label;
      },
    );
  } catch (e) {
    console.warn("Gagal listen onnx-download-progress:", e);
  }
});

onUnmounted(() => {
  if (unlistenProgress) unlistenProgress();
});
</script>