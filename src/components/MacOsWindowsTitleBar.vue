<template>
  <header
    data-tauri-drag-region
    class="h-10 w-full bg-base-200 flex items-center shrink-0 z-50 select-none relative"
    style="border-bottom: 1px solid color-mix(in srgb, var(--color-base-content) 8%, transparent)"
  >
    <!-- Title / Content -->
    <div class="absolute inset-0 flex items-center justify-center pointer-events-none" data-tauri-drag-region>
        <span class="text-[11px] font-bold text-secondary tracking-[0.15em] uppercase">Cliptzy</span>
    </div>
  </header>
</template>

<script setup lang="ts">
// Kita tidak memerlukan tombol kustom lagi karena kita mengaktifkan `titleBarStyle: "Overlay"` di Tauri.
// OS (macOS) akan mengambil alih tombol dan membuat window memiliki rounded corner native.
</script>


