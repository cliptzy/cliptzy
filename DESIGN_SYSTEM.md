# 🎨 Cliptzy Design System (Google I/O 2024 Style)

## 📌 Overview
Design system ini diadaptasi dari gaya visual **Google I/O 2024**. Karakteristik utamanya adalah penggabungan **Bentuk Geometris (Geometric Shapes)**, **Outline Tebal (Thick Borders)**, dan **Warna Kontras Tinggi (High Contrast Colors)**, dengan pendekatan *flat* (tanpa *drop shadow* tebal ala Neo-Brutalism murni). 

Secara default, aplikasi diatur ke **Dark Mode**, namun mendukung transisi mulus ke Light Mode dengan palet yang ekuivalen.

---

## 📐 Prinsip Utama (Core Principles)
1. **Tidak Ada Shadow Tebal**: Hindari penggunaan shadow solid (`shadow-[4px_4px_0px_#000]`). Efek elevasi dicapai melalui border dan susunan *layering*.
2. **Border Konsisten**: Gunakan border dengan ketebalan sedang hingga tebal (`border-2` atau `border-[3px]`). Warna border default: `#000000` (Light) atau `#333333` (Dark Mode - jika dirasa kontras, tapi sering kali outline cerah/putih digunakan di dark mode Google I/O). Di sini kita gunakan outline *high contrast*.
3. **Sudut Membulat yang Lembut & Geometris**: Gunakan variasi `rounded-[32px]`, `rounded-full`, dan bentuk tak lazim (seperti `rounded-t-full` untuk arch/kubah).
4. **Warna Khas Google**: Pemakaian warna dasar yang mencolok sebagai aksen (Blue, Red, Yellow, Green).

---

## 🎨 Palet Warna (Color Palette)

### 1. Warna Brand (Aksen Geometris)
Warna ini tetap konsisten baik di Light maupun Dark mode untuk mempertahankan identitas.
- **Blue**: `#4285F4`
- **Red**: `#EA4335`
- **Yellow**: `#FBBC04`
- **Green**: `#34A853`

### 2. Tema: Dark Mode (Default)
- **Background Utama**: `#121212` (atau `#202124` standar Google)
- **Background Surface / Card**: `#1E1E1E` (atau `#28292C`)
- **Teks Utama**: `#FFFFFF` atau `#E8EAED`
- **Teks Sekunder**: `#9AA0A6`
- **Border / Outline**: `#3C4043` (Abu-abu gelap) atau `#5F6368` (Abu-abu medium)

### 3. Tema: Light Mode
- **Background Utama**: `#F8F9FA`
- **Background Surface / Card**: `#FFFFFF`
- **Teks Utama**: `#202124` (atau `#000000`)
- **Teks Sekunder**: `#5F6368`
- **Border / Outline**: `#000000` (Hitam pekat untuk efek pop-out grafis)

---

## 🔲 Komponen & Elemen UI

### 1. Card / Panel (Surface)
- **Dark Mode**: `bg-[#1E1E1E] border-[3px] border-[#3C4043] rounded-[32px]`
- **Light Mode**: `bg-white border-[3px] border-black rounded-[32px]`

### 2. Button (Pill Shape)
- **Primary Dark**: `bg-[#4285F4] text-white border-[2px] border-transparent hover:brightness-110 rounded-full` (Alternatif jika menggunakan outline: `bg-transparent border-[2px] border-[#E8EAED] text-white`)
- **Primary Light**: `bg-[#4285F4] text-white border-[3px] border-black rounded-full hover:bg-opacity-90`
- **Secondary Dark**: `bg-transparent border-[2px] border-[#5F6368] text-white hover:bg-[#3C4043] rounded-full`
- **Secondary Light**: `bg-white border-[3px] border-black text-black hover:bg-[#F8F9FA] rounded-full`

### 3. Dekorasi Geometris (Shapes)
- **Arch / Kubah**: `rounded-t-full` (Cocok ditempatkan di pinggiran kontainer)
- **Circle**: `rounded-full` proporsional (contoh: `w-32 h-32`)
- **Pill / Kapsul**: `rounded-full` memanjang (contoh: `w-48 h-20`)
- **Half Circle**: Kombinasi border parsial, misalnya hilangkan `border-t` jika menempel di atas langit-langit layar.

### 4. Typography
- Header harus tebal (`font-black` atau `font-extrabold`) dengan *tracking* rapat (`tracking-tighter` atau `tracking-tight`).
- Paragraf/Deskripsi menggunakan medium/semibold untuk keterbacaan.

---

## ⚙️ Implementasi Vue/Tailwind

Gunakan Tailwind *Dark Mode Configuration* berbasis *class*:
Di `tailwind.config.js` / Vite plugin:
```js
export default {
  darkMode: 'class', // <--- Aktifkan class strategy
  // ...
}
```

State Management (Pinia/Composable) untuk Theme:
```ts
// Terapkan class "dark" ke elemen <html> atau <body> secara default.
import { useDark, useToggle } from '@vueuse/core'

export const isDark = useDark({
  selector: 'html',
  attribute: 'class',
  valueDark: 'dark',
  valueLight: 'light',
  initialValue: 'dark', // Default ke Dark Mode
})

export const toggleDark = useToggle(isDark)
```

**Contoh Template Komponen Dual-Mode:**
```html
<!-- Background utama menyesuaikan dark/light -->
<div class="bg-[#F8F9FA] dark:bg-[#202124] text-black dark:text-white transition-colors">
  
  <!-- Card / Surface menyesuaikan border & bg -->
  <div class="bg-white dark:bg-[#28292C] border-[3px] border-black dark:border-[#5F6368] rounded-[32px] p-6">
    <h2 class="text-2xl font-black">Title</h2>
    <p class="text-gray-600 dark:text-gray-400">Description text goes here.</p>
  </div>
  
</div>
```
