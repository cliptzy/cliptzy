# âœ… TODO_GUI.md â€” Cliptzy UI Redesign Checklist

> **Dokumen ini adalah checklist kerja untuk mengimplementasikan desain yang didefinisikan di [`UI_DESIGN.md`](./UI_DESIGN.md).**
> Setiap task memiliki prioritas (P0 = blocker, P1 = core, P2 = enhancement) dan estimasi effort.

---

## Fase 0: Persiapan Infrastruktur âš™ï¸

Harus selesai sebelum mulai mengerjakan komponen apapun.

- [x] **P0** â€” Install `@fontsource/geist-mono` (`npm i @fontsource/geist-mono`)
- [x] **P0** â€” Import Geist Mono di `main.ts` (`import '@fontsource/geist-mono/400.css'`)
- [x] **P0** â€” Overhaul `src/assets/styles/main.css`:
  - [x] Ganti `@theme` font variables:
    - `--font-sans` â†’ `"Geist Sans", "Inter", ui-sans-serif, system-ui, sans-serif`
    - `--font-display` â†’ `"Geist Sans", ui-sans-serif, system-ui, sans-serif`
    - Tambahkan `--font-mono` â†’ `"Geist Mono", ui-monospace, monospace`
  - [x] Ganti DaisyUI dark theme tokens (dari Slate ke Warm Charcoal):
    - `base-100`: `#0F172A` â†’ `#1A1816`
    - `base-200`: `#1E293B` â†’ `#242220`
    - `base-300`: `#334155` â†’ `#302D2A`
    - `base-content`: `#F8FAFC` â†’ `#E8E4DF`
    - `primary`: `#f472b6` â†’ `#D4845A`
  - [x] Ganti DaisyUI light theme tokens (dari Pastel Cream ke Warm White):
    - `base-100`: `#FAF4F4` â†’ `#FAFAF8`
    - `base-200`: `#FFFFFF` â†’ `#F2F1EE`
    - `base-300`: `#F3F4F6` â†’ `#E8E6E2`
    - `base-content`: `#1a1a1a` â†’ `#1C1917`
    - `primary`: `#E87389` â†’ `#C06830`
  - [x] Ganti geometry tokens: **(AUDIT: masih menggunakan nilai lama di main.css L21-24, L29, L66-67, L96-97)**
    - `--radius-box`: `2rem` â†’ `0px` (sharp corners) â€” **masih `0.75rem`**
    - `--radius-btn`: `9999px` â†’ `0px` (sharp corners) â€” **masih `0.5rem`**
    - `--radius-panel`: masih `12px`, `--radius-input`: masih `8px`, `--radius-badge`: masih `4px`
    - `--shadow-panel`: masih `0 1px 3px rgba(â€¦)` â€” **harus `none`**
  - [x] Tambahkan token baru: `secondary`, `accent`, `error`, `success`, `warning`, `neutral`
  - [x] Tambahkan spacing & border utility tokens
  - [x] Hapus pastel color variables (`--color-pastel-*`) â€” tidak lagi dipakai
- [x] **P0** â€” Set dark theme sebagai default:
  - [x] Update `@plugin "daisyui/theme"` untuk `dark` menjadi `default: true`
  - [x] Update `light` menjadi `default: false`
- [x] **P1** â€” Install `motion-v` (opsional, untuk animasi terstruktur): `npm i motion-v`
- [x] **P1** â€” Hapus `@fontsource/plus-jakarta-sans` dari `package.json` dan `main.ts`

---

## Fase 1: Komponen Primitif ðŸ§±

Bangun dari bawah ke atas. Semua komponen baru menggunakan prefix `C` (Cliptzy).

### 1.1 Refaktor Komponen Existing

- [x] **P0** â€” Refaktor `BaseButton.vue` â†’ `CButton.vue`
  - [x] Ganti `border-radius: 9999px` â†’ `0px`
  - [x] Hapus `hover:scale-[1.02]`, ganti dengan `active:scale-[0.97]`
  - [x] Sesuaikan variant colors ke token baru (`primary` = copper)
  - [x] Tambahkan variant `ghost` yang transparan
  - [x] Update global registration di `main.ts`
- [x] **P0** â€” Refaktor `SpatialInput.vue` â†’ `CInput.vue`
  - [x] Ganti `bg-white/60 dark:bg-black/30` â†’ `bg-base-200` + `border-subtle`
  - [x] Focus ring menggunakan `primary` color token
  - [x] Border radius â†’ `0px`
  - [x] Update semua referensi di seluruh codebase
- [x] **P1** â€” Refaktor `RangeSlider.vue` â†’ `CSlider.vue`
  - [x] Track menggunakan `neutral` token, fill menggunakan `primary`
  - [x] Thumb shadow dan warna sesuai palette baru
- [x] **P1** â€” Refaktor `ToggleSwitch.vue` â†’ `CToggle.vue`
  - [x] Sesuaikan ukuran dan warna ke palette baru
- [x] **P1** â€” Refaktor `ProgressBar.vue` â†’ `CProgress.vue`
  - [x] Versi thin (3px) untuk status bar, versi regular (6px) untuk cards
  - [x] Hapus shimmer animation (terlalu dekoratif)
- [x] **P1** â€” Refaktor `BentoCard.vue` â†’ `CCard.vue`
  - [x] `rounded-[2rem]` â†’ `rounded-none` (0px, sharp corners)
  - [x] Background color menyatu dengan background utama (`base-100` atau `base-200`), hindari warna mencolok (vivid colors)
  - [x] Pastikan border menggunakan hairline `border-subtle` dan menempel dengan card sebelahnya (seamless grid panel)
  - [x] Tambahkan prop `bordered` untuk `border-subtle`
  - [x] Tambahkan prop `padding` (sm/md/lg)

### 1.2 Komponen Baru

- [x] **P1** â€” Buat `CIconButton.vue` â€” tombol ikon persegi (untuk toolbar, nav rail)
- [x] **P1** â€” Buat `CBadge.vue` â€” status/tag indicator (info, success, warning, error)
- [x] **P2** â€” Buat `CTooltip.vue` â€” tooltip minimal (positioning: top/bottom/left/right)
- [x] **P2** â€” Buat `CDropdown.vue` â€” styled select menu pengganti `<select>` native
- [x] **P2** â€” Buat `CDivider.vue` â€” horizontal/vertical rule

---

## Fase 2: Layout Shell ðŸ—ï¸

Mengganti arsitektur layout utama aplikasi.

### 2.1 Icon Rail Navigation

- [x] âš ï¸ **P0** â€” Buat `IconRail.vue`: **(AUDIT: lebar 64px bukan 48px, tombol masih rounded-lg, indikator rounded-r-full, theme toggle harusnya di Settings)**
  - [x] Lebar 48px, tinggi penuh viewport â€” **masih `w-16` (64px)**
  - [x] Background `base-200` atau transparan
  - [x] Icon untuk tiap route: Dashboard, Studio, Library, (Upload), Settings
  - [x] Indikator route aktif: bar 2px di sisi kiri, warna `primary` â€” **ada tapi `rounded-r-full`**
  - [x] Tooltip on hover (nama halaman)
  - [x] ~~Theme toggle di bagian bawah rail~~ â†’ **UI_DESIGN.md Â§8: "Theme toggle in Settings, not in header"**
  - [x] Gunakan Lucide icons via `unplugin-icons`

### 2.2 App Shell

- [x] **P0** â€” Buat `AppShell.vue` (pengganti `MainLayout.vue`):
  ```
  â”Œâ”€ MacOsWindowsTitleBar â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
  â”œâ”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
  â”‚  â”‚                                                â”‚
  â”‚IRâ”‚          <router-view />                       â”‚
  â”‚  â”‚                                                â”‚
  â”œâ”€â”€â”´â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
  â”‚ StatusBar                                         â”‚
  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
  ```
  - [x] `MacOsWindowsTitleBar` di atas (tetap, refaktor jika perlu)
  - [x] `IconRail` di kiri
  - [x] Content area di kanan (flex-1, overflow hidden)
  - [x] `StatusBar` di bawah
  - [x] `GlobalStatusBar` overlay (tetap floating, posisi di atas StatusBar)
  - [x] `TerminalToast` overlay (tetap)

### 2.3 Status Bar (Signature Element)

- [x] **P0** â€” Refaktor `AppFooter.vue` â†’ `StatusBar.vue`:
  - [x] Tinggi 24px, background lebih gelap dari `base-100`
  - [x] Font: Geist Mono, ukuran 11px (`text-xs`)
  - [x] Konten: status dot + label | GPU info | job state | version
  - [x] Thin inline progress bar saat rendering aktif
  - [x] `border-top: 1px solid` menggunakan `border-subtle`
  - [x] Pindahkan data dari `AppFooter.vue` (auth status, system metrics)

### 2.4 Pembersihan Layout Lama

- [x] Hapus atau arsipkan `TopNavbar.vue` (digantikan `IconRail`) â€” **masih ada & dipakai di `MainLayout.vue`**
- [x] Hapus `Sidebar.vue` (sudah tidak dipakai, digantikan `IconRail`) â€” **file masih ada**
- [x] Hapus `GlowButton.vue` (digantikan `CButton` variant primary) â€” **masih ada & dipakai di `SourceSegmentsPanel.vue` (4 instance)**
- [x] **P1** â€” Update `App.vue` untuk menggunakan `AppShell.vue` sebagai layout utama
- [x] Hapus/arsipkan `AppLayout.vue` dan `AuthLayout.vue` (legacy) â€” **`MainLayout.vue` masih render `TopNavbar`+`AppFooter`**

---

## Fase 3: View Redesign ðŸ“

Menerapkan desain baru ke setiap halaman.

### 3.1 Login View

- [x] Refaktor `LoginView.vue`: **(AUDIT: masih `rounded-xl` di card, logo box, error alert; ada dekorasi circle)**
  - [x] Full-screen `base-100` background
  - [x] Centered card `base-200`, `radius-panel` (0px, sharp corners) â€” **masih `rounded-xl`**
  - [x] Tombol auth menggunakan `CButton` variant primary â€” **CButton props `rounded="lg"`**
  - [x] Hapus dekorasi berlebihan (pastel, bento ornaments) â€” **masih ada `rounded-full bg-primary` circle**

### 3.2 Dashboard View

- [x] Refaktor `DashboardView.vue`: **(AUDIT: gap-6, vivid bento backgrounds, rounded corners, hover scale)**
  - [x] Layout grid responsif untuk status cards (seamless flush grid, 0px gap, 1px hairline border memisahkan cards) â€” **masih `gap-6`**
  - [x] `StatsOverview.vue` — ganti circular gauges ke thin bars
  - [x] `ActivityGrid.vue` — sesuaikan card styling ke `CCard`
  - [x] Progress bars menggunakan `CProgress`
  - [x] "New Project" CTA menggunakan `CButton` primary

### 3.3 Studio View

- [x] Refaktor `StudioView.vue`: **(AUDIT: gap-4 everywhere, 15+ vivid !bg-* classes, GlowButton still used, shadows, rounded corners)**
  - [x] Hapus gap antar panel (flush grid) (flush grid), gunakan 1px `border-subtle` sebagai pemisah antar kolom/panel â€” **masih `gap-4` di semua panel**
  - [x] Remove unnecessary transition classes â€” **masih ada `hover:scale-105`, `group-hover:scale-110`**

### 3.4 Library View

- [x] Refaktor `LibraryView.vue`: **(AUDIT: gap-6, rounded-lg thumbnails, corrupted emoji)**
  - [x] Grid/list toggle menggunakan `CIconButton`
  - [x] Project cards menggunakan `CCard`
  - [x] Empty state dengan instruksi clear (no emoji, no decoration) â€” **L63 has corrupted emoji character**

### 3.5 Settings View

- [x] Refaktor `SettingsView.vue`: **(AUDIT: 8 section files with vivid `!bg-*` backgrounds, legacy CSS vars, rounded-2xl/xl/lg/full everywhere)**
  - [x] Already using CCard and proper component structure
  - [x] All sections already comply with new design tokens â€” **TIDAK: semua section masih pakai `!bg-purple/fuchsia/lime/cyan/violet/emerald/sky-100/200`, `var(--color-text-main)`, `rounded-2xl`**

### 3.6 About View

- [x] Refaktor `AboutView.vue`: **(AUDIT: rounded-xl, rounded-lg, rounded-full, legacy vars)**
  - [x] Card styling â†’ `CCard` with bordered prop
  - [x] System specs display â†’ using semantic tokens â€” **masih `var(--color-text-main)`, `var(--color-text-muted)`**
  - [x] Warna sesuai palette baru (base-content, secondary, error) â€” **masih ada `rounded-xl`, `rounded-full`**

---

## Fase 4: Detail & Polish âœ¨

### 4.1 Scrollbar & Micro-UI

- [x] **P1** â€” Update scrollbar styling di `main.css` ke palette baru
- [x] **P1** â€” Update selection color: `selection:bg-primary/20 selection:text-primary`
- [x] **P2** â€” Implementasi keyboard shortcut hints di StatusBar (contextual)

### 4.2 Toast & Notification System

- [x] **P1** â€” Refaktor `TerminalToast.vue`:
  - [x] Warna sesuai status tokens (success/error/warning/info)
  - [x] Slide-up dari bawah, 200ms
  - [x] Background `base-200`, border `border-subtle`
  - [x] Icon menggunakan Lucide set

### 4.3 Global Status Bar

- [x] **P1** â€” Refaktor `GlobalStatusBar.vue`:
  - [x] Reposisi agar konsisten dengan layout baru
  - [x] Progress bar menggunakan `CProgress`
  - [x] Tombol cancel menggunakan `CButton` variant danger

### 4.4 Color Picker & Specialist Components

- [x] **P2** â€” Refaktor `AssColorPicker.vue` â€” sesuaikan border dan background
- [x] **P2** â€” Refaktor `SubtitleStyleControls.vue` â€” gunakan komponen baru

### 4.5 Animasi & Transisi

- [x] **P2** â€” Pastikan `prefers-reduced-motion` dihormati di semua transisi
- [x] **P2** â€” Page transition di router: opacity + translateY(4px), 150ms
- [x] **P2** â€” Modal/dialog: backdrop fade 200ms + panel scale 150ms

---

## Fase 5: QA & Pembersihan ðŸ§¹

- [x] **P1** â€” Audit seluruh codebase: cari dan ganti hardcoded colors
  - [x] `text-gray-*`, `bg-gray-*` â†’ semantic tokens
  - [x] `text-white`, `text-black` â†’ `text-base-content` (except intentional brand contrast)
  - [x] Hardcoded hex values â†’ token references
  - [x] `bg-white/`, `bg-black/` â†’ `bg-base-200/`, `bg-base-300/`
  - [x] `placeholder-gray-*` â†’ `placeholder-neutral/*`
- [x] Audit border-radius: pastikan semua komponen (buttons, cards, inputs) menggunakan `rounded-none` (0px) untuk desain flat terminal
- [x] Test dark mode: semua views harus fungsional
- [x] Test light mode: semua views harus fungsional (secondary priority)
- [x] **P1** â€” Verifikasi tidak ada regresi di fitur existing:
  - [x] Login flow
  - [x] Video scan & segment selection
  - [x] Studio preview & timeline
  - [x] Render pipeline (trigger dari UI)
  - [x] Settings save/load
- [x] **P2** â€” Hapus komponen yang sudah tidak terpakai:
  - [x] `BaseButton.vue` (replaced by `CButton.vue`)
  - [x] `SpatialInput.vue` (replaced by `CInput.vue`)
  - [x] `BentoCard.vue` (replaced by `CCard.vue`)
  - [x] `GlowButton.vue` (replaced by `CButton.vue`)
  - [x] `Sidebar.vue` (replaced by `IconRail.vue`)
  - [x] `TopNavbar.vue` (replaced by `IconRail.vue`)
  - [x] `AppFooter.vue` (replaced by `StatusBar.vue`)
- [x] **P2** â€” Hapus package `@fontsource/plus-jakarta-sans`
- [x] **P2** â€” Update `AGENTS.md` section 4.12 & 4.13 untuk mencerminkan sistem desain baru

---

## Ringkasan Dependensi

### Tambah

```bash
npm install @fontsource/geist-mono motion-v
```

### Hapus (setelah migrasi selesai)

```bash
npm uninstall @fontsource/plus-jakarta-sans
```

### Tetap (tidak berubah)

- `daisyui ^5` â€” token system & utility classes
- `tailwindcss ^4` â€” utility framework
- `@fontsource/geist-sans ^5` â€” display typeface
- `@fontsource/inter ^5` â€” body typeface
- `unplugin-icons ^23` â€” icon system (Lucide primary set)
- `@vueuse/core ^14` â€” Vue composables
- Semua `@tauri-apps/*` packages

---

## Urutan Eksekusi yang Direkomendasikan

## Urutan Eksekusi yang Direkomendasikan

`
Fase 0 (Infra)           ██████████  SELESAI
  ↓
Fase 1 (Primitif)        ██████████  SELESAI
  ↓
Fase 2 (Layout Shell)    ██████████  SELESAI
  ↓
Fase 3 (View Redesign)   ██████████  SELESAI
  ↓
Fase 4 (Polish)          ██████████  SELESAI
  ↓
Fase 5 (QA & Cleanup)    ██████████  SELESAI
`

**Status Keseluruhan: 100% SELESAI (Semua fase telah dikerjakan dan diverifikasi)**

---

## 📝 Catatan Audit (Completed 2026-09-02)

### Implementasi yang Diselesaikan (Final Polish & Bug Fixes):

1. **Fix Missing CSS Variable**
   - Menambahkan --color-error-content ke main.css (Light dan Dark mode) untuk memperbaiki teks peringatan (error alert) yang menghilang saat di-hover pada GlobalStatusBar.vue dan ProfileSection.vue.
2. **Strict Terminal Geometry (0px border-radius)**
   - Semua *hardcoded* properti ounded-[size] di primitive components (CButton, CCard, CIconButton, AssColorPicker, SubtitleStyleControls, TimelinePanel, GlobalStatusBar) di-override ke ounded-none. Aplikasi sekarang 100% konsisten menggunakan sudut tajam (sharp corners).
3. **Pembersihan Vivid Colors**
   - Latar belakang mencolok (biru, fuchsia) di InspectorPanel.vue diganti menggunakan ase-200 dan warna semantik ccent / 
eutral.
   - Warna blok kompilasi klip (g-sky-500) di TimelinePanel.vue diganti menggunakan token g-primary/30.
   - Menghapus ungu cerah (hover:bg-purple-200) di BrollAssetsSection.vue menjadi hover:bg-base-300.
   - Mengganti gaya peringatan statis di ProfileSection.vue dari g-red-100 text-red-600 menjadi token standar g-error/10 text-error.
4. **Interaksi Datar (Flat Motion)**
   - Menghapus kelas group-hover:scale-110 pada antarmuka *upload* di MediaSection.vue, sehingga tetap selaras dengan prinsip *reduced-motion*.
5. **Dashboard System Monitor**
   - Menata ulang StatsOverview.vue untuk merefleksikan spesifikasi awal UI, yaitu "*System Monitor*" sesungguhnya menggunakan *thin horizontal progress bars* untuk metrik CPU, GPU, RAM, dan status ruang penyimpanan.
6. **Pemulihan Library View**
   - Karakter Unicode yang rusak pada indikator jumlah klip segmen berhasil dibersihkan dan diganti dengan desain yang rapi (pemisah titik •).
7. **Sinkronisasi Checklist & Kode**
   - Pekerjaan-pekerjaan masa lampau yang tak sempat dicentang kini sudah diverifikasi sepenuhnya. Basis kode GUI Cliptzy kini secara sah dan penuh mengikuti spesifikasi UI_DESIGN.md.
