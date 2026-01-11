<template>
  <div class="border-2 border-green-200 rounded-lg bg-white overflow-hidden shadow-sm">
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
          </tr>
        </thead>
        <tbody class="bg-white divide-y divide-gray-100">
          <tr v-for="subject in subjects" :key="`final-${subject.id}`">
            <td class="px-3 py-2 font-semibold text-xs text-gray-900 bg-gray-50 truncate max-w-[140px]" 
                :title="subject.name">
              {{ subject.name }}
            </td>
            
            <!-- Consultas: 4 celdas -->
            <td
              v-for="cellIndex in 4"
              :key="`cons-${cellIndex}`"
              class="px-0.5 py-1 text-center"
              :class="cellIndex === 1 ? 'border-l border-gray-300' : ''"
            >
              <span
                :class="[
                  'inline-block w-11 h-7 leading-7 text-center border border-gray-200 rounded text-xs',
                  getCellValue(subject.values[consultasStartIndex + cellIndex - 1]) 
                    ? 'bg-green-50 text-green-800 font-semibold' 
                    : 'bg-gray-50 text-gray-400'
                ]"
              >
                {{ getCellValue(subject.values[consultasStartIndex + cellIndex - 1]) || '-' }}
              </span>
            </td>
            
            <!-- Exámenes Finales: 5 celdas -->
            <td
              v-for="cellIndex in 5"
              :key="`exam-${cellIndex}`"
              class="px-0.5 py-1 text-center"
              :class="cellIndex === 1 ? 'border-l border-gray-300' : ''"
            >
              <span
                :class="[
                  'inline-block w-11 h-7 leading-7 text-center border border-gray-200 rounded text-xs',
                  getCellValue(subject.values[examenesStartIndex + cellIndex - 1]) 
                    ? 'bg-green-50 text-green-800 font-semibold' 
                    : 'bg-gray-50 text-gray-400'
                ]"
              >
                {{ getCellValue(subject.values[examenesStartIndex + cellIndex - 1]) || '-' }}
              </span>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { getCellValue, type WeekDateInfo } from '../utils/balance-table'

interface Subject {
  id: string
  name: string
  values: (number | string)[]
}

interface Props {
  subjects: Subject[]
  weeksCount?: number
  finalWeekDates?: WeekDateInfo[]
}

const props = withDefaults(defineProps<Props>(), {
  weeksCount: 15,
  finalWeekDates: () => []
})

// Índice de inicio para consultas = semanas * 4 celdas por semana
const consultasStartIndex = computed(() => props.weeksCount * 4)

// Índice de inicio para exámenes = consultas + 4 celdas
const examenesStartIndex = computed(() => consultasStartIndex.value + 4)
</script>
