<script setup lang="ts">
import { ref, watch } from 'vue'
import IconSettings from '~icons/lucide/settings'
import { useSettingsStore } from '../../stores/settings'
import { open } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'

const settingsStore = useSettingsStore()
const config = settingsStore.config

// Auto-save to config.json when config changes
watch(
  () => settingsStore.config,
  async (newConfig) => {
    try {
      await invoke('save_config_file', { configJson: JSON.stringify(newConfig, null, 2) })
    } catch (e) {
      console.error('Failed to save config.json:', e)
    }
  },
  { deep: true }
)

const activeTab = ref('ai') // 'ai', 'subtitle', 'video', 'media', 'advanced'

const tabs = [
  { id: 'ai', label: 'AI & Efek' },
  { id: 'subtitle', label: 'Subtitle' },
  { id: 'video', label: 'Video' },
  { id: 'media', label: 'Media' },
  { id: 'advanced', label: 'Advanced' }
]

async function pickFile(configKey: 'intro_video' | 'outro_video' | 'watermark_image' | 'video_frame', filters: { name: string, extensions: string[] }[]) {
  try {
    const selected = await open({
      multiple: false,
      directory: false,
      filters
    })

    if (selected && typeof selected === 'string') {
      // Extract original filename
      const filename = selected.split(/[/\\]/).pop() || 'file.ext'
      
      // Copy to assets dir in backend
      const savedPath = await invoke<string>('copy_asset_file', {
        sourcePath: selected,
        filename
      })
      
      // Update config
      config[configKey] = savedPath
    }
  } catch (error) {
    console.error('Failed to pick file:', error)
  }
}

async function removeFile(configKey: 'intro_video' | 'outro_video' | 'watermark_image' | 'video_frame') {
  config[configKey] = null
}
</script>

<template>
  <div class="border-[3px] border-black dark:border-[#3C4043] rounded-[32px] bg-white dark:bg-[#1E1E1E] p-6 transition-colors flex flex-col h-[400px]">
    <h3 class="text-xl font-black mb-4 flex items-center gap-3">
      <IconSettings class="w-6 h-6 text-[#4285F4]" />
      Konfigurasi Proses
    </h3>
    
    <!-- Tab Navigation -->
    <div class="flex gap-2 mb-4 border-b-2 border-gray-200 dark:border-gray-700 pb-2 overflow-x-auto whitespace-nowrap min-h-[44px]">
      <button 
        v-for="tab in tabs" 
        :key="tab.id"
        @click="activeTab = tab.id"
        :class="[
          'px-4 py-2 rounded-full font-bold text-sm transition-all border-2',
          activeTab === tab.id 
            ? 'bg-black text-white border-black dark:bg-white dark:text-black dark:border-white' 
            : 'bg-transparent text-gray-500 border-transparent hover:bg-gray-100 dark:hover:bg-gray-800'
        ]"
      >
        {{ tab.label }}
      </button>
    </div>

    <!-- Tab Content -->
    <div class="flex-1 overflow-y-auto overflow-x-hidden pr-2 space-y-4">
      
      <!-- AI & Efek Tab -->
      <div v-if="activeTab === 'ai'" class="space-y-4">
        <label class="flex items-center justify-between cursor-pointer group">
          <span class="font-bold text-md">Deteksi Emosi Wajah</span>
          <div class="relative w-12 h-6 bg-gray-200 dark:bg-gray-700 rounded-full border-[2px] border-black dark:border-[#5F6368]">
            <input type="checkbox" v-model="config.ai.use_emotion_detection" class="sr-only" />
            <div :class="['absolute top-0.5 left-0.5 bg-white border-[2px] border-black dark:border-transparent w-4 h-4 rounded-full transition-transform', config.ai.use_emotion_detection ? 'translate-x-6 bg-[#4285F4] dark:bg-[#4285F4]' : '']"></div>
          </div>
        </label>
        
        <label class="flex items-center justify-between cursor-pointer group">
          <span class="font-bold text-md">Deteksi Momen (Highlight)</span>
          <div class="relative w-12 h-6 bg-gray-200 dark:bg-gray-700 rounded-full border-[2px] border-black dark:border-[#5F6368]">
            <input type="checkbox" v-model="config.ai.use_highlight" class="sr-only" />
            <div :class="['absolute top-0.5 left-0.5 bg-white border-[2px] border-black dark:border-transparent w-4 h-4 rounded-full transition-transform', config.ai.use_highlight ? 'translate-x-6 bg-[#4285F4] dark:bg-[#4285F4]' : '']"></div>
          </div>
        </label>
        
        <label class="flex items-center justify-between cursor-pointer group">
          <span class="font-bold text-md">Analisis Emosi Suara</span>
          <div class="relative w-12 h-6 bg-gray-200 dark:bg-gray-700 rounded-full border-[2px] border-black dark:border-[#5F6368]">
            <input type="checkbox" v-model="config.ai.use_voice_analysis" class="sr-only" />
            <div :class="['absolute top-0.5 left-0.5 bg-white border-[2px] border-black dark:border-transparent w-4 h-4 rounded-full transition-transform', config.ai.use_voice_analysis ? 'translate-x-6 bg-[#4285F4] dark:bg-[#4285F4]' : '']"></div>
          </div>
        </label>
        
        <label class="flex items-center justify-between cursor-pointer group">
          <span class="font-bold text-md">Deteksi Event Suara</span>
          <div class="relative w-12 h-6 bg-gray-200 dark:bg-gray-700 rounded-full border-[2px] border-black dark:border-[#5F6368]">
            <input type="checkbox" v-model="config.ai.use_audio_analysis" class="sr-only" />
            <div :class="['absolute top-0.5 left-0.5 bg-white border-[2px] border-black dark:border-transparent w-4 h-4 rounded-full transition-transform', config.ai.use_audio_analysis ? 'translate-x-6 bg-[#4285F4] dark:bg-[#4285F4]' : '']"></div>
          </div>
        </label>

        <label class="flex items-center justify-between cursor-pointer group">
          <span class="font-bold text-md">Deteksi Emosi Teks</span>
          <div class="relative w-12 h-6 bg-gray-200 dark:bg-gray-700 rounded-full border-[2px] border-black dark:border-[#5F6368]">
            <input type="checkbox" v-model="config.ai.use_text_analysis" class="sr-only" />
            <div :class="['absolute top-0.5 left-0.5 bg-white border-[2px] border-black dark:border-transparent w-4 h-4 rounded-full transition-transform', config.ai.use_text_analysis ? 'translate-x-6 bg-[#4285F4] dark:bg-[#4285F4]' : '']"></div>
          </div>
        </label>
        
        <label class="flex items-center justify-between cursor-pointer group">
          <span class="font-bold text-md">Otomatis Tambah Meme</span>
          <div class="relative w-12 h-6 bg-gray-200 dark:bg-gray-700 rounded-full border-[2px] border-black dark:border-[#5F6368]">
            <input type="checkbox" v-model="config.ai.use_add_meme" class="sr-only" />
            <div :class="['absolute top-0.5 left-0.5 bg-white border-[2px] border-black dark:border-transparent w-4 h-4 rounded-full transition-transform', config.ai.use_add_meme ? 'translate-x-6 bg-[#4285F4] dark:bg-[#4285F4]' : '']"></div>
          </div>
        </label>

        <label class="flex items-center justify-between cursor-pointer group">
          <span class="font-bold text-md">Generate Intro AI</span>
          <div class="relative w-12 h-6 bg-gray-200 dark:bg-gray-700 rounded-full border-[2px] border-black dark:border-[#5F6368]">
            <input type="checkbox" v-model="config.ai.use_generate_intro" class="sr-only" />
            <div :class="['absolute top-0.5 left-0.5 bg-white border-[2px] border-black dark:border-transparent w-4 h-4 rounded-full transition-transform', config.ai.use_generate_intro ? 'translate-x-6 bg-[#4285F4] dark:bg-[#4285F4]' : '']"></div>
          </div>
        </label>
      </div>

      <!-- Subtitle Tab -->
      <div v-if="activeTab === 'subtitle'" class="space-y-4">
        <label class="flex items-center justify-between cursor-pointer group mb-2">
          <span class="font-bold text-md">Aktifkan Auto-Subtitle</span>
          <div class="relative w-12 h-6 bg-gray-200 dark:bg-gray-700 rounded-full border-[2px] border-black dark:border-[#5F6368]">
            <input type="checkbox" v-model="config.subtitle.enabled" class="sr-only" />
            <div :class="['absolute top-0.5 left-0.5 bg-white border-[2px] border-black dark:border-transparent w-4 h-4 rounded-full transition-transform', config.subtitle.enabled ? 'translate-x-6 bg-[#4285F4] dark:bg-[#4285F4]' : '']"></div>
          </div>
        </label>
        
        <div :class="{'opacity-50 pointer-events-none': !config.subtitle.enabled}" class="space-y-3 border-t-2 border-dashed border-gray-300 dark:border-gray-600 pt-3">
          <div class="flex flex-col gap-1 min-w-0">
            <span class="text-sm font-bold text-gray-600 dark:text-gray-400 truncate">Model Whisper</span>
            <select v-model="config.subtitle.whisper_model" class="w-full min-w-0 border-2 border-black dark:border-gray-600 rounded-xl p-2 bg-transparent font-semibold focus:outline-none focus:border-[#4285F4] truncate">
              <option value="tiny">Tiny (Sangat Cepat)</option>
              <option value="base">Base (Cepat)</option>
              <option value="small">Small (Seimbang)</option>
              <option value="medium">Medium (Akurat)</option>
            </select>
          </div>
          
          <div class="flex flex-col gap-1 min-w-0">
            <span class="text-sm font-bold text-gray-600 dark:text-gray-400 truncate">Gaya Animasi</span>
            <select v-model="config.subtitle.animation" class="w-full min-w-0 border-2 border-black dark:border-gray-600 rounded-xl p-2 bg-transparent font-semibold focus:outline-none focus:border-[#4285F4] truncate">
              <option value="none">Tidak Ada</option>
              <option value="karaoke">Karaoke (Kata per kata)</option>
              <option value="pop">Pop (Timbul)</option>
            </select>
          </div>
          
          <div class="flex gap-4">
            <div class="flex-1 flex flex-col gap-1 min-w-0">
              <span class="text-sm font-bold text-gray-600 dark:text-gray-400 truncate">Font</span>
              <input type="text" v-model="config.subtitle.font" class="w-full min-w-0 border-2 border-black dark:border-gray-600 rounded-xl p-2 bg-transparent font-semibold focus:outline-none focus:border-[#4285F4]" />
            </div>
            <div class="flex-1 flex flex-col gap-1 min-w-0">
              <span class="text-sm font-bold text-gray-600 dark:text-gray-400 truncate">Ukuran Font</span>
              <input type="number" v-model="config.subtitle.font_size" class="w-full min-w-0 border-2 border-black dark:border-gray-600 rounded-xl p-2 bg-transparent font-semibold focus:outline-none focus:border-[#4285F4]" />
            </div>
          </div>
          
          <div class="flex flex-col gap-1 min-w-0">
            <span class="text-sm font-bold text-gray-600 dark:text-gray-400 truncate">Gaya Subtitle</span>
            <select v-model="config.subtitle.style" class="w-full min-w-0 border-2 border-black dark:border-gray-600 rounded-xl p-2 bg-transparent font-semibold focus:outline-none focus:border-[#4285F4] truncate">
              <option value="plain">Polos (Standard)</option>
              <option value="outline">Garis Tepi (Outline)</option>
              <option value="box">Kotak (Background Box)</option>
            </select>
          </div>

          <div class="flex gap-4">
            <div class="flex-1 flex flex-col gap-1 min-w-0">
              <span class="text-sm font-bold text-gray-600 dark:text-gray-400 truncate">Warna Teks</span>
              <input type="text" v-model="config.subtitle.color" class="w-full min-w-0 border-2 border-black dark:border-gray-600 rounded-xl p-2 bg-transparent font-semibold focus:outline-none focus:border-[#4285F4]" placeholder="&H0000FFFF" />
            </div>
            <div class="flex-1 flex flex-col gap-1 min-w-0">
              <span class="text-sm font-bold text-gray-600 dark:text-gray-400 truncate">Warna BG</span>
              <input type="text" v-model="config.subtitle.bg_color" class="w-full min-w-0 border-2 border-black dark:border-gray-600 rounded-xl p-2 bg-transparent font-semibold focus:outline-none focus:border-[#4285F4]" placeholder="&H80000000" />
            </div>
          </div>

          <div class="flex gap-4">
            <div class="flex-1 flex flex-col gap-1 min-w-0">
              <span class="text-sm font-bold text-gray-600 dark:text-gray-400 truncate">Posisi</span>
              <select v-model="config.subtitle.location" class="w-full min-w-0 border-2 border-black dark:border-gray-600 rounded-xl p-2 bg-transparent font-semibold focus:outline-none focus:border-[#4285F4] truncate">
                <option value="bottom">Bawah</option>
                <option value="center">Tengah</option>
                <option value="top">Atas</option>
              </select>
            </div>
            <div class="flex-1 flex flex-col gap-1 min-w-0">
              <span class="text-sm font-bold text-gray-600 dark:text-gray-400 truncate">Maks Kata</span>
              <input type="number" v-model="config.subtitle.max_words" class="w-full min-w-0 border-2 border-black dark:border-gray-600 rounded-xl p-2 bg-transparent font-semibold focus:outline-none focus:border-[#4285F4]" />
            </div>
          </div>
        </div>
      </div>

      <!-- Video Tab -->
      <div v-if="activeTab === 'video'" class="space-y-4">
        <div class="flex gap-4">
          <div class="flex-1 flex flex-col gap-1 min-w-0">
            <span class="text-sm font-bold text-gray-600 dark:text-gray-400 truncate">Padding Klip (detik)</span>
            <input type="number" v-model="config.padding" class="w-full min-w-0 border-2 border-black dark:border-gray-600 rounded-xl p-2 bg-transparent font-semibold focus:outline-none focus:border-[#4285F4]" />
          </div>
          <div class="flex-1 flex flex-col gap-1 min-w-0">
            <span class="text-sm font-bold text-gray-600 dark:text-gray-400 truncate">Min Durasi (detik)</span>
            <input type="number" v-model="config.min_duration" class="w-full min-w-0 border-2 border-black dark:border-gray-600 rounded-xl p-2 bg-transparent font-semibold focus:outline-none focus:border-[#4285F4]" />
          </div>
        </div>

        <div class="flex flex-col gap-1 min-w-0">
          <span class="text-sm font-bold text-gray-600 dark:text-gray-400 truncate">Mode Crop / Fokus Face</span>
          <select v-model="config.crop_mode" class="w-full min-w-0 border-2 border-black dark:border-gray-600 rounded-xl p-2 bg-transparent font-semibold focus:outline-none focus:border-[#4285F4] truncate">
            <option value="default">Otomatis (Cari Wajah)</option>
            <option value="center">Tengah Kaku</option>
            <option value="multi-face">Dua Wajah (Split Screen)</option>
          </select>
        </div>
        
        <label class="flex items-center justify-between cursor-pointer group mt-4">
          <span class="font-bold text-md">Gabungkan Semua Segmen</span>
          <div class="relative w-12 h-6 bg-gray-200 dark:bg-gray-700 rounded-full border-[2px] border-black dark:border-[#5F6368]">
            <input type="checkbox" v-model="config.merge_clips" class="sr-only" />
            <div :class="['absolute top-0.5 left-0.5 bg-white border-[2px] border-black dark:border-transparent w-4 h-4 rounded-full transition-transform', config.merge_clips ? 'translate-x-6 bg-[#4285F4] dark:bg-[#4285F4]' : '']"></div>
          </div>
        </label>
        
        <div class="flex flex-col gap-1 mt-2 min-w-0">
          <span class="text-sm font-bold text-gray-600 dark:text-gray-400 truncate">Posisi Watermark</span>
          <select v-model="config.watermark_position" class="w-full min-w-0 border-2 border-black dark:border-gray-600 rounded-xl p-2 bg-transparent font-semibold focus:outline-none focus:border-[#4285F4] truncate">
            <option value="center">Tengah</option>
            <option value="bottom-right">Bawah Kanan</option>
            <option value="top-left">Atas Kiri</option>
            <option value="hidden">Sembunyikan</option>
          </select>
        </div>
      </div>

      <!-- Media Tab -->
      <div v-if="activeTab === 'media'" class="space-y-4">
        <!-- Video Intro -->
        <div class="flex flex-col gap-1 min-w-0 border-b-2 border-gray-100 dark:border-gray-800 pb-3">
          <span class="text-sm font-bold text-gray-600 dark:text-gray-400 truncate">Video Intro</span>
          <div class="flex items-center gap-3">
            <button @click="pickFile('intro_video', [{name: 'Video', extensions: ['mp4', 'mkv', 'mov', 'avi']}])" class="px-4 py-2 bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 rounded-xl font-bold text-sm transition-colors cursor-pointer border-[2px] border-black dark:border-transparent">
              Pilih File
            </button>
            <div class="flex-1 truncate text-sm font-semibold">
              <span v-if="config.intro_video" class="text-green-600 dark:text-green-400">✓ {{ config.intro_video.split(/[/\\]/).pop() }}</span>
              <span v-else class="text-gray-400">Belum diset</span>
            </div>
            <button v-if="config.intro_video" @click="removeFile('intro_video')" class="text-red-500 hover:text-red-700 font-bold px-2 py-1">X</button>
          </div>
        </div>

        <!-- Video Outro -->
        <div class="flex flex-col gap-1 min-w-0 border-b-2 border-gray-100 dark:border-gray-800 pb-3">
          <span class="text-sm font-bold text-gray-600 dark:text-gray-400 truncate">Video Outro</span>
          <div class="flex items-center gap-3">
            <button @click="pickFile('outro_video', [{name: 'Video', extensions: ['mp4', 'mkv', 'mov', 'avi']}])" class="px-4 py-2 bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 rounded-xl font-bold text-sm transition-colors cursor-pointer border-[2px] border-black dark:border-transparent">
              Pilih File
            </button>
            <div class="flex-1 truncate text-sm font-semibold">
              <span v-if="config.outro_video" class="text-green-600 dark:text-green-400">✓ {{ config.outro_video.split(/[/\\]/).pop() }}</span>
              <span v-else class="text-gray-400">Belum diset</span>
            </div>
            <button v-if="config.outro_video" @click="removeFile('outro_video')" class="text-red-500 hover:text-red-700 font-bold px-2 py-1">X</button>
          </div>
        </div>

        <!-- Watermark -->
        <div class="flex flex-col gap-1 min-w-0 border-b-2 border-gray-100 dark:border-gray-800 pb-3">
          <span class="text-sm font-bold text-gray-600 dark:text-gray-400 truncate">Watermark Image</span>
          <div class="flex items-center gap-3">
            <button @click="pickFile('watermark_image', [{name: 'Image', extensions: ['png', 'jpg', 'jpeg', 'webp']}])" class="px-4 py-2 bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 rounded-xl font-bold text-sm transition-colors cursor-pointer border-[2px] border-black dark:border-transparent">
              Pilih File
            </button>
            <div class="flex-1 truncate text-sm font-semibold">
              <span v-if="config.watermark_image" class="text-green-600 dark:text-green-400">✓ {{ config.watermark_image.split(/[/\\]/).pop() }}</span>
              <span v-else class="text-gray-400">Belum diset</span>
            </div>
            <button v-if="config.watermark_image" @click="removeFile('watermark_image')" class="text-red-500 hover:text-red-700 font-bold px-2 py-1">X</button>
          </div>
        </div>

        <!-- Video Frame Overlay -->
        <div class="flex flex-col gap-1 min-w-0">
          <span class="text-sm font-bold text-gray-600 dark:text-gray-400 truncate">Video Frame Overlay (Image)</span>
          <div class="flex items-center gap-3">
            <button @click="pickFile('video_frame', [{name: 'Image', extensions: ['png', 'webp']}])" class="px-4 py-2 bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 rounded-xl font-bold text-sm transition-colors cursor-pointer border-[2px] border-black dark:border-transparent">
              Pilih File
            </button>
            <div class="flex-1 truncate text-sm font-semibold">
              <span v-if="config.video_frame" class="text-green-600 dark:text-green-400">✓ {{ config.video_frame.split(/[/\\]/).pop() }}</span>
              <span v-else class="text-gray-400">Belum diset</span>
            </div>
            <button v-if="config.video_frame" @click="removeFile('video_frame')" class="text-red-500 hover:text-red-700 font-bold px-2 py-1">X</button>
          </div>
        </div>
      </div>

      <!-- Advanced Tab -->
      <div v-if="activeTab === 'advanced'" class="space-y-4">
        <div class="flex flex-col gap-1 min-w-0">
          <span class="text-sm font-bold text-gray-600 dark:text-gray-400 truncate">Akselerasi Hardware</span>
          <select v-model="config.hw_accel" class="w-full min-w-0 border-2 border-black dark:border-gray-600 rounded-xl p-2 bg-transparent font-semibold focus:outline-none focus:border-[#4285F4] truncate">
            <option value="cpu">CPU (Lambat & Aman)</option>
            <option value="cuda">NVIDIA CUDA</option>
            <option value="qsv">Intel QSV</option>
            <option value="mac">Apple Silicon (VideoToolbox)</option>
          </select>
        </div>
        
        <div class="flex gap-4">
          <div class="flex-1 flex flex-col gap-1 min-w-0">
            <span class="text-sm font-bold text-gray-600 dark:text-gray-400 truncate">Bahasa TTS</span>
            <input type="text" v-model="config.tts_language" class="w-full min-w-0 border-2 border-black dark:border-gray-600 rounded-xl p-2 bg-transparent font-semibold focus:outline-none focus:border-[#4285F4]" placeholder="id" />
          </div>
          <div class="flex-1 flex flex-col gap-1 min-w-0">
            <span class="text-sm font-bold text-gray-600 dark:text-gray-400 truncate">Suara TTS</span>
            <input type="text" v-model="config.tts_voice" class="w-full min-w-0 border-2 border-black dark:border-gray-600 rounded-xl p-2 bg-transparent font-semibold focus:outline-none focus:border-[#4285F4]" placeholder="female" />
          </div>
        </div>
      </div>

    </div>
  </div>
</template>
