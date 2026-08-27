# 🎨 Cliptzy Design System (Spatial Bento Box)

## 📌 Overview
Design system ini dirancang khusus untuk memaksimalkan kapabilitas aplikasi Desktop native Cliptzy (berbasis Tauri + Vue 3). Gaya visual yang digunakan adalah **"Spatial Bento Box"** — antarmuka NLE (Non-Linear Editing) yang revolusioner, modern, *out-of-the-box*, berkelas profesional, dan responsif.

Kita membuang desain generik ("AI Slop" dengan gradien ungu/pink) dan beralih ke tata letak berbasis grid modular (Bento) yang padat informasi, efisien, dan bersih dengan dominasi *Deep Dark Mode*.

---

## 📐 Prinsip Utama (Core Principles)
1. **Bento Grid System**: Tata letak modular menggunakan kartu (cards) dengan sudut membulat yang presisi, menampung banyak data tanpa membuat UI terasa penuh.
2. **Deep Dark Mode (OLED-inspired)**: Palet warna dominan gelap pekat (True Black & Deep Zinc) untuk mengurangi kelelahan mata kreator saat mengedit video dalam waktu lama.
3. **High Contrast Accents**: Menggunakan aksen warna cerah (seperti *Electric Lime*) khusus untuk *Call to Action* (CTA) dan state aktif.
4. **Micro-Glassmorphism & Spatial Depth**: Efek *blur* tipis (`backdrop-filter`) berlapis pada panel mengambang, memberi ilusi kedalaman tanpa memberatkan *rendering* aplikasi.
5. **Borderless / Custom Window Frame**: Memanfaatkan *frameless window* dari Tauri dengan custom titlebar, membuat UI terasa menyatu mulus ke layar.

---

## 🎨 Palet Warna (Color Palette)

Seluruh aplikasi berjalan secara eksklusif dalam **Dark Mode**. Warna diimplementasikan via CSS Variables di Tailwind CSS.

### 1. Warna Dasar & Permukaan (Backgrounds)
- **Base (Background Absolut)**: `#09090B` (Sangat gelap, nyaris hitam pekat). Digunakan untuk *app shell* / body utama.
- **Surface (Warna Kartu Bento)**: `#18181B` (Zinc-900). Digunakan untuk latar belakang komponen, sidebar, dan panel.

### 2. Aksen & Interaksi
- **Accent Primary (Electric Lime)**: `#D9F99D` (Lime-200). Digunakan untuk tombol utama, indikator progress, dan elemen aktif/terpilih. Warna ini dipilih karena sangat kontras dan segar terhadap latar gelap.
- **Accent Hover/Glow**: Efek kilau neon pada tombol *Execute* atau *Generate*.

### 3. Teks & Garis
- **Teks Utama**: `#FFFFFF` atau `#F4F4F5` (Zinc-50).
- **Teks Sekunder (Muted)**: `#A1A1AA` (Zinc-400).
- **Border Subtle (Garis Pemisah)**: `#27272A` (Zinc-800). Sangat tipis (`border`), 1px solid.

---

## 🔲 Komponen & Elemen UI

### 1. Bento Card
Kartu penampung informasi atau form dengan gaya "Bento Box".
- **Styling**: `bg-[#18181B] border border-[#27272A] rounded-2xl p-4` (atau `rounded-[24px]` untuk panel utama).
- **Efek**: Tanpa *drop shadow* tebal. Gunakan *subtle inner highlight* jika diperlukan.

### 2. Tombol Utama (Action Core / Glow Button)
Tombol utama (seperti *Generate Clips*) tidak boleh terlihat biasa.
- **Styling**: Latar belakang `#D9F99D` teks `#09090B` dengan *font weight* tebal.
- **Hover State**: Efek *cyber-glow* (`shadow-[0_0_15px_rgba(217,249,157,0.4)]`).
- **Rounding**: `rounded-xl` atau `rounded-2xl` konsisten dengan Bento Card.

### 3. Kontrol Spatial (Inputs, Sliders, Switches)
- **Inputs**: Latar belakang transparan atau sangat gelap (`bg-[#09090B]`), dengan `border-[#27272A]` yang menyala (focus) menggunakan aksen warna *Lime*.
- **Toggle Switches**: Desain *pill* klasik, indikator hijau/lime menyala saat aktif.

### 4. Typography
- Menggunakan font geometris modern (seperti *Geist*, *Inter Tight*, atau *Space Grotesk*).
- **Header**: `font-bold` atau `font-black` dengan *tracking* netral hingga rapat (`tracking-tight`).
- **Data/Angka**: Gunakan `font-mono` atau *tabular nums* untuk durasi, timestamp, dan metrik analitik (agar lebar karakter statis dan rapi).

---

## ⚙️ Integrasi Tata Letak (Layout Structure)

- **The Grid (Shell)**: Dibangun menggunakan CSS Flexbox atau Grid.
- **Sidebar**: *Floating* / responsif, dengan state aktif ditandai oleh garis vertikal bercahaya atau mask transisi (bukan sekadar blok latar warna penuh).
- **Status Bar**: `GlobalStatusBar.vue` di sudut layar dengan indikator *real-time processing* ala *Command Line Output* yang elegan.
- **Transisi**: Animasi harus *fluid* dan cepat. Gunakan kombinasi `scale` tipis (`98%` -> `100%`) dan `fade`.
