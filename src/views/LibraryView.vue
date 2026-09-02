<template>
  <div class="h-full flex flex-col gap-0 max-w-7xl mx-auto p-4">
    <div class="flex items-center justify-between mb-4">
      <h1 class="text-2xl font-bold text-base-content">Library</h1>
      <div class="flex gap-2">
        <CIconButton
          icon="list"
          :active="viewMode === 'list'"
          @click="viewMode = 'list'"
          title="List view"
        />
        <CIconButton
          icon="grid"
          :active="viewMode === 'grid'"
          @click="viewMode = 'grid'"
          title="Grid view"
        />
      </div>
    </div>
    
    <LibraryGrid 
      :jobs="jobs" 
      :view-mode="viewMode" 
      :is-loading="isLoading"
      :format-date="formatDate"
      @resume="handleResume"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { useVideoStore } from '../stores/video';
import { useJobHistory, type JobHistory } from '../composables/useJobHistory';
import LibraryGrid from '../components/library/LibraryGrid.vue';

const viewMode = ref<'grid' | 'list'>('grid');
const router = useRouter();
const videoStore = useVideoStore();

const { jobs, isLoading, loadHistory, formatDate } = useJobHistory();

const handleResume = async (job: JobHistory) => {
  if (job.mode) {
    await router.push(`/studio/${job.mode}`);
    if (job.url) {
      await videoStore.previewVideo(job.url);
    }
  }
};

onMounted(() => {
  loadHistory();
});
</script>
