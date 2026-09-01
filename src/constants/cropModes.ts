/**
 * Crop mode definitions for the clipper mode inspector panel.
 * Each entry includes a human-readable label, icon, and description
 * displayed in the UI so users can differentiate between layouts.
 */
export interface CropModeOption {
  value: string;
  label: string;
  icon: string;
  description: string;
  requiresFaces: boolean;
  requiresBroll: boolean;
  isBeta: boolean;
}

export const CROP_MODES: CropModeOption[] = [
  {
    value: "default",
    label: "Center Crop",
    icon: "🎯",
    description: "Crop the center of the video to match the target aspect ratio.",
    requiresFaces: false,
    requiresBroll: false,
    isBeta: false,
  },
  {
    value: "full",
    label: "Full (Letterbox)",
    icon: "📺",
    description: "Keep the full video with a letterboxed/pillarboxed background.",
    requiresFaces: false,
    requiresBroll: false,
    isBeta: false,
  },
  {
    value: "full_face",
    label: "Full Face",
    icon: "🎭",
    description: "Letterbox layout with smooth dynamic face tracking overlay.",
    requiresFaces: true,
    requiresBroll: false,
    isBeta: false,
  },
  {
    value: "center_face",
    label: "Center Face (Track)",
    icon: "👤",
    description: "Center crop that dynamically follows the detected face.",
    requiresFaces: true,
    requiresBroll: false,
    isBeta: false,
  },
  {
    value: "split_face",
    label: "Split Face (Gameplay + Facecam)",
    icon: "🎮",
    description:
      "Top half shows center-cropped content; bottom half shows dynamic facecam tracking.",
    requiresFaces: true,
    requiresBroll: false,
    isBeta: false,
  },
  {
    value: "multi_face",
    label: "Multi Face (Podcast)",
    icon: "👥",
    description:
      "Top two-thirds show both faces; bottom splits into two panels tracking each face individually.",
    requiresFaces: true,
    requiresBroll: false,
    isBeta: false,
  },
  {
    value: "split_broll",
    label: "Split B-roll (Main + B-roll)",
    icon: "🎬",
    description:
      "Top half shows center-cropped main video; bottom half plays a random B-roll loop.",
    requiresFaces: false,
    requiresBroll: true,
    isBeta: true,
  },
  {
    value: "passthrough",
    label: "Passthrough (No Crop)",
    icon: "➡️",
    description: "Keep original resolution without any cropping applied.",
    requiresFaces: false,
    requiresBroll: false,
    isBeta: false,
  },
] as const;

/**
 * Helper: get the display metadata for a given crop mode value.
 */
export function getCropModeInfo(mode: string): CropModeOption | undefined {
  return CROP_MODES.find((m) => m.value === mode);
}
