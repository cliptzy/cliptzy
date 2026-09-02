<template>
 <!-- Dependensi Sistem -->
 <div class="bg-base-100 border border-neutral p-6 flex flex-col gap-5">
 <h2
 class="text-lg font-black text-base-content tracking-wide flex items-center gap-2 shrink-0"
 >
 <IconPackage class="w-5 h-5" /> Dependensi Eksternal
 </h2>
 <!-- Dependency Status -->
 <div class="flex flex-col gap-3">
 <div
 class="flex justify-between items-center bg-base-200 border border-neutral p-3 rounded-none border-none shrink-0"
 >
 <div class="flex flex-col">
 <span
 class="text-sm font-bold text-base-content"
 >FFmpeg</span
 >
 <span
 class="text-xs font-bold"
 :class="
 depsStatus.ffmpeg_installed
 ? 'text-secondary '
 : 'text-error'
 "
 >{{ depsStatus.ffmpeg_version }}</span
 >
 </div>
 <IconCheckCircle
 v-if="depsStatus.ffmpeg_installed"
 class="w-5 h-5 text-secondary"
 />
 <IconXCircle v-else class="w-5 h-5 text-error" />
 </div>

 <div
 class="flex justify-between items-center bg-base-200 border border-neutral p-3 rounded-none border-none shrink-0"
 >
 <div class="flex flex-col">
 <span
 class="text-sm font-bold text-base-content"
 >Deno</span
 >
 <span
 class="text-xs font-bold"
 :class="
 depsStatus.deno_installed
 ? 'text-secondary '
 : 'text-error'
 "
 >{{ depsStatus.deno_version }}</span
 >
 </div>
 <IconCheckCircle
 v-if="depsStatus.deno_installed"
 class="w-5 h-5 text-secondary"
 />
 <IconXCircle v-else class="w-5 h-5 text-error" />
 </div>
 </div>

 <div
 v-if="isInstallingDeps"
 class="flex flex-col gap-2 shrink-0 mt-auto"
 >
 <div
 class="flex justify-between text-[10px] text-base-content font-bold"
 >
 <span>{{ installProgressText }}</span>
 <span>{{ Math.round(installProgressPercent) }}%</span>
 </div>
 <CProgress
 class="w-full"
 heightClass="h-2"
 :progress="installProgressPercent"
 />
 </div>

 <button
 @click="runInstallDeps"
 :disabled="isInstallingDeps"
 class="w-full py-3 mt-auto rounded-none text-xs font-bold transition-colors disabled:opacity-50 disabled:cursor-not-allowed  bg-primary text-primary-content hover:bg-primary/90 shrink-0"
 >
 <span
 v-if="isInstallingDeps"
 class="flex items-center justify-center gap-2"
 >
 <IconLoader class="w-4 h-4 animate-spin" /> Menginstal...
 </span>
 <span v-else>Instalasi Otomatis</span>
 </button>
 </div>

 <!-- Engine & API -->
 <div class="bg-base-100 border border-neutral p-6 flex flex-col gap-5 !bg-base-200">
 <h2
 class="text-lg font-black text-base-content tracking-wide flex items-center gap-2 shrink-0"
 >
 <IconCpu class="w-5 h-5" /> Engine & Hardware
 </h2>

 <!-- Hardware Accel -->
 <div class="flex flex-col gap-3 shrink-0">
 <span class="text-xs font-bold text-base-content"
 >Akselerasi Rendering (FFmpeg)</span
 >
 <div class="grid grid-cols-2 gap-2">
 <button
 class="py-3 px-2 rounded-none transition-all text-xs font-bold text-center disabled:opacity-50 disabled:cursor-not-allowed"
 :disabled="!availableAccels.includes('cpu')"
 :class="
 settings.config.hw_accel === 'cpu'
 ? 'bg-primary text-[var(--color-primary-content)] shadow-[0_4px_15px_rgba(232,115,137,0.3)]'
 : 'bg-base-200 border border-neutral text-base-content hover:bg-base-300'
 "
 @click="settings.config.hw_accel = 'cpu'"
 >
 CPU
 </button>
 <button
 class="py-3 px-2 rounded-none transition-all text-xs font-bold text-center disabled:opacity-50 disabled:cursor-not-allowed"
 :disabled="!availableAccels.includes('mac')"
 :class="
 settings.config.hw_accel === 'mac'
 ? 'bg-primary text-[var(--color-primary-content)] shadow-[0_4px_15px_rgba(232,115,137,0.3)]'
 : 'bg-base-200 border border-neutral text-base-content hover:bg-base-300'
 "
 @click="settings.config.hw_accel = 'mac'"
 >
 Mac
 </button>
 <button
 class="py-3 px-2 rounded-none transition-all text-xs font-bold text-center disabled:opacity-50 disabled:cursor-not-allowed"
 :disabled="!availableAccels.includes('nvidia')"
 :class="
 settings.config.hw_accel === 'nvidia'
 ? 'bg-primary text-[var(--color-primary-content)] shadow-[0_4px_15px_rgba(232,115,137,0.3)]'
 : 'bg-base-200 border border-neutral text-base-content hover:bg-base-300'
 "
 @click="settings.config.hw_accel = 'nvidia'"
 >
 NVENC
 </button>
 <button
 class="py-3 px-2 rounded-none transition-all text-xs font-bold text-center disabled:opacity-50 disabled:cursor-not-allowed"
 :disabled="!availableAccels.includes('amd')"
 :class="
 settings.config.hw_accel === 'amd'
 ? 'bg-primary text-[var(--color-primary-content)] shadow-[0_4px_15px_rgba(232,115,137,0.3)]'
 : 'bg-base-200 border border-neutral text-base-content hover:bg-base-300'
 "
 @click="settings.config.hw_accel = 'amd'"
 >
 AMF
 </button>
 </div>
 <span
 class="text-[10px] text-secondary font-medium"
 v-if="isLoadingAccels"
 >Mendeteksi hardware yang tersedia...</span
 >
 </div>

 <!-- Threads -->
 <div class="flex flex-col gap-2 mt-2 shrink-0">
 <div class="flex justify-between items-center">
 <span class="text-xs font-bold text-base-content"
 >Maksimum Worker/Thread</span
 >
 <span
 class="text-xs font-black bg-primary text-[var(--color-primary-content)] px-3 py-1 rounded-none"
 >{{ settings.config.max_workers }}</span
 >
 </div>
 <input
 type="range"
 min="1"
 max="16"
 v-model.number="settings.config.max_workers"
 class="w-full h-2 bg-neutral rounded-none appearance-none cursor-pointer mt-2 accent-primary"
 />
 </div>

 <!-- Debug Face Tracking -->
 <div
 class="flex items-center justify-between bg-base-200 border border-neutral p-4 rounded-none shrink-0 mt-2"
 >
 <div class="flex flex-col">
 <span class="text-sm font-bold text-base-content"
 >Debug Mode</span
 >
 <span
 class="text-[10px] font-medium text-secondary"
 >Render hasil analisis emosi</span
 >
 </div>
 <button
 @click="
 settings.config.debug_mode = !settings.config.debug_mode
 "
 class="relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-none border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2"
 :class="
 settings.config.debug_mode
 ? 'bg-primary'
 : 'bg-neutral'
 "
 role="switch"
 :aria-checked="settings.config.debug_mode"
 >
 <span
 class="pointer-events-none inline-block h-5 w-5 transform rounded-none bg-base-200shadow ring-0 transition duration-200 ease-in-out"
 :class="
 settings.config.debug_mode
 ? 'translate-x-5'
 : 'translate-x-0'
 "
 ></span>
 </button>
 </div>
 </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useSettingsStore } from "../../stores/settings";
import CProgress from "../CProgress.vue";

// Icons
import IconCpu from "~icons/lucide/cpu";
import IconPackage from "~icons/lucide/package";
import IconCheckCircle from "~icons/lucide/check-circle-2";
import IconXCircle from "~icons/lucide/x-circle";
import IconLoader from "~icons/lucide/loader-2";

const settings = useSettingsStore();
const availableAccels = ref<string[]>(["cpu"]);
const isLoadingAccels = ref(true);

const depsStatus = ref({
 ffmpeg_installed: false,
 ffmpeg_version: "Memeriksa...",
 deno_installed: false,
 deno_version: "Memeriksa...",
});
const isInstallingDeps = ref(false);
const installProgressText = ref("");
const installProgressPercent = ref(0);

let unlistenDepsProgress: any = null;

const checkDeps = async () => {
 try {
 const status: any = await invoke("check_dependencies");
 depsStatus.value = status;
 } catch (e) {
 console.error("Gagal memeriksa dependensi:", e);
 }
};

const runInstallDeps = async () => {
 isInstallingDeps.value = true;
 installProgressText.value = "Menyiapkan instalasi...";
 installProgressPercent.value = 0;
 try {
 await invoke("install_dependencies");
 await checkDeps();
 } catch (e) {
 console.error("Instalasi gagal:", e);
 installProgressText.value = "Instalasi Gagal!";
 } finally {
 isInstallingDeps.value = false;
 }
};

onMounted(async () => {
 checkDeps();

 unlistenDepsProgress = await listen("deps-progress", (event: any) => {
 installProgressText.value = event.payload.step;
 installProgressPercent.value = event.payload.progress;
 });

 try {
 const accels = await invoke<string[]>("get_available_hwaccels");
 availableAccels.value = accels;

 // Auto fallback to CPU if current settings is not available
 if (!accels.includes(settings.config.hw_accel)) {
 settings.config.hw_accel = "cpu";
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


