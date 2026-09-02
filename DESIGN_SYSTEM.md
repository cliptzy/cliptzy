# 🎨 Cliptzy Design System (Minimalist Pastel Bento Box)

## 📌 Overview
Design system ini dirancang ulang (re-imagined) untuk memberikan tampilan yang lebih *fresh*, modern, dan bersahabat (friendly) pada aplikasi Desktop native Cliptzy (berbasis Tauri + Vue 3). Gaya visual yang digunakan adalah **"Minimalist Pastel Bento Box"** yang terinspirasi dari tren UI *link-in-bio* dan *personal dashboard* bergaya ceria namun profesional.

Kita resmi membuang desain agresif ("Deep Dark Mode" OLED pekat dan gaya Cyberpunk/Neon), beralih ke tata letak berbasis grid modular (Bento) yang padat informasi, efisien, sangat bersih (*flat*), dan kini mendukung **Dual Theme (Light Pastel & Slate Dark Mode)** dengan transisi animasi yang mulus.

---

## 📐 Prinsip Utama (Core Principles)
1. **Bento Grid System**: Tata letak modular menggunakan kartu (*cards*) dengan ukuran grid (`col-span` dan `row-span`) yang bervariasi agar *layout* tidak terlihat kaku.
2. **Proportional Rounded Geometry**: Penggunaan sudut membulat yang disesuaikan dengan ukuran elemen. Kartu besar/utama menggunakan `rounded-[2rem]` (32px), sedangkan kartu kecil, tombol, dan input menggunakan `rounded-2xl` (16px) atau `rounded-3xl` (24px) untuk menjaga ruang konten tetap efisien dan tidak terpotong.
3. **Flat & Solid (No Borders, No Glow)**: Membuang seluruh *border* kasar dan efek *glow* di aplikasi. Elemen UI dipisahkan oleh perbedaan warna blok solid dan bayangan jatuh (*drop shadow*) yang sangat halus/subtle.
4. **Dual Theme (Light & Dark)**: 
   - *Light Mode* menggunakan warna latar pastel terang yang memberikan kesan tenang.
   - *Dark Mode* beralih ke warna *Slate* (biru keabu-abuan) yang elegan dan nyaman di mata (bukan murni `#000000`).
5. **Fluid Transitions**: Setiap perpindahan warna tema, status hover, atau interaksi klik harus menggunakan transisi yang lembut (`transition-all duration-300 ease-out`).

---

## 🎨 Palet Warna (Color Palette)

Diimplementasikan penuh menggunakan utilitas *Light* dan *Dark* dari Tailwind CSS, dengan dukungan tema dari DaisyUI (lihat bagian integrasi).

### 1. Warna Dasar & Permukaan (Backgrounds)
- **Base (Light)**: `#FAF4F4` (Pastel Pinkish-Cream). Digunakan untuk latar belakang aplikasi utama.
- **Base (Dark)**: `#0F172A` (Slate 900).
- **Surface/Card (Light)**: `#FFFFFF` (Putih bersih).
- **Surface/Card (Dark)**: `#1E293B` (Slate 800).

### 2. Aksen & Kartu Khusus (Solid Pastels)
Kartu-kartu spesifik di dalam Bento Grid dapat diwarnai secara penuh (*full-bleed*) untuk memberikan penanda visual tanpa merusak estetik minimalis. 
**⚠️ Aturan Aksesibilitas Mutlak**: Teks di atas warna aksen pastel *Light Mode* **HARUS** berwarna gelap (mis. `text-gray-900`), jangan pernah menggunakan teks putih di atas pastel terang.
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

### 1. Tipografi Berkarakter (Dual Font System)
Untuk menghindari kesan *template* generik, kita menggunakan perpaduan dua jenis font:
- **Display/Header Font (`font-display`)**: Gunakan **Plus Jakarta Sans** (atau *Outfit*). Font ini memiliki karakter geometris, ramah, dan sangat cocok untuk judul (H1-H4) atau angka-angka metrik besar pada dashboard.
- **Body & Utility Font (`font-sans`)**: Gunakan **Inter**. Dikhususkan untuk teks paragraf, input pengguna, dan elemen antarmuka yang padat data karena tingkat keterbacaannya yang luar biasa.
- **Micro-copy & Label**: Gunakan ukuran super kecil (`text-[10px]` atau `text-[11px]`), diubah ke kapital (`uppercase`), ditebalkan (`font-bold`), dan direnggangkan (`tracking-widest`) dengan font Inter untuk teks penjelas sistem atau sub-label.

### 2. Bento Card
Elemen penampung utama, pengganti panel konvensional.
- **Styling**: `bg-white dark:bg-[#1E293B] shadow-sm`. Radius disesuaikan dengan proporsi letak (`rounded-2xl` hingga `rounded-[2rem]`). Padding ideal: `p-6 lg:p-8`.
- **Interaksi Hover (Opsional)**: Kartu yang dapat di-klik harus sedikit membesar dan bayangannya menebal (`hover:scale-[1.02] hover:shadow-lg`).

### 3. Navigasi (Pill Menu)
- Tanpa *background* pembungkus kotak di belakangnya.
- Item **Aktif**: Berupa pil solid `bg-white dark:bg-[#1E293B] text-black dark:text-white px-5 py-2 rounded-full shadow-sm`.
- Item **Pasif**: Teks biasa yang akan berubah warna saat di-*hover* (`text-gray-500 hover:text-black dark:hover:text-white`).

### 4. Tombol & Input (Controls)
- **Tombol**: Menggunakan `rounded-full` dengan background solid (contoh: abu-abu muda `bg-gray-50` di *light mode* atau warna aksen Pink). Teks harus lugas menggunakan *Active Voice* (contoh: "Simpan Klip", bukan "Submit").
- **Input Field**: Tanpa *border*, mengandalkan perbedaan warna *background* (contoh: input berwarna `#F9FAFB` di atas form berwarna putih), `rounded-xl`, hilangkan garis tepi tebal saat *focus* (`outline-none`).
- **Dark Mode Toggle**: Dibuat secara fisik dengan animasi transform (`translate-x`) di dalam wadah berbentuk pil untuk memberikan kepuasan mekanis.

### 5. Komunikasi Status (Empty & Error States)
Desain yang baik mengarahkan, bukan hanya memberitahu.
- **Empty States (Keadaan Kosong)**: Jangan biarkan layar kosong hanya berisi teks. Sertakan grafis atau ilustrasi yang sesuai dengan estetika *Pastel Bento*, diikuti **Call to Action (CTA)** utama yang spesifik (misal: "Belum ada klip yang diproses. Tarik tautan YouTube ke sini.").
- **Error States**: Jangan meminta maaf atau ambigu. Jelaskan apa yang salah dalam istilah pengguna akhir dan sediakan tombol perbaikan (misal: "Koneksi ke YouTube gagal. Coba ganti tautan alternatif.").

---

## ⚙️ Integrasi Tata Letak (Layout Structure)

- **The Grid (Bento Box)**: Gunakan CSS Grid `grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 auto-rows-[minmax(180px,auto)] gap-6`.
- **Pembatasan Lebar (Constrained Layout)**: Seluruh aplikasi tidak dibuat memenuhi layar penuh (bukan 100% *width*), melainkan dibungkus dengan wadah terpusat (`max-w-5xl mx-auto` atau `max-w-6xl`) untuk menjaga proporsi kartu Bento tetap elegan baik di laptop maupun monitor besar.

---

## 🌼 Panduan Eksekusi DaisyUI (Hybrid Approach)

Sistem akan menggunakan **DaisyUI** *hanya sebagai Theme Engine (Theme Controller)* agar kode di Vue lebih bersih, **BUKAN** sebagai pengambil alih total komponen UI.

Kita akan menunggangi kemampuan DaisyUI dalam manajemen tema *Light/Dark*, tetapi akan **menimpa** variabel bawaan agar tidak merusak prinsip *Ultra-Rounded* dan *Flat (No Borders)* kita.

**Contoh Konfigurasi di `tailwind.config.js`**:
```javascript
module.exports = {
  // ...
  daisyui: {
    themes: [
      {
        light: {
          "primary": "#E87389", // Aksen Pink
          "base-100": "#FAF4F4", // Latar belakang utama (Pastel Cream)
          "base-200": "#FFFFFF", // Warna kartu
          "base-content": "#1a1a1a", // Warna teks primer
          "--rounded-box": "1.5rem", // Border radius untuk kartu (24px) - ditimpa ke 2rem jika perlu di template
          "--rounded-btn": "9999px", // Border radius untuk tombol (pill/full)
          "--border-btn": "0px", // Mematikan border kasar
        },
        dark: {
          "primary": "#f472b6",
          "base-100": "#0F172A", // Slate 900
          "base-200": "#1E293B", // Slate 800
          "base-content": "#F8FAFC", // Warna teks primer
          "--rounded-box": "1.5rem",
          "--rounded-btn": "9999px",
          "--border-btn": "0px",
        }
      }
    ]
  }
}
```
**Pendekatan Vue Component**:
- Alih-alih menulis `class="bg-white dark:bg-[#1E293B] text-black dark:text-white"`, Anda kini bisa menulis `class="bg-base-200 text-base-content"`.
- Jika kelas komponen bawaan DaisyUI (seperti `.btn` atau `.input`) terasa melanggar prinsip *Flat & Solid*, **abaikan kelas tersebut** dan gunakan *utility classes* standar Tailwind di komponen dasar Anda (misal `<BaseButton>`).
