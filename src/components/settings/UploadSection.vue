<template>
  <BentoCard
    class="col-span-1 md:col-span-2 xl:col-span-4 row-span-2 h-full overflow-y-auto custom-scrollbar p-6 flex flex-col gap-5"
  >
    <h2 class="text-lg font-black text-[var(--color-text-main)] tracking-wide flex items-center gap-2 shrink-0">
      <IconUpload class="w-5 h-5" /> Auto Upload Platform
    </h2>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-4">
      <!-- YouTube -->
      <div class="flex flex-col gap-3 bg-white/50 dark:bg-black/30 p-4 rounded-2xl">
        <div class="flex items-center justify-between">
          <span class="text-sm font-bold flex items-center gap-2"><IconYoutube class="w-4 h-4 text-red-500" /> YouTube</span>
          <ToggleSwitch v-model="settings.config.youtube.upload" />
        </div>
        <label class="flex items-center justify-between text-xs font-bold">
          <span>Auto Upload</span>
          <ToggleSwitch v-model="settings.config.youtube.auto_upload" />
        </label>
        <select v-model="settings.config.youtube.visibility" class="w-full bg-white/60 dark:bg-black/30 border-none rounded-xl p-2 text-xs font-bold cursor-pointer">
          <option value="Public">Public</option>
          <option value="Unlisted">Unlisted</option>
          <option value="Private">Private</option>
        </select>
        <input v-model="settings.config.youtube.client_id" placeholder="Client ID" class="w-full bg-white/60 dark:bg-black/30 border-none rounded-xl p-2 text-xs font-bold" />
        <input v-model="settings.config.youtube.client_secret" type="password" placeholder="Client Secret" class="w-full bg-white/60 dark:bg-black/30 border-none rounded-xl p-2 text-xs font-bold" />
        <input v-model="settings.config.youtube.session" placeholder="Session cookies path" class="w-full bg-white/60 dark:bg-black/30 border-none rounded-xl p-2 text-xs font-bold" />
      </div>

      <!-- TikTok -->
      <div class="flex flex-col gap-3 bg-white/50 dark:bg-black/30 p-4 rounded-2xl">
        <div class="flex items-center justify-between">
          <span class="text-sm font-bold flex items-center gap-2"><IconTiktok class="w-4 h-4" /> TikTok</span>
          <ToggleSwitch v-model="settings.config.tiktok.upload" />
        </div>
        <label class="flex items-center justify-between text-xs font-bold">
          <span>Auto Upload</span>
          <ToggleSwitch v-model="settings.config.tiktok.auto_upload" />
        </label>
        <select v-model="settings.config.tiktok.privacy" class="w-full bg-white/60 dark:bg-black/30 border-none rounded-xl p-2 text-xs font-bold cursor-pointer">
          <option value="Public (Semua Orang)">Public</option>
          <option value="Friends">Friends</option>
          <option value="Private">Private</option>
        </select>
        <input v-model="settings.config.tiktok.session" placeholder="Session cookies path" class="w-full bg-white/60 dark:bg-black/30 border-none rounded-xl p-2 text-xs font-bold" />
      </div>

      <!-- Instagram -->
      <div class="flex flex-col gap-3 bg-white/50 dark:bg-black/30 p-4 rounded-2xl">
        <div class="flex items-center justify-between">
          <span class="text-sm font-bold flex items-center gap-2"><IconInstagram class="w-4 h-4 text-pink-500" /> Instagram</span>
          <ToggleSwitch v-model="settings.config.instagram.upload" />
        </div>
        <label class="flex items-center justify-between text-xs font-bold">
          <span>Auto Upload</span>
          <ToggleSwitch v-model="settings.config.instagram.auto_upload" />
        </label>
        <input v-model="settings.config.instagram.business_id" placeholder="Business ID" class="w-full bg-white/60 dark:bg-black/30 border-none rounded-xl p-2 text-xs font-bold" />
        <input v-model="settings.config.instagram.access_token" type="password" placeholder="Access Token" class="w-full bg-white/60 dark:bg-black/30 border-none rounded-xl p-2 text-xs font-bold" />
        <input v-model="settings.config.instagram.session" placeholder="Session cookies path" class="w-full bg-white/60 dark:bg-black/30 border-none rounded-xl p-2 text-xs font-bold" />
      </div>
    </div>
  </BentoCard>
</template>

<script setup lang="ts">
import BentoCard from "../BentoCard.vue";
import ToggleSwitch from "../ToggleSwitch.vue";
import { useSettingsStore } from "../../stores/settings";
import IconUpload from "~icons/lucide/upload";
import IconYoutube from "~icons/lucide/youtube";
import IconTiktok from "~icons/lucide/smartphone";
import IconInstagram from "~icons/lucide/instagram";

const settings = useSettingsStore();
</script>
