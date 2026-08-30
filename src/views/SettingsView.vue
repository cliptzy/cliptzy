<template>
  <div class="h-full flex flex-col max-w-7xl mx-auto w-full pb-8 overflow-y-auto pr-2 custom-scrollbar">
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4 auto-rows-[minmax(220px,auto)] grid-flow-row-dense pt-4">
      <ProfileSection />
      <EngineSection />
      <AISection />
      <SubtitleSection />
      <MediaSection />
      <OutputSection />
      <CompilationSettingsSection />
      <UploadSection />
    </div>
  </div>
</template>

<script setup lang="ts">
import ProfileSection from '../components/settings/ProfileSection.vue';
import EngineSection from '../components/settings/EngineSection.vue';
import AISection from '../components/settings/AISection.vue';
import SubtitleSection from '../components/settings/SubtitleSection.vue';
import MediaSection from '../components/settings/MediaSection.vue';
import OutputSection from '../components/settings/OutputSection.vue';
import CompilationSettingsSection from '../components/settings/CompilationSettingsSection.vue';
import UploadSection from '../components/settings/UploadSection.vue';
</script>
