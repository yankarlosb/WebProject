<template>
  <div class="min-h-screen bg-gradient-to-br from-blue-50 to-blue-100 p-6">
    <div class="max-w-4xl mx-auto">
      <!-- Encabezado -->
      <div class="mb-8 text-center">
        <h1 class="text-3xl font-extrabold text-blue-700">Gestión de Asignaturas</h1>
        <p class="text-blue-600 mt-2 font-medium">Administra las asignaturas del sistema</p>
      </div>

      <!-- Lista de asignaturas y formulario combinado -->
      <div class="bg-white p-6 rounded-xl shadow-lg border border-blue-100">
        <div class="flex items-center justify-between mb-4">
          <h2 class="text-xl font-bold text-blue-700">Asignaturas</h2>
          <button
            type="button"
            @click="startAdd()"
            class="px-4 py-2 bg-blue-600 text-white rounded-lg text-sm font-semibold hover:bg-blue-700 shadow transition"
          >
            + Nueva Asignatura
          </button>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
          <div class="md:col-span-2">
            <div v-if="asignaturas.length === 0" class="text-gray-600 p-4">No hay asignaturas aún. Agrega una para comenzar.</div>
            <ul class="space-y-3">
              <li v-for="a in asignaturas" :key="a.id" class="p-3 rounded-lg border border-blue-100 bg-white shadow-sm flex items-center justify-between">
                <div>
                  <div class="font-semibold text-blue-800">{{ a.nombre }}</div>
                  <div class="text-sm text-gray-500">{{ a.periodo }}·{{ a.semanas }} semanas</div>
                </div>
                <div class="flex items-center gap-2">
                  <button @click="editAsignatura(a.id)" class="px-3 py-1 bg-yellow-500 text-white rounded hover:bg-yellow-600">Editar</button>
                  <button @click="removeAsignatura(a.id)" class="px-3 py-1 bg-red-500 text-white rounded hover:bg-red-600">Eliminar</button>
                </div>
              </li>
            </ul>
          </div>

          <!-- Formulario lateral -->
          <div class="md:col-span-1">
            <div class="p-4 bg-base-100 rounded-lg border border-blue-100">
              <h3 class="text-lg font-semibold text-blue-700 mb-3">{{ isEditing ? 'Editar' : 'Nueva' }} Asignatura</h3>
              <form @submit.prevent="guardar">
                <div class="mb-3">
                  <label class="block text-sm font-medium text-gray-700 mb-1">Asignatura</label>
                  <input v-model="formData.nombre" @input="markDirty" type="text" class="w-full px-3 py-2 border rounded" />
                </div>
                <div class="mb-3">
                  <label class="block text-sm font-medium text-gray-700 mb-1">Periodo</label>
                  <select v-model="formData.periodo" @change="markDirty" class="w-full px-3 py-2 border rounded">
                    <option value="Periodo 1">Periodo 1</option>
                    <option value="Periodo 2">Periodo 2</option>
                  </select>
                </div>
                <div class="mb-3">
                  <label class="block text-sm font-medium text-gray-700 mb-1">Semanas</label>
                  <select v-model="formData.semanas" @change="markDirty" class="w-full px-3 py-2 border rounded">
                    <option value="15">15</option>
                    <option value="16">16</option>
                  </select>
                </div>
                <div class="mb-3">
                  <label class="block text-sm font-medium text-gray-700 mb-1">Encuentros</label>
                  <input v-model.number="formData.encuentros" @input="markDirty" type="number" min="0" class="w-full px-3 py-2 border rounded" />
                </div>

                <div class="grid grid-cols-3 gap-2 mb-3">
                  <div v-for="tipo in tiposActividad" :key="tipo.id" class="text-center">
                    <label class="block text-sm font-medium text-gray-700">{{ tipo.label }}</label>
                    <input :value="getField(tipo.id)" @input="(e) => { onFieldInput(tipo.id, e); markDirty(); }" type="number" min="0" class="w-full px-2 py-1 border rounded text-center" />
                  </div>
                </div>

                <div class="flex gap-2">
                  <button type="submit" class="px-3 py-2 bg-blue-600 text-white rounded">Guardar</button>
                  <button type="button" @click="cancelar" class="px-3 py-2 bg-gray-200 rounded">Cancelar</button>
                </div>
              </form>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
  <ConfirmModal :show="confirmModal.show" :message="confirmModal.message" @confirm="confirmModalConfirm" @cancel="confirmModalCancel" />
  <Toast :show="toast.show" :message="toast.message" :type="toast.type" />
</template>

<script lang="ts" setup>
import { ref, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import ConfirmModal from '../components/ConfirmModal.vue'
import Toast from '../components/Toast.vue'
import { tiposActividad } from '../utils/constants'

// Tipos
type ActivityCounts = {
  c: number | null
  cp: number | null
  pl: number | null
  t: number | null
  te: number | null
  pp: number | null
  ef: number | null
}

type Asignatura = ActivityCounts & {
  id: string
  nombre: string
  periodo: string
  fechaInicio?: string
  fechaFin?: string
  semanas: string
  encuentros: number | null
}

// FormData
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

const formData = ref<Omit<Asignatura, 'id'>>(emptyForm())

// UI helpers: confirm modal and toast
const confirmModal = ref<{ show: boolean; message: string; onConfirm: (() => void) | null }>({ show: false, message: '', onConfirm: null })
const toast = ref<{ show: boolean; message: string; type: 'success' | 'error' | 'info' }>({ show: false, message: '', type: 'info' })

function openConfirm(message: string, onConfirm: () => void) {
  confirmModal.value = { show: true, message, onConfirm }
}

function confirmModalConfirm() {
  if (confirmModal.value.onConfirm) confirmModal.value.onConfirm()
  confirmModal.value = { show: false, message: '', onConfirm: null }
}

function confirmModalCancel() {
  confirmModal.value = { show: false, message: '', onConfirm: null }
}

function showToast(message: string, type: 'success' | 'error' | 'info' = 'info') {
  toast.value = { show: true, message, type }
  setTimeout(() => (toast.value.show = false), 1600)
}

// Router
const router = useRouter()

// Lista de asignaturas y estado
const asignaturas = ref<Asignatura[]>([])
const isEditing = ref(false)
const editingId = ref<string | null>(null)
const isDirty = ref(false)

function markDirty() {
  isDirty.value = true
}

const STORAGE_KEY = 'asignaturas'

function loadAsignaturas() {
  try {
    const raw = localStorage.getItem(STORAGE_KEY)
    if (raw) asignaturas.value = JSON.parse(raw)
  } catch (e) {
    console.error('Error loading asignaturas', e)
    asignaturas.value = []
  }
}

function saveAsignaturas() {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(asignaturas.value))
}

function startAdd() {
  isEditing.value = false
  editingId.value = null
  formData.value = emptyForm()
  markDirty()
}

function editAsignatura(id: string) {
  const found = asignaturas.value.find((x) => x.id === id)
  if (!found) return
  isEditing.value = true
  editingId.value = id
  // copy data to the form (omit id)
  const { id: _id, ...rest } = found
  formData.value = { ...rest }
  markDirty()
}

function removeAsignatura(id: string) {
  openConfirm('¿Eliminar asignatura? Esta acción no se puede deshacer.', () => {
    asignaturas.value = asignaturas.value.filter((x) => x.id !== id)
    saveAsignaturas()
    showToast('Asignatura eliminada', 'success')
    markDirty()
  })
}

function cancelar() {
  startAdd()
}

function guardar() {
  if (!formData.value.nombre || formData.value.nombre.trim() === '') {
    alert('El nombre de la asignatura es obligatorio')
    return
  }

  if (isEditing.value && editingId.value) {
    const idx = asignaturas.value.findIndex((x) => x.id === editingId.value)
    if (idx !== -1) {
      asignaturas.value[idx] = { id: editingId.value, ...(formData.value as any) }
    }
  } else {
    const newAsignatura: Asignatura = { id: Date.now().toString(), ...(formData.value as any) }
    asignaturas.value.unshift(newAsignatura)
  }

  saveAsignaturas()
  startAdd()
  showToast(isEditing.value ? 'Asignatura actualizada' : 'Asignatura agregada', 'success')
}

function goBack() {
  if (isDirty.value) {
    if (!confirm('Hay cambios sin guardar. ¿Deseas salir?')) return
  }
  router.back()
}

const handleEsc = (e: KeyboardEvent) => {
  if (e.key === 'Escape') goBack()
}

onMounted(() => {
  loadAsignaturas()
  window.addEventListener('keydown', handleEsc)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleEsc)
})

// Helpers to safely access form fields by string key (TypeScript-friendly)
function getField(key: string): number | string | null {
  const v: any = (formData.value as any)[key]
  return v == null ? '' : v
}

function onFieldInput(key: string, e: Event) {
  const target = e.target as HTMLInputElement
  const raw = target.value
  // store as number if numeric, else string
  if (raw === '') {
    ;(formData.value as any)[key] = null
  } else {
    const n = Number(raw)
    ;(formData.value as any)[key] = isNaN(n) ? raw : n
  }
}
</script>