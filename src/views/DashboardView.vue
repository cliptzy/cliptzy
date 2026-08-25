<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useDark, useToggle } from '@vueuse/core'
import StatCard from '../components/StatCard.vue'

// Icons
import IconActivity from '~icons/lucide/activity'
import IconVideo from '~icons/lucide/video'
import IconUpload from '~icons/lucide/upload'
import IconCpu from '~icons/lucide/cpu'
import IconMoon from '~icons/lucide/moon'
import IconSun from '~icons/lucide/sun'

const isDark = useDark()
const toggleDark = useToggle(isDark)

const stats = ref({
  totalClips: 24,
  pendingUploads: 5,
  systemLoad: '12%',
  uptime: '2h 15m'
})

onMounted(() => {
  // Fetch stats...
})
</script>

<template>
  <div class="space-y-8 p-8 min-h-screen bg-[#F8F9FA] dark:bg-[#121212] text-black dark:text-white transition-colors">
    <!-- Header Area -->
    <div class="flex flex-col md:flex-row items-start md:items-center justify-between gap-4">
      <div>
        <h1 class="text-4xl md:text-5xl font-black mb-2 tracking-tight">Monitor Sistem</h1>
        <p class="text-lg font-bold text-gray-600 dark:text-gray-400">Pantau performa engine dan status antrean klip.</p>
      </div>
      <div class="flex items-center gap-4">
        <button 
          @click="toggleDark()"
          class="p-4 border-[3px] border-black dark:border-[#3C4043] rounded-full hover:bg-gray-200 dark:hover:bg-[#28292C] transition-colors"
          title="Toggle Dark Mode"
        >
          <IconSun v-if="isDark" class="w-6 h-6 text-[#FBBC04]" />
          <IconMoon v-else class="w-6 h-6 text-[#4285F4]" />
        </button>
        <button class="border-[3px] border-black dark:border-[#3C4043] bg-[#4285F4] hover:bg-[#3367D6] dark:hover:bg-[#8AB4F8] text-white dark:text-black font-bold py-3 px-8 rounded-full transition-all flex items-center gap-2 group">
          <IconActivity class="w-5 h-5 group-hover:animate-pulse" />
          Segarkan Data
        </button>
      </div>
    </div>

    <!-- Stats Grid -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
      <StatCard 
        title="Total Klip Selesai" 
        :value="stats.totalClips" 
        accentClass="bg-[#4285F4]"
      >
        <template #footer>
          <div class="mt-4 flex items-center gap-2 text-sm font-bold text-gray-700 dark:text-gray-300">
            <IconVideo class="w-4 h-4 text-[#4285F4]" />
            <span>Siap di-review</span>
          </div>
        </template>
      </StatCard>

      <StatCard 
        title="Antrean Upload" 
        :value="stats.pendingUploads" 
        accentClass="bg-[#FBBC04]"
      >
        <template #footer>
          <div class="mt-4 flex items-center gap-2 text-sm font-bold text-gray-700 dark:text-gray-300">
            <IconUpload class="w-4 h-4 text-[#FBBC04]" />
            <span>Menunggu jadwal</span>
          </div>
        </template>
      </StatCard>

      <StatCard 
        title="Beban Prosesor" 
        :value="stats.systemLoad" 
        accentClass="bg-[#EA4335]"
      >
        <template #footer>
          <div class="mt-4 flex items-center gap-2 text-sm font-bold text-gray-700 dark:text-gray-300">
            <IconCpu class="w-4 h-4 text-[#EA4335]" />
            <span>Kapasitas normal</span>
          </div>
        </template>
      </StatCard>

      <StatCard 
        title="Waktu Aktif" 
        :value="stats.uptime" 
        accentClass="bg-[#34A853]"
      >
        <template #footer>
          <div class="mt-4 flex items-center gap-2 text-sm font-bold text-gray-700 dark:text-gray-300">
            <div class="w-3 h-3 rounded-full bg-[#34A853] border-2 border-black dark:border-white"></div>
            <span>Engine online</span>
          </div>
        </template>
      </StatCard>
    </div>

    <!-- Activity Log Preview -->
    <div class="border-[3px] border-black dark:border-[#3C4043] rounded-[32px] overflow-hidden bg-white dark:bg-[#1E1E1E] mt-8">
      <div class="border-b-[3px] border-black dark:border-[#3C4043] p-6 bg-[#F8F9FA] dark:bg-[#28292C]">
        <h2 class="text-2xl font-black tracking-tight">Aktivitas Terbaru</h2>
      </div>
      <div class="p-6">
        <div class="space-y-4">
          
          <div class="flex items-start gap-4 border-[2px] border-black dark:border-[#3C4043] rounded-[24px] p-5 hover:bg-[#F8F9FA] dark:hover:bg-[#28292C] transition-colors">
            <div class="bg-[#4285F4] text-white p-3 rounded-full border-[2px] border-black dark:border-[#3C4043] shrink-0">
              <IconVideo class="w-5 h-5" />
            </div>
            <div>
              <h4 class="font-bold text-lg leading-tight mb-1">Proses Klip Selesai</h4>
              <p class="text-sm text-gray-600 dark:text-gray-400">Podcast_Eps_12_Part2.mp4 telah selesai diproses dan dipotong.</p>
              <span class="text-xs font-black uppercase tracking-wider text-[#4285F4] mt-2 block">5 menit yang lalu</span>
            </div>
          </div>
          
          <div class="flex items-start gap-4 border-[2px] border-black dark:border-[#3C4043] rounded-[24px] p-5 hover:bg-[#F8F9FA] dark:hover:bg-[#28292C] transition-colors">
            <div class="bg-[#34A853] text-white p-3 rounded-full border-[2px] border-black dark:border-[#3C4043] shrink-0">
              <IconUpload class="w-5 h-5" />
            </div>
            <div>
              <h4 class="font-bold text-lg leading-tight mb-1">Upload Berhasil</h4>
              <p class="text-sm text-gray-600 dark:text-gray-400">Video berhasil diunggah ke TikTok & YouTube Shorts secara otomatis.</p>
              <span class="text-xs font-black uppercase tracking-wider text-[#34A853] mt-2 block">1 jam yang lalu</span>
            </div>
          </div>

        </div>
      </div>
    </div>
  </div>
</template>
