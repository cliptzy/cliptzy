# 🎨 Cliptzy Design System (Minimalist Pastel Bento Box)

## 📌 Overview
Design system ini dirancang ulang (re-imagined) untuk memberikan tampilan yang lebih *fresh*, modern, dan bersahabat (friendly) pada aplikasi Desktop native Cliptzy (berbasis Tauri + Vue 3). Gaya visual yang digunakan adalah **"Minimalist Pastel Bento Box"** yang terinspirasi dari tren UI *link-in-bio* dan *personal dashboard* bergaya ceria namun profesional.

Kita resmi membuang desain agresif ("Deep Dark Mode" OLED pekat dan gaya Cyberpunk/Neon), beralih ke tata letak berbasis grid modular (Bento) yang padat informasi, efisien, sangat bersih (*flat*), dan kini mendukung **Dual Theme (Light Pastel & Slate Dark Mode)** dengan transisi animasi yang mulus.

---

## 📐 Prinsip Utama (Core Principles)
1. **Bento Grid System**: Tata letak modular menggunakan kartu (*cards*) dengan ukuran grid (`col-span` dan `row-span`) yang bervariasi agar *layout* tidak terlihat kaku.
2. **Ultra-Rounded Geometry**: Penggunaan sudut yang sangat membulat (`rounded-[2rem]` atau `32px`) untuk memberikan kesan organik dan modern.
3. **Flat & Solid (No Borders, No Glow)**: Membuang seluruh *border* kasar dan efek *glow* di aplikasi. Elemen UI dipisahkan oleh perbedaan warna blok solid dan bayangan jatuh (*drop shadow*) yang sangat halus/subtle.
4. **Dual Theme (Light & Dark)**: 
   - *Light Mode* menggunakan warna latar pastel terang yang memberikan kesan tenang.
   - *Dark Mode* beralih ke warna *Slate* (biru keabu-abuan) yang elegan dan nyaman di mata (bukan murni `#000000`).
5. **Fluid Transitions**: Setiap perpindahan warna tema, status hover, atau interaksi klik harus menggunakan transisi yang lembut (`transition-all duration-300 ease-out`).

---

## 🎨 Palet Warna (Color Palette)

Diimplementasikan penuh menggunakan utilitas *Light* dan *Dark* dari Tailwind CSS.

### 1. Warna Dasar & Permukaan (Backgrounds)
- **Base (Light)**: `#FAF4F4` (Pastel Pinkish-Cream). Digunakan untuk latar belakang aplikasi utama.
- **Base (Dark)**: `#0F172A` (Slate 900).
- **Surface/Card (Light)**: `#FFFFFF` (Putih bersih).
- **Surface/Card (Dark)**: `#1E293B` (Slate 800).

### 2. Aksen & Kartu Khusus (Solid Pastels)
Kartu-kartu spesifik di dalam Bento Grid dapat diwarnai secara penuh (*full-bleed*) untuk memberikan penanda visual tanpa merusak estetik minimalis:
- **Pastel Blue (Info/Data)**: `#dbeafe` (Dark: `#1e3a8a` / Blue-900).
- **Pastel Green (Action/Success)**: `#6ee7b7` (Dark: `#134e4a` / Teal-900).
- **Pastel Twitter Blue (Social)**: `#93c5fd` (Dark: `#2563eb` / Blue-600).
- **Pastel Yellow (Warning/Highlight)**: `#fde68a` (Dark: `#713f12` / Yellow-900).
- **Brand/Accent (Pink)**: `#E87389` atau `#f472b6`. Digunakan untuk logo atau tombol eksekusi utama.

### 3. Teks & Tipografi
- **Primary Text (Light)**: `#1a1a1a` atau `text-gray-800` (Abu-abu sangat gelap, jangan gunakan murni `#000`).
- **Primary Text (Dark)**: `#F8FAFC` atau `text-gray-100` (Putih pucat).
- **Muted Text**: `text-gray-500` (Light) & `text-gray-400` (Dark).

---

## 🔲 Komponen & Elemen UI

### 1. Bento Card
Elemen penampung utama, pengganti panel konvensional.
- **Styling**: `bg-white dark:bg-[#1E293B] rounded-[2rem] p-6 lg:p-8 shadow-sm`.
- **Interaksi Hover (Opsional)**: Kartu yang dapat di-klik harus sedikit membesar dan bayangannya menebal (`hover:scale-[1.02] hover:shadow-lg`).

### 2. Navigasi (Pill Menu)
- Tanpa *background* pembungkus kotak di belakangnya.
- Item **Aktif**: Berupa pil solid `bg-white dark:bg-[#1E293B] text-black dark:text-white px-5 py-2 rounded-full shadow-sm`.
- Item **Pasif**: Teks biasa yang akan berubah warna saat di-*hover* (`text-gray-500 hover:text-black dark:hover:text-white`).

### 3. Tombol & Input (Controls)
- **Tombol**: Menggunakan `rounded-full` dengan background solid (contoh: abu-abu muda `bg-gray-50` di *light mode* atau warna aksen Pink).
- **Input Field**: Tanpa *border*, mengandalkan perbedaan warna *background* (contoh: input berwarna `#F9FAFB` di atas form berwarna putih), `rounded-xl`, hilangkan garis tepi tebal saat *focus* (`outline-none`).
- **Dark Mode Toggle**: Dibuat secara fisik dengan animasi transform (`translate-x`) di dalam wadah berbentuk pil untuk memberikan kepuasan mekanis.

### 4. Typography
- Menggunakan font geometris seperti **Inter**.
- **Header**: `font-black` (bobot 900) dengan *tracking* netral atau dirapatkan.
- **Micro-copy & Label**: Gunakan ukuran super kecil (`text-[10px]` atau `text-[11px]`), diubah ke kapital (`uppercase`), ditebalkan (`font-bold`), dan direnggangkan (`tracking-widest`) untuk teks penjelas sistem atau sub-label.

---

## ⚙️ Integrasi Tata Letak (Layout Structure)

- **The Grid (Bento Box)**: Gunakan CSS Grid `grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 auto-rows-[minmax(180px,auto)] gap-6`.
- **Pembatasan Lebar (Constrained Layout)**: Seluruh aplikasi tidak dibuat memenuhi layar penuh (bukan 100% *width*), melainkan dibungkus dengan wadah terpusat (`max-w-5xl mx-auto` atau `max-w-6xl`) untuk menjaga proporsi kartu Bento tetap elegan baik di laptop maupun monitor besar.
