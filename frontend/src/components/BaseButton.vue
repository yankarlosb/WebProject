<!-- Componente BaseButton: botón reutilizable -->
<template>
  <button
    :class="[
      'px-4 py-2 rounded-md font-medium transition',
      variant === 'primary' ? 'bg-primary text-white hover:bg-blue-700' :
      variant === 'secondary' ? 'bg-gray-200 text-gray-800 hover:bg-gray-300' :
      'bg-danger text-white hover:bg-red-700'
    ]"
  >
    <slot />
  </button>
</template>

<script setup lang="ts">
defineProps({
  variant: { type: String, default: 'primary' }
})
</script>