/**
 * Store de Balances
 * Maneja el estado global de los balances de carga docente
 * Sincronizado con el backend via API
 */

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { balancesService, type Balance, type BalanceSubject, type BalanceRequest } from '../services/balances'

// Re-exportar tipos para uso externo
export type { Balance, BalanceSubject, BalanceRequest }

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

// Tipo interno para el balance en edición (puede no tener ID aún)
export interface EditableBalance {
  id?: number                // undefined si es nuevo
  academic_year: string      // '1ro', '2do', '3ro', '4to'
  period: string             // '1ero', '2do'
  academic_year_text: string // '2025-2026'
  start_date: string         // 'YYYY-MM-DD'
  weeks: number
  subjects: BalanceSubject[]
}

// Module-level constant for valid activity types - avoids recreating Set on each calculation
const VALID_ACTIVITY_TYPES = new Set(['C', 'CP', 'S', 'PL', 'TE', 'T', 'PP'])

export const useBalanceStore = defineStore('balance', () => {
  // ============================================================================
  // STATE
  // ============================================================================
  
  /** Lista de todos los balances del usuario (desde la API) */
  const balances = ref<Balance[]>([])
  
  /** Balance actualmente en edición */
  const currentBalance = ref<EditableBalance | null>(null)
  
  /** Cálculos del balance actual */
  const calculations = ref<BalanceCalculation[]>([])
  
  /** Indica si hay cambios sin guardar */
  const isDirty = ref(false)
  
  /** Estado de carga */
  const isLoading = ref(false)
  
  /** Error de la última operación */
  const error = ref<string | null>(null)

  // ============================================================================
  // GETTERS
  // ============================================================================
  
  const hasUnsavedChanges = computed(() => isDirty.value)
  const hasSubjects = computed(() => (currentBalance.value?.subjects.length || 0) > 0)
  const balancesCount = computed(() => balances.value.length)
  const isNewBalance = computed(() => currentBalance.value?.id === undefined)

  // ============================================================================
  // ACTIONS - API
  // ============================================================================

  /**
   * Cargar todos los balances del usuario desde la API
   */
  async function fetchBalances(): Promise<boolean> {
    isLoading.value = true
    error.value = null

    try {
      const response = await balancesService.list()
      if (response.success && response.data) {
        balances.value = response.data
        return true
      } else {
        error.value = response.message || 'Error al cargar balances'
        return false
      }
    } catch (e) {
      error.value = 'Error de conexión al cargar balances'
      console.error('Error fetching balances:', e)
      return false
    } finally {
      isLoading.value = false
    }
  }

  /**
   * Cargar un balance específico por ID desde la API
   */
  async function loadBalance(id: number): Promise<boolean> {
    isLoading.value = true
    error.value = null

    try {
      const response = await balancesService.get(id)
      if (response.success && response.data) {
        // Convertir Balance de API a EditableBalance
        currentBalance.value = {
          id: response.data.id,
          academic_year: response.data.academic_year,
          period: response.data.period,
          academic_year_text: response.data.academic_year_text,
          start_date: response.data.start_date,
          weeks: response.data.weeks,
          subjects: response.data.subjects,
        }
        isDirty.value = false
        calculateAll()
        return true
      } else {
        error.value = response.message || 'Balance no encontrado'
        return false
      }
    } catch (e) {
      error.value = 'Error de conexión al cargar el balance'
      console.error('Error loading balance:', e)
      return false
    } finally {
      isLoading.value = false
    }
  }

  /**
   * Guardar el balance actual (crear o actualizar)
   */
  async function saveBalance(): Promise<{ success: boolean; message: string }> {
    if (!currentBalance.value) {
      return { success: false, message: 'No hay balance para guardar' }
    }

    isLoading.value = true
    error.value = null

    try {
      const request: BalanceRequest = {
        academic_year: currentBalance.value.academic_year,
        period: currentBalance.value.period,
        academic_year_text: currentBalance.value.academic_year_text,
        start_date: currentBalance.value.start_date,
        weeks: currentBalance.value.weeks,
        subjects: currentBalance.value.subjects,
      }

      let response
      if (currentBalance.value.id !== undefined) {
        // Actualizar existente
        response = await balancesService.update(currentBalance.value.id, request)
      } else {
        // Crear nuevo
        response = await balancesService.create(request)
      }

      if (response.success && response.data) {
        // Actualizar el balance actual con los datos del servidor
        currentBalance.value = {
          id: response.data.id,
          academic_year: response.data.academic_year,
          period: response.data.period,
          academic_year_text: response.data.academic_year_text,
          start_date: response.data.start_date,
          weeks: response.data.weeks,
          subjects: response.data.subjects,
        }
        isDirty.value = false
        
        // Refrescar la lista de balances
        await fetchBalances()
        
        return { success: true, message: 'Balance guardado exitosamente' }
      } else {
        error.value = response.message || 'Error al guardar el balance'
        return { success: false, message: error.value }
      }
    } catch (e) {
      error.value = 'Error de conexión al guardar'
      console.error('Error saving balance:', e)
      return { success: false, message: error.value }
    } finally {
      isLoading.value = false
    }
  }

  /**
   * Eliminar un balance por ID
   */
  async function deleteBalance(id: number): Promise<{ success: boolean; message: string }> {
    isLoading.value = true
    error.value = null

    try {
      const response = await balancesService.delete(id)
      if (response.success) {
        // Remover de la lista local
        balances.value = balances.value.filter(b => b.id !== id)
        
        // Si es el balance actual, resetear
        if (currentBalance.value?.id === id) {
          resetBalance()
        }
        
        return { success: true, message: 'Balance eliminado exitosamente' }
      } else {
        error.value = response.message || 'Error al eliminar el balance'
        return { success: false, message: error.value }
      }
    } catch (e) {
      error.value = 'Error de conexión al eliminar'
      console.error('Error deleting balance:', e)
      return { success: false, message: error.value }
    } finally {
      isLoading.value = false
    }
  }

  // ============================================================================
  // ACTIONS - LOCALES (edición en memoria)
  // ============================================================================

  /**
   * Crear un nuevo balance vacío (en memoria, no guardado aún)
   */
  function createNewBalance(
    academicYear: string = '1ro',
    period: string = '1ero',
    weeks: number = 15
  ) {
    const today = new Date().toISOString().split('T')[0] || ''
    
    currentBalance.value = {
      // No tiene ID porque aún no está guardado
      academic_year: academicYear,
      period: period,
      academic_year_text: '2025-2026',
      start_date: today,
      weeks: weeks,
      subjects: [],
    }
    calculations.value = []
    isDirty.value = false
  }

  /**
   * Agregar una asignatura al balance actual
   */
  function addSubject(subjectName: string) {
    if (!currentBalance.value) return

    // Calcular número de celdas basado en semanas
    const weeks = currentBalance.value.weeks || 15
    const cellsCount = weeks * 4 + 9 // semanas * 4 días + consultas(4) + exámenes(5)

    const newSubject: BalanceSubject = {
      id: Date.now().toString(),
      name: subjectName,
      values: Array(cellsCount).fill(''),
    }

    currentBalance.value.subjects.push(newSubject)
    isDirty.value = true
  }

  /**
   * Eliminar una asignatura del balance actual
   * Optimized: uses filter for cleaner immutable update
   */
  function removeSubject(subjectId: string) {
    if (!currentBalance.value) return

    const originalLength = currentBalance.value.subjects.length
    currentBalance.value.subjects = currentBalance.value.subjects.filter(s => s.id !== subjectId)
    
    // Only mark as dirty if something was actually removed
    if (currentBalance.value.subjects.length < originalLength) {
      isDirty.value = true
    }
  }

  /**
   * Actualizar un valor en la tabla del balance
   */
  function updateSubjectValue(subjectId: string, cellIndex: number, value: string) {
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
  function updateBalanceMetadata(updates: Partial<Omit<EditableBalance, 'id' | 'subjects'>>) {
    if (!currentBalance.value) return

    currentBalance.value = {
      ...currentBalance.value,
      ...updates,
    }
    isDirty.value = true
  }

  /**
   * Calcular totales y coeficientes para cada asignatura
   * Optimized: uses module-level Set constant for faster lookups
   */
  function calculateAll() {
    if (!currentBalance.value) return

    calculations.value = currentBalance.value.subjects.map(subject => {
      // Initialize counts with explicit type
      const counts = {
        C: 0,
        CP: 0,
        S: 0,
        PL: 0,
        TE: 0,
        T: 0,
        PP: 0,
      }
      
      let total = 0
      
      // Single pass through values with optimized type checking
      // Uses module-level VALID_ACTIVITY_TYPES constant for O(1) lookup
      for (const val of subject.values) {
        if (typeof val === 'string' && VALID_ACTIVITY_TYPES.has(val)) {
          counts[val as keyof typeof counts]++
          total++
        }
      }
      
      return {
        subjectId: subject.id,
        subjectName: subject.name,
        total,
        C: counts.C,
        CP: counts.CP,
        S: counts.S,
        PL: counts.PL,
        TE: counts.TE,
        T: counts.T,
        PP: counts.PP,
        coef: Math.round(total * 1.2 * 100) / 100, // Faster than parseFloat/toFixed
      }
    })
  }

  /**
   * Reiniciar el balance actual (sin eliminar de la DB)
   */
  function resetBalance() {
    currentBalance.value = null
    calculations.value = []
    isDirty.value = false
    error.value = null
  }

  /**
   * Marcar como sin cambios
   */
  function markAsSaved() {
    isDirty.value = false
  }

  /**
   * Limpiar error
   */
  function clearError() {
    error.value = null
  }

  return {
    // State
    balances,
    currentBalance,
    calculations,
    isDirty,
    isLoading,
    error,

    // Getters
    hasUnsavedChanges,
    hasSubjects,
    balancesCount,
    isNewBalance,

    // Actions - API
    fetchBalances,
    loadBalance,
    saveBalance,
    deleteBalance,

    // Actions - Locales
    createNewBalance,
    addSubject,
    removeSubject,
    updateSubjectValue,
    updateBalanceMetadata,
    calculateAll,
    resetBalance,
    markAsSaved,
    clearError,
  }
})
