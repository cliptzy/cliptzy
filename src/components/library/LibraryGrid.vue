<template>
  <div class="flex-1 overflow-y-auto custom-scrollbar">
    <div 
      class="w-full"
      :class="viewMode === 'grid' ? 'grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4' : 'flex flex-col gap-2'"
    >
      <CCard
        v-for="project in jobs"
        :key="project.video_id"
        :class="[viewMode === 'grid' ? 'p-4' : 'flex items-center gap-4 p-3', 'bg-base-100 border border-neutral rounded-none cursor-pointer hover:bg-base-200 transition-colors']"
        @click="$emit('resume', project)"
      >
        <div :class="viewMode === 'grid' ? 'aspect-video mb-3' : 'w-32 aspect-video shrink-0'">
          <img :src="project.thumbnail || 'https://via.placeholder.com/160x90'" class="w-full h-full object-cover bg-base-200 border border-neutral" />
        </div>
        
        <div class="flex-1 min-w-0">
          <h3 class="font-bold text-base-content truncate">{{ project.title || 'Video Tanpa Judul' }}</h3>
          <div class="flex items-center gap-2 mt-1">
            <CBadge :variant="statusVariant(project.status)" size="sm">{{ project.status }}</CBadge>
            <span class="text-[10px] font-mono text-secondary">
              {{ formatDate(project.updated_at) }}
            </span>
            <span class="text-[10px] font-mono text-secondary">
              • {{ project.mode === 'compilation' ? 'Kompilasi' : 'Kliping' }}
            </span>
          </div>
        </div>
        
        <CIconButton
          v-if="viewMode === 'list'"
          icon="play"
          size="sm"
          title="Preview"
          class="shrink-0"
        />
      </CCard>
    </div>

    <div v-if="!jobs.length && !isLoading" class="col-span-full py-12 flex flex-col items-center justify-center border-b border-neutral bg-base-100">
      <IconLibrary class="w-12 h-12 text-secondary mb-4" />
      <p class="text-base-content font-bold mb-2">Tidak ada proyek ditemukan</p>
      <p class="text-sm text-secondary">Belum ada aktivitas. Mulai buat klip di Studio!</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { JobHistory } from '../../composables/useJobHistory';
import IconLibrary from '~icons/lucide/library';

defineProps<{
  jobs: JobHistory[];
  viewMode: 'grid' | 'list';
  isLoading: boolean;
  formatDate: (timestamp: number) => string;
}>();

defineEmits<{
  (e: 'resume', job: JobHistory): void
}>();

const statusVariant = (status: string) => {
  const s = status.toLowerCase();
  if (s === 'selesai' || s === 'completed') return 'success';
  if (s === 'memproses' || s === 'processing') return 'warning';
  if (s === 'gagal' || s === 'failed') return 'error';
  return 'info';
};
</script>
