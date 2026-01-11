<template>
  <div class="space-y-4">
    <!-- Leyenda de tipos de actividad -->
    <div class="bg-white rounded-lg border p-3">
      <h4 class="text-xs font-semibold text-gray-700 mb-2">Leyenda de Actividades</h4>
      <div class="flex flex-wrap gap-2">
        <span v-for="tipo in tiposActividad" :key="tipo.value" 
          class="inline-flex items-center gap-1 px-2 py-0.5 text-xs rounded"
          :class="getActivityClass(tipo.value)"
        >
          <span class="font-semibold">{{ tipo.value || '-' }}</span>
          <span class="text-gray-500">{{ tipo.label }}</span>
        </span>
      </div>
    </div>

    <!-- Tabla consolidada por grupos de semanas -->
    <div
      v-for="(group, groupIdx) in weekGroups"
      :key="`group-${groupIdx}`"
      class="border-2 rounded-lg bg-white overflow-hidden shadow-sm"
      :class="groupIdx < weekGroups.length - 1 ? 'border-blue-200' : 'border-purple-200'"
    >
      <!-- Header del grupo -->
      <div 
        class="px-4 py-2.5 flex items-center justify-between"
        :class="groupIdx < weekGroups.length - 1 ? 'bg-blue-600' : 'bg-purple-600'"
      >
        <span class="text-sm font-semibold text-white">
          📅 Semanas {{ group.start }} - {{ group.end }}
        </span>
        <span class="text-xs text-white/80">
          {{ group.weeks.length * 4 }} celdas × {{ fragments.length }} asignaturas
        </span>
      </div>

      <!-- Tabla -->
      <div class="overflow-x-auto">
        <table class="w-full divide-y divide-gray-200">
          <!-- Header de semanas -->
          <thead :class="groupIdx < weekGroups.length - 1 ? 'bg-blue-50' : 'bg-purple-50'">
            <tr>
              <th class="px-3 py-2 text-left text-xs font-bold uppercase min-w-[160px] sticky left-0 bg-inherit z-10"
                  :class="groupIdx < weekGroups.length - 1 ? 'text-blue-700' : 'text-purple-700'">
                Asignatura
              </th>
              <th
                v-for="week in group.weeks"
                :key="`week-header-${week}`"
                colspan="4"
                class="px-2 py-3 text-center font-semibold border-l border-gray-300"
                :class="[
                  groupIdx < weekGroups.length - 1 ? 'text-blue-700' : 'text-purple-700',
                  isWeekConflict(week) ? 'bg-red-100 text-red-700' : ''
                ]"
              >
                <div class="flex flex-col items-center gap-0.5">
                  <span class="text-sm" :class="isWeekConflict(week) ? 'text-red-700 font-bold' : ''">
                    S{{ week }}
                    <span v-if="isWeekConflict(week)" class="ml-1">⚠️</span>
                  </span>
                  <span v-if="weekDates.length > 0" class="text-xs font-medium opacity-80">
                    {{ getWeekDateRange(week) }}
                  </span>
                </div>
              </th>
            </tr>
          </thead>
          
          <!-- Filas de asignaturas -->
          <tbody class="bg-white divide-y divide-gray-100">
            <tr v-for="fragment in fragments" :key="`row-${fragment.id}-${groupIdx}`">
              <!-- Nombre de asignatura (sticky) -->
              <td class="px-3 py-2 font-semibold text-xs text-gray-900 bg-gray-50 truncate max-w-[160px] sticky left-0 z-10 border-r border-gray-200" 
                  :title="fragment.asignatura?.name">
                {{ fragment.asignatura?.name || 'Asignatura' }}
              </td>
              
              <!-- Celdas de cada semana (4 por semana) -->
              <template v-for="week in group.weeks" :key="`week-cells-${week}`">
                <td
                  v-for="dayIdx in 4"
                  :key="`cell-${week}-${dayIdx}`"
                  class="px-0.5 py-1 text-center"
                  :class="[
                    dayIdx === 1 ? 'border-l border-gray-300' : '',
                    isWeekConflict(week) ? 'bg-red-50' : ''
                  ]"
                >
                  <span
                    :class="[
                      'inline-block w-10 h-6 leading-6 text-center border rounded text-xs',
                      getCellClass(fragment, (week - 1) * 4 + dayIdx - 1)
                    ]"
                    :title="getCellTooltip(fragment, (week - 1) * 4 + dayIdx - 1)"
                  >
                    {{ getCellDisplayValue(fragment, (week - 1) * 4 + dayIdx - 1) }}
                  </span>
                </td>
              </template>
            </tr>
            
            <!-- Fila de totales PP por semana -->
            <tr class="bg-gray-100 font-semibold">
              <td class="px-3 py-2 text-xs text-gray-700 sticky left-0 bg-gray-100 z-10 border-r border-gray-200">
                Total PP
              </td>
              <template v-for="week in group.weeks" :key="`total-${week}`">
                <td colspan="4" class="text-center text-xs border-l border-gray-300"
                    :class="isWeekConflict(week) ? 'bg-red-200 text-red-800' : 'text-gray-600'">
                  {{ getWeekPPCount(week) }}
                  <span v-if="isWeekConflict(week)"> (máx 2)</span>
                </td>
              </template>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- Sección de Consultas y Exámenes Finales -->
    <div class="border-2 border-red-200 rounded-lg bg-white overflow-hidden shadow-sm">
      <div class="bg-red-600 px-4 py-2.5 flex items-center justify-between">
        <span class="text-sm font-semibold text-white">📝 Consultas y Exámenes Finales</span>
        <span class="text-xs text-red-100">9 celdas × {{ fragments.length }} asignaturas</span>
      </div>

      <div class="overflow-x-auto">
        <table class="w-full divide-y divide-gray-200">
          <thead class="bg-red-50">
            <tr>
              <th class="px-3 py-2 text-left text-xs font-bold text-red-700 uppercase min-w-[160px] sticky left-0 bg-red-50 z-10">
                Asignatura
              </th>
              <th colspan="4" class="px-2 py-3 text-center font-semibold text-red-700 border-l border-gray-300">
                <div class="flex flex-col items-center gap-0.5">
                  <span class="text-sm">Consultas</span>
                  <span v-if="finalWeekDates.length > 0" class="text-xs font-medium opacity-80">
                    {{ finalWeekDates[0]?.displayRange || '' }}
                  </span>
                </div>
              </th>
              <th colspan="5" class="px-2 py-3 text-center font-semibold text-red-700 border-l border-gray-300">
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
            <tr v-for="fragment in fragments" :key="`final-${fragment.id}`">
              <td class="px-3 py-2 font-semibold text-xs text-gray-900 bg-gray-50 truncate max-w-[160px] sticky left-0 z-10 border-r border-gray-200" 
                  :title="fragment.asignatura?.name">
                {{ fragment.asignatura?.name || 'Asignatura' }}
              </td>
              
              <!-- Consultas: 4 celdas -->
              <td
                v-for="cellIdx in 4"
                :key="`cons-${cellIdx}`"
                class="px-0.5 py-1 text-center"
                :class="cellIdx === 1 ? 'border-l border-gray-300' : ''"
              >
                <span
                  :class="[
                    'inline-block w-10 h-6 leading-6 text-center border rounded text-xs',
                    getCellClass(fragment, finalStartIndex + cellIdx - 1)
                  ]"
                >
                  {{ getCellDisplayValue(fragment, finalStartIndex + cellIdx - 1) }}
                </span>
              </td>
              
              <!-- Exámenes Finales: 5 celdas -->
              <td
                v-for="cellIdx in 5"
                :key="`exam-${cellIdx}`"
                class="px-0.5 py-1 text-center"
                :class="cellIdx === 1 ? 'border-l border-gray-300' : ''"
              >
                <span
                  :class="[
                    'inline-block w-10 h-6 leading-6 text-center border rounded text-xs',
                    getCellClass(fragment, finalStartIndex + 4 + cellIdx - 1)
                  ]"
                >
                  {{ getCellDisplayValue(fragment, finalStartIndex + 4 + cellIdx - 1) }}
                </span>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { Fragment } from '@/services/balances'
import type { WeekDateInfo } from '../utils/balance-table'

interface Props {
  fragments: Fragment[]
  weeks: number
  weekDates?: WeekDateInfo[]
  finalWeekDates?: WeekDateInfo[]
}

const props = withDefaults(defineProps<Props>(), {
  weeks: 15,
  weekDates: () => [],
  finalWeekDates: () => []
})

// Obtener fecha de una semana
function getWeekDateRange(weekNumber: number): string {
  if (!props.weekDates || props.weekDates.length === 0) return ''
  const weekInfo = props.weekDates.find(w => w.weekNumber === weekNumber)
  return weekInfo ? weekInfo.displayRange : ''
}

// Tipos de actividad para la leyenda
const tiposActividad = [
  { value: 'C', label: 'Conferencia' },
  { value: 'CP', label: 'Clase Práctica' },
  { value: 'S', label: 'Seminario' },
  { value: 'PL', label: 'Práctica Lab.' },
  { value: 'T', label: 'Tutoría' },
  { value: 'TE', label: 'Trabajo Extra.' },
  { value: 'PP', label: 'Prueba Parcial' },
]

// Agrupar semanas en grupos de 4 para mejor visualización
const weekGroups = computed(() => {
  const groups = []
  for (let i = 0; i < props.weeks; i += 4) {
    const start = i + 1
    const end = Math.min(i + 4, props.weeks)
    const weekNums = []
    for (let w = start; w <= end; w++) {
      weekNums.push(w)
    }
    groups.push({ start, end, weeks: weekNums })
  }
  return groups
})

// Índice donde empiezan consultas y exámenes finales
const finalStartIndex = computed(() => props.weeks * 4)

// Calcular conflictos de PP por semana (más de 2 PP en total)
const weekPPCounts = computed(() => {
  const counts: Record<number, number> = {}
  
  for (let week = 1; week <= props.weeks; week++) {
    counts[week] = 0
    const startIdx = (week - 1) * 4
    const endIdx = startIdx + 4
    
    for (const fragment of props.fragments) {
      const values = fragment.data?.values as string[] | undefined
      if (!values || !Array.isArray(values)) continue
      
      for (let i = startIdx; i < endIdx && i < values.length; i++) {
        if (values[i] === 'PP') {
          counts[week] = (counts[week] || 0) + 1
        }
      }
    }
  }
  
  return counts
})

// Verificar si una semana tiene conflicto
function isWeekConflict(week: number): boolean {
  return (weekPPCounts.value[week] || 0) > 2
}

// Obtener conteo de PP de una semana
function getWeekPPCount(week: number): number {
  return weekPPCounts.value[week] || 0
}

// Obtener valor de una celda
function getCellDisplayValue(fragment: Fragment, cellIndex: number): string {
  const values = fragment.data?.values as string[] | undefined
  if (!values || !Array.isArray(values)) return '-'
  return values[cellIndex] || '-'
}

// Tooltip para la celda
function getCellTooltip(fragment: Fragment, cellIndex: number): string {
  const value = getCellDisplayValue(fragment, cellIndex)
  if (value === '-') return 'Sin actividad'
  
  const labels: Record<string, string> = {
    'C': 'Conferencia',
    'CP': 'Clase Práctica',
    'S': 'Seminario',
    'PL': 'Práctica de Laboratorio',
    'T': 'Tutoría',
    'TE': 'Trabajo Extraclase',
    'PP': 'Prueba Parcial',
  }
  
  return labels[value] || value
}

// Clase CSS para una celda según su valor
function getCellClass(fragment: Fragment, cellIndex: number): string {
  const value = getCellDisplayValue(fragment, cellIndex)
  
  if (value === '-' || !value) {
    return 'bg-gray-50 text-gray-300 border-gray-200'
  }
  
  // Colores por tipo de actividad
  const colors: Record<string, string> = {
    'C': 'bg-blue-100 text-blue-800 border-blue-300 font-semibold',
    'CP': 'bg-green-100 text-green-800 border-green-300 font-semibold',
    'S': 'bg-yellow-100 text-yellow-800 border-yellow-300 font-semibold',
    'PL': 'bg-purple-100 text-purple-800 border-purple-300 font-semibold',
    'T': 'bg-cyan-100 text-cyan-800 border-cyan-300 font-semibold',
    'TE': 'bg-orange-100 text-orange-800 border-orange-300 font-semibold',
    'PP': 'bg-red-100 text-red-800 border-red-300 font-bold',
  }
  
  return colors[value] || 'bg-gray-100 text-gray-800 border-gray-300 font-semibold'
}

// Clase CSS para la leyenda
function getActivityClass(value: string): string {
  if (!value) return 'bg-gray-100 text-gray-500'
  
  const colors: Record<string, string> = {
    'C': 'bg-blue-100 text-blue-800',
    'CP': 'bg-green-100 text-green-800',
    'S': 'bg-yellow-100 text-yellow-800',
    'PL': 'bg-purple-100 text-purple-800',
    'T': 'bg-cyan-100 text-cyan-800',
    'TE': 'bg-orange-100 text-orange-800',
    'PP': 'bg-red-100 text-red-800',
  }
  
  return colors[value] || 'bg-gray-100 text-gray-800'
}
</script>

<style scoped>
/* Asegurar que las columnas sticky funcionen bien */
.sticky {
  position: sticky;
}
</style>
