<template>
  <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
    <BentoCard class="p-6 bg-[var(--color-surface)]">
      <h3 class="text-sm font-bold text-gray-400 uppercase tracking-wider mb-2 flex items-center gap-2">
        <IconActivity class="w-4 h-4 text-[var(--color-accent)]" /> Total Diproses
      </h3>
      <p class="text-4xl font-black text-white">24</p>
    </BentoCard>
    <BentoCard class="p-6 bg-[var(--color-surface)]">
      <h3 class="text-sm font-bold text-gray-400 uppercase tracking-wider mb-2 flex items-center gap-2">
        <IconHardDrive class="w-4 h-4 text-blue-400" /> Sisa Penyimpanan
      </h3>
      <p class="text-4xl font-black text-white">45 GB</p>
    </BentoCard>
    <BentoCard class="p-6 bg-[var(--color-surface)]">
      <h3 class="text-sm font-bold text-gray-400 uppercase tracking-wider mb-2 flex items-center gap-2">
        <IconLink class="w-4 h-4 text-green-400" /> Platform Terhubung
      </h3>
      <div class="flex gap-2 mt-2">
        <div class="p-2 bg-white/10 rounded-lg"><IconYoutube class="w-5 h-5 text-red-500" /></div>
        <div class="p-2 bg-white/10 rounded-lg"><IconTiktok class="w-5 h-5 text-white" /></div>
      </div>
    </BentoCard>
  </div>
</template>

<script setup lang="ts">
import BentoCard from '../BentoCard.vue';
import IconActivity from '~icons/lucide/activity';
import IconHardDrive from '~icons/lucide/hard-drive';
import IconLink from '~icons/lucide/link';
import IconYoutube from '~icons/lucide/youtube';
import IconTiktok from '~icons/lucide/smartphone';
</script>
