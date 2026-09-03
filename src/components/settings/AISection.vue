<template>
 <div
 class="bg-base-100 "
 >
 <h2
 class="text-lg font-black text-base-content tracking-wide flex items-center gap-2 shrink-0"
 >
 <IconSparkles class="w-5 h-5" /> AI Provider & Model
 </h2>

 <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4">
 <!-- Provider -->
 <div class="flex flex-col gap-1.5">
 <span class="text-[10px] text-secondary uppercase font-bold tracking-widest">Provider AI</span>
 <select
 v-model="settings.config.ai.provider"
 class="w-full bg-base-200 border border-neutral rounded-none p-3 text-sm font-bold text-base-content focus:outline-none focus:ring-2 focus:ring-primary cursor-pointer "
 >
 <option v-for="p in AI_PROVIDERS" :key="p.value" :value="p.value">{{ p.label }}</option>
 </select>
 </div>

 <!-- Ollama -->
 <template v-if="settings.config.ai.provider === 'ollama'">
 <div class="flex flex-col gap-1.5">
 <span class="text-[10px] text-secondary uppercase font-bold tracking-widest">Ollama Host</span>
 <input
 v-model="settings.config.ai.ollama_host"
 type="text"
 placeholder="http://localhost:11434"
 class="w-full bg-base-200 border border-neutral rounded-none p-3 text-sm font-bold text-base-content focus:outline-none focus:ring-2 focus:ring-primary "
 />
 </div>
 <div class="flex flex-col gap-1.5">
 <span class="text-[10px] text-secondary uppercase font-bold tracking-widest">Model Ollama</span>
 <select
 v-model="settings.config.ai.ollama_model"
 class="w-full bg-base-200 border border-neutral rounded-none p-3 text-sm font-bold text-base-content focus:outline-none focus:ring-2 focus:ring-primary cursor-pointer "
 >
 <option v-for="m in OLLAMA_MODELS" :key="m" :value="m">{{ m }}</option>
 <option v-if="!OLLAMA_MODELS.includes(settings.config.ai.ollama_model as any)" :value="settings.config.ai.ollama_model">
 {{ settings.config.ai.ollama_model }} (kustom)
 </option>
 </select>
 <input
 v-model="settings.config.ai.ollama_model"
 type="text"
 placeholder="atau ketik model kustom..."
 class="w-full bg-base-200/50 border border-neutral rounded-none p-2 text-xs font-bold text-base-content focus:outline-none focus:ring-2 focus:ring-primary"
 />
 </div>
 </template>

 <!-- Gemini -->
 <template v-if="settings.config.ai.provider === 'gemini'">
 <div class="flex flex-col gap-1.5">
 <span class="text-[10px] text-secondary uppercase font-bold tracking-widest">Gemini API Key</span>
 <input
 v-model="settings.config.ai.gemini_key"
 type="password"
 placeholder="AIza..."
 class="w-full bg-base-200 border border-neutral rounded-none p-3 text-sm font-bold text-base-content focus:outline-none focus:ring-2 focus:ring-primary "
 />
 </div>
 <div class="flex flex-col gap-1.5">
 <span class="text-[10px] text-secondary uppercase font-bold tracking-widest">Model Gemini</span>
 <select
 v-model="settings.config.ai.gemini_model"
 class="w-full bg-base-200 border border-neutral rounded-none p-3 text-sm font-bold text-base-content focus:outline-none focus:ring-2 focus:ring-primary cursor-pointer "
 >
 <option v-for="m in GEMINI_MODELS" :key="m" :value="m">{{ m }}</option>
 <option v-if="!GEMINI_MODELS.includes(settings.config.ai.gemini_model as any)" :value="settings.config.ai.gemini_model">
 {{ settings.config.ai.gemini_model }} (kustom)
 </option>
 </select>
 </div>
 </template>

 <!-- OpenAI -->
 <template v-if="settings.config.ai.provider === 'openai'">
 <div class="flex flex-col gap-1.5">
 <span class="text-[10px] text-secondary uppercase font-bold tracking-widest">OpenAI API Key</span>
 <input
 v-model="settings.config.ai.openai_key"
 type="password"
 placeholder="sk-..."
 class="w-full bg-base-200 border border-neutral rounded-none p-3 text-sm font-bold text-base-content focus:outline-none focus:ring-2 focus:ring-primary "
 />
 </div>
 <div class="flex flex-col gap-1.5">
 <span class="text-[10px] text-secondary uppercase font-bold tracking-widest">Base URL</span>
 <input
 v-model="settings.config.ai.openai_base_url"
 type="text"
 placeholder="https://api.openai.com"
 class="w-full bg-base-200 border border-neutral rounded-none p-3 text-sm font-bold text-base-content focus:outline-none focus:ring-2 focus:ring-primary "
 />
 </div>
 <div class="flex flex-col gap-1.5">
 <span class="text-[10px] text-secondary uppercase font-bold tracking-widest">Model</span>
 <div class="flex items-end gap-2">
 <select
 v-model="settings.config.ai.openai_model"
 class="flex-1 bg-base-200 border border-neutral rounded-none p-3 text-sm font-bold text-base-content focus:outline-none focus:ring-2 focus:ring-primary cursor-pointer"
 >
 <option v-if="isLoadingOpenaiModels" value="" disabled>Loading models...</option>
 <option v-else-if="!openaiModelsLoaded && !settings.config.ai.openai_model" value="" disabled>Click refresh to load</option>
 <option v-if="settings.config.ai.openai_model && !openaiModels.includes(settings.config.ai.openai_model)" :value="settings.config.ai.openai_model">{{ settings.config.ai.openai_model }} (tersimpan)</option>
 <option v-for="m in openaiModels" :key="m" :value="m">{{ m }}</option>
 </select>
 <button
 @click="loadOpenaiModels"
 :disabled="isLoadingOpenaiModels || !settings.config.ai.openai_key || !settings.config.ai.openai_base_url"
 class="shrink-0 px-3 py-2 bg-primary/10 border border-primary/20 rounded-none text-xs font-bold text-primary hover:bg-primary/20 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
 title="Refresh models from API"
 >
 <svg v-if="isLoadingOpenaiModels" class="animate-spin h-4 w-4 inline-block" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
 <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
 <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
 </svg>
 <span v-else>Refresh</span>
 </button>
 </div>
 <div v-if="openaiModelsError" class="mt-1 text-[10px] text-red-400">
 {{ openaiModelsError }}
 </div>
 </div>
 </template>
 </div>

 <!-- Fallback keys when not active provider -->
 <div
 v-if="settings.config.ai.provider !== 'openai' && settings.config.ai.provider !== 'gemini'"
 class="grid grid-cols-1 md:grid-cols-2 gap-3 pt-2 border-t border-neutral dark:border-neutral"
 >
 <div class="flex flex-col gap-1.5">
 <span class="text-[10px] text-secondary uppercase font-bold">Gemini Key (cadangan)</span>
 <input v-model="settings.config.ai.gemini_key" type="password" placeholder="AIza..." class="w-full bg-base-200 border border-neutral rounded-none p-3 text-sm font-bold focus:outline-none focus:ring-2 focus:ring-primary" />
 </div>
 <div class="flex flex-col gap-1.5">
 <span class="text-[10px] text-secondary uppercase font-bold">OpenAI Key (cadangan)</span>
 <input v-model="settings.config.ai.openai_key" type="password" placeholder="sk-..." class="w-full bg-base-200 border border-neutral rounded-none p-3 text-sm font-bold focus:outline-none focus:ring-2 focus:ring-primary" />
 </div>
 </div>

 <!-- AI Feature Toggles -->
 <div class="pt-3 border-t border-neutral dark:border-neutral">
 <span class="text-[10px] text-secondary uppercase font-bold tracking-widest">Fitur AI Analitik</span>
 <div class="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 gap-3 mt-3">
 <label class="flex items-center justify-between bg-base-200/50 p-3 rounded-none cursor-pointer">
 <div class="flex flex-col pr-2"><span class="text-sm font-bold">AI Highlight</span><span class="text-[10px] text-secondary">Deteksi momen viral</span></div>
 <CToggle v-model="settings.config.ai.use_highlight" />
 </label>
 <label class="flex items-center justify-between bg-base-200/50 p-3 rounded-none cursor-pointer">
 <div class="flex flex-col pr-2"><span class="text-sm font-bold">Generate Intro</span><span class="text-[10px] text-secondary">Intro teks via AI</span></div>
 <CToggle v-model="settings.config.ai.use_generate_intro" />
 </label>
 <label class="flex items-center justify-between bg-base-200/50 p-3 rounded-none cursor-pointer">
 <div class="flex flex-col pr-2"><span class="text-sm font-bold">Visual Emotion</span><span class="text-[10px] text-secondary">Emosi wajah (ONNX)</span></div>
 <CToggle v-model="settings.config.ai.use_emotion_detection" />
 </label>
 <label class="flex items-center justify-between bg-base-200/50 p-3 rounded-none cursor-pointer">
 <div class="flex flex-col pr-2"><span class="text-sm font-bold">Voice Tone</span><span class="text-[10px] text-secondary">Intonasi vokal</span></div>
 <CToggle v-model="settings.config.ai.use_voice_analysis" />
 </label>
 <label class="flex items-center justify-between bg-base-200/50 p-3 rounded-none cursor-pointer">
 <div class="flex flex-col pr-2"><span class="text-sm font-bold">Audio Event</span><span class="text-[10px] text-secondary">Tawa, teriak, dll</span></div>
 <CToggle v-model="settings.config.ai.use_audio_analysis" />
 </label>
 <label class="flex items-center justify-between bg-base-200/50 p-3 rounded-none cursor-pointer">
 <div class="flex flex-col pr-2"><span class="text-sm font-bold">Text Sentiment</span><span class="text-[10px] text-secondary">NLP transkrip</span></div>
 <CToggle v-model="settings.config.ai.use_text_analysis" />
 </label>
 <label class="flex items-center justify-between bg-base-200/50 p-3 rounded-none cursor-pointer">
 <div class="flex flex-col pr-2"><span class="text-sm font-bold">Auto B-Roll</span><span class="text-[10px] text-secondary">Overlay meme/b-roll</span></div>
 <CToggle v-model="settings.config.ai.use_add_meme" />
 </label>
 </div>
 </div>

 <!-- Test Agent Section -->
 <div class="pt-3 border-t border-neutral dark:border-neutral mt-4">
 <div class="flex items-center justify-between mb-2">
 <span class="text-[10px] text-secondary uppercase font-bold tracking-widest">Pengujian Agen AI (Fase 4)</span>
 </div>
 <div class="flex flex-col gap-2">
 <textarea
 v-model="testPrompt"
 rows="3"
 placeholder="Masukkan instruksi ke agen..."
 class="w-full bg-base-200 border border-neutral rounded-none p-3 text-sm text-base-content focus:outline-none focus:ring-2 focus:ring-primary"
 ></textarea>
 <div class="flex gap-2 items-center">
 <button 
 @click="testAgent" 
 :disabled="isTestingAgent || !testPrompt"
 class="px-4 py-2 bg-primary text-primary-content font-bold text-sm hover:bg-primary/90 transition-colors disabled:opacity-50"
 >
 <span v-if="isTestingAgent">Menganalisis...</span>
 <span v-else>Jalankan Tool & Agen</span>
 </button>
 </div>
 <div v-if="testResult" class="mt-2 bg-base-300 p-3 text-xs font-mono overflow-auto max-h-[200px] whitespace-pre-wrap">
 {{ testResult }}
 </div>
 </div>
 </div>

 </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useSettingsStore } from "../../stores/settings";
import { AI_PROVIDERS, GEMINI_MODELS, OLLAMA_MODELS } from "../../constants/aiModels";
import IconSparkles from "~icons/lucide/sparkles";

const settings = useSettingsStore();

const openaiModels = ref<string[]>([]);
const isLoadingOpenaiModels = ref(false);
const openaiModelsLoaded = ref(false);
const openaiModelsError = ref<string | null>(null);

const loadOpenaiModels = async () => {
  if (!settings.config.ai.openai_key || !settings.config.ai.openai_base_url) {
    openaiModelsError.value = "Masukkan API key dan base URL terlebih dahulu.";
    return;
  }
  isLoadingOpenaiModels.value = true;
  openaiModelsError.value = null;
  openaiModelsLoaded.value = false;
  try {
    const models: string[] = await invoke('fetch_openai_models', {
      baseUrl: settings.config.ai.openai_base_url,
      apiKey: settings.config.ai.openai_key,
    });
    openaiModels.value = models;
    openaiModelsLoaded.value = true;
  } catch (e: any) {
    openaiModelsError.value = String(e) || "Gagal memuat model.";
  } finally {
    isLoadingOpenaiModels.value = false;
  }
};

const testPrompt = ref("Tolong temukan momen epik dari transkrip berikut:\n[0.0 - 5.0] Halo semuanya selamat datang di video saya\n[5.0 - 20.0] Wah dia nge-kill 5 orang berturut-turut! RRQ Wipeout gila banget gameplay nya!");
const isTestingAgent = ref(false);
const testResult = ref("");

const testAgent = async () => {
  if (!testPrompt.value) return;
  isTestingAgent.value = true;
  testResult.value = "";
  try {
    const res: string = await invoke('ask_agent', {
      prompt: testPrompt.value
    });
    testResult.value = res;
  } catch (e: any) {
    testResult.value = "Error: " + String(e);
  } finally {
    isTestingAgent.value = false;
  }
};
</script>


