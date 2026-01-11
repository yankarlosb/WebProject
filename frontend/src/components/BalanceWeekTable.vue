<template>
  <div :class="[
    'border-2 rounded-lg bg-white overflow-hidden shadow-sm hover:shadow-md transition-shadow',
    colorClasses.border
  ]">
    <!-- Header -->
    <div :class="[
      'px-4 py-2.5 flex items-center justify-between',
      colorClasses.header
    ]">
      <span class="text-sm font-semibold text-white">{{ headerIcon }} {{ title }}</span>
      <span :class="[
        'text-xs',
        colorClasses.headerBadge
      ]">{{ cellsPerSubject }} celdas por asignatura</span>
    </div>

    <!-- Table -->
    <div class="overflow-x-auto">
      <table class="w-full divide-y divide-gray-200">
        <thead :class="colorClasses.tableHeader">
          <tr>
            <th class="px-3 py-2 text-left text-xs font-bold uppercase min-w-[140px]"
                :class="colorClasses.headerText">
              Asignatura
            </th>
            <th
              v-for="week in weeks"
              :key="`week-${week}`"
              :colspan="columnsPerWeek"
              :class="[
                'px-2 py-3 text-center font-semibold border-l border-gray-300',
                colorClasses.headerText
              ]"
              :title="getWeekDateRange(week)"
            >
              <div class="flex flex-col items-center gap-0.5">
                <span class="text-sm">S{{ week }}</span>
                <span v-if="weekDates.length > 0" class="text-xs font-medium opacity-80">
                  {{ getWeekDateRange(week) }}
                </span>
              </div>
            </th>
            <th v-if="showActions"
                class="px-3 py-2 text-center text-xs font-bold text-red-700 uppercase border-l border-gray-300">
              Acciones
            </th>
          </tr>
        </thead>
        <tbody class="bg-white divide-y divide-gray-100">
          <tr
            v-for="subject in subjects"
            :key="`subject-${subject.id}`"
            :class="colorClasses.rowHover"
          >
            <td class="px-3 py-2 font-semibold text-xs text-gray-900 bg-gray-50 truncate max-w-[140px]" :title="subject.name">
              {{ subject.name }}
            </td>
            <td
              v-for="(cellIndex, idx) in totalCells"
              :key="`cell-${cellIndex}`"
              class="px-0.5 py-1 text-center"
              :class="idx % columnsPerWeek === 0 ? 'border-l border-gray-300' : ''"
            >
              <select
                :value="getCellValue(subject.values[startCellIndex + idx])"
                @change="(e) => handleSelectChange(subject.id, startCellIndex + idx, e)"
                :class="[
                  'w-11 h-7 text-center border border-gray-200 rounded text-xs focus:ring-2 outline-none cursor-pointer appearance-none px-1',
                  colorClasses.focusRing,
                  getCellValue(subject.values[startCellIndex + idx]) ? `${colorClasses.cellFilled} font-semibold` : 'bg-white'
                ]"
              >
                <option v-for="tipo in tiposActividadBalance" :key="tipo.value" :value="tipo.value">
                  {{ tipo.label }}
                </option>
              </select>
            </td>
            <td v-if="showActions" class="px-3 py-2 text-center border-l border-gray-300">
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
import { getCellValue, editableColorClasses, type ColorScheme, type WeekDateInfo } from '../utils/balance-table'

interface Subject {
  id: string
  name: string
  values: (number | string)[]
}

interface Props {
  subjects: Subject[]
  title: string
  weeks: number[]
  startCellIndex: number
  columnsPerWeek?: number
  headerIcon?: string
  colorScheme?: ColorScheme
  showActions?: boolean
  weekDates?: WeekDateInfo[]  // Información de fechas de semanas
}

const props = withDefaults(defineProps<Props>(), {
  columnsPerWeek: 4,
  headerIcon: '📅',
  colorScheme: 'blue',
  showActions: false,
  weekDates: () => []
})

const emit = defineEmits<{
  'update-value': [subjectId: string, cellIndex: number, event: Event]
  'delete-subject': [subjectId: string]
}>()

// Maneja el cambio en el select
function handleSelectChange(subjectId: string, cellIndex: number, event: Event) {
  emit('update-value', subjectId, cellIndex, event)
}

// Obtener fecha de una semana
function getWeekDateRange(weekNumber: number): string {
  if (!props.weekDates || props.weekDates.length === 0) return ''
  const weekInfo = props.weekDates.find(w => w.weekNumber === weekNumber)
  return weekInfo ? weekInfo.displayRange : ''
}

// Computed
const totalCells = computed(() => props.weeks.length * props.columnsPerWeek)
const cellsPerSubject = computed(() => totalCells.value)

// Get all color classes for the current scheme - single reactive dependency
// Returns the complete ColorClasses object, avoiding multiple lookups in template
const colorClasses = computed(() => editableColorClasses[props.colorScheme])
</script>

<style scoped>
select {
  background-image: none;
}

select:focus {
  transform: scale(1.05);
  transition: transform 0.15s ease;
}

/* Ocultar flecha del select en WebKit */
select::-webkit-outer-spin-button,
select::-webkit-inner-spin-button {
  -webkit-appearance: none;
  margin: 0;
}
</style>
