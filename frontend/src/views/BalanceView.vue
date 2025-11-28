<!-- Vista de Balances: Lista con filtros y detalle (solo lectura) -->
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
          <p class="text-gray-600 mt-1">Selecciona un balance para ver sus detalles</p>
        </header>

        <!-- Filtros -->
        <AppCard class="mb-6">
          <h3 class="text-lg font-semibold text-gray-800 mb-4">Filtros</h3>
          <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-5 gap-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Año Académico</label>
              <select v-model="filters.academicYear" class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500">
                <option value="">Todos</option>
                <option v-for="year in availableYears" :key="year" :value="year">{{ year }}</option>
              </select>
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Año de Carrera</label>
              <select v-model="filters.academicYearText" class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500">
                <option value="">Todos</option>
                <option value="1ro">1ro</option>
                <option value="2do">2do</option>
                <option value="3ro">3ro</option>
                <option value="4to">4to</option>
              </select>
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Período</label>
              <select v-model="filters.period" class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500">
                <option value="">Todos</option>
                <option :value="1">1er Semestre</option>
                <option :value="2">2do Semestre</option>
              </select>
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Semanas</label>
              <select v-model="filters.weeks" class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500">
                <option value="">Todas</option>
                <option :value="15">15 semanas</option>
                <option :value="16">16 semanas</option>
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
            v-for="balance in filteredBalances"
            :key="balance.id"
            class="bg-white rounded-lg border border-gray-200 p-5 cursor-pointer transition hover:shadow-lg hover:border-blue-400 hover:-translate-y-0.5"
            @click="selectBalance(balance.id)"
          >
            <div class="flex justify-between items-start mb-3">
              <h3 class="font-semibold text-gray-900 text-lg">{{ balance.name }}</h3>
              <span class="bg-blue-100 text-blue-800 text-xs font-medium px-2.5 py-0.5 rounded-full">
                {{ balance.weeks }} semanas
              </span>
            </div>
            <div class="space-y-1 text-sm text-gray-600 mb-4">
              <p><span class="font-medium">Año Académico:</span> {{ balance.academic_year }}</p>
              <p><span class="font-medium">Año de Carrera:</span> {{ balance.academic_year_text }}</p>
              <p><span class="font-medium">Período:</span> {{ balance.period === 1 ? '1er Semestre' : '2do Semestre' }}</p>
              <p><span class="font-medium">Asignaturas:</span> {{ balance.subjects?.length || 0 }}</p>
            </div>
            <div class="border-t border-gray-100 pt-3">
              <span class="text-blue-600 font-medium text-sm flex items-center">
                Ver detalles
                <svg class="w-4 h-4 ml-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                </svg>
              </span>
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
            <h1 class="text-3xl font-bold text-blue-700">{{ balance.name }}</h1>
            <div class="flex flex-wrap gap-2 mt-3">
              <span class="bg-blue-100 text-blue-800 text-sm font-medium px-3 py-1 rounded-full">
                {{ balance.academic_year }}
              </span>
              <span class="bg-purple-100 text-purple-800 text-sm font-medium px-3 py-1 rounded-full">
                {{ balance.academic_year_text }}
              </span>
              <span class="bg-green-100 text-green-800 text-sm font-medium px-3 py-1 rounded-full">
                {{ balance.period === 1 ? '1er Semestre' : '2do Semestre' }}
              </span>
              <span class="bg-orange-100 text-orange-800 text-sm font-medium px-3 py-1 rounded-full">
                {{ balance.weeks }} semanas
              </span>
            </div>
          </header>

          <!-- Tabs de asignaturas -->
          <AppCard v-if="balance.subjects && balance.subjects.length > 0" padding="none">
            <AppTabs v-model="activeSubjectTab" :tabs="subjectTabs">
              <template #default>
                <div class="p-4">
                  <!-- Header de tabla -->
                  <div class="mb-3 flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3">
                    <div>
                      <h3 class="text-base font-bold text-blue-700">
                        {{ showAllSubjects ? 'Todas las Asignaturas' : activeSubject?.name }}
                      </h3>
                      <p class="text-xs text-gray-600 mt-0.5">
                        Vista de solo lectura · {{ showAllSubjects ? balance.subjects.length + ' asignaturas' : '1 asignatura' }}
                      </p>
                    </div>
                    <div class="flex gap-2">
                      <AppButton variant="primary" size="sm" @click="exportBalance">
                        <template #icon>
                          <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
                          </svg>
                        </template>
                        Exportar
                      </AppButton>
                      <AppButton variant="outline" size="sm" @click="goBackToList">
                        <template #icon>
                          <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
                          </svg>
                        </template>
                        Volver
                      </AppButton>
                    </div>
                  </div>

                  <!-- Tablas de balance por grupos de semanas -->
                  <div class="space-y-6">
                    <BalanceViewTable
                      v-for="(group, idx) in weekGroups"
                      :key="`weeks-${idx}`"
                      :subjects="showAllSubjects ? balance.subjects : [activeSubject!]"
                      :title="`Semanas ${group.start} - ${group.end}`"
                      :weeks="group.weeks"
                      :start-cell-index="group.startIndex"
                      :color-scheme="idx < weekGroups.length - 1 ? 'blue' : 'purple'"
                    />

                    <!-- Consultas y Exámenes Finales -->
                    <BalanceViewFinalTable
                      :subjects="showAllSubjects ? balance.subjects : [activeSubject!]"
                      :weeks-count="balance.weeks"
                    />
                  </div>
                </div>
              </template>
            </AppTabs>
          </AppCard>

          <!-- Sin asignaturas -->
          <AppCard v-else class="text-center py-12">
            <svg class="mx-auto h-12 w-12 text-gray-400 mb-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
            </svg>
            <p class="text-gray-500">No hay asignaturas en este balance</p>
          </AppCard>
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
import AppTabs from '@/components/AppTabs.vue'
import BalanceViewTable from '@/components/BalanceViewTable.vue'
import BalanceViewFinalTable from '@/components/BalanceViewFinalTable.vue'
import { balancesService, type Balance } from '@/services/balances'

const route = useRoute()
const router = useRouter()

// Estado general
const loading = ref(false)
const error = ref('')
const balances = ref<Balance[]>([])

// Estado para la vista de detalle
const selectedBalanceId = ref<number | null>(null)
const loadingDetail = ref(false)
const errorDetail = ref('')
const balance = ref<Balance | null>(null)
const activeSubjectTab = ref(-1) // -1 = "Todas"

// Filtros
const filters = ref({
  academicYear: '',
  academicYearText: '',
  period: '' as '' | number,
  weeks: '' as '' | number
})

// Años disponibles para filtrar
const availableYears = computed(() => {
  const years = new Set(balances.value.map((b: Balance) => b.academic_year))
  return Array.from(years).sort().reverse() as string[]
})

// Balances filtrados
const filteredBalances = computed(() => {
  return balances.value.filter((b: Balance) => {
    if (filters.value.academicYear && b.academic_year !== filters.value.academicYear) return false
    if (filters.value.academicYearText && b.academic_year_text !== filters.value.academicYearText) return false
    if (filters.value.period && b.period !== filters.value.period) return false
    if (filters.value.weeks && b.weeks !== filters.value.weeks) return false
    return true
  })
})

// Tabs de asignaturas (incluye "Todas" al inicio)
const subjectTabs = computed(() => {
  if (!balance.value?.subjects) return []
  const tabs = [
    { id: -1, label: 'Todas', icon: '📋' },
    ...balance.value.subjects.map((subject: { name: string }, index: number) => ({
      id: index,
      label: subject.name,
      icon: '📚'
    }))
  ]
  return tabs
})

// Mostrar todas las asignaturas o solo la seleccionada
const showAllSubjects = computed(() => activeSubjectTab.value === -1)

// Asignatura activa
const activeSubject = computed(() => {
  if (!balance.value?.subjects) return null
  return balance.value.subjects[activeSubjectTab.value] || null
})

// Grupos de semanas (de 4 en 4)
const weekGroups = computed(() => {
  if (!balance.value) return []
  const totalWeeks = balance.value.weeks
  const groups = []
  const groupSize = 4
  
  for (let i = 0; i < totalWeeks; i += groupSize) {
    const start = i + 1
    const end = Math.min(i + groupSize, totalWeeks)
    const weeks = []
    for (let w = start; w <= end; w++) {
      weeks.push(w)
    }
    groups.push({
      start,
      end,
      weeks,
      startIndex: i * 4, // 4 celdas por semana
    })
  }
  
  return groups
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
    activeSubjectTab.value = -1 // Mostrar "Todas" por defecto
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
    academicYear: '',
    academicYearText: '',
    period: '',
    weeks: ''
  }
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
  
  // Si hay ID en la query, cargar detalle
  if (route.query.id) {
    selectedBalanceId.value = Number(route.query.id)
    loadBalanceDetail()
  }
})
</script>
