<template>
  <AppCard class="mb-6">
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-3 items-end">
      <!-- Año Académico -->
      <div>
        <label class="block text-xs font-medium text-gray-700 mb-1.5">Año Académico</label>
        <select
          :value="config.academicYear"
          @change="handleUpdate('academicYear', ($event.target as HTMLSelectElement).value)"
          class="w-full px-3 py-2 text-sm border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 disabled:bg-gray-100 disabled:cursor-not-allowed"
        >
          <option v-for="opt in yearOptions" :key="opt.value" :value="opt.value">
            {{ opt.label }}
          </option>
        </select>
      </div>

      <!-- Período -->
      <div>
        <label class="block text-xs font-medium text-gray-700 mb-1.5">Período</label>
        <select
          :value="config.period"
          @change="handleUpdate('period', ($event.target as HTMLSelectElement).value)"
          class="w-full px-3 py-2 text-sm border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
        >
          <option value="1ero">1ero</option>
          <option value="2do">2do</option>
        </select>
      </div>

      <!-- Año Lectivo -->
      <div>
        <label class="block text-xs font-medium text-gray-700 mb-1.5">Año Lectivo</label>
        <input
          :value="config.academicYearText"
          type="text"
          placeholder="2025-2026"
          @input="handleUpdate('academicYearText', ($event.target as HTMLInputElement).value)"
          class="w-full px-3 py-2 text-sm border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
        />
      </div>

      <!-- Fecha Inicio -->
      <div>
        <label class="block text-xs font-medium text-gray-700 mb-1.5">Fecha Inicio</label>
        <input
          :value="config.startDate"
          type="date"
          @input="handleUpdate('startDate', ($event.target as HTMLInputElement).value)"
          class="w-full px-3 py-2 text-sm border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
        />
      </div>
    </div>

    <!-- Botones de acción -->
    <div class="flex flex-wrap gap-2 mt-4 pt-4 border-t border-gray-200">
      <AppButton
        variant="success"
        @click="$emit('calculate')"
      >
        <template #icon>
          <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 7h6m0 10v-3m-3 3h.01M9 17h.01M9 14h.01M12 14h.01M15 11h.01M12 11h.01M9 11h.01M7 21h10a2 2 0 002-2V5a2 2 0 00-2-2H7a2 2 0 00-2 2v14a2 2 0 002 2z" />
          </svg>
        </template>
        Calcular
      </AppButton>

      <AppButton
        variant="primary"
        @click="$emit('save')"
        :loading="isSaving"
        :disabled="!hasUnsavedChanges"
      >
        <template #icon>
          <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7H5a2 2 0 00-2 2v9a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-3m-1 4l-3 3m0 0l-3-3m3 3V4" />
          </svg>
        </template>
        Guardar
      </AppButton>

      <AppButton
        variant="secondary"
        disabled
      >
        <template #icon>
          <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
          </svg>
        </template>
        Exportar a Excel
      </AppButton>
    </div>
  </AppCard>
</template>

<script setup lang="ts">
import AppCard from './AppCard.vue'
import AppButton from './AppButton.vue'
import { yearOptions } from '../utils/constants'

interface BalanceConfig {
  academicYear: string
  period: string
  academicYearText: string
  startDate: string
}

interface Props {
  config: BalanceConfig
  isSaving: boolean
  hasUnsavedChanges: boolean
}

defineProps<Props>()

const emit = defineEmits<{
  'update:config': [field: keyof BalanceConfig, value: string]
  'calculate': []
  'save': []
}>()

function handleUpdate(field: keyof BalanceConfig, value: string) {
  emit('update:config', field, value)
}
</script>
