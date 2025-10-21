<template>
  <div v-if="show" class="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
    <div class="bg-white dark:bg-gray-800 rounded-lg p-6 w-96 shadow-lg">
      <div class="text-lg font-semibold mb-4 text-gray-900 dark:text-gray-100">Confirmar</div>
      <div class="mb-4 text-gray-700 dark:text-gray-200">{{ message }}</div>
      <div class="text-right space-x-2">
        <button @click="$emit('cancel')" class="px-3 py-1 bg-gray-200 rounded">Cancelar</button>
        <button @click="$emit('confirm')" class="px-3 py-1 bg-red-600 text-white rounded">Confirmar</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{ show: boolean; message: string }>()
defineEmits(['confirm', 'cancel'])
</script>

<style scoped>
/* Estilos scoped mínimos (si son necesarios) */
</style>
