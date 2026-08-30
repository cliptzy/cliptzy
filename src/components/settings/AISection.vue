<template>
  <BentoCard
    class="col-span-1 md:col-span-2 xl:col-span-4 row-span-2 h-full overflow-y-auto custom-scrollbar p-6 flex flex-col gap-5 !bg-fuchsia-100 dark:!bg-fuchsia-900/40"
  >
    <h2
      class="text-lg font-black text-[var(--color-text-main)] tracking-wide flex items-center gap-2 shrink-0"
    >
      <IconSparkles class="w-5 h-5" /> AI Provider & Model
    </h2>

    <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4">
      <!-- Provider -->
      <div class="flex flex-col gap-1.5">
        <span class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold tracking-widest">Provider AI</span>
        <select
          v-model="settings.config.ai.provider"
          class="w-full bg-white/60 dark:bg-black/30 border-none rounded-2xl p-3 text-sm font-bold text-[var(--color-text-main)] focus:outline-none focus:ring-2 focus:ring-[var(--color-accent)] cursor-pointer shadow-sm"
        >
          <option v-for="p in AI_PROVIDERS" :key="p.value" :value="p.value">{{ p.label }}</option>
        </select>
      </div>

      <!-- Ollama -->
      <template v-if="settings.config.ai.provider === 'ollama'">
        <div class="flex flex-col gap-1.5">
          <span class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold tracking-widest">Ollama Host</span>
          <input
            v-model="settings.config.ai.ollama_host"
            type="text"
            placeholder="http://localhost:11434"
            class="w-full bg-white/60 dark:bg-black/30 border-none rounded-2xl p-3 text-sm font-bold text-[var(--color-text-main)] focus:outline-none focus:ring-2 focus:ring-[var(--color-accent)] shadow-sm"
          />
        </div>
        <div class="flex flex-col gap-1.5">
          <span class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold tracking-widest">Model Ollama</span>
          <select
            v-model="settings.config.ai.ollama_model"
            class="w-full bg-white/60 dark:bg-black/30 border-none rounded-2xl p-3 text-sm font-bold text-[var(--color-text-main)] focus:outline-none focus:ring-2 focus:ring-[var(--color-accent)] cursor-pointer shadow-sm"
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
            class="w-full bg-white/40 dark:bg-black/20 border-none rounded-xl p-2 text-xs font-bold text-[var(--color-text-main)] focus:outline-none focus:ring-2 focus:ring-[var(--color-accent)]"
          />
        </div>
      </template>

      <!-- Gemini -->
      <template v-if="settings.config.ai.provider === 'gemini'">
        <div class="flex flex-col gap-1.5">
          <span class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold tracking-widest">Gemini API Key</span>
          <input
            v-model="settings.config.ai.gemini_key"
            type="password"
            placeholder="AIza..."
            class="w-full bg-white/60 dark:bg-black/30 border-none rounded-2xl p-3 text-sm font-bold text-[var(--color-text-main)] focus:outline-none focus:ring-2 focus:ring-[var(--color-accent)] shadow-sm"
          />
        </div>
        <div class="flex flex-col gap-1.5">
          <span class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold tracking-widest">Model Gemini</span>
          <select
            v-model="settings.config.ai.gemini_model"
            class="w-full bg-white/60 dark:bg-black/30 border-none rounded-2xl p-3 text-sm font-bold text-[var(--color-text-main)] focus:outline-none focus:ring-2 focus:ring-[var(--color-accent)] cursor-pointer shadow-sm"
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
          <span class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold tracking-widest">OpenAI API Key</span>
          <input
            v-model="settings.config.ai.openai_key"
            type="password"
            placeholder="sk-..."
            class="w-full bg-white/60 dark:bg-black/30 border-none rounded-2xl p-3 text-sm font-bold text-[var(--color-text-main)] focus:outline-none focus:ring-2 focus:ring-[var(--color-accent)] shadow-sm"
          />
        </div>
        <div class="flex flex-col gap-1.5">
          <span class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold tracking-widest">Base URL (opsional)</span>
          <input
            v-model="settings.config.ai.openai_base_url"
            type="text"
            placeholder="https://api.openai.com/v1"
            class="w-full bg-white/60 dark:bg-black/30 border-none rounded-2xl p-3 text-sm font-bold text-[var(--color-text-main)] focus:outline-none focus:ring-2 focus:ring-[var(--color-accent)] shadow-sm"
          />
        </div>
        <div class="flex flex-col gap-1.5">
          <span class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold tracking-widest">Model OpenAI</span>
          <select
            v-model="settings.config.ai.openai_model"
            class="w-full bg-white/60 dark:bg-black/30 border-none rounded-2xl p-3 text-sm font-bold text-[var(--color-text-main)] focus:outline-none focus:ring-2 focus:ring-[var(--color-accent)] cursor-pointer shadow-sm"
          >
            <option v-for="m in OPENAI_MODELS" :key="m" :value="m">{{ m }}</option>
            <option v-if="!OPENAI_MODELS.includes(settings.config.ai.openai_model as any)" :value="settings.config.ai.openai_model">
              {{ settings.config.ai.openai_model }} (kustom)
            </option>
          </select>
        </div>
      </template>
    </div>

    <!-- Fallback keys when not active provider -->
    <div
      v-if="settings.config.ai.provider !== 'openai' && settings.config.ai.provider !== 'gemini'"
      class="grid grid-cols-1 md:grid-cols-2 gap-3 pt-2 border-t border-fuchsia-200 dark:border-fuchsia-800/50"
    >
      <div class="flex flex-col gap-1.5">
        <span class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold">Gemini Key (cadangan)</span>
        <input v-model="settings.config.ai.gemini_key" type="password" placeholder="AIza..." class="w-full bg-white/60 dark:bg-black/30 border-none rounded-2xl p-3 text-sm font-bold focus:outline-none focus:ring-2 focus:ring-[var(--color-accent)]" />
      </div>
      <div class="flex flex-col gap-1.5">
        <span class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold">OpenAI Key (cadangan)</span>
        <input v-model="settings.config.ai.openai_key" type="password" placeholder="sk-..." class="w-full bg-white/60 dark:bg-black/30 border-none rounded-2xl p-3 text-sm font-bold focus:outline-none focus:ring-2 focus:ring-[var(--color-accent)]" />
      </div>
    </div>

    <!-- AI Feature Toggles -->
    <div class="pt-3 border-t border-fuchsia-200 dark:border-fuchsia-800/50">
      <span class="text-[10px] text-[var(--color-text-muted)] uppercase font-bold tracking-widest">Fitur AI Analitik</span>
      <div class="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 gap-3 mt-3">
        <label class="flex items-center justify-between bg-white/50 dark:bg-black/30 p-3 rounded-2xl cursor-pointer">
          <div class="flex flex-col pr-2"><span class="text-sm font-bold">AI Highlight</span><span class="text-[10px] text-[var(--color-text-muted)]">Deteksi momen viral</span></div>
          <ToggleSwitch v-model="settings.config.ai.use_highlight" />
        </label>
        <label class="flex items-center justify-between bg-white/50 dark:bg-black/30 p-3 rounded-2xl cursor-pointer">
          <div class="flex flex-col pr-2"><span class="text-sm font-bold">Generate Intro</span><span class="text-[10px] text-[var(--color-text-muted)]">Intro teks via AI</span></div>
          <ToggleSwitch v-model="settings.config.ai.use_generate_intro" />
        </label>
        <label class="flex items-center justify-between bg-white/50 dark:bg-black/30 p-3 rounded-2xl cursor-pointer">
          <div class="flex flex-col pr-2"><span class="text-sm font-bold">Visual Emotion</span><span class="text-[10px] text-[var(--color-text-muted)]">Emosi wajah (ONNX)</span></div>
          <ToggleSwitch v-model="settings.config.ai.use_emotion_detection" />
        </label>
        <label class="flex items-center justify-between bg-white/50 dark:bg-black/30 p-3 rounded-2xl cursor-pointer">
          <div class="flex flex-col pr-2"><span class="text-sm font-bold">Voice Tone</span><span class="text-[10px] text-[var(--color-text-muted)]">Intonasi vokal</span></div>
          <ToggleSwitch v-model="settings.config.ai.use_voice_analysis" />
        </label>
        <label class="flex items-center justify-between bg-white/50 dark:bg-black/30 p-3 rounded-2xl cursor-pointer">
          <div class="flex flex-col pr-2"><span class="text-sm font-bold">Audio Event</span><span class="text-[10px] text-[var(--color-text-muted)]">Tawa, teriak, dll</span></div>
          <ToggleSwitch v-model="settings.config.ai.use_audio_analysis" />
        </label>
        <label class="flex items-center justify-between bg-white/50 dark:bg-black/30 p-3 rounded-2xl cursor-pointer">
          <div class="flex flex-col pr-2"><span class="text-sm font-bold">Text Sentiment</span><span class="text-[10px] text-[var(--color-text-muted)]">NLP transkrip</span></div>
          <ToggleSwitch v-model="settings.config.ai.use_text_analysis" />
        </label>
        <label class="flex items-center justify-between bg-white/50 dark:bg-black/30 p-3 rounded-2xl cursor-pointer">
          <div class="flex flex-col pr-2"><span class="text-sm font-bold">Auto B-Roll</span><span class="text-[10px] text-[var(--color-text-muted)]">Overlay meme/b-roll</span></div>
          <ToggleSwitch v-model="settings.config.ai.use_add_meme" />
        </label>
      </div>
    </div>
  </BentoCard>
</template>

<script setup lang="ts">
import BentoCard from "../BentoCard.vue";
import ToggleSwitch from "../ToggleSwitch.vue";
import { useSettingsStore } from "../../stores/settings";
import { AI_PROVIDERS, GEMINI_MODELS, OPENAI_MODELS, OLLAMA_MODELS } from "../../constants/aiModels";
import IconSparkles from "~icons/lucide/sparkles";

const settings = useSettingsStore();
</script>
