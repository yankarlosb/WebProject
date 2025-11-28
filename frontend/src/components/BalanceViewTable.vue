<template>
  <div :class="[
    'border-2 rounded-lg bg-white overflow-hidden shadow-sm',
    borderColorClass
  ]">
    <!-- Header -->
    <div :class="[
      'px-4 py-2.5 flex items-center justify-between',
      headerColorClass
    ]">
      <span class="text-sm font-semibold text-white">{{ headerIcon }} {{ title }}</span>
      <span :class="['text-xs', headerBadgeColorClass]">
        {{ cellsPerSubject }} celdas por asignatura
      </span>
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
          </tr>
        </thead>
        <tbody class="bg-white divide-y divide-gray-100">
          <tr
            v-for="subject in subjects"
            :key="`subject-${subject.id}`"
          >
            <td class="px-3 py-2 font-semibold text-xs text-gray-900 bg-gray-50 truncate max-w-[140px]" 
                :title="subject.name">
              {{ subject.name }}
            </td>
            <td
              v-for="(cellIndex, idx) in totalCells"
              :key="`cell-${cellIndex}`"
              class="px-0.5 py-1 text-center"
              :class="idx % columnsPerWeek === 0 ? 'border-l border-gray-300' : ''"
            >
              <span
                :class="[
                  'inline-block w-11 h-7 leading-7 text-center border border-gray-200 rounded text-xs',
                  getCellValue(subject.values[startCellIndex + idx]) ? `${cellFilledClass} font-semibold` : 'bg-gray-50 text-gray-400'
                ]"
              >
                {{ getCellValue(subject.values[startCellIndex + idx]) || '-' }}
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
}

const props = withDefaults(defineProps<Props>(), {
  columnsPerWeek: 4,
  headerIcon: '📅',
  colorScheme: 'blue',
})

function getCellValue(value: number | string | undefined): string {
  if (value === undefined || value === null || value === 0 || value === '') {
    return ''
  }
  if (typeof value === 'number') {
    return ''
  }
  return value
}

const totalCells = computed(() => props.weeks.length * props.columnsPerWeek)
const cellsPerSubject = computed(() => totalCells.value)

const colorClasses = {
  blue: {
    border: 'border-blue-200',
    header: 'bg-gradient-to-r from-blue-600 to-blue-500',
    headerBadge: 'text-blue-100',
    tableHeader: 'bg-blue-50',
    headerText: 'text-blue-700',
    cellFilled: 'bg-blue-50 text-blue-800'
  },
  purple: {
    border: 'border-purple-200',
    header: 'bg-gradient-to-r from-purple-600 to-purple-500',
    headerBadge: 'text-purple-100',
    tableHeader: 'bg-purple-50',
    headerText: 'text-purple-700',
    cellFilled: 'bg-purple-50 text-purple-800'
  },
  green: {
    border: 'border-green-200',
    header: 'bg-gradient-to-r from-green-600 to-green-500',
    headerBadge: 'text-green-100',
    tableHeader: 'bg-green-50',
    headerText: 'text-green-700',
    cellFilled: 'bg-green-50 text-green-800'
  }
} as const

const borderColorClass = computed(() => colorClasses[props.colorScheme].border)
const headerColorClass = computed(() => colorClasses[props.colorScheme].header)
const headerBadgeColorClass = computed(() => colorClasses[props.colorScheme].headerBadge)
const tableHeaderColorClass = computed(() => colorClasses[props.colorScheme].tableHeader)
const headerTextColorClass = computed(() => colorClasses[props.colorScheme].headerText)
const cellFilledClass = computed(() => colorClasses[props.colorScheme].cellFilled)
</script>
