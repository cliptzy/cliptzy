<template>
  <div
    data-tauri-drag-region
    class="h-10 w-full bg-transparent sticky top-0 z-[999] select-none"
  >
    <!-- Pada macOS, tombol native (close, minimize, expand) akan otomatis dirender oleh OS di sisi kiri area ini -->
  </div>
</template>

<script setup lang="ts">
// Kita tidak memerlukan tombol kustom lagi karena kita mengaktifkan `titleBarStyle: "Overlay"` di Tauri.
// OS (macOS) akan mengambil alih tombol dan membuat window memiliki rounded corner native.
</script>
