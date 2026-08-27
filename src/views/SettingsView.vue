<template>
  <div class="h-full flex flex-col max-w-7xl mx-auto w-full pb-8 overflow-y-auto pr-2 custom-scrollbar">
    <!-- Layout Grid Utama (Strict Bento Box Grid) -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4 auto-rows-[260px] grid-flow-row-dense pt-4">
      <!-- Komponen-komponen ini sekarang akan me-render BentoCard langsung sebagai item grid (karena <div class="flex flex-col"> di dalamnya akan dihapus) -->
      <ProfileSection />
      <EngineSection />
      <MediaSection />
    </div>
  </div>
</template>

<script setup lang="ts">
import ProfileSection from '../components/settings/ProfileSection.vue';
import EngineSection from '../components/settings/EngineSection.vue';
import MediaSection from '../components/settings/MediaSection.vue';
</script>
