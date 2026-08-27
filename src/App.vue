<script setup lang="ts">
import { computed, ref, onMounted } from "vue";
import { useRoute } from "vue-router";

import { invoke } from "@tauri-apps/api/core";
import MainLayout from "./layouts/MainLayout.vue";
import BlankLayout from "./layouts/BlankLayout.vue";

const route = useRoute();

interface SystemSpecs {
    meets_requirements: boolean;
    current_memory_gb: number;
    required_memory_gb: number;
    current_cpu_cores: number;
    required_cpu_cores: number;
    missing_reasons: string[];
}

const specs = ref<SystemSpecs | null>(null);

onMounted(async () => {
    try {
        const settingsStore = (await import('./stores/settings')).useSettingsStore();
        await settingsStore.loadFromBackend();
        specs.value = await invoke<SystemSpecs>("check_system_specs");
    } catch (error) {
        console.error("Failed to get system specs or settings:", error);
    }
});

const closeApp = async () => {
    await invoke("exit_app", { code: 1 });
};

const layout = computed(() => {
    return route.name === "login" ? BlankLayout : MainLayout;
});
</script>

<template>
    <div
        v-if="specs && !specs.meets_requirements"
        class="fixed inset-0 z-[10000] bg-black/90 backdrop-blur-md flex items-center justify-center p-6"
    >
        <div
            class="bg-[var(--color-surface)] border border-red-500/50 rounded-2xl p-8 max-w-md w-full shadow-[0_0_40px_rgba(239,68,68,0.2)]"
        >
            <div
                class="flex items-center justify-center w-16 h-16 rounded-full bg-red-500/20 text-red-500 mb-6 mx-auto"
            >
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="32"
                    height="32"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                >
                    <path
                        d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z"
                    />
                    <path d="M12 9v4" />
                    <path d="M12 17h.01" />
                </svg>
            </div>

            <h2 class="text-xl font-bold text-[var(--color-text-main)] text-center mb-2">
                Spesifikasi Minimal Tidak Terpenuhi
            </h2>
            <p class="text-sm text-[var(--color-text-muted)] text-center mb-6">
                Aplikasi tidak dapat dilanjutkan karena sistem Anda tidak
                memenuhi spesifikasi minimal untuk menjalankan proses AI &
                Rendering.
            </p>

            <div
                class="bg-black/50 border border-[var(--color-subtle)] rounded-xl p-4 mb-6"
            >
                <ul class="space-y-2">
                    <li
                        v-for="reason in specs.missing_reasons"
                        :key="reason"
                        class="flex items-start gap-2 text-sm text-red-400"
                    >
                        <span class="text-red-500 mt-0.5">•</span>
                        <span>{{ reason }}</span>
                    </li>
                </ul>
            </div>

            <button
                @click="closeApp"
                class="w-full py-3 bg-red-600 hover:bg-red-700 text-[var(--color-text-main)] font-medium rounded-xl transition-colors duration-200"
            >
                Tutup Aplikasi
            </button>
        </div>
    </div>

    <div v-else>
        <component :is="layout" />
    </div>
</template>
