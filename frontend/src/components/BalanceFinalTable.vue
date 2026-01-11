<template>
  <div class="border-2 border-green-200 rounded-lg bg-white overflow-hidden shadow-sm hover:shadow-md transition-shadow">
    <div class="bg-gradient-to-r from-green-600 to-green-500 px-4 py-2.5 flex items-center justify-between">
      <span class="text-sm font-semibold text-white">✅ Consultas y Exámenes Finales</span>
      <span class="text-xs text-green-100">9 celdas por asignatura</span>
    </div>
    
    <div class="overflow-x-auto">
      <table class="w-full divide-y divide-gray-200">
        <thead class="bg-green-50">
          <tr>
            <th class="px-3 py-2 text-left text-xs font-bold text-green-700 uppercase min-w-[140px]">
              Asignatura
            </th>
            <th colspan="4" class="px-2 py-3 text-center font-semibold text-green-700 border-l border-gray-300">
              <div class="flex flex-col items-center gap-0.5">
                <span class="text-sm">Consultas</span>
                <span v-if="finalWeekDates.length > 0" class="text-xs font-medium opacity-80">
                  {{ finalWeekDates[0]?.displayRange || '' }}
                </span>
              </div>
            </th>
            <th colspan="5" class="px-2 py-3 text-center font-semibold text-green-700 border-l border-gray-300">
              <div class="flex flex-col items-center gap-0.5">
                <span class="text-sm">Exámenes Finales</span>
                <span v-if="finalWeekDates.length > 1" class="text-xs font-medium opacity-80">
                  {{ finalWeekDates[1]?.displayRange || '' }}
                </span>
              </div>
            </th>
            <th class="px-3 py-2 text-center text-xs font-bold text-red-700 uppercase border-l border-gray-300">
              Acciones
            </th>
          </tr>
        </thead>
        <tbody class="bg-white divide-y divide-gray-100">
          <tr
            v-for="subject in subjects"
            :key="`final-${subject.id}`"
            class="hover:bg-green-50/50 transition-colors"
          >
            <td class="px-3 py-2 font-semibold text-xs text-gray-900 bg-gray-50 truncate max-w-[140px]" :title="subject.name">
              {{ subject.name }}
            </td>
            
            <!-- Consultas: 4 celdas -->
            <td
              v-for="cellIndex in 4"
              :key="`cons-${cellIndex}`"
              class="px-0.5 py-1 text-center"
              :class="cellIndex === 1 ? 'border-l border-gray-300' : ''"
            >
              <select
                :value="getCellValue(subject.values[consultasStartIndex + cellIndex - 1])"
                @change="(e) => $emit('update-value', subject.id, consultasStartIndex + cellIndex - 1, e)"
                class="w-11 h-7 text-center border border-gray-200 rounded text-xs focus:ring-2 focus:ring-green-500 focus:border-green-500 outline-none cursor-pointer appearance-none px-1"
                :class="getCellValue(subject.values[consultasStartIndex + cellIndex - 1]) ? 'bg-green-50 font-semibold' : 'bg-white'"
              >
                <option v-for="tipo in tiposActividadBalance" :key="tipo.value" :value="tipo.value">
                  {{ tipo.label }}
                </option>
              </select>
            </td>
            
            <!-- Exámenes Finales: 5 celdas -->
            <td
              v-for="cellIndex in 5"
              :key="`exam-${cellIndex}`"
              class="px-0.5 py-1 text-center"
              :class="cellIndex === 1 ? 'border-l border-gray-300' : ''"
            >
              <select
                :value="getCellValue(subject.values[examenesStartIndex + cellIndex - 1])"
                @change="(e) => $emit('update-value', subject.id, examenesStartIndex + cellIndex - 1, e)"
                class="w-11 h-7 text-center border border-gray-200 rounded text-xs focus:ring-2 focus:ring-green-500 focus:border-green-500 outline-none cursor-pointer appearance-none px-1"
                :class="getCellValue(subject.values[examenesStartIndex + cellIndex - 1]) ? 'bg-green-50 font-semibold' : 'bg-white'"
              >
                <option v-for="tipo in tiposActividadBalance" :key="tipo.value" :value="tipo.value">
                  {{ tipo.label }}
                </option>
              </select>
            </td>
            
            <!-- Botón eliminar -->
            <td class="px-3 py-2 text-center border-l border-gray-300">
              <button
                @click="$emit('delete-subject', subject.id)"
                class="inline-flex items-center gap-1 px-2 py-1 text-xs font-medium text-red-700 bg-red-50 hover:bg-red-100 rounded-lg transition-colors"
              >
                <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                </svg>
                Eliminar
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { tiposActividadBalance } from '../utils/constants'
import { getCellValue, type WeekDateInfo } from '../utils/balance-table'

interface Subject {
  id: string
  name: string
  values: (number | string)[]
}

interface Props {
  subjects: Subject[]
  weeksCount?: number  // Número de semanas lectivas (15 o 16)
  finalWeekDates?: WeekDateInfo[]
}

const props = withDefaults(defineProps<Props>(), {
  weeksCount: 15,
  finalWeekDates: () => []
})

defineEmits<{
  'update-value': [subjectId: string, cellIndex: number, event: Event]
  'delete-subject': [subjectId: string]
}>()

// Índice de inicio para consultas = semanas * 4 celdas por semana
const consultasStartIndex = computed(() => props.weeksCount * 4)

// Índice de inicio para exámenes = consultas + 4 celdas
const examenesStartIndex = computed(() => consultasStartIndex.value + 4)
</script>

<style scoped>
select {
  background-image: none;
}

select:focus {
  transform: scale(1.05);
  transition: transform 0.15s ease;
}
</style>
