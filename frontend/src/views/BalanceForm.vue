<template>
  <div class="min-h-screen bg-gradient-to-br from-blue-50 to-blue-100">
    <!-- HEADER -->
    <div class="bg-gradient-to-r from-blue-600 via-blue-500 to-blue-400 shadow-lg px-8 py-6">
      <div class="flex items-center justify-between">
        <div class="flex items-center space-x-4">
          <div class="w-12 h-12 flex items-center justify-center bg-white/20 rounded-full shadow-lg">
            <svg width="28" height="28" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <rect x="3" y="3" width="18" height="18" rx="4" fill="url(#g)" />
              <defs><linearGradient id="g" x1="0" x2="1"><stop offset="0" stop-color="#4fa6ff"/><stop offset="1" stop-color="#0b5fff"/></linearGradient></defs>
            </svg>
          </div>
          <h1 class="text-2xl font-bold text-white tracking-wide drop-shadow">CiberBalance</h1>
        </div>

        <div class="text-sm text-blue-100 font-semibold">
          <router-link to="/dashboard" class="text-blue-200 hover:underline">Dashboard</router-link>
          <span class="mx-2">></span>
          <span>Balance de Carga</span>
        </div>
      </div>
    </div>

    <!-- CONTROL PANEL -->
    <div class="p-6 border-b border-blue-100 rounded-xl shadow-md mt-8 bg-white">
      <div class="flex flex-wrap gap-6 items-end justify-between">
        <div>
          <label class="block text-sm font-semibold text-gray-700 mb-2">Año Académico</label>
          <select
            v-model="academicYear"
            @input="markDirty"
            class="px-3 py-2 border border-gray-200 rounded-lg focus:ring-2 focus:ring-blue-400 focus:border-blue-400 outline-none text-base bg-white/80"
          >
            <option v-for="opt in yearOptions" :key="opt.value" :value="opt.value">
              {{ opt.label }}
            </option>
          </select>
        </div>

        <div>
          <label class="block text-base font-semibold text-blue-700 mb-2">Período</label>
          <select
            v-model="period"
            @input="markDirty"
            class="px-4 py-3 border border-blue-200 rounded-xl focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none text-lg"
          >
            <option value="1ero">1ero</option>
            <option value="2do">2do</option>
          </select>
        </div>

        <div>
          <label class="block text-base font-semibold text-blue-700 mb-2">Año Lectivo</label>
          <input
            v-model="academicYearText"
            @input="markDirty"
            type="text"
            class="px-4 py-3 border border-blue-200 rounded-xl focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none text-lg"
          />
        </div>

        <div>
          <label class="block text-base font-semibold text-blue-700 mb-2">Fecha Inicio</label>
          <input
            v-model="startDate"
            @input="markDirty"
            type="date"
            class="px-4 py-3 border border-blue-200 rounded-xl focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none text-lg"
          />
        </div>

        <div class="flex items-center gap-3">
          <button
            @click="calculateAll"
            class="px-4 py-2 bg-green-600 hover:bg-green-700 text-white rounded-lg text-sm font-semibold shadow transition"
          >
            Calcular
          </button>
          <button @click="saveProgress" class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg text-sm font-semibold shadow transition">Guardar</button>
        </div>
      </div>
    </div>

    <!-- MAIN TABLE -->
    <div class="p-10">
      <div class="mb-10 text-center">
        <div class="text-2xl font-extrabold text-blue-700 drop-shadow">FACULTAD DE CIBERSEGURIDAD</div>
        <div class="text-3xl font-bold text-blue-900 mt-2">Balance de Carga Docente</div>
        <div class="text-lg text-blue-500 mt-2">
          Carrera Ingeniería en Ciberseguridad | Curso: <span class="font-bold">{{ academicYearText }}</span> | Período: <span class="font-bold">{{ period }}</span>
        </div>
      </div>

      <div class="overflow-x-auto border border-gray-200 rounded-xl shadow-md bg-white">
        <table class="min-w-full divide-y divide-gray-100">
    <thead class="bg-gray-50">
            <tr class="text-gray-700">
              <th rowspan="3" class="px-4 py-3 text-left text-sm font-medium text-gray-700 w-56">Asignaturas</th>
              <!-- Semanas 1-15 -->
              <th v-for="week in 15" :key="`w${week}`" colspan="4" class="px-2 py-2 text-center text-xs font-medium text-gray-600 border-l border-gray-200">
                Semana {{ week }}
              </th>
              <th colspan="4" class="px-2 py-2 text-center text-xs font-medium text-gray-600 border-l border-gray-200">SEMANA DE CONSULTAS</th>
              <th colspan="5" class="px-2 py-2 text-center text-xs font-medium text-gray-600 border-l border-gray-200">EXÁMENES FINALES</th>
              <th colspan="5" class="px-2 py-2 text-center text-xs font-medium text-gray-600 border-l border-gray-200">EXÁMENES EXTRA I</th>
              <th colspan="5" class="px-2 py-2 text-center text-xs font-medium text-gray-600 border-l border-gray-200">EXÁMENES EXTRA II</th>
            </tr>
            <tr>
              <template v-for="week in 15" :key="`d${week}`">
                <td v-for="day in 4" :key="`d${week}-${day}`" class="px-2 py-1 text-center text-xs text-gray-500 border-l border-gray-300">&nbsp;</td>
              </template>
              <td v-for="i in 4" :key="'sc-' + i" class="px-2 py-1 text-center text-xs text-gray-500 border-l border-gray-300">&nbsp;</td>
              <td v-for="i in 5" :key="'ef-' + i" class="px-2 py-1 text-center text-xs text-gray-500 border-l border-gray-300">&nbsp;</td>
              <td v-for="i in 5" :key="'ex1-' + i" class="px-2 py-1 text-center text-xs text-gray-500 border-l border-gray-300">&nbsp;</td>
              <td v-for="i in 5" :key="'ex2-' + i" class="px-2 py-1 text-center text-xs text-gray-500 border-l border-gray-300">&nbsp;</td>
            </tr>
            <tr>
              <template v-for="week in 15" :key="`day${week}`">
                <td v-for="day in 4" :key="`day${week}-${day}`" class="px-2 py-1 text-center text-xs font-medium text-gray-700 border-l border-gray-300">
                  {{ day }}
                </td>
              </template>
              <td v-for="i in 4" :key="'scd-' + i" class="px-2 py-1 text-center text-xs font-medium text-gray-700 border-l border-gray-300">Día</td>
              <td v-for="i in 5" :key="'efd-' + i" class="px-2 py-1 text-center text-xs font-medium text-gray-700 border-l border-gray-300">Día</td>
              <td v-for="i in 5" :key="'ex1d-' + i" class="px-2 py-1 text-center text-xs font-medium text-gray-700 border-l border-gray-300">Día</td>
              <td v-for="i in 5" :key="'ex2d-' + i" class="px-2 py-1 text-center text-xs font-medium text-gray-700 border-l border-gray-300">Día</td>
            </tr>
          </thead>
    <tbody class="divide-y divide-gray-100">
            <tr v-for="subject in subjects" :key="subject.id" class="hover:bg-gray-50 transition">
              <td class="px-4 py-3 font-semibold text-gray-900">{{ subject.name }}</td>
              <td v-for="(_, cellIndex) in subject.values" :key="cellIndex" class="px-2 py-1 border-l border-gray-100">
                <input
                  type="number"
                  v-model.number="subject.values[cellIndex]"
                  min="0"
                  @input="markDirty"
                  class="w-12 text-center border border-gray-200 rounded-md px-2 py-1 focus:ring-1 focus:ring-blue-400 focus:border-blue-400 outline-none text-sm bg-white"
                />
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- CALCULATIONS TABLE -->
    <div class="px-10 pb-10">
      <div class="overflow-x-auto border border-blue-100 rounded-xl shadow-lg bg-white">
        <table class="min-w-full divide-y divide-blue-100">
          <thead class="bg-blue-50">
            <tr class="text-blue-700">
              <th class="px-6 py-4 text-left text-base font-bold">Asignaturas</th>
              <th class="px-6 py-4 text-center text-base font-bold">Total</th>
              <th class="px-6 py-4 text-center text-base font-bold">C</th>
              <th class="px-6 py-4 text-center text-base font-bold">CP</th>
              <th class="px-6 py-4 text-center text-base font-bold">S</th>
              <th class="px-6 py-4 text-center text-base font-bold">PL</th>
              <th class="px-6 py-4 text-center text-base font-bold">TE</th>
              <th class="px-6 py-4 text-center text-base font-bold">T</th>
              <th class="px-6 py-4 text-center text-base font-bold">PP</th>
              <th class="px-6 py-4 text-center text-base font-bold">Coef.</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-blue-50">
            <tr v-for="calc in calculations" :key="calc.subjectId" class="hover:bg-blue-50 transition">
              <td class="px-6 py-4 text-blue-900 font-semibold">{{ calc.subjectName }}</td>
              <td class="px-6 py-4 text-center text-blue-800">{{ calc.total }}</td>
              <td class="px-6 py-4 text-center text-blue-800">{{ calc.C }}</td>
              <td class="px-6 py-4 text-center text-blue-800">{{ calc.CP }}</td>
              <td class="px-6 py-4 text-center text-blue-800">{{ calc.S }}</td>
              <td class="px-6 py-4 text-center text-blue-800">{{ calc.PL }}</td>
              <td class="px-6 py-4 text-center text-blue-800">{{ calc.TE }}</td>
              <td class="px-6 py-4 text-center text-blue-800">{{ calc.T }}</td>
              <td class="px-6 py-4 text-center text-blue-800">{{ calc.PP }}</td>
              <td class="px-6 py-4 text-center text-blue-800">{{ calc.coef }}</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- ACTIONS PANEL -->
    <div class="p-6 border-t border-gray-200 rounded-xl shadow-2xl flex flex-wrap gap-4 mt-8 bg-white">
        <button
          @click="addSubject"
          class="flex items-center gap-3 px-6 py-3 rounded-full text-white font-semibold shadow-2xl transform transition hover:-translate-y-1 hover:shadow-3xl bg-gradient-to-r from-green-600 to-green-700 border border-green-700"
        >
          <svg class="w-5 h-5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M12 5v14M5 12h14" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></svg>
          <span>Agregar Asignatura</span>
        </button>
        <button
          @click="showDeleteModal = true;"
          class="flex items-center gap-3 px-6 py-3 rounded-full text-white font-semibold shadow-2xl transform transition hover:-translate-y-1 hover:shadow-3xl bg-gradient-to-r from-yellow-600 to-yellow-700 border border-yellow-700"
        >
          <svg class="w-5 h-5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M3 6h18M8 6v12a2 2 0 002 2h4a2 2 0 002-2V6" stroke="white" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"/><path d="M10 11v6M14 11v6" stroke="white" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"/></svg>
          <span>Eliminar Asignatura</span>
        </button>
        <button
          @click="resetAll"
          class="flex items-center gap-3 px-6 py-3 rounded-full text-white font-semibold shadow-2xl transform transition hover:-translate-y-1 hover:shadow-3xl bg-gradient-to-r from-red-600 to-red-700 border border-red-700"
        >
          <svg class="w-5 h-5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M3 6h18M8 6v12a2 2 0 002 2h4a2 2 0 002-2V6" stroke="white" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"/><path d="M10 11v6M14 11v6" stroke="white" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"/></svg>
          <span>Reiniciar</span>
        </button>
    </div>
    
  <!-- Modal de eliminación -->
    <div v-if="showDeleteModal" class="fixed inset-0 z-50 flex items-center justify-center bg-black/40">
      <div class="bg-white rounded-lg p-6 w-96">
        <h3 class="text-lg font-semibold mb-4">Eliminar Asignatura</h3>
        <div v-if="subjects.length === 0" class="text-gray-600">No hay asignaturas en el balance.</div>
        <ul class="space-y-2 max-h-48 overflow-auto">
          <li v-for="(s, idx) in subjects" :key="s.id" class="flex justify-between items-center p-2 border rounded">
            <div>
              <div class="font-semibold">{{ s.name }}</div>
            </div>
            <div>
              <button @click="confirmDelete(idx)" class="px-3 py-1 bg-red-500 text-white rounded">Eliminar</button>
            </div>
          </li>
        </ul>
        <div class="mt-4 text-right">
          <button @click="showDeleteModal = false" class="px-3 py-1 bg-gray-200 rounded">Cerrar</button>
        </div>
      </div>
    </div>
    
  <!-- Modal selector para asignaturas guardadas -->
    <div v-if="showSelectModal" class="fixed inset-0 z-50 flex items-center justify-center bg-black/40">
      <div class="bg-white rounded-lg p-6 w-96">
        <h3 class="text-lg font-semibold mb-4">Seleccionar Asignatura</h3>
        <div v-if="savedAsignaturas.length === 0" class="text-gray-600">No hay asignaturas guardadas. Ve a Gestión de Asignaturas para crear.</div>
        <ul class="space-y-2 max-h-64 overflow-auto">
          <li v-for="a in savedAsignaturas" :key="a.id" class="flex justify-between items-center p-2 border rounded">
            <div>
              <div class="font-semibold">{{ a.nombre }}</div>
              <div class="text-sm text-gray-500">{{ a.periodo }}·{{ a.semanas }} semanas</div>
            </div>
            <div>
              <button @click="addAsignaturaToBalance(a)" class="px-3 py-1 bg-blue-600 text-white rounded">Agregar</button>
            </div>
          </li>
        </ul>
        <div class="mt-4 text-right">
          <button @click="showSelectModal = false" class="px-3 py-1 bg-gray-200 rounded">Cerrar</button>
        </div>
      </div>
    </div>
  </div>
  <!-- Componentes reutilizables ConfirmModal y Toast -->
  <ConfirmModal :show="showConfirm" :message="confirmState.message" @confirm="onConfirm" @cancel="onCancel" />
  <Toast :show="toastState.show" :message="toastState.message" :type="toastState.type" />
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import ConfirmModal from '../components/ConfirmModal.vue'
import Toast from '../components/Toast.vue'
import { yearOptions } from '../utils/constants'
import { useRouter, useRoute } from 'vue-router'

// Tipos
interface Subject {
  id: string
  name: string
  values: number[]
}

interface Calculation {
  subjectId: string
  subjectName: string
  total: number
  C: number
  CP: number
  S: number
  PL: number
  TE: number
  T: number
  PP: number
  coef: number
}

// Router
const router = useRouter()

// Estado reactivo
const academicYear = ref<string>('1ro')
const period = ref<string>('1ero')
const academicYearText = ref<string>('2025-2026')
const startDate = ref<string>('2025-09-08')
const subjects = ref<Subject[]>([])
const calculations = ref<Calculation[]>([])
const isDirty = ref(false)
// UI state for modal/toast
const confirmState = ref<{ message: string; onConfirm: (() => void) | null }>({ message: '', onConfirm: null })
const showConfirm = ref(false)
const toastState = ref<{ show: boolean; message: string; type: 'success' | 'error' | 'info' }>({ show: false, message: '', type: 'info' })

function openConfirm(message: string, onConfirm: () => void) {
  confirmState.value = { message, onConfirm }
  showConfirm.value = true
}

function onConfirm() {
  showConfirm.value = false
  if (confirmState.value.onConfirm) confirmState.value.onConfirm()
}

function onCancel() {
  showConfirm.value = false
  confirmState.value = { message: '', onConfirm: null }
}

function showToast(message: string, type: 'success' | 'error' | 'info' = 'info') {
  toastState.value = { show: true, message, type }
  setTimeout(() => (toastState.value.show = false), 1600)
}

// yearOptions importado desde constantes compartidas

function markDirty() {
  isDirty.value = true
}

// Métodos
function attemptGoBack() {
  if (isDirty.value) {
    if (!confirm('Hay cambios sin guardar. ¿Deseas salir?')) return
  }
  router.back()
}

const handleEsc = (e: KeyboardEvent) => {
  if (e.key === 'Escape') attemptGoBack()
}

// Modal + selection of saved asignaturas
const showSelectModal = ref(false)
const savedAsignaturas = ref<any[]>([])

function loadSavedAsignaturas() {
  try {
    const raw = localStorage.getItem('asignaturas')
    if (!raw) {
      savedAsignaturas.value = []
      return
    }
    savedAsignaturas.value = JSON.parse(raw)
  } catch (e) {
    console.error('Error loading saved asignaturas', e)
    savedAsignaturas.value = []
  }
}

const addSubject = () => {
  // Abrir el selector con las asignaturas guardadas
  loadSavedAsignaturas()
  showSelectModal.value = true
  isDirty.value = true
}

const addAsignaturaToBalance = (a: any) => {
  // Mapear la asignatura guardada a una fila del balance
  subjects.value.push({ id: a.id, name: a.nombre || a.name || 'Asignatura', values: Array(79).fill(0) })
  showSelectModal.value = false
  isDirty.value = true
}

// deletion modal state
const showDeleteModal = ref(false)

function confirmDelete(index: number) {
  if (index < 0 || index >= subjects.value.length) return
  const s = subjects.value[index]
  if (!s) return
  openConfirm(`Eliminar "${s.name}"?`, () => {
    subjects.value.splice(index, 1)
    showDeleteModal.value = false
    isDirty.value = true
  })
}


const calculateAll = () => {
  calculations.value = subjects.value.map((subj) => {
    const total = subj.values.reduce((sum, val) => sum + val, 0)
    return {
      subjectId: subj.id,
      subjectName: subj.name,
      total,
      C: 0,
      CP: 0,
      S: 0,
      PL: 0,
      TE: 0,
      T: 0,
      PP: 0,
      coef: parseFloat((total * 1.2).toFixed(2)),
    }
  })
}

const resetAll = () => {
  openConfirm('¿Reiniciar todo?', () => {
    subjects.value = []
    calculations.value = []
    academicYearText.value = '2025-2026'
    period.value = '1ero'
    academicYear.value = '1ro'
    startDate.value = '2025-09-08'
    isDirty.value = true
  })
}


const route = useRoute()

const saveProgress = () => {
  // determinar id: usar query id si se está editando un balance existente, si no crear uno nuevo
  const existingId = (route.query.id as string) || null
  const id = existingId || Date.now().toString()

  const dataToSave = {
    id,
    subjects: subjects.value,
    academicYear: academicYear.value,
    period: period.value,
    academicYearText: academicYearText.value,
    startDate: startDate.value,
    savedAt: new Date().toISOString(),
  }

  // Save full balance under a dedicated key
  try {
    localStorage.setItem(`balance_${id}`, JSON.stringify(dataToSave))
  } catch (e) {
    console.error('Error saving balance', e)
    showToast('Error al guardar', 'error')
    return
  }

  // Update recent balances list
  try {
    const raw = localStorage.getItem('recentBalances')
    const recent = raw ? JSON.parse(raw) : []
    // remove existing with same id if present
    const filtered = recent.filter((r: any) => r.id !== id)
    const summary = {
      id,
      year: academicYearText.value,
      period: period.value,
      date: new Date().toLocaleString(),
    }
    filtered.unshift(summary)
    // keep max 10
    localStorage.setItem('recentBalances', JSON.stringify(filtered.slice(0, 10)))
  } catch (e) {
    console.error('Error updating recent balances', e)
  }

  // also keep a working snapshot for quick resume
  localStorage.setItem('balanceData', JSON.stringify(dataToSave))
  showToast('Progreso guardado', 'success')
  isDirty.value = false
  // wait a bit so toast is visible, then navigate back
  setTimeout(() => router.push('/dashboard'), 700)
}

// Inicialización
onMounted(() => {
  // Si la query de la ruta tiene id, cargar ese balance guardado para edición
  const idFromQuery = (route.query.id as string) || null
  if (idFromQuery) {
    try {
      const raw = localStorage.getItem(`balance_${idFromQuery}`)
      if (raw) {
        const data = JSON.parse(raw)
        subjects.value = data.subjects || []
        academicYear.value = data.academicYear || '1ro'
        period.value = data.period || '1ero'
        academicYearText.value = data.academicYearText || '2025-2026'
        startDate.value = data.startDate || '2025-09-08'
      }
    } catch (e) {
      console.error('Error al cargar balance por id', e)
    }
    return
  }

  // otherwise load working snapshot (unsaved progress)
  const saved = localStorage.getItem('balanceData')
  if (saved) {
    try {
      const data = JSON.parse(saved)
      subjects.value = data.subjects || []
      academicYear.value = data.academicYear || '1ro'
      period.value = data.period || '1ero'
      academicYearText.value = data.academicYearText || '2025-2026'
      startDate.value = data.startDate || '2025-09-08'
    } catch (e) {
      console.error('Error al cargar datos guardados', e)
    }
  }
  
  // Agregar listener para ESC
  window.addEventListener('keydown', handleEsc)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleEsc)
})
</script>