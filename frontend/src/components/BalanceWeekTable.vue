<template>
  <div :class="[
    'border-2 rounded-lg bg-white overflow-hidden shadow-sm hover:shadow-md transition-shadow',
    borderColorClass
  ]">
    <!-- Header -->
    <div :class="[
      'px-4 py-2.5 flex items-center justify-between',
      headerColorClass
    ]">
      <span class="text-sm font-semibold text-white">{{ headerIcon }} {{ title }}</span>
      <span :class="[
        'text-xs',
        headerBadgeColorClass
      ]">{{ cellsPerSubject }} celdas por asignatura</span>
    </div>

    <!-- Table -->
    <div class="overflow-x-auto">
      <table class="w-full divide-y divide-gray-200">
        <thead :class="tableHeaderColorClass">
          <tr>
            <th class="px-3 py-2 text-left text-xs font-bold uppercase min-w-[140px]"
                :class="headerTextColorClass">
              Asignatura
            </th>
            <th
              v-for="week in weeks"
              :key="`week-${week}`"
              :colspan="columnsPerWeek"
              :class="[
                'px-1 py-2 text-center text-xs font-semibold border-l border-gray-300',
                headerTextColorClass
              ]"
            >
              S{{ week }}
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
            :class="rowHoverColorClass"
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
                  focusRingClass,
                  getCellValue(subject.values[startCellIndex + idx]) ? `${cellFilledClass} font-semibold` : 'bg-white'
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
  colorScheme?: 'blue' | 'purple' | 'green'
  showActions?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  columnsPerWeek: 4,
  headerIcon: '📅',
  colorScheme: 'blue',
  showActions: false
})

const emit = defineEmits<{
  'update-value': [subjectId: string, cellIndex: number, event: Event]
  'delete-subject': [subjectId: string]
}>()

// Convierte valores numéricos legacy a string o devuelve el string
function getCellValue(value: number | string | undefined): string {
  if (value === undefined || value === null || value === 0 || value === '') {
    return ''
  }
  // Si es un número mayor a 0, lo tratamos como valor legacy (se podría mapear o ignorar)
  if (typeof value === 'number') {
    return ''
  }
  return value
}

// Maneja el cambio en el select
function handleSelectChange(subjectId: string, cellIndex: number, event: Event) {
  emit('update-value', subjectId, cellIndex, event)
}

// Computed
const totalCells = computed(() => props.weeks.length * props.columnsPerWeek)
const cellsPerSubject = computed(() => totalCells.value)

// Color classes based on scheme - defined as const to avoid recreation
const colorClasses = {
  blue: {
    border: 'border-blue-200',
    header: 'bg-gradient-to-r from-blue-600 to-blue-500',
    headerBadge: 'text-blue-100',
    tableHeader: 'bg-blue-50',
    headerText: 'text-blue-700',
    rowHover: 'hover:bg-blue-50/50 transition-colors',
    focusRing: 'focus:ring-blue-500 focus:border-blue-500',
    cellFilled: 'bg-blue-50'
  },
  purple: {
    border: 'border-purple-200',
    header: 'bg-gradient-to-r from-purple-600 to-purple-500',
    headerBadge: 'text-purple-100',
    tableHeader: 'bg-purple-50',
    headerText: 'text-purple-700',
    rowHover: 'hover:bg-purple-50/50 transition-colors',
    focusRing: 'focus:ring-purple-500 focus:border-purple-500',
    cellFilled: 'bg-purple-50'
  },
  green: {
    border: 'border-green-200',
    header: 'bg-gradient-to-r from-green-600 to-green-500',
    headerBadge: 'text-green-100',
    tableHeader: 'bg-green-50',
    headerText: 'text-green-700',
    rowHover: 'hover:bg-green-50/50 transition-colors',
    focusRing: 'focus:ring-green-500 focus:border-green-500',
    cellFilled: 'bg-green-50'
  }
} as const

// Direct property access from colorClasses for template usage
// Each computed directly accesses the static object with the dynamic color scheme
const borderColorClass = computed(() => colorClasses[props.colorScheme].border)
const headerColorClass = computed(() => colorClasses[props.colorScheme].header)
const headerBadgeColorClass = computed(() => colorClasses[props.colorScheme].headerBadge)
const tableHeaderColorClass = computed(() => colorClasses[props.colorScheme].tableHeader)
const headerTextColorClass = computed(() => colorClasses[props.colorScheme].headerText)
const rowHoverColorClass = computed(() => colorClasses[props.colorScheme].rowHover)
const focusRingClass = computed(() => colorClasses[props.colorScheme].focusRing)
const cellFilledClass = computed(() => colorClasses[props.colorScheme].cellFilled)
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
