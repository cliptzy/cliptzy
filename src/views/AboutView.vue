<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import IconInfo from '~icons/lucide/info';
import IconCpu from '~icons/lucide/cpu';
import IconMemoryStick from '~icons/lucide/memory-stick';
import IconMonitorCheck from '~icons/lucide/monitor-check';
import IconMonitorX from '~icons/lucide/monitor-x';

interface SystemSpecs {
  meets_requirements: boolean;
  current_memory_gb: number;
  required_memory_gb: number;
  current_cpu_cores: number;
  required_cpu_cores: number;
  missing_reasons: string[];
}

const specs = ref<SystemSpecs | null>(null);

onMounted(async () => {
  try {
    specs.value = await invoke<SystemSpecs>('check_system_specs');
  } catch (error) {
    console.error('Failed to get system specs:', error);
  }
});
</script>

<template>
  <div class="h-full flex flex-col p-6 space-y-6 overflow-y-auto">
    <div class="flex items-center gap-4">
      <div class="w-12 h-12 rounded-none bg-base-200 border border-neutral flex items-center justify-center">
        <IconInfo class="w-6 h-6 text-base-content" />
      </div>
      <div>
        <h1 class="text-2xl font-bold tracking-tight text-base-content">About</h1>
        <p class="text-sm text-secondary">Application Info & System Specifications</p>
      </div>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-0 border-l border-r border-neutral">
      <div class="p-6 flex flex-col space-y-4 bg-base-100 border-b border-neutral">
        <h2 class="text-lg font-semibold text-base-content flex items-center gap-2">
          <IconInfo class="w-5 h-5 text-secondary" />
          Cliptzy Desktop
        </h2>
        <div class="space-y-2 text-sm text-secondary">
          <p><strong>Version:</strong> 0.1.0 (Alpha)</p>
          <p><strong>Architecture:</strong> Murni Rust & Tauri (Native)</p>
          <p><strong>UI Framework:</strong> Vue 3 + Tailwind CSS</p>
          <p class="pt-2 text-xs text-secondary">
            Aplikasi YouTube Clipper & Auto Uploader.
          </p>
        </div>
      </div>

      <div class="p-6 flex flex-col space-y-4 bg-base-100 border-b border-neutral">
        <h2 class="text-lg font-semibold text-base-content flex items-center gap-2">
          <component
            :is="specs?.meets_requirements ? IconMonitorCheck : IconMonitorX"
            class="w-5 h-5"
            :class="
              specs?.meets_requirements
                ? 'text-base-content'
                : 'text-error'
            "
          />
          System Requirements
        </h2>

        <div v-if="specs" class="space-y-4">
          <div
            class="flex items-center justify-between p-4 bg-base-200 border border-neutral rounded-none"
          >
            <div class="flex items-center gap-3">
              <IconMemoryStick class="w-5 h-5 text-secondary" />
              <div>
                <p class="text-sm font-medium text-base-content">
                  System Memory (RAM)
                </p>
                <p class="text-xs text-secondary">
                  Required: {{ specs.required_memory_gb }}GB |
                  Current: {{ specs.current_memory_gb.toFixed(1) }}GB
                </p>
              </div>
            </div>
            <div
              class="w-3 h-3 rounded-none"
              :class="
                specs.current_memory_gb >= 7.0
                  ? 'bg-base-content/10'
                  : 'bg-error/20'
              "
            ></div>
          </div>

          <div
            class="flex items-center justify-between p-4 bg-base-200 border border-neutral rounded-none"
          >
            <div class="flex items-center gap-3">
              <IconCpu class="w-5 h-5 text-secondary" />
              <div>
                <p class="text-sm font-medium text-base-content">
                  CPU Cores
                </p>
                <p class="text-xs text-secondary">
                  Required: {{ specs.required_cpu_cores }} Cores |
                  Current: {{ specs.current_cpu_cores }} Cores
                </p>
              </div>
            </div>
            <div
              class="w-3 h-3 rounded-none"
              :class="
                specs.current_cpu_cores >= specs.required_cpu_cores
                  ? 'bg-base-content/10'
                  : 'bg-error/20'
              "
            ></div>
          </div>

          <div
            v-if="!specs.meets_requirements"
            class="p-4 bg-error/10 border border-error/30 rounded-none text-error text-sm"
          >
            <p class="font-bold mb-1 text-error">
              Spesifikasi Minimal Tidak Terpenuhi:
            </p>
            <ul class="list-disc list-inside space-y-1 text-xs">
              <li v-for="reason in specs.missing_reasons" :key="reason">
                {{ reason }}
              </li>
            </ul>
          </div>
        </div>
        <div v-else class="text-sm text-secondary flex items-center justify-center h-full">
          Loading specs...
        </div>
      </div>
    </div>
  </div>
</template>


