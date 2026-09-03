<template>
 <!-- Profil Sistem -->
 <div
 class="bg-base-100 "
 >
 <h2
 class="text-lg font-black text-base-content tracking-wide flex items-center gap-2"
 >
 <IconUser class="w-5 h-5" /> Profil Sistem
 </h2>
 <div class="flex items-center justify-between group mt-auto">
 <div class="flex items-center gap-3">
 <img
 v-if="auth.avatarUrl"
 :src="auth.avatarUrl"
 class="w-12 h-12 rounded-none border-2 border-neutral"
 />
 <div
 v-else
 class="w-12 h-12 rounded-none bg-base-200 flex items-center justify-center text-secondary font-bold text-xl border-2 border-neutral"
 >
 {{
 auth.displayName
 ? auth.displayName.charAt(0).toUpperCase()
 : "U"
 }}
 </div>
 <div class="flex flex-col min-w-0">
 <span
 class="font-bold text-base-content text-base truncate"
 >{{ auth.displayName || "Google User" }}</span
 >
 <span
 class="text-xs text-secondary truncate"
 >{{ auth.email || "Email tidak tersedia" }}</span
 >
 </div>
 </div>
 <div class="flex flex-col items-end gap-2">
 <span
 class="text-[10px] font-medium text-secondary bg-base-200 px-2 py-0.5 rounded-none border border-neutral"
 >Tersinkronisasi</span
 >
 <button
 @click="handleLogout"
 class="text-[10px] font-bold text-error hover:underline"
 >
 Logout
 </button>
 </div>
 </div>
 </div>

 <!-- Sinkronisasi Cloud -->
 <div class="bg-base-100 border border-neutral p-6 flex flex-col justify-between gap-2">
 <h2
 class="text-lg font-black text-base-content tracking-wide flex items-center gap-2"
 >
 <IconCloud class="w-5 h-5" /> Sinkronisasi Cloud
 </h2>
 <div class="flex gap-2 mt-auto">
 <CButton
 @click="backupConfig"
 :disabled="isSyncing"
 variant="primary"
 class="flex-1"
 >
 <IconUploadCloud class="w-4 h-4" />
 {{ isSyncing ? "Memproses..." : "Backup" }}
 </CButton>
 <CButton
 @click="restoreConfig"
 :disabled="isSyncing"
 variant="secondary"
 class="flex-1"
 >
 <IconDownloadCloud class="w-4 h-4" /> Restore
 </CButton>
 </div>
 </div>

 <!-- Akun Sosial -->
 <div class="bg-base-100 border border-neutral p-6 flex flex-col gap-5">
 <h2
 class="text-lg font-black text-base-content tracking-wide flex items-center gap-2 shrink-0"
 >
 <IconLink class="w-5 h-5" /> Akun Sosial
 </h2>
 <div class="flex flex-col gap-3">
 <!-- YouTube -->
 <div
 class="flex flex-col p-4 bg-base-200/60 rounded-none transition-colors  shrink-0"
 >
 <div class="flex items-center justify-between">
 <div class="flex items-center gap-3">
 <div
 class="w-10 h-10 rounded-none bg-base-100 flex items-center justify-center"
 >
 <IconYoutube class="w-5 h-5 text-error" />
 </div>
 <div class="flex flex-col">
 <span
 class="font-bold text-base-content text-sm"
 >YouTube</span
 ><span
 class="text-[10px] font-bold text-secondary"
 >yt-dlp cookies</span
 >
 </div>
 </div>
 <span class="relative flex h-2 w-2"
 ><span
 class="animate-ping absolute inline-flex w-full rounded-none bg-neutral/50 opacity-75"
 ></span
 ><span
 class="relative inline-flex rounded-none h-2 w-2 bg-neutral"
 ></span
 ></span>
 </div>

 <div
 class="mt-4 pt-4 border-t border-neutral  flex flex-col gap-3"
 >
 <div class="flex justify-between items-center">
 <div class="flex flex-col">
 <span
 class="text-xs text-base-content font-bold"
 >Browser Cookies</span
 >
 <select
 v-model="settings.config.browser"
 class="mt-1 bg-base-200  border-none rounded-none py-1.5 px-2 text-[11px] font-bold text-base-content focus:outline-none focus:ring-2 focus:ring-gray-500 cursor-pointer "
 >
 <option value="">Pilih Browser</option>
 <option
 value="chrome"
 :disabled="
 !installedBrowsers.includes('chrome')
 "
 >
 Chrome
 </option>
 <option
 value="edge"
 :disabled="
 !installedBrowsers.includes('edge')
 "
 >
 Edge
 </option>
 <option
 value="firefox"
 :disabled="
 !installedBrowsers.includes('firefox')
 "
 >
 Firefox
 </option>
 <option
 value="brave"
 :disabled="
 !installedBrowsers.includes('brave')
 "
 >
 Brave
 </option>
 <option
 value="opera"
 :disabled="
 !installedBrowsers.includes('opera')
 "
 >
 Opera
 </option>
 <option
 value="vivaldi"
 :disabled="
 !installedBrowsers.includes('vivaldi')
 "
 >
 Vivaldi
 </option>
 <option
 value="safari"
 :disabled="
 !installedBrowsers.includes('safari')
 "
 >
 Safari
 </option>
 </select>
 </div>
 <CButton
 variant="danger"
 :disabled="isTestingCookies"
 @click="testCookies"
 >
 <IconYoutube class="w-3 h-3" />
 {{
 isTestingCookies ? "Testing..." : "Test yt-dlp"
 }}
 </CButton>
 </div>
 </div>
 </div>

 <!-- TikTok -->
 <div
 class="flex items-center justify-between p-4 bg-base-200/60 rounded-none transition-colors  shrink-0"
 >
 <div class="flex items-center gap-3">
 <div
 class="w-10 h-10 rounded-none bg-base-content text-base-100 dark:bg-base-content dark:text-base-100  flex items-center justify-center"
 >
 <IconTiktok class="w-5 h-5" />
 </div>
 <div class="flex flex-col">
 <span
 class="font-bold text-base-content text-sm"
 >TikTok</span
 ><span
 class="text-[10px] text-secondary font-bold"
 >Perlu Login</span
 >
 </div>
 </div>
 <CButton> Hubungkan </CButton>
 </div>

 <!-- Default Tags Input -->
 <div class="flex flex-col gap-2 mt-2 shrink-0">
 <span
 class="text-[10px] text-base-content uppercase font-bold"
 >Default Hashtags</span
 >

 <div
 class="flex flex-wrap gap-1.5"
 v-if="parsedHashtags.length > 0"
 >
 <span
 v-for="(tag, idx) in parsedHashtags"
 :key="idx"
 class="px-2 py-0.5 text-[10px] bg-base-100/10 text-secondary rounded-none font-bold "
 >
 {{ tag.startsWith("#") ? tag : "#" + tag }}
 </span>
 </div>

 <div class="relative group mt-1">
 <IconHash
 class="absolute left-3 top-2.5 w-4 h-4 text-[var(--color-secondary)]/50"
 />
 <textarea
 v-model="settings.config.default_hashtags"
 placeholder="viral fyp podcast"
 rows="2"
 class="w-full bg-base-200 border border-neutral py-2 pl-9 pr-3 text-sm font-bold text-base-content focus:outline-none focus:ring-2 focus:ring-primary placeholder:text-[var(--color-secondary)]/50 resize-y transition-all"
 ></textarea>
 </div>
 <p class="text-[9px] font-bold text-secondary">
 Pisahkan dengan spasi. Hashtag ini akan otomatis ditambahkan
 ke setiap video yang diunggah.
 </p>
 </div>
 </div>
 </div>

 <!-- Penyimpanan -->
 <div class="bg-base-100 border border-neutral p-6 flex flex-col justify-between gap-5 relative">
 <h2
 class="text-lg font-black text-base-content tracking-wide flex items-center gap-2 shrink-0"
 >
 <IconHardDrive class="w-5 h-5" /> Penyimpanan
 </h2>

 <div
 class="flex flex-col items-center justify-center relative mt-auto mb-auto"
 >
 <div
 v-if="isCalculatingSize"
 class="absolute inset-0 bg-base-200/50 flex items-center justify-center rounded-none backdrop-blur-sm z-10"
 >
 <span
 class="text-xs font-bold text-secondary animate-pulse"
 >Menghitung...</span
 >
 </div>
 <div
 class="relative w-24 h-24 flex items-center justify-center mb-4"
 >
 <svg
 class="w-full transform -rotate-90"
 viewBox="0 0 100 100"
 >
 <circle
 cx="50"
 cy="50"
 r="40"
 stroke="rgba(0,0,0,0.05)"
 stroke-width="8"
 fill="none"
 />
 <circle
 cx="50"
 cy="50"
 r="40"
 stroke="#ca8a04"
 stroke-width="8"
 fill="none"
 stroke-linecap="round"
 stroke-dasharray="251.2"
 :stroke-dashoffset="calculateDashOffset()"
 class="transition-all duration-1000 ease-out"
 />
 </svg>
 <div
 class="absolute inset-0 flex flex-col items-center justify-center"
 >
 <span
 class="text-xl font-black text-base-content"
 >{{ outputSize.toFixed(2) }}</span
 >
 <span
 class="text-[10px] text-secondary font-bold"
 >GB</span
 >
 </div>
 </div>
 <button
 @click="clearCache"
 :disabled="isClearing"
 class="w-full py-3 rounded-none bg-error/10 text-error text-xs font-bold hover:bg-error hover:text-[var(--color-error-content)] dark:hover:bg-error transition-colors flex items-center justify-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed  shrink-0"
 >
 <IconTrash2 class="w-4 h-4" />
 {{ isClearing ? "Membersihkan..." : "Bersihkan Cache" }}
 </button>
 </div>
 </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { useRouter } from "vue-router";
import { useAppStore } from "../../stores/app";
import { useAuthStore } from "../../stores/auth";
import { useSettingsStore } from "../../stores/settings";
import { invoke } from "@tauri-apps/api/core";

// Icons
import IconUser from "~icons/lucide/user";
import IconLink from "~icons/lucide/link";
import IconHardDrive from "~icons/lucide/hard-drive";
import IconYoutube from "~icons/lucide/youtube";
import IconTiktok from "~icons/lucide/smartphone";
import IconTrash2 from "~icons/lucide/trash-2";
import IconHash from "~icons/lucide/hash";
import IconCloud from "~icons/lucide/cloud";
import IconUploadCloud from "~icons/lucide/upload-cloud";
import IconDownloadCloud from "~icons/lucide/download-cloud";

const appStore = useAppStore();
const auth = useAuthStore();
const settings = useSettingsStore();
const router = useRouter();

const isTestingCookies = ref(false);

const testCookies = async () => {
 if (!settings.config.browser) {
 appStore.addToast({
 type: "error",
 title: "Tidak Ada Browser",
 message: "Silakan pilih browser terlebih dahulu.",
 });
 return;
 }

 isTestingCookies.value = true;
 appStore.addToast({
 type: "info",
 title: "Testing Cookies",
 message:
 "Mencoba fetching video dengan yt-dlp secara penuh. Tunggu sebentar...",
 });

 try {
 const result = await invoke<any>("test_youtube_cookies", {
 browserName: settings.config.browser,
 });
 if (result.valid) {
 appStore.addToast({
 type: "success",
 title: "Cookies Valid",
 message: result.message,
 duration: 5000,
 });
 } else {
 appStore.addToast({
 type: "error",
 title: "Test Gagal",
 message: result.message,
 duration: 8000,
 });
 console.error(result.stderr);
 }
 } catch (err: any) {
 appStore.addToast({
 type: "error",
 title: "Test Error",
 message: err.toString(),
 duration: 5000,
 });
 } finally {
 isTestingCookies.value = false;
 }
};

const parsedHashtags = computed(() => {
 if (!settings.config.default_hashtags) return [];
 return settings.config.default_hashtags
 .split(/\s+/)
 .filter((t) => t.trim() !== "");
});

const outputSize = ref(0.0);
const isCalculatingSize = ref(true);
const isClearing = ref(false);
const isSyncing = ref(false);
const installedBrowsers = ref<string[]>([]);

const backupConfig = async () => {
 isSyncing.value = true;
 try {
 const configData = JSON.parse(JSON.stringify(settings.config));
 await invoke("sync_config_up", { configDict: configData });
 appStore.addToast({
 type: "success",
 title: "Backup Berhasil",
 message: "Konfigurasi telah disimpan ke Supabase.",
 });
 } catch (e: any) {
 appStore.addToast({
 type: "error",
 title: "Backup Gagal",
 message: e.toString(),
 });
 } finally {
 isSyncing.value = false;
 }
};

const restoreConfig = async () => {
 isSyncing.value = true;
 try {
 const configData = await invoke<any>("sync_config_down");
 if (configData) {
 settings.config = { ...settings.config, ...configData };
 await invoke("save_config_file", {
 configJson: JSON.stringify(settings.config),
 });
 appStore.addToast({
 type: "success",
 title: "Restore Berhasil",
 message: "Konfigurasi telah dipulihkan.",
 });
 } else {
 appStore.addToast({
 type: "error",
 title: "Restore Gagal",
 message: "Tidak ada data backup ditemukan.",
 });
 }
 } catch (e: any) {
 appStore.addToast({
 type: "error",
 title: "Restore Gagal",
 message: e.toString(),
 });
 } finally {
 isSyncing.value = false;
 }
};

const calculateDashOffset = () => {
 // Max storage display is arbitrary, let's assume 10GB scale for the visual circle
 const maxGB = 10.0;
 const percentage = Math.min((outputSize.value / maxGB) * 100, 100);
 const circumference = 251.2; // 2 * Math.PI * 40
 return circumference - (percentage / 100) * circumference;
};

const refreshSize = async () => {
 isCalculatingSize.value = true;
 try {
 const size = await invoke<number>("get_output_folder_size");
 outputSize.value = size;
 } catch (e) {
 console.error("Gagal mengambil ukuran folder:", e);
 } finally {
 isCalculatingSize.value = false;
 }
};

onMounted(async () => {
 refreshSize();
 try {
 installedBrowsers.value = await invoke<string[]>(
 "get_installed_browsers",
 );
 } catch (e) {
 console.error("Gagal mendapatkan daftar browser:", e);
 }
});

const clearCache = async () => {
 isClearing.value = true;
 try {
 await invoke("clean_output_folder");
 appStore.addToast({
 type: "success",
 title: "Folder Dibersihkan",
 message: `${outputSize.value.toFixed(2)} GB file dari folder output telah dihapus.`,
 duration: 3000,
 });
 outputSize.value = 0.0;
 await refreshSize();
 } catch (e: any) {
 appStore.addToast({
 type: "error",
 title: "Gagal Membersihkan",
 message: e.toString() || "Gagal membersihkan folder output",
 });
 } finally {
 isClearing.value = false;
 }
};

const handleLogout = async () => {
 await auth.logout();
 router.push("/login");
};
</script>


