<template>
  <transition name="fade">
    <div v-if="show" class="fixed right-6 bottom-6 z-50">
      <div :class="['px-4 py-2 rounded shadow-lg text-white', variantClass]">{{ message }}</div>
    </div>
  </transition>
</template>

<script setup lang="ts">
import { computed } from 'vue'
const props = defineProps<{ show: boolean; message: string; type?: 'success' | 'error' | 'info' }>()
const variantClass = computed(() => {
  if (props.type === 'success') return 'bg-green-600'
  if (props.type === 'error') return 'bg-red-600'
  return 'bg-blue-600'
})
</script>

<style scoped>
.fade-enter-active, .fade-leave-active { transition: opacity .25s }
.fade-enter-from, .fade-leave-to { opacity: 0 }
</style>
