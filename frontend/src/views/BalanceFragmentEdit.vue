<template>
  <AppLayout>
    <!-- Loading state -->
    <div v-if="isLoading" class="flex items-center justify-center min-h-[60vh]">
      <div class="text-center">
        <div class="animate-spin rounded-full h-16 w-16 border-b-2 border-blue-600 mx-auto mb-4"></div>
        <p class="text-gray-600">Cargando fragmento...</p>
      </div>
    </div>

    <!-- Contenido principal -->
    <div v-else-if="currentFragment" class="space-y-6">
      <!-- Header -->
      <header class="mb-6">
        <nav class="flex items-center text-sm text-blue-600 mb-2">
          <router-link to="/dashboard" class="hover:underline">Dashboard</router-link>
          <svg class="w-4 h-4 mx-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
          </svg>
          <span class="text-gray-700">{{ currentFragment.balanceName }}</span>
          <svg class="w-4 h-4 mx-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
          </svg>
          <span class="text-gray-700">{{ currentFragment.asignaturaName }}</span>
        </nav>
        <div class="flex items-start justify-between">
          <div>
            <h1 class="text-3xl font-bold text-blue-700">{{ currentFragment.asignaturaName }}</h1>
            <p class="text-gray-600 mt-1">
              Balance: {{ currentFragment.balanceName }} · {{ currentFragment.weeks }} semanas
            </p>
          </div>
          <div class="flex items-center gap-2">
            <span 
              class="px-2 py-1 text-xs font-medium rounded-full"
              :class="statusClass(currentFragment.status)"
            >
              {{ statusLabel(currentFragment.status) }}
            </span>
            <span v-if="currentFragment.deadline" class="text-sm text-amber-600">
              Límite: {{ formatDate(currentFragment.deadline) }}
            </span>
          </div>
        </div>
      </header>

      <!-- Alerta si hay deadline próximo -->
      <div 
        v-if="isDeadlineNear" 
        class="bg-amber-50 border border-amber-200 rounded-lg p-4 flex items-start gap-3"
      >
        <svg class="w-5 h-5 text-amber-500 flex-shrink-0 mt-0.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
        </svg>
        <div>
          <h3 class="text-sm font-medium text-amber-800">Fecha límite próxima</h3>
          <p class="text-sm text-amber-700 mt-1">
            Este fragmento debe completarse antes del {{ formatDate(currentFragment.deadline) }}.
          </p>
        </div>
      </div>

      <!-- Información si está completado -->
      <div 
        v-if="currentFragment.status === 'completed'" 
        class="bg-green-50 border border-green-200 rounded-lg p-4 flex items-start gap-3"
      >
        <svg class="w-5 h-5 text-green-500 flex-shrink-0 mt-0.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        <div>
          <h3 class="text-sm font-medium text-green-800">Fragmento completado</h3>
          <p class="text-sm text-green-700 mt-1">
            Este fragmento ha sido marcado como completado. Puedes seguir haciendo modificaciones si es necesario.
          </p>
        </div>
      </div>

      <!-- Alerta si excede el límite de horas -->
      <div 
        v-if="isOverHoursLimit" 
        class="bg-red-50 border border-red-200 rounded-lg p-4 flex items-start gap-3"
      >
        <svg class="w-5 h-5 text-red-500 flex-shrink-0 mt-0.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
        </svg>
        <div>
          <h3 class="text-sm font-medium text-red-800">⚠️ Exceso de horas</h3>
          <p class="text-sm text-red-700 mt-1">
            Has asignado <strong>{{ totalHorasCalculadas }}</strong> horas pero la asignatura tiene planificadas 
            <strong>{{ asignaturaHours }}</strong> horas ({{ hoursPercentage }}% del límite).
          </p>
        </div>
      </div>

      <!-- Indicador de progreso de horas y checklist -->
      <div 
        v-if="asignaturaHours > 0 || activityChecklist.length > 0" 
        class="bg-white border rounded-lg p-4"
      >
        <div class="flex items-center justify-between mb-3">
          <span class="text-sm font-medium text-gray-700">Progreso de la Asignatura</span>
          <span 
            class="text-sm font-semibold"
            :class="isOverHoursLimit ? 'text-red-600' : 'text-blue-600'"
          >
            {{ totalHorasCalculadas }} / {{ asignaturaHours }} horas
          </span>
        </div>
        
        <!-- Checklist de tipos de actividad -->
        <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-5 gap-2">
          <div 
            v-for="item in activityChecklist" 
            :key="item.type"
            class="flex items-center justify-between px-2 py-1.5 rounded border text-xs"
            :class="getChecklistItemClass(item)"
          >
            <span class="font-semibold">{{ item.type }}</span>
            <span class="flex items-center gap-1">
              <span :class="item.current > item.planned ? 'text-red-600 font-bold' : ''">
                {{ item.current }}
              </span>
              <span class="text-gray-400">/</span>
              <span class="text-gray-500">{{ item.planned }}</span>
              <span v-if="item.current >= item.planned && item.planned > 0" class="text-green-600">✓</span>
              <span v-else-if="item.current > item.planned" class="text-red-600">!</span>
            </span>
          </div>
        </div>
        
        <p v-if="!isOverHoursLimit" class="text-xs text-gray-500 mt-2">
          Te quedan <strong>{{ asignaturaHours - totalHorasCalculadas }}</strong> horas disponibles
        </p>
      </div>

      <!-- Tablas de balance -->
      <AppCard title="Distribución de Carga" padding="none">
        <div class="p-4 space-y-6">
          <!-- Semanas lectivas dinámicas (grupos de 4) -->
          <div
            v-for="(group, idx) in weekGroups"
            :key="`weeks-${idx}`"
            class="border-2 rounded-lg bg-white overflow-hidden shadow-sm"
            :class="idx < weekGroups.length - 1 ? 'border-blue-200' : 'border-purple-200'"
          >
            <!-- Header -->
            <div 
              class="px-4 py-2.5 flex items-center justify-between"
              :class="idx < weekGroups.length - 1 ? 'bg-blue-600' : 'bg-purple-600'"
            >
              <span class="text-sm font-semibold text-white">📅 Semanas {{ group.start }} - {{ group.end }}</span>
              <span class="text-xs text-white/80">{{ group.weeks.length * 4 }} celdas</span>
            </div>

            <!-- Table -->
            <div class="overflow-x-auto">
              <table class="w-full divide-y divide-gray-200">
                <thead :class="idx < weekGroups.length - 1 ? 'bg-blue-50' : 'bg-purple-50'">
                  <tr>
                    <th 
                      v-for="week in group.weeks"
                      :key="`week-${week}`"
                      colspan="4"
                      class="px-2 py-3 text-center font-semibold border-l border-gray-300"
                      :class="idx < weekGroups.length - 1 ? 'text-blue-700' : 'text-purple-700'"
                      :title="getWeekDateRange(week)"
                    >
                      <div class="flex flex-col items-center gap-0.5">
                        <span class="text-sm">S{{ week }}</span>
                        <span v-if="weekDates.length > 0" class="text-xs font-medium opacity-80">
                          {{ getWeekDateRange(week) }}
                        </span>
                      </div>
                    </th>
                  </tr>
                </thead>
                <tbody class="bg-white">
                  <tr>
                    <td
                      v-for="cellIndex in group.weeks.length * 4"
                      :key="`cell-${cellIndex}`"
                      class="px-0.5 py-1 text-center"
                      :class="(cellIndex - 1) % 4 === 0 ? 'border-l border-gray-300' : ''"
                    >
                      <select
                        :value="getCellValue(group.startIndex + cellIndex - 1)"
                        @change="(e) => updateCell(group.startIndex + cellIndex - 1, e)"
                        :class="[
                          'w-11 h-7 text-center border border-gray-200 rounded text-xs focus:ring-2 outline-none cursor-pointer appearance-none px-1',
                          idx < weekGroups.length - 1 ? 'focus:ring-blue-500 focus:border-blue-500' : 'focus:ring-purple-500 focus:border-purple-500',
                          getCellValue(group.startIndex + cellIndex - 1) 
                            ? (idx < weekGroups.length - 1 ? 'bg-blue-100 font-semibold' : 'bg-purple-100 font-semibold')
                            : 'bg-white'
                        ]"
                      >
                        <option v-for="tipo in tiposActividad" :key="tipo.value" :value="tipo.value">
                          {{ tipo.label }}
                        </option>
                      </select>
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>

          <!-- Consultas y Exámenes Finales -->
          <div class="border-2 border-red-200 rounded-lg bg-white overflow-hidden shadow-sm">
            <div class="bg-red-600 px-4 py-2.5 flex items-center justify-between">
              <span class="text-sm font-semibold text-white">📝 Consultas y Exámenes Finales</span>
              <span class="text-xs text-white/80">9 celdas</span>
            </div>

            <div class="overflow-x-auto">
              <table class="w-full divide-y divide-gray-200">
                <thead class="bg-red-50">
                  <tr>
                    <th colspan="4" class="px-2 py-3 text-center font-semibold text-red-700 border-r border-gray-300">
                      <div class="flex flex-col items-center gap-0.5">
                        <span class="text-sm">Consultas</span>
                        <span v-if="finalWeekDates.length > 0" class="text-xs font-medium opacity-80">
                          {{ finalWeekDates[0]?.displayRange || '' }}
                        </span>
                      </div>
                    </th>
                    <th colspan="5" class="px-2 py-3 text-center font-semibold text-red-700">
                      <div class="flex flex-col items-center gap-0.5">
                        <span class="text-sm">Exámenes Finales</span>
                        <span v-if="finalWeekDates.length > 1" class="text-xs font-medium opacity-80">
                          {{ finalWeekDates[1]?.displayRange || '' }}
                        </span>
                      </div>
                    </th>
                  </tr>
                </thead>
                <tbody class="bg-white">
                  <tr>
                    <td
                      v-for="i in 9"
                      :key="`final-${i}`"
                      class="px-0.5 py-1 text-center"
                      :class="i === 5 ? 'border-l border-gray-300' : ''"
                    >
                      <select
                        :value="getCellValue(finalStartIndex + i - 1)"
                        @change="(e) => updateCell(finalStartIndex + i - 1, e)"
                        :class="[
                          'w-11 h-7 text-center border border-gray-200 rounded text-xs focus:ring-2 focus:ring-red-500 focus:border-red-500 outline-none cursor-pointer appearance-none px-1',
                          getCellValue(finalStartIndex + i - 1) ? 'bg-red-100 font-semibold' : 'bg-white'
                        ]"
                      >
                        <option v-for="tipo in tiposActividad" :key="tipo.value" :value="tipo.value">
                          {{ tipo.label }}
                        </option>
                      </select>
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </AppCard>

      <!-- Cálculos -->
      <AppCard title="Resumen de Horas">
        <div class="grid grid-cols-2 sm:grid-cols-4 md:grid-cols-8 gap-3">
          <div
            v-for="tipo in tiposResumen"
            :key="tipo.key"
            class="bg-gray-50 rounded-lg p-3 text-center"
          >
            <div class="text-lg font-bold text-gray-700">{{ calculations[tipo.key] }}</div>
            <div class="text-xs text-gray-500">{{ tipo.label }}</div>
          </div>
        </div>
        
        <div class="mt-4 pt-4 border-t flex items-center justify-between">
          <div>
            <span class="text-sm text-gray-600">Total: </span>
            <span class="text-lg font-bold text-blue-600">{{ calculations.total }}</span>
            <span class="text-sm text-gray-500 ml-2">→ Coeficiente: </span>
            <span class="text-lg font-bold text-green-600">{{ calculations.coef }}</span>
          </div>
        </div>
      </AppCard>

      <!-- Acciones -->
      <div class="flex items-center justify-between bg-white rounded-lg shadow p-4">
        <div class="flex items-center gap-2">
          <AppButton variant="ghost" @click="goBack">
            ← Volver
          </AppButton>
          <span v-if="balanceStore.hasUnsavedChanges" class="text-sm text-amber-600">
            Cambios sin guardar
          </span>
          <span v-if="isOverHoursLimit" class="text-sm text-red-600 font-medium">
            ⚠️ Excedes el límite de horas
          </span>
        </div>
        <div class="flex items-center gap-2">
          <AppButton 
            variant="secondary" 
            @click="saveProgress" 
            :disabled="isSaving || isOverHoursLimit"
            :title="isOverHoursLimit ? 'No puedes guardar si excedes el límite de horas' : ''"
          >
            {{ isSaving ? 'Guardando...' : 'Guardar Progreso' }}
          </AppButton>
          <AppButton 
            v-if="currentFragment.status !== 'completed'"
            variant="primary" 
            @click="completeFragment" 
            :disabled="isSaving || isOverHoursLimit"
            :title="isOverHoursLimit ? 'No puedes completar si excedes el límite de horas' : ''"
          >
            {{ isSaving ? 'Guardando...' : 'Marcar como Completado' }}
          </AppButton>
        </div>
      </div>
    </div>

    <!-- Estado vacío -->
    <div v-else class="flex items-center justify-center min-h-[60vh]">
      <div class="text-center">
        <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
        </svg>
        <h3 class="mt-2 text-sm font-medium text-gray-900">Fragmento no encontrado</h3>
        <p class="mt-1 text-sm text-gray-500">El fragmento solicitado no existe o no tienes acceso</p>
        <router-link to="/dashboard" class="mt-3 inline-flex items-center text-blue-600 hover:underline">
          ← Volver al Dashboard
        </router-link>
      </div>
    </div>
  </AppLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, reactive } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useBalanceStore } from '../stores/balance'
import { useUIStore } from '../stores/ui'
import AppLayout from '../components/AppLayout.vue'
import AppCard from '../components/AppCard.vue'
import AppButton from '../components/AppButton.vue'
import { tiposActividadBalance, HORAS_POR_TIPO } from '../utils/constants'
import { getWeekDates, getFinalWeeksDates } from '../utils/balance-table'

const router = useRouter()
const route = useRoute()
const balanceStore = useBalanceStore()
const uiStore = useUIStore()

// State
const isLoading = ref(true)
const isSaving = ref(false)

// Computed
const currentFragment = computed(() => balanceStore.currentFragment)

// Tipos de actividad para el select
const tiposActividad = tiposActividadBalance

// Tipos para el resumen
const tiposResumen: { key: keyof typeof calculations; label: string }[] = [
  { key: 'C', label: 'C' },
  { key: 'CP', label: 'CP' },
  { key: 'S', label: 'S' },
  { key: 'PL', label: 'PL' },
  { key: 'TE', label: 'TE' },
  { key: 'T', label: 'T' },
  { key: 'PP', label: 'PP' },
  { key: 'EC', label: 'EC' },
]

// Cálculos reactivos
const calculations = reactive({
  C: 0,
  CP: 0,
  S: 0,
  PL: 0,
  TE: 0,
  T: 0,
  PP: 0,
  EC: 0,
  total: 0,
  coef: 0,
})

// Grupos de semanas
const weekGroups = computed(() => {
  const weeks = currentFragment.value?.weeks || 15
  const groups = []
  
  for (let i = 0; i < weeks; i += 4) {
    const start = i + 1
    const end = Math.min(i + 4, weeks)
    const weekNums = []
    for (let w = start; w <= end; w++) {
      weekNums.push(w)
    }
    groups.push({
      start,
      end,
      weeks: weekNums,
      startIndex: i * 4,
    })
  }
  
  return groups
})

// Fechas calculadas para cada semana
const weekDates = computed(() => {
  if (!currentFragment.value) return []
  return getWeekDates(
    currentFragment.value.startDate,
    currentFragment.value.weeks,
    currentFragment.value.nonAcademicPeriods
  )
})

// Fechas de las semanas finales (consultas y exámenes)
const finalWeekDates = computed(() => {
  if (!currentFragment.value) return []
  return getFinalWeeksDates(
    currentFragment.value.startDate,
    currentFragment.value.weeks,
    currentFragment.value.nonAcademicPeriods
  )
})

// Obtener el rango de fechas de una semana específica
function getWeekDateRange(weekNumber: number): string {
  const week = weekDates.value.find(w => w.weekNumber === weekNumber)
  return week ? week.displayRange : ''
}

// Índice donde empiezan las consultas y exámenes
const finalStartIndex = computed(() => {
  const weeks = currentFragment.value?.weeks || 15
  return weeks * 4
})

// Deadline próximo (menos de 3 días)
const isDeadlineNear = computed(() => {
  if (!currentFragment.value?.deadline) return false
  const deadline = new Date(currentFragment.value.deadline)
  const now = new Date()
  const diffDays = Math.ceil((deadline.getTime() - now.getTime()) / (1000 * 60 * 60 * 24))
  return diffDays <= 3 && diffDays >= 0
})

// ============================================================================
// Cálculo de horas y validaciones
// ============================================================================

// Obtener horas planificadas de la asignatura del balance
const asignaturaHours = computed(() => {
  // El fragmento debería tener la info de la asignatura cargada desde el balance
  // Si no, usamos 0 como fallback
  return currentFragment.value?.asignaturaHours || 0
})

// Calcular total de horas según tipo de actividad
const totalHorasCalculadas = computed(() => {
  const values = (currentFragment.value?.data?.values as string[]) || []
  let total = 0
  
  for (const val of values) {
    if (val && typeof val === 'string') {
      const horas = HORAS_POR_TIPO[val] ?? 2 // Por defecto 2 horas si no está definido
      total += horas
    }
  }
  
  return total
})

// Verificar si se excede el límite de horas
const isOverHoursLimit = computed(() => {
  if (asignaturaHours.value <= 0) return false
  return totalHorasCalculadas.value > asignaturaHours.value
})

// Porcentaje de horas usadas
const hoursPercentage = computed(() => {
  if (asignaturaHours.value <= 0) return 0
  return Math.round((totalHorasCalculadas.value / asignaturaHours.value) * 100)
})

// ============================================================================
// Checklist de actividades planificadas vs actuales
// ============================================================================

interface ChecklistItem {
  type: string
  label: string
  planned: number
  current: number
}

// Generar checklist comparando plan de asignatura vs distribución actual
const activityChecklist = computed<ChecklistItem[]>(() => {
  const plan = currentFragment.value?.asignaturaPlan
  if (!plan) return []
  
  // Contar actividades actuales
  const values = (currentFragment.value?.data?.values as string[]) || []
  const currentCounts: Record<string, number> = {}
  
  for (const val of values) {
    if (val && typeof val === 'string') {
      currentCounts[val] = (currentCounts[val] || 0) + 1
    }
  }
  
  // Tipos de actividad con sus labels
  const activityTypes = [
    { key: 'c', type: 'C', label: 'Conferencias' },
    { key: 'cp', type: 'CP', label: 'Clases Prácticas' },
    { key: 's', type: 'S', label: 'Seminarios' },
    { key: 'pl', type: 'PL', label: 'Prácticas Lab.' },
    { key: 't', type: 'T', label: 'Tutorías' },
    { key: 'te', type: 'TE', label: 'Trabajo Extra.' },
    { key: 'pp', type: 'PP', label: 'Pruebas Parciales' },
  ]
  
  // Filtrar solo los que tienen valor planificado > 0
  const checklist: ChecklistItem[] = []
  
  for (const act of activityTypes) {
    const planned = plan[act.key as keyof typeof plan] ?? 0
    const current = currentCounts[act.type] || 0
    
    // Mostrar si tiene planificado o tiene actual
    if (planned > 0 || current > 0) {
      checklist.push({
        type: act.type,
        label: act.label,
        planned,
        current,
      })
    }
  }
  
  return checklist
})

// Clase CSS para cada item del checklist
function getChecklistItemClass(item: ChecklistItem): string {
  if (item.current > item.planned) {
    // Excede lo planificado
    return 'bg-red-50 border-red-200 text-red-700'
  }
  if (item.current >= item.planned && item.planned > 0) {
    // Completado
    return 'bg-green-50 border-green-200 text-green-700'
  }
  if (item.current > 0) {
    // En progreso
    return 'bg-blue-50 border-blue-200 text-blue-700'
  }
  // Pendiente
  return 'bg-gray-50 border-gray-200 text-gray-600'
}

// Métodos
function getCellValue(index: number): string {
  if (!currentFragment.value?.data?.values) return ''
  const values = currentFragment.value.data.values as string[]
  return values[index] || ''
}

function updateCell(index: number, event: Event) {
  const target = event.target as HTMLSelectElement
  const value = target.value
  balanceStore.updateFragmentCell(index, value)
  recalculate()
}

function recalculate() {
  const values = (currentFragment.value?.data?.values as string[]) || []
  
  // Reset
  calculations.C = 0
  calculations.CP = 0
  calculations.S = 0
  calculations.PL = 0
  calculations.TE = 0
  calculations.T = 0
  calculations.PP = 0
  calculations.EC = 0
  calculations.total = 0
  
  // Contar
  const validKeys = ['C', 'CP', 'S', 'PL', 'TE', 'T', 'PP', 'EC'] as const
  for (const val of values) {
    if (val && typeof val === 'string' && validKeys.includes(val as typeof validKeys[number])) {
      calculations[val as typeof validKeys[number]]++
      calculations.total++
    }
  }
  
  calculations.coef = Math.round(calculations.total * 1.2 * 100) / 100
}

function formatDate(date: string | null): string {
  if (!date) return ''
  return new Date(date).toLocaleDateString('es-ES', {
    year: 'numeric',
    month: 'short',
    day: 'numeric'
  })
}

function statusClass(status: string): string {
  switch (status) {
    case 'pending': return 'bg-yellow-100 text-yellow-700'
    case 'in_progress': return 'bg-blue-100 text-blue-700'
    case 'completed': return 'bg-green-100 text-green-700'
    default: return 'bg-gray-100 text-gray-700'
  }
}

function statusLabel(status: string): string {
  switch (status) {
    case 'pending': return 'Pendiente'
    case 'in_progress': return 'En Progreso'
    case 'completed': return 'Completado'
    default: return status
  }
}

function goBack() {
  router.push('/dashboard')
}

async function saveProgress() {
  isSaving.value = true
  
  const result = await balanceStore.saveFragment(false)
  
  isSaving.value = false
  
  if (result.success) {
    uiStore.showSuccess(result.message)
  } else {
    uiStore.showError(result.message)
  }
}

async function completeFragment() {
  uiStore.openConfirm({
    title: 'Completar Fragmento',
    message: '¿Estás seguro de marcar este fragmento como completado? Podrás seguir editándolo si es necesario.',
    confirmText: 'Sí, completar',
    cancelText: 'Cancelar',
    onConfirm: async () => {
      isSaving.value = true
      
      const result = await balanceStore.saveFragment(true)
      
      isSaving.value = false
      
      if (result.success) {
        uiStore.showSuccess(result.message)
      } else {
        uiStore.showError(result.message)
      }
    },
  })
}

// Inicialización
onMounted(async () => {
  isLoading.value = true
  
  try {
    const balanceId = route.query.balanceId as string
    const asignaturaId = route.query.asignaturaId as string
    
    if (!balanceId || !asignaturaId) {
      uiStore.showError('Parámetros inválidos')
      router.push('/dashboard')
      return
    }
    
    const loaded = await balanceStore.loadFragment(
      parseInt(balanceId, 10),
      parseInt(asignaturaId, 10)
    )
    
    if (!loaded) {
      uiStore.showError('Fragmento no encontrado')
    } else {
      // Inicializar valores si no existen
      if (!currentFragment.value?.data?.values) {
        const weeks = currentFragment.value?.weeks || 15
        const cellsCount = weeks * 4 + 9 // semanas * 4 días + consultas(4) + exámenes(5)
        balanceStore.updateFragmentData({ values: Array(cellsCount).fill('') })
      }
      recalculate()
    }
  } catch (error) {
    console.error('Error cargando fragmento:', error)
    uiStore.showError('Error al cargar fragmento')
  } finally {
    isLoading.value = false
  }
})
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
