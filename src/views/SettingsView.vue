<template>
  <div class="h-full flex flex-col max-w-7xl mx-auto w-full gap-6 pb-8 overflow-y-auto pr-2">
    <!-- Layout Grid Utama -->
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6 pt-4">
      <!-- KOLOM 1: Profil, Akun Sosial & Penyimpanan -->
      <ProfileSection />

      <!-- KOLOM 2: Engine & Subtitle -->
      <EngineSection />

      <!-- KOLOM 3: Editing, Asset & Lainnya -->
      <MediaSection />
    </div>
  </div>
</template>

<script setup lang="ts">
import ProfileSection from '../components/settings/ProfileSection.vue';
import EngineSection from '../components/settings/EngineSection.vue';
import MediaSection from '../components/settings/MediaSection.vue';
</script>
