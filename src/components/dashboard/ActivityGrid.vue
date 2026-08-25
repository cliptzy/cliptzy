<template>
  <div class="flex-1 min-h-0 mt-2 flex flex-col">
    <div class="flex items-center justify-between mb-4">
      <h3 class="text-xl font-bold">Aktivitas Terakhir & Antrean</h3>
      <button class="text-sm text-gray-400 hover:text-[var(--color-accent)] transition-colors flex items-center gap-1">
        Lihat Semua <IconArrowRight class="w-4 h-4" />
      </button>
    </div>
    
    <TransitionGroup name="list" tag="div" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 overflow-y-auto pb-8 pr-2 relative custom-scrollbar">
      <!-- Empty State -->
      <div v-if="!recentActivities.length" key="empty" class="col-span-full py-12 flex flex-col items-center justify-center border-2 border-dashed border-[var(--color-subtle)] rounded-[24px]">
        <IconInbox class="w-12 h-12 text-gray-600 mb-3" />
        <p class="text-gray-400">Belum ada aktivitas. Mulai buat klip di Studio!</p>
      </div>

      <!-- Activity Cards -->
      <BentoCard v-for="activity in recentActivities" :key="activity.id" class="p-4 flex gap-4 items-start hover:border-gray-500 transition-colors group cursor-pointer">
        <div class="w-20 h-28 bg-gray-800 rounded-lg overflow-hidden shrink-0 relative">
          <img :src="activity.thumbnail" class="w-full h-full object-cover group-hover:scale-110 transition-transform duration-500" />
          <div class="absolute bottom-1 right-1 bg-black/70 px-1.5 rounded text-[10px] font-mono">{{ activity.duration }}</div>
        </div>
        <div class="flex flex-col py-1 h-full">
          <span class="text-[10px] uppercase font-bold tracking-wider mb-1" :class="activity.status === 'Selesai' ? 'text-[var(--color-accent)]' : 'text-yellow-400'">
            {{ activity.status }}
          </span>
          <h4 class="font-semibold text-sm line-clamp-2 mb-2 group-hover:text-[var(--color-accent)] transition-colors">{{ activity.title }}</h4>
          <div class="mt-auto flex items-center gap-2 text-xs text-gray-500">
            <IconCalendar class="w-3 h-3" /> {{ activity.time }}
          </div>
        </div>
      </BentoCard>
    </TransitionGroup>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import BentoCard from '../BentoCard.vue';
import IconArrowRight from '~icons/lucide/arrow-right';
import IconInbox from '~icons/lucide/inbox';
import IconCalendar from '~icons/lucide/calendar';

// Mock data
const recentActivities = ref([
  {
    id: 1,
    title: 'Podcast 10 Jam tentang Coding',
    thumbnail: 'https://images.unsplash.com/photo-1611162617474-5b21e879e113?q=80&w=160&auto=format&fit=crop',
    duration: '0:59',
    status: 'Selesai',
    time: '2 jam yang lalu'
  },
  {
    id: 2,
    title: 'Review Gadget Terbaru',
    thumbnail: 'https://images.unsplash.com/photo-1511707171634-5f897ff02aa9?q=80&w=160&auto=format&fit=crop',
    duration: '0:35',
    status: 'Selesai',
    time: 'Kemarin'
  },
  {
    id: 3,
    title: 'Cara Bikin Aplikasi Tauri',
    thumbnail: 'https://images.unsplash.com/photo-1498050108023-c5249f4df085?q=80&w=160&auto=format&fit=crop',
    duration: '0:42',
    status: 'Memproses...',
    time: 'Baru saja'
  }
]);
</script>

<style scoped>
.list-move,
.list-enter-active,
.list-leave-active {
  transition: all 0.5s cubic-bezier(0.55, 0, 0.1, 1);
}

.list-enter-from,
.list-leave-to {
  opacity: 0;
  transform: scale(0.9) translateY(20px);
}

.list-leave-active {
  position: absolute;
}

.custom-scrollbar::-webkit-scrollbar {
  width: 6px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: rgba(255, 255, 255, 0.02);
  border-radius: 8px;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.1);
  border-radius: 8px;
}
.custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.2);
}
</style>
