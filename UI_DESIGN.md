# 🎨 UI_DESIGN.md — Cliptzy Visual Identity Redesign

> **Codename**: "Cutting Room"
> **Direction**: Dark-first terminal-tool aesthetic inspired by [poolside.ai](https://poolside.ai) and [opencode.ai](https://opencode.ai), adapted for a video-centric desktop app.

---

## 1. Design Brief & Grounding

**Product**: Cliptzy — a desktop app (Tauri + Vue 3) for YouTube content creators who clip, edit, and redistribute video segments. The audience is creators who understand tools, not casual users. They value speed, clarity, and a workspace that stays out of their way.

**Single Job of the UI**: Make it effortless to go from a YouTube URL to a rendered, subtitled short-form clip — in as few cognitive steps as possible.

**Aesthetic North Star**: The references (Poolside, OpenCode) share a specific language:
- Deep, warm dark backgrounds (not cold blue-blacks)
- Restrained, muted accent colors
- Monospace or semi-monospace typography for interface chrome
- Content density that respects the user's expertise
- Borders and dividers that whisper, not shout
- Spatial clarity through negative space, not decoration

This is **not** a marketing site redesign. This is a **workspace tool** redesign. The reference sites inform the *visual language* (color, type, surface treatment), but the layout must serve a desktop app with sidebars, panels, and real-time video preview.

---

## 2. Design Token System

### 2.1 Color Palette

The current design uses Pastel Cream light (`#FAF4F4`) and cold Slate dark (`#0F172A`). Both themes will be overhauled. The new palette draws from the warm charcoal and copper tones of the references.

| Token Name         | Dark Mode          | Light Mode         | Role                                    |
|--------------------|--------------------|--------------------|------------------------------------------|
| `base-100`         | `#1A1816`          | `#FAFAF8`          | App background, deepest layer            |
| `base-200`         | `#242220`          | `#F2F1EE`          | Panel/card surfaces                      |
| `base-300`         | `#302D2A`          | `#E8E6E2`          | Elevated surfaces, hover states          |
| `base-content`     | `#E8E4DF`          | `#1C1917`          | Primary text                             |
| `primary`          | `#D4845A`          | `#C06830`          | Accent (warm copper/terracotta)          |
| `primary-content`  | `#1A1816`          | `#FFFFFF`          | Text on primary                          |
| `secondary`        | `#8B8685`          | `#78716C`          | Muted UI elements, secondary text        |
| `accent`           | `#7AADBA`          | `#4A909E`          | Info/status highlights (teal)            |
| `error`            | `#E06C6C`          | `#DC2626`          | Destructive actions, errors              |
| `success`          | `#6BBF6B`          | `#16A34A`          | Completion states                        |
| `warning`          | `#E0B84A`          | `#CA8A04`          | Caution indicators                       |
| `neutral`          | `#3A3734`          | `#D6D3CE`          | Dividers, subtle borders                 |

**Design rationale**: The warm charcoal `#1A1816` (vs cold `#0F172A`) mirrors Poolside/OpenCode's signature warmth. The copper `#D4845A` accent is pulled from the muted amber tones visible in the OpenCode screenshot — it reads as "crafted tool" rather than "SaaS product". The teal secondary avoids the default acid-green cliché.

### 2.2 Typography

**Current state**: Inter (body) + Plus Jakarta Sans (display) — both are safe, geometric sans-serifs that could appear on any dashboard. Neither carries personality.

**New type system**:

| Role       | Typeface                     | Weight(s)         | Usage                                       |
|------------|------------------------------|-------------------|----------------------------------------------|
| Display    | **Geist Sans** (already installed) | 600, 700      | Page titles, section headers, hero text      |
| Body       | **Inter** (keep)             | 400, 500          | Body text, descriptions, form labels         |
| Mono/UI    | **Geist Mono** *(new)*       | 400               | Timestamps, file names, status bar, metadata |

**Why Geist**: Vercel's Geist family is purpose-built for developer tooling — slightly narrower than Inter, with sharper terminals that read as technical without being hostile. Geist Mono replaces the need for JetBrains Mono while staying in-family. The `@fontsource/geist-sans` is already installed but underutilized.

**Type Scale** (rem, base 16px):

```
--text-xs:    0.6875rem / 11px  — status bar, badges
--text-sm:    0.8125rem / 13px  — secondary labels, timestamps
--text-base:  0.9375rem / 15px  — body text, inputs
--text-lg:    1.125rem  / 18px  — section titles
--text-xl:    1.5rem    / 24px  — page headings
--text-2xl:   2rem      / 32px  — hero display (rare)
```

**Letter spacing**: Display at `-0.02em`, Body at `0`, Mono at `0.02em`. This creates a subtle rhythm: headings feel tight and intentional, body is neutral, technical data breathes.

### 2.3 Geometry & Surfaces

| Token              | Value         | Notes                                           |
|--------------------|---------------|--------------------------------------------------|
| `--radius-panel`   | `0px`         | Main content panels, bento cards (sharp corners) |
| `--radius-input`   | `0px`         | Form inputs, small cards                         |
| `--radius-btn`     | `0px`         | Buttons (sharp corners)                          |
| `--radius-badge`   | `0px`         | Tags, status indicators                          |
| `--border-subtle`  | `1px solid color-mix(in srgb, var(--color-base-content) 8%, transparent)` | Hairline panel dividers |
| `--border-active`  | `1px solid var(--color-primary)` | Focus/active states                |
| `--shadow-panel`   | `none`        | Removed for flat, continuous terminal grid feel  |

**Key shift**: From `12px` rounded panels and `8px` inputs to `0px` sharp corners everywhere. The design moves away from floating, gap-separated bento boxes. Instead, panels sit flush against each other with zero gaps, separated only by a 1px hairline border (`border-subtle`). Background colors of panels should blend with the main background (using `base-100` or `base-200`) without vivid or full-color fills, creating a continuous, unbroken workspace grid.

### 2.4 Spacing System

Use a 4px grid with an 8px base unit. Key landmarks:

```
--space-1:  4px    — tight inline gaps
--space-2:  8px    — between related elements
--space-3:  12px   — input padding, small card padding
--space-4:  16px   — standard card padding, section gaps
--space-6:  24px   — between sections within a panel
--space-8:  32px   — between major layout regions
--space-12: 48px   — page-level breathing room
```

---

## 3. Layout Architecture

### 3.1 Current vs. New

**Current**: Top floating pill navbar + full-width scrollable content area + floating status bar + bottom footer bar.

**New**: Compact header bar → workspace panels. Chrome should be *compressed* to maximize workspace.

```
┌──────────────────────────────────────────────────────────┐
│ ▪ ▪ ▪  Cliptzy                    [tabs]    [⚙] [user]  │  ← Title bar (28px, drag region)
├──────────────────────────────────────────────────────────┤
│ [🏠] [📹] [✂️] [📤] [⚙]  ← Route nav (icon-only sidebar) │
│  ┌─────────────────────────────────────────────────────┐ │
│  │                                                     │ │
│  │              Main Workspace Area                    │ │
│  │         (varies by route/context)                   │ │
│  │                                                     │ │
│  └─────────────────────────────────────────────────────┘ │
├──────────────────────────────────────────────────────────┤
│ ● Connected  │  GPU: NVIDIA RTX  │  Job: idle  │  v1.2  │  ← Status bar (24px, mono)
└──────────────────────────────────────────────────────────┘
```

**Option A — Left icon rail** (recommended):
A 48px-wide icon rail on the left replaces the top navbar tabs. Each icon is a route: Dashboard, Studio, Library, Upload Queue, Settings. This reclaims ~48px of vertical height that the current top navbar uses — critical for a video editor.

**Option B — Compact top bar**:
Keep a compressed top bar with icon+text tabs, but shrink from the current size to a single 36px-high bar.

> **Recommendation: Option A.** Video tools are workspace-density apps. Every vertical pixel matters for the timeline and preview. A left rail is the industry standard (Premiere, DaVinci, Figma).

### 3.2 Studio View Layout (Most Complex)

```
┌────┬───────────────────────────────────────┬──────────┐
│    │  ┌──────────────────────────────────┐  │          │
│ ■  │  │                                  │  │ Inspector│
│ ■  │  │       9:16 Video Preview         │  │ Panel    │
│ ■  │  │     (YouTube IFrame Player)      │  │          │
│ ■  │  │                                  │  │ [Face]   │
│ ■  │  └──────────────────────────────────┘  │ [Sub]    │
│    │  ┌──────────────────────────────────┐  │ [Brand]  │
│    │  │    ▶ Timeline / Waveform         │  │ [Export] │
│    │  └──────────────────────────────────┘  │          │
├────┴─────────────────────────────────────────┴──────────┤
│ status bar                                              │
└─────────────────────────────────────────────────────────┘
```

### 3.3 Dashboard Layout

```
┌────┬─────────────────────────────────────────────────────┐
│    │  Welcome back.                                      │
│ ■  │                                                     │
│ ■  │  ┌──────────────────┐  ┌──────────────────┐        │
│ ■  │  │ System Monitor   │  │ Active Jobs      │        │
│ ■  │  │ CPU ▓▓▓▓░░ 62%   │  │ 2 rendering      │        │
│ ■  │  │ GPU ▓▓▓░░░ 45%   │  │ 1 queued         │        │
│    │  │ RAM ▓▓░░░░ 34%   │  │                  │        │
│    │  └──────────────────┘  └──────────────────┘        │
│    │                                                     │
│    │  ┌──────────── Recent Projects ────────────────┐   │
│    │  │  video_id_abc  │  video_id_xyz  │  ...       │   │
│    │  └──────────────────────────────────────────────┘   │
└────┴─────────────────────────────────────────────────────┘
```

---

## 4. Signature Element

> **The Cutting Room Monitor**

Every design needs one memorable visual moment. For Cliptzy, it's the **status bar** — reimagined as a terminal-style monitor strip that runs along the bottom:

- Dark monospaced text on a slightly darker-than-base strip (`base-100` darkened, or a new `base-50` token)
- Live-updating data: connection status dot (green/amber/red), GPU name, active job state, processing progress with a thin inline progress bar
- Keyboard shortcut hints that appear contextually (à la Poolside's `shift+tab Cycle mode` overlay)
- Subtle `1px` top border using `border-subtle`

This anchors the "professional tool" feeling. The status bar is the user's constant companion — it should feel like a cockpit instrument, not an afterthought.

---

## 5. Component Library Strategy

### 5.1 Third-Party Dependencies

| Package                    | Version | Purpose                                           | Action       |
|----------------------------|---------|---------------------------------------------------|--------------|
| `daisyui`                  | ^5      | Theme tokens, utility classes, base component CSS | **Keep**     |
| `tailwindcss`              | ^4      | Utility framework                                 | **Keep**     |
| `@fontsource/geist-sans`   | ^5      | Display typeface                                  | **Keep**     |
| `@fontsource/geist-mono`   | —       | Monospace UI typeface                             | **Add**      |
| `@fontsource/inter`        | ^5      | Body typeface                                     | **Keep**     |
| `@fontsource/plus-jakarta-sans` | ^5 | *(unused after migration)*                        | **Remove**   |
| `unplugin-icons`           | ^23     | Icon system via Iconify                           | **Keep**     |
| `@vueuse/core`             | ^14     | Vue composables                                   | **Keep**     |
| `motion-v`                 | —       | Animation library for Vue                         | **Add** (optional) |

> **Why NOT shadcn-vue?** DaisyUI v5 already provides the semantic token system and base component styles. Adding shadcn-vue would create a dual component system with competing conventions. Instead, we build on DaisyUI's token layer and create purpose-built Cliptzy primitives.

### 5.2 Primitive Components (Build/Refactor)

| Component              | Status    | Notes                                                  |
|------------------------|-----------|--------------------------------------------------------|
| `CButton.vue`          | Refactor  | Replace `BaseButton.vue`. Variants: primary, secondary, ghost, danger. Size: sm/md/lg. **No more pill shape.** |
| `CInput.vue`           | Refactor  | Replace `SpatialInput.vue`. Consistent border, focus ring using new tokens. |
| `CSlider.vue`          | Refactor  | Replace `RangeSlider.vue`. Styled track/thumb with new palette. |
| `CCard.vue`            | New       | Panel container with `base-100` or `base-200` bg, subtle border, `radius-panel` (0px), sits flush with others. |
| `CIconButton.vue`      | New       | Square icon-only button for toolbar actions (sharp corners).           |
| `CBadge.vue`           | New       | Status/tag indicator. Variants: info, success, warning, error (sharp corners). |
| `CTooltip.vue`         | New       | Minimal tooltip (dark bg, small text, 0px radius).     |
| `CDropdown.vue`        | New       | Replacement for native selects with new styling.       |
| `CToggle.vue`          | Refactor  | Refactor `ToggleSwitch.vue`. Properly themed.          |
| `CProgress.vue`        | Refactor  | Refactor `ProgressBar.vue`. Thin bar for status + render queue. |
| `CDivider.vue`         | New       | Horizontal/vertical rule using `border-subtle`.        |

### 5.3 Layout Components

| Component              | Status    | Notes                                                  |
|------------------------|-----------|--------------------------------------------------------|
| `AppShell.vue`         | New       | Replaces `MainLayout.vue`. Icon rail + workspace + status bar. |
| `IconRail.vue`         | New       | Left navigation rail (48px wide).                      |
| `StatusBar.vue`        | Refactor  | Replaces `AppFooter.vue`. Terminal-style monitor strip. |
| `PanelGroup.vue`       | New       | Resizable panel container (for Studio split view).     |
| `PanelHeader.vue`      | New       | Consistent panel title bar with actions slot.          |

---

## 6. Motion & Interaction Design

### 6.1 Principles

- **Reduced motion by default**: Respect `prefers-reduced-motion`. All animations via CSS transitions, not JavaScript animation loops.
- **Functional, not decorative**: Transitions exist to maintain spatial continuity (panel opens → slides in) or indicate state (button loading → spinner).
- **One orchestrated moment**: The page-transition between routes: a subtle 150ms opacity + translateY(4px) fade-up. Nothing else animates on page load.

### 6.2 Specific Interactions

| Element            | Interaction           | Behavior                                     |
|--------------------|-----------------------|----------------------------------------------|
| Buttons            | Hover                 | `background-color` shift, 150ms ease         |
| Buttons            | Active/Click          | `scale(0.97)` for 100ms, then release        |
| Cards/Panels       | Hover                 | Subtle `border-color` lighten, 200ms         |
| Nav rail icons     | Active route          | Left 2px accent-colored bar indicator        |
| Page transitions   | Route change          | 150ms `opacity` + `translateY(4px)` fade-up  |
| Toast              | Appear/Dismiss        | Slide up from bottom, 200ms                  |
| Status bar progress| Active render         | Smooth `width` transition on progress bar    |
| Modals/Dialogs     | Open/Close            | Backdrop fade 200ms + panel scale 150ms      |

---

## 7. View-by-View Design Notes

### 7.1 Login View
- Full-screen dark background (`base-100`)
- Centered card (`base-200`) with Cliptzy logo, Supabase auth form
- No decoration beyond the logo and form. Clean entry point.

### 7.2 Dashboard View
- Grid of status cards: System Monitor, Active Jobs, Recent Projects
- System monitor uses thin horizontal progress bars (not circular gauges)
- Cards use `base-200` on `base-100`, with `border-subtle`
- "New Project" prominent CTA button in primary color

### 7.3 Studio View (Most Complex)
- Three-column layout: IconRail | Preview+Timeline | Inspector
- Preview: dark `base-100` background behind the 9:16 iframe, creating a "monitor" feel
- Timeline: horizontal waveform with segment markers, on `base-200` strip
- Inspector: scrollable right panel with collapsible sections (Face Tracking, Subtitle Style, Branding, Export)
- Each inspector section is a `CCard` with `PanelHeader`

### 7.4 Library View
- Grid/list toggle for past projects
- Each project card shows: thumbnail, video title, segment count, render status
- Filter/sort controls in a compact toolbar

### 7.5 Settings View
- Vertical sections, each in a `CCard`
- Clean form inputs with labels, descriptions, and toggles
- "Test yt-dlp" and "Test Cookies" buttons styled as secondary variant

---

## 8. Dark/Light Theme Strategy

**Dark mode is primary.** The references are both dark-first. Cliptzy's users are creators who often work in dimly lit environments with video content on screen.

- Dark theme ships as default
- Light theme is available but secondary
- Theme toggle in Settings (not in header — avoid visual noise)
- All components must work in both themes via DaisyUI token system
- No hardcoded colors in components — always use semantic tokens

---

## 9. Iconography

**Current**: `unplugin-icons` with `@iconify/json` — keep it.

**Style guide**:
- Use **Lucide** icon set as primary (clean, consistent stroke icons)
- 20px default size for UI icons, 16px for inline/status
- `currentColor` stroke, 1.5px stroke width
- No filled icons except for active states (e.g., filled star for favorited)

---

## 10. Anti-Patterns to Avoid

1. ❌ Pill-shaped buttons (`border-radius: 9999px`) — reads as consumer app
2. ❌ Pastel cream backgrounds — contradicts the reference direction
3. ❌ Pink/rose accent (`#E87389`) — too playful for tool aesthetic
4. ❌ Any border-radius on panels — too bubbly, use `0px` (sharp corners) for continuous terminal feel
5. ❌ Gaps between workspace panels — panels should sit flush with 1px hairline borders
6. ❌ Vivid/full color block backgrounds for bento panels — use background-matching colors (`base-100` or `base-200`)
7. ❌ Multiple competing accent colors in one view
8. ❌ Animations on every element — one orchestrated moment only
9. ❌ `hover:scale-[1.02]` on buttons — too bouncy, use `0.97` on active only
10. ❌ Floating pill-shaped navbar — replace with icon rail
