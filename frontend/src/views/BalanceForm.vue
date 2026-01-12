<template>
  <AppLayout>
    <!-- Loading state -->
    <div v-if="isLoading" class="flex items-center justify-center min-h-[60vh]">
      <div class="text-center">
        <div class="animate-spin rounded-full h-16 w-16 border-b-2 border-blue-600 mx-auto mb-4"></div>
        <p class="text-gray-600">Cargando...</p>
      </div>
    </div>

    <!-- Crear nuevo balance -->
    <div v-else-if="isCreating" class="space-y-6">
      <!-- Header -->
      <header class="mb-6">
        <nav class="flex items-center text-sm text-blue-600 mb-2">
          <router-link to="/dashboard" class="hover:underline">Dashboard</router-link>
          <svg class="w-4 h-4 mx-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
          </svg>
          <span class="text-gray-700">Nuevo Balance</span>
        </nav>
        <h1 class="text-3xl font-bold text-blue-700">Crear Balance de Carga</h1>
        <p class="text-gray-600 mt-1">Configura los datos del balance y selecciona las asignaturas</p>
      </header>

      <!-- Configuración del balance -->
      <AppCard title="Configuración del Balance">
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          <!-- Año académico -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Año Académico</label>
            <select
              v-model="formData.academic_year"
              class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
            >
              <option value="1ro">1er Año</option>
              <option value="2do">2do Año</option>
              <option value="3ro">3er Año</option>
              <option value="4to">4to Año</option>
            </select>
          </div>

          <!-- Período -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Período</label>
            <select
              v-model="formData.period"
              class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
            >
              <option value="1ero">1er Semestre</option>
              <option value="2do">2do Semestre</option>
            </select>
          </div>

          <!-- Texto del año -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Curso</label>
            <input
              v-model="formData.academic_year_text"
              type="text"
              placeholder="2025-2026"
              class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
            />
          </div>

          <!-- Fecha de inicio -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Fecha de Inicio</label>
            <input
              v-model="formData.start_date"
              type="date"
              class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
            />
          </div>

          <!-- Semanas -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Semanas Lectivas</label>
            <input
              v-model.number="formData.weeks"
              type="number"
              min="1"
              max="20"
              class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
            />
          </div>

          <!-- Deadline -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Fecha límite (opcional)</label>
            <input
              v-model="formData.deadline"
              type="date"
              class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
            />
          </div>
        </div>
      </AppCard>

      <!-- Períodos No Académicos -->
      <AppCard title="Períodos No Académicos (opcional)">
        <p class="text-sm text-gray-600 mb-4">
          Define los intervalos de vacaciones o días feriados que se excluirán del calendario académico. 
          Las semanas se calcularán automáticamente saltando estos períodos y los fines de semana.
        </p>

        <!-- Lista de períodos -->
        <div v-if="formData.non_academic_periods.length > 0" class="space-y-3 mb-4">
          <div 
            v-for="(period, index) in formData.non_academic_periods" 
            :key="index"
            class="flex items-center gap-3 p-3 bg-gray-50 rounded-lg border border-gray-200"
          >
            <div class="flex-1 grid grid-cols-1 md:grid-cols-3 gap-3">
              <div>
                <label class="block text-xs text-gray-500 mb-1">Desde</label>
                <input
                  v-model="period.start"
                  type="date"
                  class="w-full px-2 py-1.5 text-sm border border-gray-300 rounded focus:ring-blue-500 focus:border-blue-500"
                />
              </div>
              <div>
                <label class="block text-xs text-gray-500 mb-1">Hasta</label>
                <input
                  v-model="period.end"
                  type="date"
                  class="w-full px-2 py-1.5 text-sm border border-gray-300 rounded focus:ring-blue-500 focus:border-blue-500"
                />
              </div>
              <div>
                <label class="block text-xs text-gray-500 mb-1">Descripción</label>
                <input
                  v-model="period.name"
                  type="text"
                  placeholder="Ej: Vacaciones de invierno"
                  class="w-full px-2 py-1.5 text-sm border border-gray-300 rounded focus:ring-blue-500 focus:border-blue-500"
                />
              </div>
            </div>
            <button
              type="button"
              @click="removePeriod(index)"
              class="p-2 text-red-500 hover:bg-red-50 rounded-lg transition-colors"
              title="Eliminar período"
            >
              <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
              </svg>
            </button>
          </div>
        </div>

        <!-- Botón agregar período -->
        <button
          type="button"
          @click="addPeriod"
          class="inline-flex items-center gap-2 px-4 py-2 text-sm font-medium text-blue-600 bg-blue-50 rounded-lg hover:bg-blue-100 transition-colors"
        >
          <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
          </svg>
          Agregar período no académico
        </button>
      </AppCard>

      <!-- Selección de asignaturas -->
      <AppCard title="Seleccionar Asignaturas">
        <p class="text-sm text-gray-600 mb-4">
          Selecciona las asignaturas que formarán parte de este balance. 
          Se creará un fragmento para cada asignatura, asignado a su Jefe de Asignatura.
        </p>

        <div v-if="asignaturasStore.asignaturasList.length === 0" class="text-center py-8 bg-gray-50 rounded-lg">
          <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
          </svg>
          <h3 class="mt-2 text-sm font-medium text-gray-900">No hay asignaturas disponibles</h3>
          <p class="mt-1 text-sm text-gray-500">Primero debes crear asignaturas en el sistema</p>
          <router-link to="/asignaturas" class="mt-3 inline-flex items-center text-blue-600 hover:underline">
            Ir a Asignaturas
            <svg class="w-4 h-4 ml-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6" />
            </svg>
          </router-link>
        </div>

        <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
          <div
            v-for="asignatura in asignaturasStore.asignaturasList"
            :key="asignatura.id"
            @click="toggleAsignatura(asignatura.id)"
            class="border rounded-lg p-3 cursor-pointer transition-all"
            :class="isSelected(asignatura.id) 
              ? 'border-blue-500 bg-blue-50 ring-2 ring-blue-200' 
              : 'border-gray-200 hover:border-blue-300 hover:bg-gray-50'"
          >
            <div class="flex items-start justify-between">
              <div class="flex-1 min-w-0">
                <h4 class="text-sm font-medium text-gray-900 truncate">{{ asignatura.name }}</h4>
                <p class="text-xs text-gray-500 mt-0.5">{{ asignatura.year }} - {{ asignatura.semester }}</p>
                <p v-if="asignatura.leader_id" class="text-xs text-blue-600 mt-1 truncate">
                  Jefe asignado
                </p>
                <p v-else class="text-xs text-amber-600 mt-1">
                  ⚠️ Sin Jefe de Asignatura
                </p>
              </div>
              <div class="ml-2 flex-shrink-0">
                <div 
                  class="w-5 h-5 rounded border-2 flex items-center justify-center transition-colors"
                  :class="isSelected(asignatura.id) 
                    ? 'border-blue-500 bg-blue-500' 
                    : 'border-gray-300'"
                >
                  <svg v-if="isSelected(asignatura.id)" class="w-3 h-3 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M5 13l4 4L19 7" />
                  </svg>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Contador de selección -->
        <div class="mt-4 pt-4 border-t flex items-center justify-between">
          <span class="text-sm text-gray-600">
            {{ selectedCount }} asignatura(s) seleccionada(s)
          </span>
          <div class="flex items-center gap-2">
            <AppButton variant="ghost" size="sm" @click="cancelCreation">
              Cancelar
            </AppButton>
            <AppButton 
              variant="primary" 
              size="sm" 
              :disabled="selectedCount === 0 || isSaving"
              @click="createBalance"
            >
              {{ isSaving ? 'Creando...' : 'Crear Balance' }}
            </AppButton>
          </div>
        </div>
      </AppCard>
    </div>

    <!-- Ver balance existente (con fragmentos) -->
    <div v-else-if="currentBalance" class="space-y-6">
      <!-- Header -->
      <header class="mb-6">
        <nav class="flex items-center text-sm text-blue-600 mb-2">
          <router-link to="/dashboard" class="hover:underline">Dashboard</router-link>
          <svg class="w-4 h-4 mx-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
          </svg>
          <span class="text-gray-700">{{ currentBalance.name }}</span>
        </nav>
        <div class="flex items-start justify-between">
          <div>
            <h1 class="text-3xl font-bold text-blue-700">{{ currentBalance.name }}</h1>
            <p class="text-gray-600 mt-1">
              Inicio: {{ formatDate(currentBalance.start_date) }} · {{ currentBalance.weeks }} semanas
              <span v-if="currentBalance.deadline" class="ml-2 text-amber-600">
                · Límite: {{ formatDate(currentBalance.deadline) }}
              </span>
            </p>
          </div>
          <div class="flex items-center gap-2">
            <span 
              class="px-2 py-1 text-xs font-medium rounded-full"
              :class="statusClass(currentBalance.status)"
            >
              {{ statusLabel(currentBalance.status) }}
            </span>
          </div>
        </div>
      </header>

      <!-- Progreso general -->
      <AppCard title="Progreso del Balance">
        <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-4">
          <div class="text-center p-3 bg-gray-50 rounded-lg">
            <div class="text-2xl font-bold text-gray-700">{{ currentBalance.progress.total }}</div>
            <div class="text-xs text-gray-500">Total</div>
          </div>
          <div class="text-center p-3 bg-yellow-50 rounded-lg">
            <div class="text-2xl font-bold text-yellow-600">{{ currentBalance.progress.pending }}</div>
            <div class="text-xs text-gray-500">Pendientes</div>
          </div>
          <div class="text-center p-3 bg-blue-50 rounded-lg">
            <div class="text-2xl font-bold text-blue-600">{{ currentBalance.progress.in_progress }}</div>
            <div class="text-xs text-gray-500">En progreso</div>
          </div>
          <div class="text-center p-3 bg-green-50 rounded-lg">
            <div class="text-2xl font-bold text-green-600">{{ currentBalance.progress.completed }}</div>
            <div class="text-xs text-gray-500">Completados</div>
          </div>
        </div>

        <!-- Barra de progreso -->
        <div class="w-full bg-gray-200 rounded-full h-2.5">
          <div 
            class="bg-green-600 h-2.5 rounded-full transition-all duration-300"
            :style="{ width: `${currentBalance.progress.percentage}%` }"
          ></div>
        </div>
        <p class="text-sm text-gray-500 text-center mt-2">
          {{ currentBalance.progress.percentage.toFixed(0) }}% completado
        </p>
      </AppCard>

      <!-- Lista de fragmentos -->
      <AppCard title="Fragmentos por Asignatura">
        <div class="space-y-3">
          <div
            v-for="fragment in currentBalance.fragments"
            :key="fragment.id"
            class="border rounded-lg p-4"
            :class="fragmentBorderClass(fragment.status)"
          >
            <div class="flex items-start justify-between">
              <div class="flex-1 min-w-0">
                <h4 class="font-medium text-gray-900">
                  {{ fragment.asignatura?.name || 'Asignatura eliminada' }}
                </h4>
                <p v-if="fragment.subject_leader" class="text-sm text-gray-600 mt-0.5">
                  Jefe: {{ fragment.subject_leader.name }}
                </p>
                <p v-else class="text-sm text-amber-600 mt-0.5">
                  ⚠️ Sin Jefe de Asignatura asignado
                </p>
              </div>
              <div class="ml-4 flex items-center gap-2">
                <span 
                  class="px-2 py-1 text-xs font-medium rounded-full"
                  :class="fragmentStatusClass(fragment.status)"
                >
                  {{ fragmentStatusLabel(fragment.status) }}
                </span>
                <!-- Botón para editar si leader tiene permiso -->
                <AppButton
                  variant="ghost"
                  size="sm"
                  @click="editFragment(fragment)"
                >
                  <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z" />
                  </svg>
                </AppButton>
              </div>
            </div>
            <p v-if="fragment.completed_at" class="text-xs text-gray-400 mt-2">
              Completado: {{ formatDate(fragment.completed_at) }}
            </p>
          </div>
        </div>

        <!-- Acciones del balance -->
        <div class="mt-6 pt-4 border-t flex items-center justify-between">
          <AppButton variant="ghost" size="sm" @click="goBack">
            ← Volver
          </AppButton>
          <div class="flex items-center gap-2">
            <AppButton 
              v-if="currentBalance.status !== 'completed'"
              variant="secondary" 
              size="sm" 
              @click="showEditModal = true"
            >
              Editar Configuración
            </AppButton>
            <AppButton 
              variant="danger" 
              size="sm" 
              @click="confirmDelete"
            >
              Eliminar Balance
            </AppButton>
          </div>
        </div>
      </AppCard>
    </div>

    <!-- Estado vacío -->
    <div v-else class="flex items-center justify-center min-h-[60vh]">
      <div class="text-center">
        <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
        </svg>
        <h3 class="mt-2 text-sm font-medium text-gray-900">Balance no encontrado</h3>
        <p class="mt-1 text-sm text-gray-500">El balance solicitado no existe o no tienes acceso</p>
        <router-link to="/dashboard" class="mt-3 inline-flex items-center text-blue-600 hover:underline">
          ← Volver al Dashboard
        </router-link>
      </div>
    </div>

    <!-- Modal de edición de configuración -->
    <AppModal v-model="showEditModal" title="Editar Configuración" size="md">
      <div class="space-y-4">
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Fecha límite</label>
          <input
            v-model="editData.deadline"
            type="date"
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
          />
        </div>
      </div>
      <template #footer>
        <div class="flex justify-end gap-2">
          <AppButton variant="ghost" @click="showEditModal = false">Cancelar</AppButton>
          <AppButton variant="primary" @click="saveChanges" :disabled="isSaving">
            {{ isSaving ? 'Guardando...' : 'Guardar Cambios' }}
          </AppButton>
        </div>
      </template>
    </AppModal>
  </AppLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useBalanceStore, type Fragment } from '../stores/balance'
import { useAsignaturasStore } from '../stores/asignaturas'
import { useUIStore } from '../stores/ui'
import AppLayout from '../components/AppLayout.vue'
import AppCard from '../components/AppCard.vue'
import AppButton from '../components/AppButton.vue'
import AppModal from '../components/AppModal.vue'
import type { NonAcademicPeriod } from '../services/balances'

const router = useRouter()
const route = useRoute()
const balanceStore = useBalanceStore()
const asignaturasStore = useAsignaturasStore()
const uiStore = useUIStore()

// State
const isLoading = ref(true)
const isSaving = ref(false)
const showEditModal = ref(false)

// Form data para crear balance
const formData = ref({
  academic_year: '1ro',
  period: '1ero',
  academic_year_text: '2025-2026',
  start_date: new Date().toISOString().split('T')[0],
  weeks: 15,
  deadline: '',
  allow_leader_edit: false,
  non_academic_periods: [] as NonAcademicPeriod[],
})

// Data para editar balance existente
const editData = ref({
  deadline: '',
  allow_leader_edit: false,
})

// Asignaturas seleccionadas para crear
const selectedAsignaturas = ref<number[]>([])

// Computed
const isCreating = computed(() => !route.query.id && balanceStore.editableBalance !== null)
const currentBalance = computed(() => balanceStore.currentBalance)
const selectedCount = computed(() => selectedAsignaturas.value.length)

// Métodos
function isSelected(id: number): boolean {
  return selectedAsignaturas.value.includes(id)
}

function toggleAsignatura(id: number) {
  const idx = selectedAsignaturas.value.indexOf(id)
  if (idx >= 0) {
    selectedAsignaturas.value.splice(idx, 1)
  } else {
    selectedAsignaturas.value.push(id)
  }
}

// Funciones para períodos no académicos
function addPeriod() {
  // Calcular fecha por defecto: 2 semanas antes de la semana 15
  const startDate = formData.value.start_date ? new Date(formData.value.start_date) : new Date()
  const defaultStart = new Date(startDate)
  defaultStart.setDate(defaultStart.getDate() + (13 * 7)) // Semana 14
  const defaultEnd = new Date(defaultStart)
  defaultEnd.setDate(defaultEnd.getDate() + 13) // 2 semanas
  
  formData.value.non_academic_periods.push({
    start: defaultStart.toISOString().slice(0, 10),
    end: defaultEnd.toISOString().slice(0, 10),
    name: 'Vacaciones',
  })
}

function removePeriod(index: number) {
  formData.value.non_academic_periods.splice(index, 1)
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

function cancelCreation() {
  balanceStore.cancelNewBalance()
  router.push('/dashboard')
}

async function createBalance() {
  if (selectedAsignaturas.value.length === 0) {
    uiStore.showWarning('Debes seleccionar al menos una asignatura')
    return
  }

  isSaving.value = true
  
  // Actualizar el store con los datos del formulario
  balanceStore.updateEditableBalance({
    academic_year: formData.value.academic_year,
    period: formData.value.period,
    academic_year_text: formData.value.academic_year_text,
    start_date: formData.value.start_date,
    weeks: formData.value.weeks,
    deadline: formData.value.deadline || null,
    allow_leader_edit: formData.value.allow_leader_edit,
    selectedAsignaturas: selectedAsignaturas.value,
    non_academic_periods: formData.value.non_academic_periods,
  })

  const result = await balanceStore.createBalance()
  
  isSaving.value = false
  
  if (result.success) {
    uiStore.showSuccess(result.message)
    if (result.balanceId) {
      router.push({ path: '/balance', query: { id: result.balanceId } })
    } else {
      router.push('/dashboard')
    }
  } else {
    uiStore.showError(result.message)
  }
}

function editFragment(fragment: Fragment) {
  router.push({ 
    path: '/balance/fragment', 
    query: { 
      balanceId: fragment.balance_id, 
      asignaturaId: fragment.asignatura_id 
    } 
  })
}

function goBack() {
  router.push('/dashboard')
}

function confirmDelete() {
  if (!currentBalance.value) return
  
  uiStore.openConfirm({
    title: 'Eliminar Balance',
    message: `¿Estás seguro de eliminar "${currentBalance.value.name}"? Esta acción eliminará todos los fragmentos asociados.`,
    confirmText: 'Sí, eliminar',
    cancelText: 'Cancelar',
    onConfirm: async () => {
      const result = await balanceStore.deleteBalance(currentBalance.value!.id)
      if (result.success) {
        uiStore.showSuccess(result.message)
        router.push('/dashboard')
      } else {
        uiStore.showError(result.message)
      }
    },
  })
}

async function saveChanges() {
  if (!currentBalance.value) return
  
  isSaving.value = true
  
  const result = await balanceStore.updateBalance(currentBalance.value.id, {
    deadline: editData.value.deadline || undefined,
    allow_leader_edit: editData.value.allow_leader_edit,
  })
  
  isSaving.value = false
  showEditModal.value = false
  
  if (result.success) {
    uiStore.showSuccess(result.message)
  } else {
    uiStore.showError(result.message)
  }
}

// Inicialización
onMounted(async () => {
  isLoading.value = true
  
  try {
    // Cargar asignaturas para la selección
    if (asignaturasStore.asignaturas.length === 0) {
      await asignaturasStore.fetchAsignaturas()
    }
    
    const balanceId = route.query.id as string
    
    if (balanceId) {
      // Cargar balance existente
      const loaded = await balanceStore.loadBalance(parseInt(balanceId, 10))
      if (!loaded) {
        uiStore.showError('Balance no encontrado')
      } else if (currentBalance.value) {
        // Preparar datos de edición
        editData.value = {
          deadline: currentBalance.value.deadline || '',
          allow_leader_edit: currentBalance.value.allow_leader_edit,
        }
      }
    } else {
      // Modo creación
      balanceStore.startNewBalance()
    }
  } catch (error) {
    console.error('Error inicializando:', error)
    uiStore.showError('Error al cargar datos')
  } finally {
    isLoading.value = false
  }
})

// Watch para sincronizar cambios
watch(() => currentBalance.value, (balance) => {
  if (balance) {
    editData.value = {
      deadline: balance.deadline || '',
      allow_leader_edit: balance.allow_leader_edit,
    }
  }
})
</script>
