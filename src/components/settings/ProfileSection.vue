<template>
    <!-- Profil Sistem -->
    <BentoCard
        class="col-span-1 md:col-span-1 xl:col-span-1 row-span-1 h-full overflow-y-auto custom-scrollbar p-6 flex flex-col justify-between gap-2 !bg-purple-200 dark:!bg-purple-900/40"
    >
        <h2
            class="text-lg font-black text-gray-900 dark:text-gray-100 tracking-wide flex items-center gap-2"
        >
            <IconUser class="w-5 h-5" /> Profil Sistem
        </h2>
        <div class="flex items-center justify-between group mt-auto">
            <div class="flex items-center gap-4">
                <img
                    v-if="auth.avatarUrl"
                    :src="auth.avatarUrl"
                    class="w-12 h-12 rounded-full border-2 border-white/50"
                />
                <div
                    v-else
                    class="w-12 h-12 rounded-full bg-gray-500/20 flex items-center justify-center text-gray-700 dark:text-gray-300 font-bold text-xl border-2 border-gray-200 dark:border-gray-800"
                >
                    {{
                        auth.displayName
                            ? auth.displayName.charAt(0).toUpperCase()
                            : "U"
                    }}
                </div>
                <div class="flex flex-col min-w-0">
                    <span
                        class="font-bold text-gray-900 dark:text-gray-100 text-base truncate"
                        >{{ auth.displayName || "Google User" }}</span
                    >
                    <span
                        class="text-xs text-gray-700 dark:text-gray-300 truncate"
                        >{{ auth.email || "Email tidak tersedia" }}</span
                    >
                </div>
            </div>
            <div class="flex flex-col items-end gap-2">
                <span
                    class="text-[10px] font-medium text-gray-600 bg-gray-100 dark:text-gray-400 dark:bg-gray-400/10 px-2 py-0.5 rounded-full border border-gray-200 dark:border-gray-400/20"
                    >Tersinkronisasi</span
                >
                <button
                    @click="handleLogout"
                    class="text-[10px] font-bold text-red-500 hover:underline"
                >
                    Logout
                </button>
            </div>
        </div>
    </BentoCard>

    <!-- Sinkronisasi Cloud -->
    <BentoCard
        class="col-span-1 md:col-span-1 xl:col-span-1 row-span-1 h-full overflow-y-auto custom-scrollbar p-6 flex flex-col justify-between gap-2"
    >
        <h2
            class="text-lg font-black text-gray-900 dark:text-gray-100 tracking-wide flex items-center gap-2"
        >
            <IconCloud class="w-5 h-5" /> Sinkronisasi Cloud
        </h2>
        <div class="flex gap-2 mt-auto">
            <button
                @click="backupConfig"
                :disabled="isSyncing"
                class="flex-1 py-3 px-3 rounded-2xl text-xs font-bold transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2 shadow-sm bg-blue-600 text-white hover:bg-blue-700"
            >
                <IconUploadCloud class="w-4 h-4" />
                {{ isSyncing ? "Memproses..." : "Backup" }}
            </button>
            <button
                @click="restoreConfig"
                :disabled="isSyncing"
                class="flex-1 py-3 px-3 bg-white/60 dark:bg-black/30 text-gray-900 dark:text-gray-100 hover:bg-white dark:hover:bg-black/50 rounded-2xl text-xs font-bold transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2 shadow-sm"
            >
                <IconDownloadCloud class="w-4 h-4" /> Restore
            </button>
        </div>
    </BentoCard>

    <!-- Akun Sosial -->
    <BentoCard
        class="col-span-1 md:col-span-2 xl:col-span-2 row-span-2 h-full overflow-y-auto custom-scrollbar p-6 flex flex-col gap-5"
    >
        <h2
            class="text-lg font-black text-gray-900 dark:text-gray-100 tracking-wide flex items-center gap-2 shrink-0"
        >
            <IconLink class="w-5 h-5" /> Akun Sosial
        </h2>
        <div class="flex flex-col gap-3">
            <!-- YouTube -->
            <div
                class="flex flex-col p-4 bg-white/60 dark:bg-black/30 rounded-2xl transition-colors shadow-sm shrink-0"
            >
                <div class="flex items-center justify-between">
                    <div class="flex items-center gap-3">
                        <div
                            class="w-10 h-10 rounded-full bg-red-100 dark:bg-red-500/20 flex items-center justify-center"
                        >
                            <IconYoutube class="w-5 h-5 text-red-500" />
                        </div>
                        <div class="flex flex-col">
                            <span
                                class="font-bold text-gray-900 dark:text-gray-100 text-sm"
                                >YouTube</span
                            ><span
                                class="text-[10px] font-bold text-gray-700 dark:text-gray-300"
                                >yt-dlp cookies</span
                            >
                        </div>
                    </div>
                    <span class="relative flex h-2 w-2"
                        ><span
                            class="animate-ping absolute inline-flex h-full w-full rounded-full bg-gray-400 opacity-75"
                        ></span
                        ><span
                            class="relative inline-flex rounded-full h-2 w-2 bg-gray-500"
                        ></span
                    ></span>
                </div>

                <div
                    class="mt-4 pt-4 border-t border-gray-300 dark:border-gray-800 flex flex-col gap-3"
                >
                    <div class="flex justify-between items-center">
                        <div class="flex flex-col">
                            <span
                                class="text-xs text-gray-900 dark:text-gray-100 font-bold"
                                >Browser Cookies</span
                            >
                            <select
                                v-model="settings.config.browser"
                                class="mt-1 bg-white/80 dark:bg-black/50 border-none rounded-xl py-1.5 px-2 text-[11px] font-bold text-gray-900 dark:text-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-500 cursor-pointer shadow-sm"
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
                        <button
                            @click="testCookies"
                            :disabled="isTestingCookies"
                            class="text-[10px] font-bold px-4 py-2 rounded-xl transition-all disabled:opacity-50 flex items-center gap-1 shadow-sm bg-red-500 text-white hover:bg-red-600"
                        >
                            <IconYoutube class="w-3 h-3" />
                            {{
                                isTestingCookies ? "Testing..." : "Test yt-dlp"
                            }}
                        </button>
                    </div>
                </div>
            </div>

            <!-- TikTok -->
            <div
                class="flex items-center justify-between p-4 bg-white/60 dark:bg-black/30 rounded-2xl transition-colors shadow-sm shrink-0"
            >
                <div class="flex items-center gap-3">
                    <div
                        class="w-10 h-10 rounded-full bg-black text-white dark:bg-white dark:text-black shadow-sm flex items-center justify-center"
                    >
                        <IconTiktok class="w-5 h-5" />
                    </div>
                    <div class="flex flex-col">
                        <span
                            class="font-bold text-gray-900 dark:text-gray-100 text-sm"
                            >TikTok</span
                        ><span
                            class="text-[10px] text-gray-700 dark:text-gray-300 font-bold"
                            >Perlu Login</span
                        >
                    </div>
                </div>
                <button
                    class="text-[10px] font-bold px-3 py-1.5 rounded-xl hover:scale-105 transition-transform shadow-sm bg-black text-white hover:bg-gray-800"
                >
                    Hubungkan
                </button>
            </div>

            <!-- Default Tags Input -->
            <div class="flex flex-col gap-2 mt-2 shrink-0">
                <span
                    class="text-[10px] text-gray-900 dark:text-gray-100 uppercase font-bold"
                    >Default Hashtags</span
                >

                <div
                    class="flex flex-wrap gap-1.5"
                    v-if="parsedHashtags.length > 0"
                >
                    <span
                        v-for="(tag, idx) in parsedHashtags"
                        :key="idx"
                        class="px-2 py-0.5 text-[10px] bg-gray-700/10 text-gray-700 dark:bg-gray-400/20 dark:text-gray-300 rounded-full font-bold shadow-sm"
                    >
                        {{ tag.startsWith("#") ? tag : "#" + tag }}
                    </span>
                </div>

                <div class="relative group mt-1">
                    <IconHash
                        class="absolute left-3 top-2.5 w-4 h-4 text-gray-700/50 dark:text-gray-300/50"
                    />
                    <textarea
                        v-model="settings.config.default_hashtags"
                        placeholder="viral fyp podcast"
                        rows="2"
                        class="w-full bg-white/60 dark:bg-black/30 border-none rounded-2xl py-2 pl-9 pr-3 text-sm font-bold text-gray-900 dark:text-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-500 placeholder-gray-700/50 dark:placeholder-gray-300/50 resize-y custom-scrollbar transition-all shadow-sm"
                    ></textarea>
                </div>
                <p
                    class="text-[9px] font-bold text-gray-700 dark:text-gray-300"
                >
                    Pisahkan dengan spasi. Hashtag ini akan otomatis ditambahkan
                    ke setiap video yang diunggah.
                </p>
            </div>
        </div>
    </BentoCard>

    <!-- Penyimpanan -->
    <BentoCard
        class="col-span-1 md:col-span-1 xl:col-span-1 row-span-1 h-full overflow-y-auto custom-scrollbar p-6 flex flex-col justify-between gap-5 relative"
    >
        <h2
            class="text-lg font-black text-gray-900 dark:text-gray-100 tracking-wide flex items-center gap-2 shrink-0"
        >
            <IconHardDrive class="w-5 h-5" /> Penyimpanan
        </h2>

        <div
            class="flex flex-col items-center justify-center relative mt-auto mb-auto"
        >
            <div
                v-if="isCalculatingSize"
                class="absolute inset-0 bg-gray-100/50 dark:bg-gray-900/50 flex items-center justify-center rounded-2xl backdrop-blur-sm z-10"
            >
                <span
                    class="text-xs font-bold text-gray-700 dark:text-gray-300 animate-pulse"
                    >Menghitung...</span
                >
            </div>
            <div
                class="relative w-24 h-24 flex items-center justify-center mb-4"
            >
                <svg
                    class="w-full h-full transform -rotate-90"
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
                        class="text-xl font-black text-gray-900 dark:text-gray-100"
                        >{{ outputSize.toFixed(2) }}</span
                    >
                    <span
                        class="text-[10px] text-gray-700 dark:text-gray-300 font-bold"
                        >GB</span
                    >
                </div>
            </div>
            <button
                @click="clearCache"
                :disabled="isClearing"
                class="w-full py-3 rounded-full text-red-600 bg-red-100 dark:bg-red-900/30 dark:text-red-400 text-xs font-bold hover:bg-red-500 hover:text-white dark:hover:bg-red-500 dark:hover:text-white transition-colors flex items-center justify-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed shadow-sm shrink-0"
            >
                <IconTrash2 class="w-4 h-4" />
                {{ isClearing ? "Membersihkan..." : "Bersihkan Cache" }}
            </button>
        </div>
    </BentoCard>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { useRouter } from "vue-router";
import { useAppStore } from "../../stores/app";
import { useAuthStore } from "../../stores/auth";
import { useSettingsStore } from "../../stores/settings";
import { invoke } from "@tauri-apps/api/core";
import BentoCard from "../BentoCard.vue";

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
