import type { SubtitleConfig } from "../stores/settings";

/** Preset gaya subtitle — memetakan ke `animation` + `border_style` yang dipakai backend ASS. */
export type SubtitlePresetId = "plain" | "hormozi" | "karaoke" | "brutalist";

export const SUBTITLE_PRESETS: {
  id: SubtitlePresetId;
  label: string;
  description: string;
}[] = [
  {
    id: "plain",
    label: "Plain",
    description: "Teks sederhana tanpa animasi kata",
  },
  {
    id: "hormozi",
    label: "Hormozi",
    description: "Kata aktif disorot & huruf kapital",
  },
  {
    id: "karaoke",
    label: "Karaoke",
    description: "Kata aktif disorot satu per satu",
  },
  {
    id: "brutalist",
    label: "Brutalist Box",
    description: "Kotak tebal dengan gaya retro",
  },
];

export const SUBTITLE_FONTS = [
  { value: "Arial", label: "Arial" },
  { value: "Impact", label: "Impact" },
  { value: "Bangers", label: "Bangers" },
  { value: "Inter", label: "Inter" },
  { value: "TheBoldFont", label: "TheBoldFont" },
  { value: "Courier New", label: "Courier New" },
] as const;

export const SUBTITLE_LOCATIONS = [
  { value: "bottom", label: "Bawah" },
  { value: "center", label: "Tengah" },
  { value: "top", label: "Atas" },
] as const;

export function detectSubtitlePreset(
  animation: string,
  borderStyle: number,
): SubtitlePresetId {
  if (borderStyle === 3) return "brutalist";
  if (animation === "hormozi") return "hormozi";
  if (animation === "karaoke") return "karaoke";
  return "plain";
}

/** Sinkronkan `animation`, `border_style`, dan `style` (legacy) dari satu preset. */
export function applySubtitlePreset(
  preset: SubtitlePresetId,
  subtitle: SubtitleConfig,
): void {
  switch (preset) {
    case "hormozi":
      subtitle.animation = "hormozi";
      subtitle.border_style = 1;
      subtitle.style = "plain";
      break;
    case "karaoke":
      subtitle.animation = "karaoke";
      subtitle.border_style = 1;
      subtitle.style = "karaoke";
      break;
    case "brutalist":
      subtitle.animation = "none";
      subtitle.border_style = 3;
      subtitle.style = "boxed";
      break;
    case "plain":
    default:
      subtitle.animation = "none";
      subtitle.border_style = 1;
      subtitle.style = "plain";
      break;
  }
}

/** Konversi warna ASS (`&HAABBGGRR`) ke hex `#RRGGBB`. */
export function assToHex(ass: string): string {
  if (!ass || ass.length < 10 || !ass.startsWith("&H")) return "#FFFFFF";
  const bb = ass.substring(4, 6);
  const gg = ass.substring(6, 8);
  const rr = ass.substring(8, 10);
  return `#${rr}${gg}${bb}`;
}

/** Opacity 0–100 dari channel alpha ASS (00 = opaque, FF = transparan). */
export function assToOpacity(ass: string): number {
  if (!ass || ass.length < 4 || !ass.startsWith("&H")) return 100;
  const aa = Number.parseInt(ass.substring(2, 4), 16);
  if (Number.isNaN(aa)) return 100;
  return Math.round((1 - aa / 255) * 100);
}

/** Konversi hex `#RRGGBB` + opacity 0–100 ke format ASS `&HAABBGGRR`. */
export function hexToAss(hex: string, opacity = 100): string {
  const clean = hex.replace("#", "").padStart(6, "0").slice(0, 6);
  const rr = clean.substring(0, 2);
  const gg = clean.substring(2, 4);
  const bb = clean.substring(4, 6);
  const alpha = Math.round((1 - opacity / 100) * 255);
  const aa = alpha.toString(16).padStart(2, "0").toUpperCase();
  return `&H${aa}${bb}${gg}${rr}`.toUpperCase();
}
