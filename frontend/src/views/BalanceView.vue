<!-- Vista de Balances: Lista con filtros y detalle (vista de fragmentos) -->
<template>
  <AppLayout>
    <!-- Vista Lista (sin ID) -->
    <template v-if="!selectedBalanceId">
      <div class="max-w-7xl mx-auto">
        <!-- Header -->
        <header class="mb-6">
          <nav class="flex items-center text-sm text-blue-600 mb-2">
            <router-link to="/dashboard" class="hover:underline">Dashboard</router-link>
            <svg class="w-4 h-4 mx-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
            </svg>
            <span class="text-gray-700">Ver Balances</span>
          </nav>
          <h1 class="text-3xl font-bold text-blue-700">Todos los Balances</h1>
          <p class="text-gray-600 mt-1">Selecciona un balance para ver sus detalles y progreso</p>
        </header>

        <!-- Filtros -->
        <AppCard class="mb-6">
          <h3 class="text-lg font-semibold text-gray-800 mb-4">Filtros</h3>
          <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Curso Académico</label>
              <select v-model="filters.academicYearText" class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500">
                <option value="">Todos</option>
                <option v-for="year in availableAcademicYears" :key="year" :value="year">{{ year }}</option>
              </select>
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Período</label>
              <select v-model="filters.period" class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500">
                <option value="">Todos</option>
                <option value="1ero">1er Semestre</option>
                <option value="2do">2do Semestre</option>
              </select>
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Estado</label>
              <select v-model="filters.status" class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500">
                <option value="">Todos</option>
                <option value="draft">Borrador</option>
                <option value="in_progress">En Progreso</option>
                <option value="completed">Completado</option>
              </select>
            </div>

            <div class="flex items-end">
              <AppButton variant="outline" size="sm" @click="clearFilters" class="w-full">
                Limpiar filtros
              </AppButton>
            </div>
          </div>
        </AppCard>

        <!-- Loading -->
        <div v-if="loading" class="flex items-center justify-center min-h-[40vh]">
          <div class="text-center">
            <div class="animate-spin rounded-full h-16 w-16 border-b-2 border-blue-600 mx-auto mb-4"></div>
            <p class="text-gray-600">Cargando balances...</p>
          </div>
        </div>

        <!-- Error -->
        <AppCard v-else-if="error" class="text-center py-8">
          <p class="text-red-600 mb-4">{{ error }}</p>
          <AppButton variant="primary" @click="loadBalances">Reintentar</AppButton>
        </AppCard>

        <!-- Lista de Balances -->
        <div v-else-if="filteredBalances.length > 0" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          <div
            v-for="bal in filteredBalances"
            :key="bal.id"
            class="bg-white rounded-lg border border-gray-200 p-5 cursor-pointer transition hover:shadow-lg hover:border-blue-400 hover:-translate-y-0.5"
            @click="selectBalance(bal.id)"
          >
            <div class="flex justify-between items-start mb-3">
              <h3 class="font-semibold text-gray-900 text-lg">{{ bal.name }}</h3>
              <span 
                class="text-xs font-medium px-2.5 py-0.5 rounded-full"
                :class="statusClass(bal.status)"
              >
                {{ statusLabel(bal.status) }}
              </span>
            </div>
            <div class="space-y-1 text-sm text-gray-600 mb-4">
              <p><span class="font-medium">Año Académico:</span> {{ bal.academic_year }}</p>
              <p><span class="font-medium">Curso:</span> {{ bal.academic_year_text }}</p>
              <p><span class="font-medium">Período:</span> {{ bal.period === '1ero' ? '1er Semestre' : '2do Semestre' }}</p>
            </div>
            
            <!-- Barra de progreso -->
            <div class="mb-3">
              <div class="flex items-center justify-between text-xs text-gray-500 mb-1">
                <span>Progreso</span>
                <span>{{ bal.progress.percentage.toFixed(0) }}%</span>
              </div>
              <div class="w-full bg-gray-200 rounded-full h-2">
                <div 
                  class="bg-green-600 h-2 rounded-full transition-all"
                  :style="{ width: `${bal.progress.percentage}%` }"
                ></div>
              </div>
            </div>
            
            <div class="border-t border-gray-100 pt-3 flex justify-between items-center">
              <span class="text-blue-600 font-medium text-sm flex items-center">
                Ver detalles
                <svg class="w-4 h-4 ml-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                </svg>
              </span>
              <button
                v-if="authStore.isLeader"
                @click.stop="confirmDeleteBalance(bal)"
                class="text-red-600 hover:text-red-800 hover:bg-red-50 p-1.5 rounded-lg transition-colors"
                title="Eliminar balance"
              >
                <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                </svg>
              </button>
            </div>
          </div>
        </div>

        <!-- Sin resultados -->
        <AppCard v-else class="text-center py-12">
          <svg class="mx-auto h-12 w-12 text-gray-400 mb-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
          </svg>
          <p v-if="balances.length === 0" class="text-gray-500">No hay balances registrados</p>
          <p v-else class="text-gray-500">No se encontraron balances con los filtros seleccionados</p>
        </AppCard>
      </div>
    </template>

    <!-- Vista Detalle (con ID) -->
    <template v-else>
      <div class="max-w-7xl mx-auto">
        <!-- Loading -->
        <div v-if="loadingDetail" class="flex items-center justify-center min-h-[60vh]">
          <div class="text-center">
            <div class="animate-spin rounded-full h-16 w-16 border-b-2 border-blue-600 mx-auto mb-4"></div>
            <p class="text-gray-600">Cargando balance...</p>
          </div>
        </div>

        <!-- Error -->
        <AppCard v-else-if="errorDetail" class="text-center py-8">
          <p class="text-red-600 mb-4">{{ errorDetail }}</p>
          <div class="flex gap-3 justify-center">
            <AppButton variant="outline" @click="goBackToList">← Volver a la lista</AppButton>
            <AppButton variant="primary" @click="loadBalanceDetail">Reintentar</AppButton>
          </div>
        </AppCard>

        <!-- Contenido del Balance -->
        <template v-else-if="balance">
          <!-- Header con breadcrumb -->
          <header class="mb-6">
            <nav class="flex items-center text-sm text-blue-600 mb-2">
              <router-link to="/dashboard" class="hover:underline">Dashboard</router-link>
              <svg class="w-4 h-4 mx-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
              </svg>
              <button @click="goBackToList" class="hover:underline">Ver Balances</button>
              <svg class="w-4 h-4 mx-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
              </svg>
              <span class="text-gray-700">{{ balance.name }}</span>
            </nav>
            <div class="flex items-start justify-between">
              <div>
                <h1 class="text-3xl font-bold text-blue-700">{{ balance.name }}</h1>
                <div class="flex flex-wrap gap-2 mt-3">
                  <span class="bg-blue-100 text-blue-800 text-sm font-medium px-3 py-1 rounded-full">
                    {{ balance.academic_year }}
                  </span>
                  <span class="bg-purple-100 text-purple-800 text-sm font-medium px-3 py-1 rounded-full">
                    {{ balance.academic_year_text }}
                  </span>
                  <span class="bg-green-100 text-green-800 text-sm font-medium px-3 py-1 rounded-full">
                    {{ balance.period === '1ero' ? '1er Semestre' : '2do Semestre' }}
                  </span>
                  <span class="bg-orange-100 text-orange-800 text-sm font-medium px-3 py-1 rounded-full">
                    {{ balance.weeks }} semanas
                  </span>
                </div>
              </div>
              <span 
                class="px-3 py-1 text-sm font-medium rounded-full"
                :class="statusClass(balance.status)"
              >
                {{ statusLabel(balance.status) }}
              </span>
            </div>
          </header>

          <!-- Tabs de navegación -->
          <div class="mb-6 border-b border-gray-200">
            <nav class="flex gap-4" aria-label="Tabs">
              <button
                @click="activeDetailTab = 'summary'"
                class="py-3 px-1 text-sm font-medium border-b-2 transition-colors"
                :class="activeDetailTab === 'summary' 
                  ? 'border-blue-600 text-blue-600' 
                  : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'"
              >
                📊 Resumen
              </button>
              <button
                @click="activeDetailTab = 'fullview'"
                class="py-3 px-1 text-sm font-medium border-b-2 transition-colors"
                :class="activeDetailTab === 'fullview' 
                  ? 'border-blue-600 text-blue-600' 
                  : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'"
              >
                📅 Vista Completa del Balance
              </button>
            </nav>
          </div>

          <!-- Tab: Resumen -->
          <div v-show="activeDetailTab === 'summary'">
            <!-- Progreso general -->
            <AppCard title="Progreso del Balance" class="mb-6">
              <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-4">
                <div class="text-center p-3 bg-gray-50 rounded-lg">
                  <div class="text-2xl font-bold text-gray-700">{{ balance.progress.total }}</div>
                  <div class="text-xs text-gray-500">Total Fragmentos</div>
                </div>
              <div class="text-center p-3 bg-yellow-50 rounded-lg">
                <div class="text-2xl font-bold text-yellow-600">{{ balance.progress.pending }}</div>
                <div class="text-xs text-gray-500">Pendientes</div>
              </div>
              <div class="text-center p-3 bg-blue-50 rounded-lg">
                <div class="text-2xl font-bold text-blue-600">{{ balance.progress.in_progress }}</div>
                <div class="text-xs text-gray-500">En Progreso</div>
              </div>
              <div class="text-center p-3 bg-green-50 rounded-lg">
                <div class="text-2xl font-bold text-green-600">{{ balance.progress.completed }}</div>
                <div class="text-xs text-gray-500">Completados</div>
              </div>
            </div>

            <div class="w-full bg-gray-200 rounded-full h-3">
              <div 
                class="bg-green-600 h-3 rounded-full transition-all duration-300"
                :style="{ width: `${balance.progress.percentage}%` }"
              ></div>
            </div>
            <p class="text-sm text-gray-500 text-center mt-2">
              {{ balance.progress.percentage.toFixed(0) }}% completado
            </p>
          </AppCard>

          <!-- Alertas de conflictos de exámenes parciales -->
          <AppCard 
            v-if="examConflicts.length > 0" 
            title="⚠️ Conflictos de Exámenes Parciales" 
            class="mb-6 border-2 border-red-200"
          >
            <div class="bg-red-50 rounded-lg p-4 mb-4">
              <p class="text-sm text-red-700">
                <strong>Importante:</strong> No deben haber más de 2 pruebas parciales en la misma semana.
                Se detectaron las siguientes semanas con conflictos:
              </p>
            </div>
            <div class="space-y-2">
              <div 
                v-for="conflict in examConflicts" 
                :key="conflict.week"
                class="flex items-center gap-3 p-3 bg-red-100 rounded-lg border border-red-200"
              >
                <div class="flex-shrink-0 w-10 h-10 bg-red-600 text-white rounded-full flex items-center justify-center font-bold">
                  S{{ conflict.week }}
                </div>
                <div class="flex-1">
                  <p class="text-sm font-medium text-red-800">
                    {{ conflict.totalCount }} pruebas parciales en la semana {{ conflict.week }} (máximo 2)
                  </p>
                  <p class="text-xs text-red-600">
                    <span v-for="(exam, idx) in conflict.exams" :key="exam.asignatura">
                      {{ exam.asignatura }} ({{ exam.count }} PP){{ idx < conflict.exams.length - 1 ? ', ' : '' }}
                    </span>
                  </p>
                </div>
              </div>
            </div>
          </AppCard>

          <!-- Lista de fragmentos -->
          <AppCard title="Fragmentos por Asignatura">
            <div v-if="balance.fragments && balance.fragments.length > 0" class="space-y-3">
              <div
                v-for="fragment in balance.fragments"
                :key="fragment.id"
                class="border rounded-lg p-4"
                :class="fragmentBorderClass(fragment.status)"
              >
                <div class="flex items-start justify-between">
                  <div class="flex-1 min-w-0">
                    <h4 class="font-medium text-gray-900">
                      {{ fragment.asignatura?.name || 'Asignatura' }}
                    </h4>
                    <p v-if="fragment.subject_leader" class="text-sm text-gray-600 mt-0.5">
                      Responsable: {{ fragment.subject_leader.name }}
                    </p>
                    <p v-else class="text-sm text-amber-600 mt-0.5">
                      ⚠️ Sin Jefe de Asignatura asignado
                    </p>
                    
                    <!-- Vista previa de progreso del fragmento -->
                    <div v-if="hasFragmentData(fragment)" class="mt-2">
                      <!-- Resumen de tipos -->
                      <div class="flex flex-wrap gap-1.5 mb-2">
                        <span 
                          v-for="(count, type) in getFragmentSummary(fragment)" 
                          :key="type"
                          class="inline-flex items-center px-2 py-0.5 text-xs font-medium rounded bg-gray-100 text-gray-700"
                        >
                          {{ type }}: {{ count }}
                        </span>
                      </div>
                      
                      <!-- Distribución semanal visual -->
                      <div class="mt-2 mb-2">
                        <p class="text-xs text-gray-500 mb-1">Distribución por semanas:</p>
                        <div class="flex flex-wrap gap-0.5">
                          <div 
                            v-for="week in getWeeklyDistribution(fragment)" 
                            :key="week.num"
                            class="text-center"
                            :title="`S${week.num}: ${week.activities.join(', ') || 'vacía'}`"
                          >
                            <div 
                              class="w-8 h-6 text-xs flex items-center justify-center rounded border"
                              :class="getWeekClass(week)"
                            >
                              {{ week.num }}
                            </div>
                          </div>
                        </div>
                        <div class="flex gap-3 mt-1 text-xs text-gray-400">
                          <span class="flex items-center gap-1">
                            <span class="w-3 h-3 rounded bg-blue-100 border border-blue-300"></span> Clases
                          </span>
                          <span class="flex items-center gap-1">
                            <span class="w-3 h-3 rounded bg-red-100 border border-red-300"></span> PP
                          </span>
                          <span class="flex items-center gap-1">
                            <span class="w-3 h-3 rounded bg-gray-100 border border-gray-200"></span> Vacía
                          </span>
                        </div>
                      </div>
                      
                      <p class="text-xs text-gray-500 mt-1">
                        Total: {{ getFragmentTotalHours(fragment) }} horas de {{ fragment.asignatura?.hours || 0 }} planificadas
                        <span v-if="isFragmentOverHours(fragment)" class="text-red-600 font-medium ml-1">⚠️ Excede límite</span>
                      </p>
                    </div>
                    <p v-else-if="fragment.status !== 'completed'" class="text-xs text-gray-400 mt-2 italic">
                      Sin datos ingresados aún
                    </p>
                  </div>
                  <div class="ml-4 flex items-center gap-2">
                    <span 
                      class="px-2 py-1 text-xs font-medium rounded-full"
                      :class="fragmentStatusClass(fragment.status)"
                    >
                      {{ fragmentStatusLabel(fragment.status) }}
                    </span>
                    <!-- Botón para ver/editar fragmento -->
                    <!-- Leader con permisos O SubjectLeader dueño del fragmento -->
                    <AppButton
                      v-if="canEditFragment(fragment)"
                      variant="ghost"
                      size="sm"
                      @click="viewFragment(fragment)"
                      :title="getFragmentButtonTitle(fragment)"
                    >
                      <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                      </svg>
                    </AppButton>
                  </div>
                </div>
                <p v-if="fragment.completed_at" class="text-xs text-gray-400 mt-2">
                  Completado: {{ formatDate(fragment.completed_at) }}
                </p>
              </div>
            </div>

            <div v-else class="text-center py-8">
              <p class="text-gray-500">No hay fragmentos en este balance</p>
            </div>

            <!-- Acciones -->
            <div class="mt-6 pt-4 border-t flex items-center justify-between">
              <AppButton variant="ghost" @click="goBackToList">
                ← Volver a la lista
              </AppButton>
              <div class="flex items-center gap-2">
                <AppButton v-if="authStore.isLeader" variant="secondary" size="sm" @click="editBalance">
                  Editar Balance
                </AppButton>
              </div>
            </div>
          </AppCard>
          </div>

          <!-- Tab: Vista Completa -->
          <div v-show="activeDetailTab === 'fullview'">
            <AppCard title="Vista Completa del Balance" class="mb-6">
              <template #header>
                <div class="flex items-center justify-between">
                  <h3 class="text-lg font-semibold text-gray-900">Vista Completa del Balance</h3>
                  <AppButton variant="ghost" @click="goBackToList">
                    ← Volver a la lista
                  </AppButton>
                </div>
              </template>
              
              <BalanceFullView 
                v-if="balance.fragments && balance.fragments.length > 0"
                :fragments="balance.fragments" 
                :weeks="balance.weeks"
                :week-dates="weekDates"
                :final-week-dates="finalWeekDates"
              />
              
              <div v-else class="text-center py-8">
                <p class="text-gray-500">No hay fragmentos en este balance</p>
              </div>
            </AppCard>
          </div>
        </template>
      </div>
    </template>
  </AppLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import AppLayout from '@/components/AppLayout.vue'
import AppCard from '@/components/AppCard.vue'
import AppButton from '@/components/AppButton.vue'
import BalanceFullView from '@/components/BalanceFullView.vue'
import { balancesService, type Balance, type BalanceListItem, type Fragment } from '@/services/balances'
import { useAuthStore } from '@/stores/auth'
import { useUIStore } from '@/stores/ui'
import { HORAS_POR_TIPO } from '@/utils/constants'
import { getWeekDates, getFinalWeeksDates } from '@/utils/balance-table'

const route = useRoute()
const router = useRouter()
const authStore = useAuthStore()
const uiStore = useUIStore()

// Estado general
const loading = ref(false)
const error = ref('')
const balances = ref<BalanceListItem[]>([])

// Estado para la vista de detalle
const selectedBalanceId = ref<number | null>(null)
const loadingDetail = ref(false)
const errorDetail = ref('')
const balance = ref<Balance | null>(null)
const activeDetailTab = ref<'summary' | 'fullview'>('summary')

// Filtros
const filters = ref({
  academicYearText: '',
  period: '',
  status: '' as '' | 'draft' | 'in_progress' | 'completed'
})

// Cursos académicos disponibles para filtrar
const availableAcademicYears = computed(() => {
  const years = new Set(balances.value.map((b) => b.academic_year_text))
  return Array.from(years).sort().reverse() as string[]
})

// Balances filtrados
const filteredBalances = computed(() => {
  return balances.value.filter((b) => {
    if (filters.value.academicYearText && b.academic_year_text !== filters.value.academicYearText) return false
    if (filters.value.period && b.period !== filters.value.period) return false
    if (filters.value.status && b.status !== filters.value.status) return false
    return true
  })
})

// Fechas de semanas para el balance actual
const weekDates = computed(() => {
  if (!balance.value) return []
  return getWeekDates(
    balance.value.start_date,
    balance.value.weeks,
    balance.value.non_academic_periods || []
  )
})

// Fechas de semanas finales (consultas y exámenes)
const finalWeekDates = computed(() => {
  if (!balance.value) return []
  return getFinalWeeksDates(
    balance.value.start_date,
    balance.value.weeks,
    balance.value.non_academic_periods || []
  )
})

// Helpers para estados
function statusClass(status: string): string {
  switch (status) {
    case 'draft': return 'bg-gray-100 text-gray-700'
    case 'in_progress': return 'bg-blue-100 text-blue-700'
    case 'completed': return 'bg-green-100 text-green-700'
    default: return 'bg-gray-100 text-gray-700'
  }
}

function statusLabel(status: string): string {
  switch (status) {
    case 'draft': return 'Borrador'
    case 'in_progress': return 'En Progreso'
    case 'completed': return 'Completado'
    default: return status
  }
}

function fragmentStatusClass(status: string): string {
  switch (status) {
    case 'pending': return 'bg-yellow-100 text-yellow-700'
    case 'in_progress': return 'bg-blue-100 text-blue-700'
    case 'completed': return 'bg-green-100 text-green-700'
    default: return 'bg-gray-100 text-gray-700'
  }
}

function fragmentStatusLabel(status: string): string {
  switch (status) {
    case 'pending': return 'Pendiente'
    case 'in_progress': return 'En Progreso'
    case 'completed': return 'Completado'
    default: return status
  }
}

function fragmentBorderClass(status: string): string {
  switch (status) {
    case 'pending': return 'border-yellow-200'
    case 'in_progress': return 'border-blue-200'
    case 'completed': return 'border-green-200'
    default: return 'border-gray-200'
  }
}

function formatDate(date: string | null): string {
  if (!date) return ''
  return new Date(date).toLocaleDateString('es-ES', {
    year: 'numeric',
    month: 'short',
    day: 'numeric'
  })
}

// ============================================================================
// Funciones para vista previa de fragmentos y cálculo de horas
// ============================================================================

// Verificar si un fragmento tiene datos ingresados
function hasFragmentData(fragment: Fragment): boolean {
  const values = fragment.data?.values as string[] | undefined
  if (!values || !Array.isArray(values)) return false
  return values.some(v => v && v !== '')
}

// Obtener resumen de tipos de actividad en el fragmento
function getFragmentSummary(fragment: Fragment): Record<string, number> {
  const values = fragment.data?.values as string[] | undefined
  if (!values || !Array.isArray(values)) return {}
  
  const summary: Record<string, number> = {}
  values.forEach(v => {
    if (v && v !== '') {
      summary[v] = (summary[v] || 0) + 1
    }
  })
  return summary
}

// Distribución por semanas para visualización
interface WeekDistribution {
  num: number
  activities: string[]
  hasExam: boolean
  isEmpty: boolean
}

// Obtener distribución semanal del fragmento
function getWeeklyDistribution(fragment: Fragment): WeekDistribution[] {
  const weeks = balance.value?.weeks || 15
  const values = fragment.data?.values as string[] | undefined
  const distribution: WeekDistribution[] = []
  
  for (let weekNum = 1; weekNum <= weeks; weekNum++) {
    const startIdx = (weekNum - 1) * 4
    const endIdx = startIdx + 4
    const activities: string[] = []
    let hasExam = false
    
    if (values && Array.isArray(values)) {
      for (let i = startIdx; i < endIdx && i < values.length; i++) {
        const val = values[i]
        if (val && val !== '') {
          activities.push(val)
          if (val === 'PP') {
            hasExam = true
          }
        }
      }
    }
    
    distribution.push({
      num: weekNum,
      activities,
      hasExam,
      isEmpty: activities.length === 0
    })
  }
  
  return distribution
}

// Clase CSS para una semana según su contenido
function getWeekClass(week: WeekDistribution): string {
  if (week.hasExam) {
    return 'bg-red-100 border-red-300 text-red-700 font-medium'
  }
  if (!week.isEmpty) {
    return 'bg-blue-100 border-blue-300 text-blue-700'
  }
  return 'bg-gray-50 border-gray-200 text-gray-400'
}

// Calcular total de horas del fragmento
function getFragmentTotalHours(fragment: Fragment): number {
  const summary = getFragmentSummary(fragment)
  let total = 0
  
  for (const [tipo, count] of Object.entries(summary)) {
    const horasPorTipo = HORAS_POR_TIPO[tipo] ?? 2 // Por defecto 2 horas si no está definido
    total += horasPorTipo * count
  }
  
  return total
}

// Verificar si el fragmento excede las horas planificadas de la asignatura
function isFragmentOverHours(fragment: Fragment): boolean {
  const totalHoras = getFragmentTotalHours(fragment)
  const horasPlanificadas = fragment.asignatura?.hours || 0
  return horasPlanificadas > 0 && totalHoras > horasPlanificadas
}

// ============================================================================
// Detección de conflictos de exámenes parciales y finales
// ============================================================================

interface ExamConflict {
  week: number
  exams: { asignatura: string; type: string; count: number }[]
  totalCount: number
}

// Tipos que cuentan como exámenes (PP = Prueba Parcial, no incluimos EF porque está en sección aparte)
const EXAM_TYPES = ['PP']

// Detectar semanas con más de 2 exámenes (PP) en total
const examConflicts = computed<ExamConflict[]>(() => {
  if (!balance.value?.fragments) return []
  
  const weeks = balance.value.weeks || 15
  // Mapa: semana -> lista de exámenes {asignatura, tipo, cantidad}
  const weekExams: Record<number, { asignatura: string; type: string }[]> = {}
  
  for (const fragment of balance.value.fragments) {
    const values = fragment.data?.values as string[] | undefined
    if (!values || !Array.isArray(values)) continue
    
    const asignaturaName = fragment.asignatura?.name || 'Asignatura'
    
    // Revisar solo semanas lectivas (4 celdas por semana)
    for (let weekNum = 1; weekNum <= weeks; weekNum++) {
      const startIdx = (weekNum - 1) * 4
      const endIdx = startIdx + 4
      
      // Contar TODOS los exámenes en esta semana para esta asignatura
      for (let i = startIdx; i < endIdx && i < values.length; i++) {
        const val = values[i]
        if (val && EXAM_TYPES.includes(val)) {
          if (!weekExams[weekNum]) {
            weekExams[weekNum] = []
          }
          // Agregar cada ocurrencia de examen (no solo una por asignatura)
          weekExams[weekNum]!.push({ asignatura: asignaturaName, type: val })
        }
      }
    }
  }
  
  // Filtrar semanas con más de 2 exámenes en total
  const conflicts: ExamConflict[] = []
  for (const [weekStr, exams] of Object.entries(weekExams)) {
    if (exams.length > 2) {
      // Agrupar por asignatura para mostrar mejor
      const bySubject: Record<string, { type: string; count: number }> = {}
      for (const exam of exams) {
        if (!bySubject[exam.asignatura]) {
          bySubject[exam.asignatura] = { type: exam.type, count: 0 }
        }
        bySubject[exam.asignatura]!.count++
      }
      
      conflicts.push({
        week: parseInt(weekStr, 10),
        exams: Object.entries(bySubject).map(([asignatura, data]) => ({
          asignatura,
          type: data.type,
          count: data.count
        })),
        totalCount: exams.length
      })
    }
  }
  
  return conflicts.sort((a, b) => a.week - b.week)
})

// Cargar lista de balances
async function loadBalances() {
  loading.value = true
  error.value = ''
  try {
    const response = await balancesService.list()
    balances.value = response.data || []
  } catch (err: unknown) {
    error.value = err instanceof Error ? err.message : 'Error al cargar los balances'
  } finally {
    loading.value = false
  }
}

// Cargar detalle de un balance
async function loadBalanceDetail() {
  if (!selectedBalanceId.value) return
  
  loadingDetail.value = true
  errorDetail.value = ''
  try {
    const response = await balancesService.get(selectedBalanceId.value)
    balance.value = response.data || null
  } catch (err: unknown) {
    errorDetail.value = err instanceof Error ? err.message : 'Error al cargar el balance'
  } finally {
    loadingDetail.value = false
  }
}

// Seleccionar un balance
function selectBalance(id: number) {
  selectedBalanceId.value = id
  router.push({ path: '/balance/view', query: { id: String(id) } })
}

// Volver a la lista
function goBackToList() {
  selectedBalanceId.value = null
  balance.value = null
  router.push({ path: '/balance/view' })
}

// Limpiar filtros
function clearFilters() {
  filters.value = {
    academicYearText: '',
    period: '',
    status: ''
  }
}

// Ver/Editar fragmento
function viewFragment(fragment: Fragment) {
  router.push({ 
    path: '/balance/fragment', 
    query: { 
      balanceId: fragment.balance_id.toString(), 
      asignaturaId: fragment.asignatura_id.toString() 
    } 
  })
}

// Verificar si el usuario puede editar un fragmento
function canEditFragment(fragment: Fragment): boolean {
  // Leader siempre puede editar cualquier fragmento
  if (authStore.isLeader) {
    return true
  }
  
  // SubjectLeader puede editar su propio fragmento
  if (authStore.isSubjectLeader && fragment.subject_leader_id === authStore.user?.id) {
    return true
  }
  
  return false
}

// Obtener título del botón según el contexto
function getFragmentButtonTitle(fragment: Fragment): string {
  if (authStore.isLeader) {
    return 'Ver/Editar fragmento'
  }
  if (fragment.status === 'completed') {
    return 'Ver fragmento completado'
  }
  return 'Editar mi fragmento'
}

// Editar balance
function editBalance() {
  if (balance.value) {
    router.push({ path: '/balance', query: { id: balance.value.id.toString() } })
  }
}

// Confirmar eliminación de balance
function confirmDeleteBalance(balanceItem: BalanceListItem) {
  uiStore.openConfirm({
    title: 'Eliminar Balance',
    message: `¿Estás seguro de que deseas eliminar "${balanceItem.name}"? Esta acción eliminará todos los fragmentos asociados.`,
    confirmText: 'Sí, eliminar',
    cancelText: 'Cancelar',
    onConfirm: async () => {
      try {
        await balancesService.delete(balanceItem.id)
        await loadBalances()
        uiStore.showSuccess('Balance eliminado correctamente')
      } catch (err: unknown) {
        const message = err instanceof Error ? err.message : 'Error al eliminar el balance'
        uiStore.showError(message)
      }
    }
  })
}

// Observar cambios en la query
watch(() => route.query.id, (newId) => {
  if (newId) {
    selectedBalanceId.value = Number(newId)
    loadBalanceDetail()
  } else {
    selectedBalanceId.value = null
    balance.value = null
  }
}, { immediate: true })

onMounted(() => {
  loadBalances()
  
  if (route.query.id) {
    selectedBalanceId.value = Number(route.query.id)
    loadBalanceDetail()
  }
})
</script>
