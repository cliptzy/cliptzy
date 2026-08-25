<script setup lang="ts">
import { ref, watch } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'
import IconYoutube from '~icons/lucide/youtube'
import IconFileVideo from '~icons/lucide/file-video'
import IconSearch from '~icons/lucide/search'
import IconCookie from '~icons/lucide/cookie'
import IconFlame from '~icons/lucide/flame'
import IconBrain from '~icons/lucide/brain'
import IconListOrdered from '~icons/lucide/list-ordered'
import IconHand from '~icons/lucide/hand'
import IconCheck from '~icons/lucide/check'
import IconX from '~icons/lucide/x'

const props = defineProps<{
  urlInput: string
  cookiesPath: string | null
  scanMethod: string
  analyzeStatus: 'idle' | 'scanning' | 'done'
}>()

const emit = defineEmits<{
  (e: 'update:urlInput', value: string): void
  (e: 'update:cookiesPath', value: string): void
  (e: 'update:scanMethod', value: string): void
  (e: 'analyze'): void
}>()

const scanMethods = [
  { id: 'heatmap', label: 'Heatmap', icon: IconFlame, tooltip: 'Berdasarkan Grafik Replay (Paling Sering Diulang)' },
  { id: 'ai', label: 'AI', icon: IconBrain, tooltip: 'Deteksi Momen Sorotan Secara Pintar' },
  { id: 'sequential', label: 'Berurutan', icon: IconListOrdered, tooltip: 'Potong dari awal ke akhir secara berurutan' },
  { id: 'manual', label: 'Manual', icon: IconHand, tooltip: 'Tandai titik potong secara manual' }
]

const pickVideoFile = async () => {
  const file = await open({
    multiple: false,
    filters: [{ name: 'Video', extensions: ['mp4', 'mkv', 'avi', 'mov', 'webm'] }]
  })
  if (file) {
    emit('update:urlInput', file as string)
  }
}

const isCookieSaving = ref(false)
const cookieValidation = ref<{
  status: 'idle' | 'validating' | 'valid' | 'invalid'
  message: string
}>({ status: 'idle', message: '' })

/** Validate cookies file via Rust backend */
const validateCookies = async (path: string | null) => {
  if (!path || path.trim() === '') {
    cookieValidation.value = { status: 'idle', message: '' }
    return
  }

  cookieValidation.value = { status: 'validating', message: 'Memvalidasi cookies...' }

  try {
    const result = await invoke<{ valid: boolean; reason: string; message: string }>('validate_cookies_file', {
      cookiesPath: path
    })
    if (result.valid) {
      cookieValidation.value = { status: 'valid', message: result.message }
    } else {
      cookieValidation.value = { status: 'invalid', message: result.message }
    }
  } catch (e: any) {
    cookieValidation.value = { status: 'invalid', message: e || 'Gagal memvalidasi cookies' }
  }
}

// Watch cookiesPath changes and validate
watch(() => props.cookiesPath, (newPath) => {
  validateCookies(newPath)
}, { immediate: true })

const pickCookiesFile = async () => {
  const file = await open({
    multiple: false,
    filters: [{ name: 'Text', extensions: ['txt'] }]
  })
  if (file) {
    isCookieSaving.value = true
    try {
      const destPath = await invoke<string>('copy_cookies_file', { sourcePath: file as string })
      emit('update:cookiesPath', destPath)
    } catch (e) {
      console.error('Failed to copy cookies', e)
      emit('update:cookiesPath', file as string)
    } finally {
      isCookieSaving.value = false
    }
  }
}
</script>

<template>
  <div class="border-[3px] border-black dark:border-[#3C4043] rounded-[32px] bg-white dark:bg-[#1E1E1E] p-8 relative overflow-hidden group transition-colors">
    <div class="absolute -top-12 -right-12 w-32 h-32 bg-[#FBBC04] rounded-full border-[3px] border-black dark:border-transparent opacity-30 dark:opacity-10 group-hover:scale-110 transition-transform"></div>
    
    <h2 class="text-2xl font-black mb-6 flex items-center gap-3 relative z-10">
      <IconYoutube class="w-7 h-7 text-[#EA4335]" />
      Sumber Video
    </h2>
    
    <!-- Video URL / Path -->
    <div class="flex flex-col md:flex-row gap-4 relative z-10 mb-4">
      <div class="flex-1 relative">
        <input 
          :value="urlInput"
          @input="emit('update:urlInput', ($event.target as HTMLInputElement).value)"
          type="text" 
          placeholder="Tempel tautan YouTube atau Path Lokal..." 
          class="w-full border-[3px] border-black dark:border-[#5F6368] rounded-full bg-[#F8F9FA] dark:bg-[#28292C] px-6 py-4 pr-12 font-bold focus:outline-none focus:border-[#4285F4] dark:focus:border-[#8AB4F8] transition-colors text-lg"
          :disabled="analyzeStatus === 'scanning'"
        />
        <div v-if="urlInput" class="absolute inset-y-0 right-4 flex items-center pointer-events-none">
          <div class="bg-[#34A853] text-white p-1 rounded-full border-2 border-black">
            <IconCheck class="w-4 h-4" />
          </div>
        </div>
      </div>
      <button 
        @click="pickVideoFile"
        class="border-[3px] border-black dark:border-[#5F6368] rounded-full bg-white dark:bg-[#28292C] hover:bg-gray-100 dark:hover:bg-[#3C4043] text-black dark:text-white font-bold py-4 px-6 transition-colors flex items-center justify-center gap-2 whitespace-nowrap"
        :disabled="analyzeStatus === 'scanning'"
      >
        <IconFileVideo class="w-5 h-5" />
        Pilih File
      </button>
    </div>

    <!-- Cookies Path -->
    <div class="flex flex-col md:flex-row gap-4 relative z-10 mb-8">
      <div class="flex-1 relative">
        <div class="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none">
          <IconCookie class="w-5 h-5 text-gray-500" />
        </div>
        <input 
          :value="cookiesPath"
          @input="emit('update:cookiesPath', ($event.target as HTMLInputElement).value)"
          type="text" 
          placeholder="Path cookies.txt (Opsional untuk YouTube)" 
          class="w-full border-[3px] rounded-full bg-[#F8F9FA] dark:bg-[#28292C] pl-12 pr-12 py-3 font-bold text-sm focus:outline-none transition-colors"
          :class="[
            cookieValidation.status === 'invalid' 
              ? 'border-[#EA4335] focus:border-[#EA4335]'
              : cookieValidation.status === 'valid'
                ? 'border-[#34A853] focus:border-[#34A853]'
                : 'border-black dark:border-[#5F6368] focus:border-[#FBBC04]'
          ]"
          :disabled="analyzeStatus === 'scanning' || isCookieSaving"
        />
        <!-- Validating spinner -->
        <div v-if="cookieValidation.status === 'validating'" class="absolute inset-y-0 right-4 flex items-center pointer-events-none">
          <svg class="animate-spin h-4 w-4 text-gray-500" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
        </div>
        <!-- Valid: green check -->
        <div v-else-if="cookieValidation.status === 'valid'" class="absolute inset-y-0 right-4 flex items-center pointer-events-none" :title="cookieValidation.message">
          <div class="bg-[#34A853] text-white p-0.5 rounded-full border-2 border-black">
            <IconCheck class="w-3 h-3" />
          </div>
        </div>
        <!-- Invalid: red X -->
        <div v-else-if="cookieValidation.status === 'invalid'" class="absolute inset-y-0 right-4 flex items-center pointer-events-none" :title="cookieValidation.message">
          <div class="bg-[#EA4335] text-white p-0.5 rounded-full border-2 border-black">
            <IconX class="w-3 h-3" />
          </div>
        </div>
      </div>
      <button 
        @click="pickCookiesFile"
        class="border-[3px] border-black dark:border-[#5F6368] rounded-full bg-white dark:bg-[#28292C] hover:bg-gray-100 dark:hover:bg-[#3C4043] text-black dark:text-white font-bold py-3 px-6 text-sm transition-colors flex items-center justify-center gap-2 whitespace-nowrap min-w-[120px]"
        :disabled="analyzeStatus === 'scanning' || isCookieSaving"
      >
        <template v-if="isCookieSaving">
          <svg class="animate-spin h-4 w-4 text-black dark:text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
          Menyimpan...
        </template>
        <template v-else>
          Browse
        </template>
      </button>
    </div>
    <!-- Cookie validation message -->
    <p v-if="cookieValidation.status === 'invalid'" class="text-xs font-bold text-[#EA4335] -mt-6 mb-6 ml-4 relative z-10">
      ⚠️ {{ cookieValidation.message }}
    </p>
    <p v-else-if="cookieValidation.status === 'valid'" class="text-xs font-bold text-[#34A853] -mt-6 mb-6 ml-4 relative z-10">
      ✅ {{ cookieValidation.message }}
    </p>

    <!-- Scan Method & Action -->
    <div class="border-t-[3px] border-black dark:border-[#3C4043] pt-6 relative z-10 flex flex-col lg:flex-row lg:items-center justify-between gap-6">
      
      <div class="flex-1">
        <h3 class="text-sm font-black text-gray-500 dark:text-gray-400 mb-3 uppercase tracking-wider">Metode Pemindaian</h3>
        <div class="flex flex-wrap gap-2">
          <button
            v-for="method in scanMethods"
            :key="method.id"
            @click="emit('update:scanMethod', method.id)"
            :title="method.tooltip"
            :class="[
              'px-4 py-2 rounded-full border-[2px] font-bold text-sm flex items-center gap-2 transition-all',
              scanMethod === method.id 
                ? 'border-black bg-black text-white dark:bg-white dark:text-black dark:border-transparent' 
                : 'border-black dark:border-[#5F6368] bg-transparent text-black dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-[#3C4043]'
            ]"
            :disabled="analyzeStatus === 'scanning'"
          >
            <component :is="method.icon" class="w-4 h-4" />
            {{ method.label }}
          </button>
        </div>
      </div>

      <button 
        @click="emit('analyze')"
        :disabled="!urlInput || analyzeStatus === 'scanning'"
        class="border-[3px] border-black dark:border-transparent rounded-full bg-[#4285F4] hover:bg-[#3367D6] text-white font-bold py-4 px-8 transition-colors flex items-center justify-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed shrink-0"
      >
        <template v-if="analyzeStatus === 'idle' || analyzeStatus === 'done'">
          <IconSearch class="w-5 h-5" />
          Pindai Video ({{ scanMethods.find(m => m.id === scanMethod)?.label }})
        </template>
        <template v-else>
          <svg class="animate-spin h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
          Memindai...
        </template>
      </button>
    </div>
  </div>
</template>
