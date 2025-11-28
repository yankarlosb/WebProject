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
import { getCellValue, viewColorClasses, type ColorScheme } from '../utils/balance-table'

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
}

const props = withDefaults(defineProps<Props>(), {
  columnsPerWeek: 4,
  headerIcon: '📅',
  colorScheme: 'blue',
})

const totalCells = computed(() => props.weeks.length * props.columnsPerWeek)
const cellsPerSubject = computed(() => totalCells.value)

const borderColorClass = computed(() => viewColorClasses[props.colorScheme].border)
const headerColorClass = computed(() => viewColorClasses[props.colorScheme].header)
const headerBadgeColorClass = computed(() => viewColorClasses[props.colorScheme].headerBadge)
const tableHeaderColorClass = computed(() => viewColorClasses[props.colorScheme].tableHeader)
const headerTextColorClass = computed(() => viewColorClasses[props.colorScheme].headerText)
const cellFilledClass = computed(() => viewColorClasses[props.colorScheme].cellFilled)
</script>
