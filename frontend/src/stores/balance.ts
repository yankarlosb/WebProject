/**
 * Store de Balances
 * Maneja el estado global de los balances de carga docente
 */

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export interface BalanceSubject {
  id: string
  name: string
  values: number[] // 79 valores (15 semanas × 4 días + semana consultas + exámenes)
}

export interface Balance {
  id: string
  academicYear: string // '1ro', '2do', '3ro', '4to'
  period: string // '1ero', '2do'
  academicYearText: string // '2025-2026'
  startDate: string
  subjects: BalanceSubject[]
  savedAt: string
  updatedAt?: string
}

export interface BalanceCalculation {
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

const STORAGE_KEY_PREFIX = 'balance_'
const RECENT_BALANCES_KEY = 'recentBalances'

export const useBalanceStore = defineStore('balance', () => {
  // State
  const currentBalance = ref<Balance | null>(null)
  const calculations = ref<BalanceCalculation[]>([])
  const isDirty = ref(false) // Indica si hay cambios sin guardar
  const isLoading = ref(false)

  // Getters
  const hasUnsavedChanges = computed(() => isDirty.value)
  const hasSubjects = computed(() => (currentBalance.value?.subjects.length || 0) > 0)

  // Actions

  /**
   * Crear un nuevo balance vacío
   */
  function createNewBalance(academicYear: string = '1ro', period: string = '1ero') {
    const today = new Date().toISOString().split('T')
    currentBalance.value = {
      id: Date.now().toString(),
      academicYear,
      period,
      academicYearText: '2025-2026',
      startDate: today[0] || '',
      subjects: [],
      savedAt: new Date().toISOString(),
    }
    calculations.value = []
    isDirty.value = false
  }

  /**
   * Cargar un balance existente por ID
   */
  function loadBalance(id: string): boolean {
    try {
      const raw = localStorage.getItem(`${STORAGE_KEY_PREFIX}${id}`)
      if (!raw) return false

      const balance = JSON.parse(raw) as Balance
      currentBalance.value = balance
      isDirty.value = false
      
      // Recalcular automáticamente
      calculateAll()
      
      return true
    } catch (error) {
      console.error('Error cargando balance:', error)
      return false
    }
  }

  /**
   * Guardar el balance actual
   */
  function saveBalance(): boolean {
    if (!currentBalance.value) return false

    try {
      const balance = {
        ...currentBalance.value,
        updatedAt: new Date().toISOString(),
      }

      // Guardar el balance completo
      localStorage.setItem(
        `${STORAGE_KEY_PREFIX}${balance.id}`,
        JSON.stringify(balance)
      )

      // Actualizar lista de recientes
      updateRecentBalances(balance)

      currentBalance.value = balance
      isDirty.value = false

      return true
    } catch (error) {
      console.error('Error guardando balance:', error)
      return false
    }
  }

  /**
   * Actualizar lista de balances recientes
   */
  function updateRecentBalances(balance: Balance) {
    try {
      const raw = localStorage.getItem(RECENT_BALANCES_KEY)
      const recent = raw ? JSON.parse(raw) : []

      // Remover si ya existe
      const filtered = recent.filter((r: any) => r.id !== balance.id)

      // Agregar al inicio
      const summary = {
        id: balance.id,
        year: balance.academicYearText,
        period: balance.period,
        date: new Date(balance.updatedAt || balance.savedAt).toLocaleString(),
      }

      filtered.unshift(summary)

      // Mantener máximo 10
      localStorage.setItem(
        RECENT_BALANCES_KEY,
        JSON.stringify(filtered.slice(0, 10))
      )
    } catch (error) {
      console.error('Error actualizando recientes:', error)
    }
  }

  /**
   * Obtener lista de balances recientes
   */
  function getRecentBalances(): any[] {
    try {
      const raw = localStorage.getItem(RECENT_BALANCES_KEY)
      return raw ? JSON.parse(raw) : []
    } catch (error) {
      console.error('Error obteniendo recientes:', error)
      return []
    }
  }

  /**
   * Agregar una asignatura al balance
   */
  function addSubject(subjectName: string) {
    if (!currentBalance.value) return

    const newSubject: BalanceSubject = {
      id: Date.now().toString(),
      name: subjectName,
      values: Array(79).fill(0), // 15 semanas × 4 días + extras
    }

    currentBalance.value.subjects.push(newSubject)
    isDirty.value = true
  }

  /**
   * Eliminar una asignatura del balance
   */
  function removeSubject(subjectId: string) {
    if (!currentBalance.value) return

    const index = currentBalance.value.subjects.findIndex(s => s.id === subjectId)
    if (index !== -1) {
      currentBalance.value.subjects.splice(index, 1)
      isDirty.value = true
    }
  }

  /**
   * Actualizar un valor en la tabla del balance
   */
  function updateSubjectValue(subjectId: string, cellIndex: number, value: number) {
    if (!currentBalance.value) return

    const subject = currentBalance.value.subjects.find(s => s.id === subjectId)
    if (subject && cellIndex >= 0 && cellIndex < subject.values.length) {
      subject.values[cellIndex] = value
      isDirty.value = true
    }
  }

  /**
   * Actualizar metadatos del balance
   */
  function updateBalanceMetadata(updates: Partial<Omit<Balance, 'id' | 'subjects'>>) {
    if (!currentBalance.value) return

    currentBalance.value = {
      ...currentBalance.value,
      ...updates,
    }
    isDirty.value = true
  }

  /**
   * Calcular totales y coeficientes
   */
  function calculateAll() {
    if (!currentBalance.value) return

    calculations.value = currentBalance.value.subjects.map(subject => {
      const total = subject.values.reduce((sum, val) => sum + val, 0)
      
      return {
        subjectId: subject.id,
        subjectName: subject.name,
        total,
        C: 0, // TODO: Implementar lógica de cálculo según reglas
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

  /**
   * Reiniciar el balance actual
   */
  function resetBalance() {
    currentBalance.value = null
    calculations.value = []
    isDirty.value = false
  }

  /**
   * Eliminar un balance guardado
   */
  function deleteBalance(id: string): boolean {
    try {
      // Eliminar del storage
      localStorage.removeItem(`${STORAGE_KEY_PREFIX}${id}`)

      // Actualizar recientes
      const raw = localStorage.getItem(RECENT_BALANCES_KEY)
      if (raw) {
        const recent = JSON.parse(raw)
        const filtered = recent.filter((r: any) => r.id !== id)
        localStorage.setItem(RECENT_BALANCES_KEY, JSON.stringify(filtered))
      }

      // Si es el balance actual, resetearlo
      if (currentBalance.value?.id === id) {
        resetBalance()
      }

      return true
    } catch (error) {
      console.error('Error eliminando balance:', error)
      return false
    }
  }

  /**
   * Marcar como sin cambios
   */
  function markAsSaved() {
    isDirty.value = false
  }

  return {
    // State
    currentBalance,
    calculations,
    isDirty,
    isLoading,

    // Getters
    hasUnsavedChanges,
    hasSubjects,

    // Actions
    createNewBalance,
    loadBalance,
    saveBalance,
    getRecentBalances,
    addSubject,
    removeSubject,
    updateSubjectValue,
    updateBalanceMetadata,
    calculateAll,
    resetBalance,
    deleteBalance,
    markAsSaved,
  }
})
