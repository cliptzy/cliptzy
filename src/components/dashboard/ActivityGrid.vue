<template>
 <div class="flex-1 min-h-0 flex flex-col bg-base-100">
    <div class="flex items-center justify-between p-4 bg-base-200 border-b border-neutral">
      <h3 class="text-sm font-black text-base-content tracking-wide">Aktivitas Terakhir & Antrean</h3>
      <button class="text-xs font-bold text-secondary hover:text-base-content transition-colors flex items-center gap-1 bg-base-300 px-3 py-1.5 rounded-none border border-neutral">
        Lihat Semua <IconArrowRight class="w-4 h-4" />
      </button>
    </div>
    
    <TransitionGroup name="list" tag="div" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-0 overflow-y-auto relative custom-scrollbar">
      <!-- Empty State -->
      <div v-if="!recentActivities.length" key="empty" class="col-span-full py-12 flex flex-col items-center justify-center border-b border-neutral bg-base-100">
        <IconInbox class="w-12 h-12 text-secondary mb-3" />
        <p class="text-secondary font-bold text-sm">Belum ada aktivitas. Mulai buat klip di Studio!</p>
      </div>

      <!-- Activity Cards -->
      <div v-for="activity in recentActivities" :key="activity.video_id" @click="handleResume(activity)" class="p-4 flex gap-4 items-start bg-base-100 border-b border-r border-neutral transition-colors group cursor-pointer hover:bg-base-200">
        <div class="w-20 h-28 bg-base-200 border border-neutral rounded-none overflow-hidden shrink-0 relative">
          <img :src="activity.thumbnail || 'https://via.placeholder.com/160x90'" class="w-full h-full object-cover transition-transform duration-500" />
          <div class="absolute bottom-1 right-1 bg-base-100/70 text-base-content px-1.5 py-0.5 rounded-none text-[10px] font-mono font-bold">{{ activity.mode === 'compilation' ? 'Kompilasi' : 'Kliping' }}</div>
        </div>
        <div class="flex flex-col py-1 h-full">
          <span class="text-[10px] uppercase tracking-widest mb-1 text-secondary">
            {{ activity.status }}
          </span>
          <h4 class="font-bold text-sm line-clamp-2 mb-2 text-base-content transition-colors">{{ activity.title || 'Video Tanpa Judul' }}</h4>
          <div class="mt-auto flex items-center gap-2 text-xs font-bold text-secondary">
            <IconCalendar class="w-3 h-3" /> {{ formatTime(activity.updated_at) }}
          </div>
        </div>
      </div>
    </TransitionGroup>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router';
import { useVideoStore } from '../../stores/video';
import { useJobHistory, type JobHistory } from '../../composables/useJobHistory';

import IconArrowRight from '~icons/lucide/arrow-right';
import IconInbox from '~icons/lucide/inbox';
import IconCalendar from '~icons/lucide/calendar';

const router = useRouter();
const videoStore = useVideoStore();
const { jobs: recentActivities, formatTime } = useJobHistory();

const handleResume = async (job: JobHistory) => {
  if (job.mode) {
    await router.push(`/studio/${job.mode}`);
    if (job.url) {
      await videoStore.previewVideo(job.url);
    }
  }
};
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


