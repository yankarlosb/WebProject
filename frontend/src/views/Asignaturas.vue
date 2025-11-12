/**
 * Asignaturas - Vista refactorizada
 * Gestión de asignaturas con el nuevo sistema de componentes y stores
 */
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
    <div class="mb-6">
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
        <p class="mt-1 text-sm text-gray-500">
          {{ filtroSeleccionado ? 'No hay asignaturas en este período' : 'Comienza agregando tu primera asignatura' }}
        </p>
        <div class="mt-6" v-if="!filtroSeleccionado">
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
              <h3 class="font-bold text-blue-800 text-lg">{{ asignatura.nombre }}</h3>
              <p class="text-sm text-gray-500 mt-1">
                <span class="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-blue-100 text-blue-800">
                  {{ asignatura.periodo }}
                </span>
              </p>
            </div>
            
            <!-- Menú de acciones -->
            <div class="flex gap-1">
              <button
                @click="openEditModal(asignatura)"
                class="p-2 text-yellow-600 hover:bg-yellow-50 rounded-lg transition-colors"
                title="Editar"
              >
                <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                </svg>
              </button>
              <button
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
              <span class="font-semibold text-gray-900">{{ asignatura.semanas }}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-gray-600">Encuentros:</span>
              <span class="font-semibold text-gray-900">{{ asignatura.encuentros || 'N/A' }}</span>
            </div>
            
            <!-- Tipos de actividad -->
            <div class="pt-2 border-t border-gray-200">
              <p class="text-xs text-gray-500 mb-2">Tipos de actividad:</p>
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
      <form @submit.prevent="handleSave" class="space-y-4">
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <!-- Nombre -->
          <div class="md:col-span-2">
            <AppInput
              v-model="form.nombre"
              label="Nombre de la asignatura"
              placeholder="Ej: Programación Web"
              required
              :error="errors.nombre"
            />
          </div>

          <!-- Periodo -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">Período</label>
            <select
              v-model="form.periodo"
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
              v-model="form.semanas"
              required
              class="w-full px-4 py-2.5 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
            >
              <option value="15">15</option>
              <option value="16">16</option>
            </select>
          </div>

          <!-- Encuentros -->
          <AppInput
            v-model.number="form.encuentros"
            type="number"
            label="Encuentros"
            placeholder="0"
            min="0"
          />
        </div>

        <!-- Tipos de actividad -->
        <div>
          <h3 class="text-sm font-medium text-gray-700 mb-3">Tipos de Actividad</h3>
          <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
            <AppInput
              v-for="tipo in tiposActividad"
              :key="tipo.id"
              v-model.number="(form as any)[tipo.id]"
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
import { ref, computed } from 'vue'
import { useAsignaturasStore, type Asignatura } from '../stores/asignaturas'
import { useUIStore } from '../stores/ui'
import AppLayout from '../components/AppLayout.vue'
import AppCard from '../components/AppCard.vue'
import AppButton from '../components/AppButton.vue'
import AppInput from '../components/AppInput.vue'
import AppModal from '../components/AppModal.vue'
import StatsCard from '../components/StatsCard.vue'
import { tiposActividad } from '../utils/constants'

const asignaturasStore = useAsignaturasStore()
const uiStore = useUIStore()

// Estado
const showModal = ref(false)
const isEditing = ref(false)
const editingId = ref<string | null>(null)
const isSaving = ref(false)
const filtroSeleccionado = ref('')

// Formulario
const emptyForm = (): Omit<Asignatura, 'id'> => ({
  nombre: '',
  periodo: 'Periodo 1',
  fechaInicio: '',
  fechaFin: '',
  semanas: '15',
  encuentros: null,
  c: null,
  cp: null,
  pl: null,
  t: null,
  te: null,
  pp: null,
  ef: null,
})

const form = ref<Omit<Asignatura, 'id'>>(emptyForm())
const errors = ref({ nombre: '' })

// Computed
const asignaturasFiltradas = computed(() => {
  if (!filtroSeleccionado.value) {
    return asignaturasStore.asignaturasList
  }
  return asignaturasStore.asignaturasByPeriodo(filtroSeleccionado.value)
})

// Métodos
function openCreateModal() {
  isEditing.value = false
  editingId.value = null
  form.value = emptyForm()
  errors.value = { nombre: '' }
  showModal.value = true
}

function openEditModal(asignatura: Asignatura) {
  isEditing.value = true
  editingId.value = asignatura.id
  
  // Copiar datos al formulario
  const { id, ...rest } = asignatura
  form.value = { ...rest }
  
  errors.value = { nombre: '' }
  showModal.value = true
}

function closeModal() {
  showModal.value = false
  setTimeout(() => {
    form.value = emptyForm()
    isEditing.value = false
    editingId.value = null
    errors.value = { nombre: '' }
  }, 200)
}

async function handleSave() {
  // Validación
  errors.value = { nombre: '' }
  
  if (!form.value.nombre.trim()) {
    errors.value.nombre = 'El nombre es requerido'
    uiStore.showWarning('Por favor completa todos los campos requeridos')
    return
  }

  isSaving.value = true

  try {
    if (isEditing.value && editingId.value) {
      // Actualizar
      asignaturasStore.updateAsignatura(editingId.value, form.value)
      uiStore.showSuccess('Asignatura actualizada correctamente')
    } else {
      // Crear
      asignaturasStore.addAsignatura(form.value)
      uiStore.showSuccess('Asignatura creada correctamente')
    }

    closeModal()
  } catch (error) {
    console.error('Error guardando asignatura:', error)
    uiStore.showError('Error al guardar la asignatura')
  } finally {
    isSaving.value = false
  }
}

function confirmDelete(asignatura: Asignatura) {
  uiStore.openConfirm({
    title: 'Eliminar Asignatura',
    message: `¿Estás seguro de que deseas eliminar "${asignatura.nombre}"? Esta acción no se puede deshacer.`,
    confirmText: 'Sí, eliminar',
    cancelText: 'Cancelar',
    onConfirm: () => {
      asignaturasStore.deleteAsignatura(asignatura.id)
      uiStore.showSuccess('Asignatura eliminada correctamente')
    },
  })
}
</script>
