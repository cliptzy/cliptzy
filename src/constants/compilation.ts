export const COMPILATION_TYPES = [
  {
    value: "reaction",
    label: "Reaksi Restreamer (16:9, tanpa crop)",
    description: "Video horizontal penuh, hanya trim durasi. Cocok untuk MPL ID / VOD panjang.",
  },
  {
    value: "meme_shorts",
    label: "Meme Shorts (Vertikal)",
    description: "Kompilasi meme vertikal seperti top moments / windah basudara.",
  },
] as const;

export const COMPILATION_CROP_MODES = [
  { value: "none", label: "Tanpa Crop (Resolusi Asli)" },
  { value: "default", label: "Center Crop" },
  { value: "center_face", label: "Center Face" },
  { value: "full", label: "Full + Blur Background" },
  { value: "full_face", label: "Face Track + Full" },
  { value: "split_face", label: "Split Face (Gameplay + Facecam)" },
  { value: "multi_face", label: "Multi Face (Podcast 2 Orang)" },
  { value: "split_broll", label: "Split B-roll (Main + B-roll)" },
] as const;

export type CompilationType = (typeof COMPILATION_TYPES)[number]["value"];

export function isReactionCompilation(type: string): boolean {
  return type === "reaction";
}

export function applyCompilationTypeDefaults(
  type: CompilationType,
  compilation: {
    compilation_type: string;
    crop_mode: string;
    max_segment_duration: number;
    use_tts?: boolean;
    output_ratio?: string;
  },
) {
  compilation.compilation_type = type;
  if (type === "reaction") {
    compilation.crop_mode = "none";
    compilation.max_segment_duration = 0;
    if (compilation.use_tts !== undefined) {
      compilation.use_tts = false;
    }
  } else {
    compilation.crop_mode = compilation.crop_mode === "none" ? "default" : compilation.crop_mode;
    if (compilation.max_segment_duration === 0) {
      compilation.max_segment_duration = 180;
    }
  }
}
