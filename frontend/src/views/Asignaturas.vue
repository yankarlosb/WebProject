<!--
  Asignaturas - Vista refactorizada
  Gestión de asignaturas con el nuevo sistema de componentes y stores
-->
<template>
  <AppLayout>
    <!-- Header -->
    <div class="mb-8">
      <h1 class="text-3xl font-bold text-blue-700">Gestión de Asignaturas</h1>
      <p class="text-gray-600 mt-2">Administra las asignaturas del sistema académico</p>
    </div>

    <!-- Estadísticas rápidas -->
    <div class="grid grid-cols-1 sm:grid-cols-3 gap-6 mb-8">
      <StatsCard
        title="Total Asignaturas"
        :value="asignaturasStore.asignaturasCount"
        icon="book"
        color="blue"
      />
      <StatsCard
        title="Periodo 1"
        :value="asignaturasStore.asignaturasByPeriodo('Periodo 1').length"
        icon="calendar"
        color="green"
      />
      <StatsCard
        title="Periodo 2"
        :value="asignaturasStore.asignaturasByPeriodo('Periodo 2').length"
        icon="calendar"
        color="purple"
      />
    </div>

    <!-- Botón de nueva asignatura -->
    <div v-if="authStore.isLeader" class="mb-6">
      <AppButton
        variant="primary"
        size="lg"
        @click="openCreateModal"
      >
        <template #icon>
          <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
          </svg>
        </template>
        Nueva Asignatura
      </AppButton>
    </div>

    <!-- Lista de asignaturas -->
    <AppCard>
      <template #header>
        <div class="flex items-center justify-between">
          <h2 class="text-xl font-bold text-blue-700">Asignaturas Registradas</h2>
          
          <!-- Filtro por período -->
          <div class="flex items-center gap-2">
            <label class="text-sm font-medium text-gray-700">Filtrar:</label>
            <select
              v-model="filtroSeleccionado"
              class="px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-sm"
            >
              <option value="">Todos</option>
              <option value="Periodo 1">Periodo 1</option>
              <option value="Periodo 2">Periodo 2</option>
            </select>
          </div>
        </div>
      </template>

      <!-- Estado vacío -->
      <div v-if="asignaturasFiltradas.length === 0" class="text-center py-12">
        <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
        </svg>
        <h3 class="mt-2 text-sm font-medium text-gray-900">No hay asignaturas</h3>
        <div class="mt-6" v-if="!filtroSeleccionado && authStore.isLeader">
          <AppButton variant="primary" @click="openCreateModal">
            <template #icon>
              <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
              </svg>
            </template>
            Crear Asignatura
          </AppButton>
        </div>
      </div>

      <!-- Grid de tarjetas -->
      <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        <div
          v-for="asignatura in asignaturasFiltradas"
          :key="asignatura.id"
          class="bg-white border border-blue-100 rounded-lg p-4 hover:shadow-lg transition-shadow duration-200"
        >
          <!-- Header de la tarjeta -->
          <div class="flex items-start justify-between mb-3">
            <div class="flex-1">
              <h3 class="font-bold text-blue-800 text-lg">{{ asignatura.name }}</h3>
              <p class="text-sm text-gray-500 mt-1">
                <span class="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-blue-100 text-blue-800">
                  {{ asignatura.semester }}
                </span>
                <span class="ml-2 inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-green-100 text-green-800">
                  {{ asignatura.year }}
                </span>
              </p>
            </div>
            
            <!-- Menú de acciones -->
            <div class="flex gap-1">
              <!-- Botón Editar: Solo Leaders pueden editar -->
              <button
                v-if="authStore.isLeader"
                @click="openEditModal(asignatura)"
                class="p-2 text-yellow-600 hover:bg-yellow-50 rounded-lg transition-colors"
                title="Editar"
              >
                <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                </svg>
              </button>
              
              <!-- Botón Eliminar: Solo Leaders -->
              <button
                v-if="authStore.isLeader"
                @click="confirmDelete(asignatura)"
                class="p-2 text-red-600 hover:bg-red-50 rounded-lg transition-colors"
                title="Eliminar"
              >
                <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                </svg>
              </button>
            </div>
          </div>

          <!-- Detalles -->
          <div class="space-y-2 text-sm">
            <div class="flex justify-between">
              <span class="text-gray-600">Semanas:</span>
              <span class="font-semibold text-gray-900">{{ asignatura.weeks || 'N/A' }}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-gray-600">Horas Totales:</span>
              <span class="font-semibold text-gray-900">{{ asignatura.hours }}</span>
            </div>
            
            <!-- Tipos de actividad -->
            <div class="pt-2 border-t border-gray-200">
              <p class="text-xs text-gray-500 mb-2">Tipos de actividad (horas):</p>
              <div class="grid grid-cols-3 gap-2">
                <div v-for="tipo in tiposActividad" :key="tipo.id" class="text-center">
                  <span class="text-xs font-medium text-gray-700">{{ tipo.label }}</span>
                  <p class="text-sm font-bold text-blue-700">{{ (asignatura as any)[tipo.id] || 0 }}</p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </AppCard>

    <!-- Modal de Crear/Editar -->
    <AppModal
      v-model="showModal"
      :title="isEditing ? 'Editar Asignatura' : 'Nueva Asignatura'"
      size="lg"
    >
      <!-- Formulario de CREACIÓN (Leaders) - Solo datos básicos -->
      <form v-if="!isEditing" @submit.prevent="handleSave" class="space-y-4">
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <!-- Nombre -->
          <div class="md:col-span-2">
            <AppInput
              v-model="createForm.name"
              label="Nombre de la asignatura"
              placeholder="Ej: Programación Web"
              required
              :error="errors.name"
            />
          </div>

          <!-- Jefe de Asignatura -->
          <div class="md:col-span-2">
            <label class="block text-sm font-medium text-gray-700 mb-2">Jefe de Asignatura *</label>
            
            <!-- Indicador de carga -->
            <div v-if="asignaturasStore.isLoading" class="text-sm text-gray-500 py-2">
              Cargando jefes de asignatura...
            </div>
            
            <!-- Mensaje si no hay jefes -->
            <div v-else-if="asignaturasStore.subjectLeaders.length === 0" class="text-sm text-amber-600 py-2">
              ⚠️ No hay jefes de asignatura disponibles. Por favor, crea usuarios con rol "Jefe de Asignatura" primero.
            </div>
            
            <!-- Selector -->
            <select
              v-else
              v-model="createForm.leader_user_name"
              required
              class="w-full px-4 py-2.5 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
              :class="{ 'border-red-500': errors.leader }"
            >
              <option value="">Selecciona un jefe de asignatura</option>
              <option 
                v-for="leader in asignaturasStore.subjectLeaders" 
                :key="leader.id"
                :value="leader.user_name"
              >
                {{ leader.name }} ({{ leader.user_name }})
              </option>
            </select>
            <p v-if="errors.leader" class="mt-1 text-sm text-red-600">{{ errors.leader }}</p>
            
            <!-- Debug info (comentar en producción) -->
            <p class="mt-1 text-xs text-gray-400">
              Jefes cargados: {{ asignaturasStore.subjectLeaders.length }}
            </p>
          </div>

          <!-- Año -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">Año</label>
            <select
              v-model="createForm.year"
              required
              class="w-full px-4 py-2.5 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
            >
              <option value="1ro">1ro.ICS</option>
              <option value="2do">2do.ICS</option>
              <option value="3ro">3ro.ICS</option>
              <option value="4to">4to.ICS</option>
            </select>
          </div>

          <!-- Semestre -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">Semestre</label>
            <select
              v-model="createForm.semester"
              required
              class="w-full px-4 py-2.5 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
            >
              <option value="Periodo 1">Periodo 1</option>
              <option value="Periodo 2">Periodo 2</option>
            </select>
          </div>
        </div>
      </form>

      <!-- Formulario de EDICIÓN (Leaders) - Todos los campos -->
      <form v-else @submit.prevent="handleSave" class="space-y-4">
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <!-- Nombre -->
          <div class="md:col-span-2">
            <AppInput
              v-model="editForm.name"
              label="Nombre de la asignatura"
              placeholder="Ej: Programación Web"
              required
              :error="errors.name"
            />
          </div>

          <!-- Año -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">Año</label>
            <select
              v-model="editForm.year"
              required
              class="w-full px-4 py-2.5 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
            >
              <option value="1ro">1ro.ICS</option>
              <option value="2do">2do.ICS</option>
              <option value="3ro">3ro.ICS</option>
              <option value="4to">4to.ICS</option>
            </select>
          </div>

          <!-- Semestre -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">Semestre</label>
            <select
              v-model="editForm.semester"
              required
              class="w-full px-4 py-2.5 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
            >
              <option value="Periodo 1">Periodo 1</option>
              <option value="Periodo 2">Periodo 2</option>
            </select>
          </div>

          <!-- Semanas -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">Semanas</label>
            <select
              v-model.number="editForm.weeks"
              required
              class="w-full px-4 py-2.5 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
            >
              <option :value="15">15</option>
              <option :value="16">16</option>
            </select>
          </div>

          <!-- Horas Totales -->
          <AppInput
            v-model.number="editForm.hours"
            type="number"
            label="Horas Totales"
            placeholder="0"
            min="0"
            required
          />
        </div>

        <!-- Tipos de actividad -->
        <div>
          <h3 class="text-sm font-medium text-gray-700 mb-3">Tipos de Actividad (Horas)</h3>
          <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
            <AppInput
              v-for="tipo in tiposActividad"
              :key="tipo.id"
              v-model.number="(editForm as any)[tipo.id]"
              type="number"
              :label="tipo.label"
              placeholder="0"
              min="0"
              size="sm"
            />
          </div>
        </div>
      </form>

      <template #footer>
        <AppButton
          variant="ghost"
          @click="closeModal"
        >
          Cancelar
        </AppButton>
        
        <AppButton
          variant="primary"
          @click="handleSave"
          :loading="isSaving"
        >
          <template #icon>
            <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
            </svg>
          </template>
          {{ isEditing ? 'Actualizar' : 'Crear' }}
        </AppButton>
      </template>
    </AppModal>
  </AppLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useAsignaturasStore, type Asignatura } from '../stores/asignaturas'
import { useAuthStore } from '../stores/auth'
import { useUIStore } from '../stores/ui'
import type { UpdateAsignaturaData } from '../services/asignaturas'
import AppLayout from '../components/AppLayout.vue'
import AppCard from '../components/AppCard.vue'
import AppButton from '../components/AppButton.vue'
import AppInput from '../components/AppInput.vue'
import AppModal from '../components/AppModal.vue'
import StatsCard from '../components/StatsCard.vue'
import { tiposActividad } from '../utils/constants'
import { isValidSubjectName, isPositiveNumber } from '../utils/validation'

const asignaturasStore = useAsignaturasStore()
const authStore = useAuthStore()
const uiStore = useUIStore()

// Estado
const showModal = ref(false)
const isEditing = ref(false)
const editingId = ref<number | null>(null)
const isSaving = ref(false)
const filtroSeleccionado = ref('')

// Formulario para CREAR (Leaders) - solo datos básicos
interface CreateForm {
  name: string
  year: string
  semester: string
  leader_user_name: string
}

// Formulario para EDITAR (Leaders) - todos los datos
interface EditForm extends Omit<UpdateAsignaturaData, 'hours'> {
  hours: number
}

/**
 * Returns the default create form values for resetting the form
 * @returns CreateForm with default values
 */
const getDefaultCreateForm = (): CreateForm => ({
  name: '',
  year: '1ro',
  semester: 'Periodo 1',
  leader_user_name: ''
})

/**
 * Returns the default edit form values for resetting the form
 * @returns EditForm with default values
 */
const getDefaultEditForm = (): EditForm => ({
  name: '',
  year: '',
  semester: '',
  c: null,
  cp: null,
  s: null,
  pl: null,
  te: null,
  t: null,
  pp: null,
  ec: null,
  tc: null,
  ef: null,
  hours: 0,
  weeks: 15
})

/**
 * Returns the default error state for form validation
 * @returns Object with empty error strings for name and leader fields
 */
const getDefaultErrors = () => ({ name: '', leader: '' })

const createForm = ref<CreateForm>(getDefaultCreateForm())

const editForm = ref<EditForm>(getDefaultEditForm())

const errors = ref(getDefaultErrors())

// Computed
const asignaturasFiltradas = computed(() => {
  if (!filtroSeleccionado.value) {
    return asignaturasStore.asignaturasList
  }
  return asignaturasStore.asignaturasByPeriodo(filtroSeleccionado.value)
})

// Cargar datos iniciales
onMounted(async () => {
  await asignaturasStore.loadAsignaturas()
  
  // Si es Leader, cargar lista de jefes de asignatura
  if (authStore.isLeader) {
    await asignaturasStore.loadSubjectLeaders()
  }
})

// Métodos
async function openCreateModal() {
  isEditing.value = false
  editingId.value = null
  createForm.value = getDefaultCreateForm()
  errors.value = getDefaultErrors()
  
  // Cargar jefes de asignatura si no están cargados
  if (authStore.isLeader && asignaturasStore.subjectLeaders.length === 0) {
    await asignaturasStore.loadSubjectLeaders()
  }
  
  showModal.value = true
}

function openEditModal(asignatura: Asignatura) {
  isEditing.value = true
  editingId.value = asignatura.id
  
  // Copiar datos al formulario de edición
  editForm.value = {
    name: asignatura.name,
    year: asignatura.year,
    semester: asignatura.semester,
    c: asignatura.c,
    cp: asignatura.cp,
    s: asignatura.s,
    pl: asignatura.pl,
    te: asignatura.te,
    t: asignatura.t,
    pp: asignatura.pp,
    ec: asignatura.ec,
    tc: asignatura.tc,
    ef: asignatura.ef,
    hours: asignatura.hours,
    weeks: asignatura.weeks
  }
  
  errors.value = getDefaultErrors()
  showModal.value = true
}

function closeModal() {
  showModal.value = false
  setTimeout(() => {
    createForm.value = getDefaultCreateForm()
    editForm.value = getDefaultEditForm()
    isEditing.value = false
    editingId.value = null
    errors.value = getDefaultErrors()
  }, 200)
}

async function handleSave() {
  // Validación
  errors.value = getDefaultErrors()
  
  if (isEditing.value) {
    // Validar formulario de edición
    const trimmedName = editForm.value.name.trim()
    if (!trimmedName) {
      errors.value.name = 'El nombre es requerido'
      uiStore.showWarning('Por favor completa todos los campos requeridos')
      return
    }
    if (!isValidSubjectName(trimmedName)) {
      errors.value.name = 'Nombre de asignatura inválido'
      uiStore.showError('Nombre de asignatura inválido')
      return
    }
    // Validar números positivos en horas
    if (!isPositiveNumber(editForm.value.hours)) {
      uiStore.showError('Horas inválidas')
      return
    }
  } else {
    // Validar formulario de creación
    const trimmedName = createForm.value.name.trim()
    if (!trimmedName) {
      errors.value.name = 'El nombre es requerido'
      uiStore.showWarning('Por favor completa todos los campos requeridos')
      return
    }
    if (!isValidSubjectName(trimmedName)) {
      errors.value.name = 'Nombre de asignatura inválido'
      uiStore.showError('Nombre de asignatura inválido')
      return
    }
    if (!createForm.value.leader_user_name) {
      errors.value.leader = 'Debes seleccionar un jefe de asignatura'
      uiStore.showWarning('Por favor selecciona un jefe de asignatura')
      return
    }
  }

  isSaving.value = true

  try {
    if (isEditing.value && editingId.value) {
      // Actualizar (Leader)
      const result = await asignaturasStore.updateAsignatura(editingId.value, editForm.value)
      if (result.success) {
        uiStore.showSuccess('Asignatura actualizada correctamente')
        closeModal()
      } else {
        uiStore.showError(result.error || 'Error al actualizar la asignatura')
      }
    } else {
      // Crear (Leader)
      const result = await asignaturasStore.createAsignatura(createForm.value)
      if (result.success) {
        uiStore.showSuccess('Asignatura creada correctamente')
        closeModal()
      } else {
        uiStore.showError(result.error || 'Error al crear la asignatura')
      }
    }
  } catch (error) {
    console.error('Error guardando asignatura:', error)
    uiStore.showError('Error al guardar la asignatura')
  } finally {
    isSaving.value = false
  }
}

async function confirmDelete(asignatura: Asignatura) {
  uiStore.openConfirm({
    title: 'Eliminar Asignatura',
    message: `¿Estás seguro de que deseas eliminar "${asignatura.name}"? Esta acción no se puede deshacer.`,
    confirmText: 'Sí, eliminar',
    cancelText: 'Cancelar',
    onConfirm: async () => {
      const result = await asignaturasStore.deleteAsignatura(asignatura.id)
      if (result.success) {
        uiStore.showSuccess('Asignatura eliminada correctamente')
      } else {
        uiStore.showError(result.error || 'Error al eliminar la asignatura')
      }
    },
  })
}
</script>
