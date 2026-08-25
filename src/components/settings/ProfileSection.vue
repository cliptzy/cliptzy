<template>
  <div class="flex flex-col gap-6">
    <!-- Profil Sistem -->
    <section class="flex flex-col gap-3">
      <h2 class="text-sm font-bold text-gray-400 uppercase tracking-widest flex items-center gap-2">
        <IconUser class="w-4 h-4 text-[var(--color-accent)]" /> Profil Sistem
      </h2>
      <BentoCard class="p-5 flex items-center justify-between group">
        <div class="flex items-center gap-4">
          <img v-if="auth.avatarUrl" :src="auth.avatarUrl" class="w-12 h-12 rounded-full border-2 border-[var(--color-subtle)]" />
          <div v-else class="w-12 h-12 rounded-full bg-[var(--color-accent)]/20 flex items-center justify-center text-[var(--color-accent)] font-bold text-xl border-2 border-[var(--color-subtle)]">
            {{ auth.displayName ? auth.displayName.charAt(0).toUpperCase() : 'U' }}
          </div>
          <div class="flex flex-col min-w-0">
            <span class="font-bold text-white text-base truncate">{{ auth.displayName || 'Google User' }}</span>
            <span class="text-xs text-gray-400 truncate">{{ auth.email || 'Email tidak tersedia' }}</span>
          </div>
        </div>
        <div class="flex flex-col items-end gap-2">
          <span class="text-[10px] font-medium text-green-400 bg-green-400/10 px-2 py-0.5 rounded border border-green-400/20">Tersinkronisasi</span>
          <button @click="handleLogout" class="text-[10px] text-red-500 hover:underline">Logout</button>
        </div>
      </BentoCard>
    </section>

    <!-- Akun Sosial -->
    <section class="flex flex-col gap-3">
      <h2 class="text-sm font-bold text-gray-400 uppercase tracking-widest flex items-center gap-2">
        <IconLink class="w-4 h-4 text-[var(--color-accent)]" /> Akun Sosial
      </h2>
      <div class="flex flex-col gap-3">
        <BentoCard class="p-4 flex items-center justify-between group hover:border-gray-500 transition-colors">
          <div class="flex items-center gap-3">
            <div class="w-8 h-8 rounded-full bg-red-500/10 flex items-center justify-center"><IconYoutube class="w-4 h-4 text-red-500" /></div>
            <div class="flex flex-col"><span class="font-bold text-white text-sm">YouTube</span><span class="text-[10px] text-gray-400">@cliptzy_official</span></div>
          </div>
          <span class="relative flex h-2 w-2"><span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-green-400 opacity-75"></span><span class="relative inline-flex rounded-full h-2 w-2 bg-green-500"></span></span>
        </BentoCard>
        <BentoCard class="p-4 flex items-center justify-between group hover:border-gray-500 transition-colors">
          <div class="flex items-center gap-3">
            <div class="w-8 h-8 rounded-full bg-white/5 border border-[var(--color-subtle)] flex items-center justify-center"><IconTiktok class="w-4 h-4 text-white" /></div>
            <div class="flex flex-col"><span class="font-bold text-white text-sm">TikTok</span><span class="text-[10px] text-yellow-500">Perlu Login</span></div>
          </div>
          <button class="bg-[var(--color-accent)] text-black text-[10px] font-bold px-2 py-1 rounded hover:scale-105 transition-transform">Hubungkan</button>
        </BentoCard>
      </div>
    </section>

    <!-- Penyimpanan -->
    <section class="flex flex-col gap-3">
      <h2 class="text-sm font-bold text-gray-400 uppercase tracking-widest flex items-center gap-2">
        <IconHardDrive class="w-4 h-4 text-[var(--color-accent)]" /> Penyimpanan
      </h2>
      <BentoCard class="p-5 flex flex-col items-center">
        <div class="relative w-32 h-32 flex items-center justify-center">
          <svg class="w-full h-full transform -rotate-90" viewBox="0 0 100 100">
            <circle cx="50" cy="50" r="40" stroke="rgba(255,255,255,0.05)" stroke-width="8" fill="none" />
            <circle cx="50" cy="50" r="40" stroke="var(--color-accent)" stroke-width="8" fill="none" stroke-linecap="round" stroke-dasharray="251.2" stroke-dashoffset="62.8" class="transition-all duration-1000 ease-out shadow-[0_0_15px_var(--color-accent)]" />
          </svg>
          <div class="absolute inset-0 flex flex-col items-center justify-center">
            <span class="text-xl font-black text-white">4.2</span>
            <span class="text-[9px] text-gray-400 font-medium">GB Terpakai</span>
          </div>
        </div>
        <button @click="clearCache" class="mt-4 w-full py-2 rounded border border-red-500/30 text-red-400 text-xs font-bold hover:bg-red-500 hover:text-white transition-colors flex items-center justify-center gap-2">
          <IconTrash2 class="w-3 h-3" /> Bersihkan Cache
        </button>
      </BentoCard>
    </section>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router';
import { useAppStore } from '../../stores/app';
import { useAuthStore } from '../../stores/auth';
import BentoCard from '../BentoCard.vue';

// Icons
import IconUser from '~icons/lucide/user';
import IconLink from '~icons/lucide/link';
import IconHardDrive from '~icons/lucide/hard-drive';
import IconYoutube from '~icons/lucide/youtube';
import IconTiktok from '~icons/lucide/smartphone';
import IconTrash2 from '~icons/lucide/trash-2';

const appStore = useAppStore();
const auth = useAuthStore();
const router = useRouter();

const clearCache = () => {
  appStore.addToast({
    type: 'success',
    title: 'Cache Cleared',
    message: '3.1 GB of temporary files have been successfully deleted.',
    duration: 3000
  });
};

const handleLogout = async () => {
  await auth.logout();
  router.push('/login');
};
</script>
