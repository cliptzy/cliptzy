<template>
 <div
 class="bg-base-100 "
 >
 <h2 class="text-lg font-black text-base-content tracking-wide flex items-center gap-2 shrink-0">
 <IconMonitor class="w-5 h-5" /> Output & Pipeline
 </h2>

 <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
 <div class="flex flex-col gap-1.5">
 <span class="text-[10px] text-secondary uppercase font-bold">Folder Output</span>
 <input v-model="settings.config.output_dir" type="text" class="w-full bg-base-200 border border-neutral rounded-none p-3 text-sm font-bold focus:outline-none focus:ring-2 focus:ring-primary " />
 </div>
 <div class="flex flex-col gap-1.5">
 <span class="text-[10px] text-secondary uppercase font-bold">Rasio Output Default</span>
 <select
 v-model="settings.config.output_ratio"
 @change="onRatioChange"
 class="w-full bg-base-200 border border-neutral rounded-none p-3 text-sm font-bold focus:outline-none focus:ring-2 focus:ring-primary cursor-pointer "
 >
 <option value="9:16">9:16 (Shorts)</option>
 <option value="1:1">1:1 (Square)</option>
 <option value="16:9">16:9 (Landscape)</option>
 <option value="original">Original</option>
 </select>
 </div>
 <div class="flex flex-col gap-1.5">
 <span class="text-[10px] text-secondary uppercase font-bold">Lebar Output (px)</span>
 <input v-model.number="settings.config.out_width" type="number" min="0" placeholder="Auto" class="w-full bg-base-200 border border-neutral rounded-none p-3 text-sm font-bold focus:outline-none focus:ring-2 focus:ring-primary " />
 </div>
 <div class="flex flex-col gap-1.5">
 <span class="text-[10px] text-secondary uppercase font-bold">Tinggi Output (px)</span>
 <input v-model.number="settings.config.out_height" type="number" min="0" placeholder="Auto" class="w-full bg-base-200 border border-neutral rounded-none p-3 text-sm font-bold focus:outline-none focus:ring-2 focus:ring-primary " />
 </div>
 <div class="flex flex-col gap-1.5">
 <span class="text-[10px] text-secondary uppercase font-bold">Skor Minimum ({{ settings.config.min_score }})</span>
 <input type="range" min="0" max="1" step="0.05" v-model.number="settings.config.min_score" class="w-full h-2 bg-neutral rounded-none appearance-none cursor-pointer mt-2 accent-primary" />
 </div>
 <div class="flex flex-col gap-1.5">
 <span class="text-[10px] text-secondary uppercase font-bold">Maks Klip ({{ settings.config.max_clips }})</span>
 <input type="range" min="1" max="50" v-model.number="settings.config.max_clips" class="w-full h-2 bg-neutral rounded-none appearance-none cursor-pointer mt-2 accent-primary" />
 </div>
 <div class="flex flex-col gap-1.5">
 <span class="text-[10px] text-secondary uppercase font-bold">Interval Upload ({{ settings.config.upload_interval }}s)</span>
 <input type="range" min="0" max="300" step="5" v-model.number="settings.config.upload_interval" class="w-full h-2 bg-neutral rounded-none appearance-none cursor-pointer mt-2 accent-primary" />
 </div>
 <div class="flex flex-col gap-1.5">
 <span class="text-[10px] text-secondary uppercase font-bold">Split Stack (Top/Bottom)</span>
 <div class="flex gap-2">
 <input v-model.number="settings.config.top_height" type="number" class="w-1/2 bg-base-200 border border-neutral rounded-none p-2 text-xs font-bold" placeholder="Top" />
 <input v-model.number="settings.config.bottom_height" type="number" class="w-1/2 bg-base-200 border border-neutral rounded-none p-2 text-xs font-bold" placeholder="Bottom" />
 </div>
 </div>
 </div>

 <div class="flex flex-col gap-3 pt-2 border-t border-neutral dark:border-neutral">
 <label class="flex items-center justify-between bg-base-200/50 p-3 rounded-none cursor-pointer">
 <span class="text-sm font-bold text-base-content">Gabung Klip (Merge)</span>
 <CToggle v-model="settings.config.merge_clips" />
 </label>
 <label class="flex items-center justify-between bg-base-200/50 p-3 rounded-none cursor-pointer">
 <span class="text-sm font-bold text-base-content">Kunci UI (UI Locked)</span>
 <CToggle v-model="settings.config.ui_locked" />
 </label>
 </div>
 </div>
</template>

<script setup lang="ts">
import { useSettingsStore } from "../../stores/settings";
import IconMonitor from "~icons/lucide/monitor";

const settings = useSettingsStore();

const onRatioChange = () => {
 settings.setRatioPreset(settings.config.output_ratio);
};
</script>


