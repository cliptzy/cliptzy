import { defineStore } from 'pinia';
import { ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';

export interface SubtitleConfig {
  enabled: boolean;
  style: string;
  whisper_model: string;
  font: string;
  fonts_dir: string | null;
  location: string;
  delay: number;
  font_size: number;
  color: string;
  bg_color: string;
  border_style: number;
  animation: string;
  max_words: number;
}

export interface AIConfig {
  provider: string;
  ollama_host: string;
  ollama_model: string;
  gemini_key: string;
  gemini_model: string;
  openai_key: string;
  openai_model: string;
  openai_base_url: string;
  use_highlight: boolean;
  use_generate_intro: boolean;
  use_emotion_detection: boolean;
  use_voice_analysis: boolean;
  use_audio_analysis: boolean;
  use_text_analysis: boolean;
  use_add_meme: boolean;
}

export interface YoutubeConfig {
  upload: boolean;
  session: string | null;
  client_id: string;
  client_secret: string;
  visibility: string;
  auto_upload: boolean;
}

export interface TikTokConfig {
  upload: boolean;
  session: string;
  privacy: string;
  auto_upload: boolean;
}

export interface InstagramConfig {
  upload: boolean;
  business_id: string;
  access_token: string;
  session: string;
  auto_upload: boolean;
}

export interface CompilationConfig {
  ordering: string;
  numbering_duration: number;
  use_tts: boolean;
  tts_template: string;
  use_subtitle: boolean;
  crop_mode: string;
}

export interface AppConfig {
  output_dir: string;
  min_duration: number;
  min_score: number;
  max_clips: number;
  padding: number;
  top_height: number;
  bottom_height: number;
  
  intro_video: string | null;
  outro_video: string | null;
  watermark_image: string | null;
  video_frame: string | null;
  watermark_position: string;
  
  output_ratio: string;
  out_width: number | null;
  out_height: number | null;
  
  job_dir: string;
  crop_mode: string;
  face_tracking_mode: string;
  merge_clips: boolean;
  ui_locked: boolean;
  
  upload_interval: number;
  hw_accel: string;
  debug_mode: boolean;
  max_workers: number;
  
  tts_language: string;
  tts_voice: string;
  default_hashtags: string;
  browser: string | null;
  
  subtitle: SubtitleConfig;
  ai: AIConfig;
  youtube: YoutubeConfig;
  tiktok: TikTokConfig;
  instagram: InstagramConfig;
  compilation: CompilationConfig;
}

const defaultSettings: AppConfig = {
  output_dir: 'clips',
  min_duration: 60,
  min_score: 0.40,
  max_clips: 10,
  padding: 10,
  top_height: 960,
  bottom_height: 320,
  
  intro_video: null,
  outro_video: null,
  watermark_image: null,
  video_frame: null,
  watermark_position: "center",
  
  output_ratio: "9:16",
  out_width: 720,
  out_height: 1280,
  
  job_dir: "",
  crop_mode: "default",
  face_tracking_mode: "cinematic",
  merge_clips: false,
  ui_locked: false,
  
  upload_interval: 0.0,
  hw_accel: "cpu",
  debug_mode: false,
  max_workers: 2,
  
  tts_language: "default",
  tts_voice: "female",
  default_hashtags: "#Shorts #Viral #Cliptzy #fyp",
  browser: null,
  
  subtitle: {
    enabled: true,
    style: "plain",
    whisper_model: "small",
    font: "Arial",
    fonts_dir: null,
    location: "bottom",
    delay: 0.0,
    font_size: 60,
    color: "&H0000FFFF",
    bg_color: "&H80000000",
    border_style: 3,
    animation: "none",
    max_words: 3
  },
  ai: {
    provider: "ollama",
    ollama_host: "http://localhost:11434",
    ollama_model: "llama3",
    gemini_key: "",
    gemini_model: "gemini-1.5-flash",
    openai_key: "",
    openai_model: "gpt-4o-mini",
    openai_base_url: "",
    use_highlight: false,
    use_generate_intro: false,
    use_emotion_detection: true,
    use_voice_analysis: true,
    use_audio_analysis: true,
    use_text_analysis: true,
    use_add_meme: true
  },
  youtube: {
    upload: false,
    session: "cred/yt_cookies.txt",
    client_id: "",
    client_secret: "",
    visibility: "Public",
    auto_upload: false
  },
  tiktok: {
    upload: false,
    session: "cred/tiktok_cookies.txt",
    privacy: "Public (Semua Orang)",
    auto_upload: false
  },
  instagram: {
    upload: false,
    business_id: "",
    access_token: "",
    session: "cred/instagram_cookies.txt",
    auto_upload: false
  },
  compilation: {
    ordering: "countdown",
    numbering_duration: 3.0,
    use_tts: true,
    tts_template: "Nomor {n}! {name}!",
    use_subtitle: true,
    crop_mode: "default"
  }
};

import { useAppStore } from './app';

export const useSettingsStore = defineStore('settings', () => {
  const config = ref<AppConfig>(defaultSettings);
  const isLoaded = ref(false);
  const appStore = useAppStore();
  
  const loadFromBackend = async () => {
    try {
      const json = await invoke<string>('load_config_file');
      config.value = JSON.parse(json) as AppConfig;
    } catch (e) {
      console.error("Failed to load config from backend, using defaults:", e);
      config.value = defaultSettings;
    } finally {
      isLoaded.value = true;
    }
  };

  const toDict = () => {
    return JSON.parse(JSON.stringify(config.value));
  };
  
  const setRatioPreset = (preset: string) => {
    config.value.output_ratio = preset;
    if (preset === '9:16') {
      config.value.out_width = 720;
      config.value.out_height = 1280;
    } else if (preset === '1:1') {
      config.value.out_width = 720;
      config.value.out_height = 720;
    } else if (preset === '16:9') {
      config.value.out_width = 1280;
      config.value.out_height = 720;
    } else if (preset === 'original') {
      config.value.out_width = null;
      config.value.out_height = null;
    }
  };

  let debounceTimer: any = null;
  // Watch for changes and sync to Rust backend ONLY if initial load is done
  watch(() => config.value, (newConfig) => {
    if (!isLoaded.value) return; // Jangan save config default saat aplikasi baru start

    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(async () => {
      try {
        await invoke('save_config_file', { configJson: JSON.stringify(newConfig) });
        appStore.addToast({
          title: 'Pengaturan Tersimpan',
          message: 'Konfigurasi berhasil disinkronisasi ke engine Rust.',
          type: 'success',
          duration: 3000
        });
      } catch (e: any) {
        appStore.addToast({
          title: 'Gagal Menyimpan',
          message: String(e),
          type: 'error',
          duration: 5000
        });
        console.error("Failed to save config to backend:", e);
      }
    }, 1500);
  }, { deep: true });

  return { 
    config, 
    isLoaded,
    loadFromBackend,
    toDict,
    setRatioPreset 
  };
});
