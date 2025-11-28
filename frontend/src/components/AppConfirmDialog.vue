<!--
  AppConfirmDialog - Diálogo de confirmación global
  Conectado con uiStore para manejo centralizado
-->
<template>
  <AppModal
    :model-value="confirmDialog.show"
    size="sm"
    :close-on-overlay="false"
    :close-on-escape="false"
    hide-close
  >
    <div class="text-center py-4">
      <!-- Icon de advertencia -->
      <div class="mx-auto flex items-center justify-center h-16 w-16 rounded-full bg-yellow-100 mb-4">
        <svg 
          class="h-8 w-8 text-yellow-600" 
          fill="none" 
          viewBox="0 0 24 24" 
          stroke="currentColor"
        >
          <path 
            stroke-linecap="round" 
            stroke-linejoin="round" 
            stroke-width="2" 
            d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" 
          />
        </svg>
      </div>

      <!-- Título -->
      <h3 class="text-lg font-bold text-gray-900 mb-2">
        {{ confirmDialog.title }}
      </h3>

      <!-- Mensaje -->
      <p class="text-sm text-gray-600 mb-6">
        {{ confirmDialog.message }}
      </p>
    </div>

    <!-- Footer con botones -->
    <template #footer>
      <AppButton
        variant="ghost"
        size="md"
        @click="handleCancel"
      >
        {{ confirmDialog.cancelText }}
      </AppButton>
      
      <AppButton
        variant="danger"
        size="md"
        @click="handleConfirm"
      >
        {{ confirmDialog.confirmText }}
      </AppButton>
    </template>
  </AppModal>
</template>

<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { useUIStore } from '../stores/ui'
import AppModal from './AppModal.vue'
import AppButton from './AppButton.vue'

const uiStore = useUIStore()
const { confirmDialog } = storeToRefs(uiStore)

function handleConfirm() {
  uiStore.closeConfirm(true)
}

function handleCancel() {
  uiStore.closeConfirm(false)
}
</script>
