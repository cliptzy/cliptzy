<template>
  <div class="h-full w-full flex flex-row overflow-hidden bg-base-100">
    <!-- Left Sidebar (Navigation) -->
    <aside class="w-64 h-full border-r border-neutral bg-base-200 flex flex-col shrink-0">
      <div class="p-6 pb-2">
        <h1 class="text-xl font-bold tracking-tight text-base-content">Settings</h1>
      </div>
      <nav class="flex-1 overflow-y-auto py-4 px-3 flex flex-col gap-1 custom-scrollbar">
        <button
          v-for="tab in tabs"
          :key="tab.id"
          @click="activeTab = tab.id"
          class="w-full flex items-center justify-between px-3 py-2 text-sm text-left transition-colors"
          :class="activeTab === tab.id ? 'bg-base-300 text-base-content font-bold' : 'text-secondary hover:bg-base-300/50 hover:text-base-content'"
        >
          <div class="flex items-center gap-3">
            <component :is="tab.icon" class="w-4 h-4" :class="activeTab === tab.id ? 'text-primary' : ''" />
            <span>{{ tab.label }}</span>
          </div>
        </button>
      </nav>
    </aside>

    <!-- Right Content Area -->
    <main class="flex-1 h-full overflow-y-auto custom-scrollbar relative bg-base-100">
      <div class="max-w-4xl mx-auto p-8 lg:p-12 pb-24 flex flex-col gap-8">
        <!-- Render Active Component -->
        <component :is="activeComponent" />
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';

// Icons
import IconUser from '~icons/lucide/user';
import IconCpu from '~icons/lucide/cpu';
import IconBrain from '~icons/lucide/brain';
import IconCaptions from '~icons/lucide/type';
import IconScissors from '~icons/lucide/scissors';
import IconFileVideo from '~icons/lucide/file-video';
import IconLayers from '~icons/lucide/layers';
import IconUpload from '~icons/lucide/upload';
import IconImages from '~icons/lucide/image';

// Sections
import ProfileSection from '../components/settings/ProfileSection.vue';
import EngineSection from '../components/settings/EngineSection.vue';
import AISection from '../components/settings/AISection.vue';
import SubtitleSection from '../components/settings/SubtitleSection.vue';
import MediaSection from '../components/settings/MediaSection.vue';
import OutputSection from '../components/settings/OutputSection.vue';
import CompilationSettingsSection from '../components/settings/CompilationSettingsSection.vue';
import UploadSection from '../components/settings/UploadSection.vue';
import BrollAssetsSection from '../components/settings/BrollAssetsSection.vue';

const tabs = [
  { id: 'profile', label: 'Profil Sistem', icon: IconUser, component: ProfileSection },
  { id: 'engine', label: 'Engine & Storage', icon: IconCpu, component: EngineSection },
  { id: 'ai', label: 'AI & Automasi', icon: IconBrain, component: AISection },
  { id: 'subtitle', label: 'Subtitle & Teks', icon: IconCaptions, component: SubtitleSection },
  { id: 'media', label: 'Standar Pemotongan', icon: IconScissors, component: MediaSection },
  { id: 'broll', label: 'Aset B-Roll', icon: IconImages, component: BrollAssetsSection },
  { id: 'output', label: 'Render & Output', icon: IconFileVideo, component: OutputSection },
  { id: 'compilation', label: 'Mode Kompilasi', icon: IconLayers, component: CompilationSettingsSection },
  { id: 'upload', label: 'Auto Upload', icon: IconUpload, component: UploadSection },
];

const activeTab = ref(tabs[0].id);

const activeComponent = computed(() => {
  return tabs.find((t) => t.id === activeTab.value)?.component || ProfileSection;
});
</script>


