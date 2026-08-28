<template>
  <div class="grid grid-cols-1 md:grid-cols-3 gap-6 pt-4">
  <BentoCard class="p-6 !bg-blue-200 dark:!bg-blue-900/40">
      <h3 class="text-sm font-black text-[var(--color-text-main)] uppercase tracking-wider mb-2 flex items-center gap-2">
        <IconActivity class="w-5 h-5 text-[var(--color-text-muted)]" /> Total Diproses
      </h3>
      <p class="text-4xl font-black text-[var(--color-text-main)]">24</p>
    </BentoCard>
  <BentoCard class="p-6 !bg-amber-200 dark:!bg-amber-900/40">
      <h3 class="text-sm font-black text-[var(--color-text-main)] uppercase tracking-wider mb-2 flex items-center gap-2">
        <IconHardDrive class="w-5 h-5 text-[var(--color-text-muted)]" /> Sisa Penyimpanan
      </h3>
      <p class="text-4xl font-black text-[var(--color-text-main)]">45 GB</p>
    </BentoCard>
  <BentoCard class="p-6 !bg-emerald-200 dark:!bg-emerald-900/40">
      <h3 class="text-sm font-black text-[var(--color-text-main)] uppercase tracking-wider mb-2 flex items-center gap-2">
        <IconLink class="w-5 h-5 text-[var(--color-text-muted)]" /> Platform Terhubung
      </h3>
      <div class="flex gap-2 mt-2">
        <div class="p-3 bg-white/50 dark:bg-black/30 rounded-xl shadow-sm"><IconYoutube class="w-6 h-6 text-red-600" /></div>
        <div class="p-3 bg-white/50 dark:bg-black/30 rounded-xl shadow-sm"><IconTiktok class="w-6 h-6 text-[var(--color-text-main)]" /></div>
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
