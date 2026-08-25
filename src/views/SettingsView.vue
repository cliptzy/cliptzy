<template>
  <div class="h-full flex flex-col max-w-7xl mx-auto w-full gap-6 pb-8 overflow-y-auto custom-scrollbar pr-2">
    <div class="flex items-center justify-between mt-4">
      <h1 class="text-3xl font-black text-white tracking-wide">Pengaturan</h1>
      <span class="text-xs font-mono text-gray-500 bg-black/50 px-3 py-1 rounded-full border border-[var(--color-subtle)]">v1.0.0-beta (Rust Engine)</span>
    </div>

    <!-- Layout Grid Utama -->
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
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

<style scoped>
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
