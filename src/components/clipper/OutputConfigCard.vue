<script setup lang="ts">
import IconScissors from '~icons/lucide/scissors'

defineProps<{
  aspectRatio: string
}>()

const emit = defineEmits<{
  (e: 'update:aspectRatio', value: string): void
  (e: 'submit'): void
}>()
</script>

<template>
  <div class="border-[3px] border-black dark:border-[#3C4043] rounded-[32px] bg-[#E8F0FE] dark:bg-[#1A233A] p-8 flex flex-col justify-between transition-colors">
    <div>
      <h3 class="text-xl font-black mb-6 text-black dark:text-white">Rasio Aspek Output</h3>
      <div class="flex gap-4">
        <button 
          @click="emit('update:aspectRatio', '9:16')"
          :class="['border-[3px] border-black dark:border-[#3C4043] rounded-full px-6 py-3 font-bold flex-1 transition-colors', aspectRatio === '9:16' ? 'bg-[#4285F4] text-white border-transparent' : 'bg-white dark:bg-[#28292C] text-black dark:text-white hover:bg-gray-50']"
        >
          9:16 (Shorts)
        </button>
        <button 
          @click="emit('update:aspectRatio', '16:9')"
          :class="['border-[3px] border-black dark:border-[#3C4043] rounded-full px-6 py-3 font-bold flex-1 transition-colors', aspectRatio === '16:9' ? 'bg-[#4285F4] text-white border-transparent' : 'bg-white dark:bg-[#28292C] text-black dark:text-white hover:bg-gray-50']"
        >
          16:9 (YouTube)
        </button>
      </div>
    </div>

    <button 
      @click="emit('submit')"
      class="mt-8 border-[3px] border-black dark:border-transparent rounded-full bg-[#34A853] hover:bg-[#2E9349] text-white font-black py-4 px-6 transition-all w-full text-xl flex justify-center items-center gap-3 active:scale-[0.98]"
    >
      <IconScissors class="w-6 h-6" />
      Proses Klip Sekarang
    </button>
  </div>
</template>
