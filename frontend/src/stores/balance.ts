/**
 * Store de Balances
 * Maneja el estado global de los balances de carga docente
 * 
 * Nueva arquitectura con fragmentos:
 * - Leader crea balance, selecciona asignaturas → se crean fragmentos automáticamente
 * - SubjectLeaders llenan sus fragmentos correspondientes
 * - Leader puede ver el progreso de todos los fragmentos
 */

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import {
  balancesService,
  fragmentsService,
  type Balance,
  type BalanceListItem,
  type Fragment,
  type PendingFragment,
  type CreateBalanceRequest,
  type UpdateBalanceRequest,
  type UpdateFragmentRequest,
  type SelectedSubject,
} from '../services/balances'

// Re-exportar tipos para uso externo
export type {
  Balance,
  BalanceListItem,
  Fragment,
  PendingFragment,
  CreateBalanceRequest,
  UpdateBalanceRequest,
  UpdateFragmentRequest,
  SelectedSubject,
}

/** Cálculos de un fragmento */
export interface FragmentCalculation {
  fragmentId: number
  asignaturaName: string
  status: string
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

/** Período no académico */
export interface NonAcademicPeriod {
  start: string
  end: string
  name: string
}

/** Balance en edición (para crear uno nuevo) */
export interface EditableBalance {
  academic_year: string      // '1ro', '2do', '3ro', '4to'
  period: string             // '1ero', '2do'
  academic_year_text: string // '2025-2026'
  start_date: string         // 'YYYY-MM-DD'
  weeks: number
  deadline: string | null
  allow_leader_edit: boolean
  selectedAsignaturas: number[] // IDs de asignaturas seleccionadas
  non_academic_periods: NonAcademicPeriod[] // Períodos no académicos (vacaciones, etc.)
}

/** Fragmento en edición (para SubjectLeader) */
export interface EditableFragment {
  fragmentId: number
  balanceId: number
  balanceName: string
  asignaturaId: number
  asignaturaName: string
  asignaturaHours: number  // Horas planificadas de la asignatura
  // Cantidades planificadas por tipo de clase
  asignaturaPlan: {
    c: number | null
    cp: number | null
    s: number | null
    pl: number | null
    te: number | null
    t: number | null
    pp: number | null
    ec: number | null
    tc: number | null
    ef: number | null
  }
  weeks: number
  startDate: string  // Fecha de inicio del balance 'YYYY-MM-DD'
  nonAcademicPeriods: NonAcademicPeriod[] // Períodos no académicos del balance
  status: 'pending' | 'in_progress' | 'completed'
  deadline: string | null
  data: Record<string, unknown>
}

// Module-level constant for valid activity types
// C, CP, S, PL, TE, T, PP, EC
const VALID_ACTIVITY_TYPES = new Set(['C', 'CP', 'S', 'PL', 'TE', 'T', 'PP', 'EC'])

export const useBalanceStore = defineStore('balance', () => {
  // ============================================================================
  // STATE
  // ============================================================================
  
  /** Lista de balances (para Leader: todos, para SubjectLeader: donde tiene fragmentos) */
  const balances = ref<BalanceListItem[]>([])
  
  /** Balance actual con sus fragmentos (vista detallada) */
  const currentBalance = ref<Balance | null>(null)
  
  /** Balance en edición (para crear nuevo) */
  const editableBalance = ref<EditableBalance | null>(null)
  
  /** Fragmentos pendientes del usuario actual (para Dashboard SubjectLeader) */
  const pendingFragments = ref<PendingFragment[]>([])
  
  /** Fragmento actualmente en edición (SubjectLeader) */
  const currentFragment = ref<EditableFragment | null>(null)
  
  /** Cálculos del balance actual */
  const calculations = ref<FragmentCalculation[]>([])
  
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
  const balancesCount = computed(() => balances.value.length)
  const pendingCount = computed(() => pendingFragments.value.length)
  const isCreatingNew = computed(() => editableBalance.value !== null)
  
  /** Progreso del balance actual */
  const currentProgress = computed(() => {
    if (!currentBalance.value) return null
    return currentBalance.value.progress
  })
  
  /** Fragmentos con SubjectLeader eliminado (necesitan reasignación) */
  const orphanedFragments = computed(() => {
    if (!currentBalance.value) return []
    return currentBalance.value.fragments.filter(f => f.subject_leader_id === null)
  })

  // ============================================================================
  // ACTIONS - BALANCES (Leader)
  // ============================================================================

  /**
   * Cargar lista de balances
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
   * Cargar balance completo con fragmentos
   */
  async function loadBalance(id: number): Promise<boolean> {
    isLoading.value = true
    error.value = null

    try {
      const response = await balancesService.get(id)
      if (response.success && response.data) {
        currentBalance.value = response.data
        calculateAllFragments()
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
   * Iniciar creación de nuevo balance
   */
  function startNewBalance() {
    const today = new Date().toISOString().split('T')[0] || ''
    
    editableBalance.value = {
      academic_year: '1ro',
      period: '1ero',
      academic_year_text: '2025-2026',
      start_date: today,
      weeks: 15,
      deadline: null,
      allow_leader_edit: false,
      selectedAsignaturas: [],
      non_academic_periods: [],
    }
    isDirty.value = false
  }

  /**
   * Actualizar balance editable
   */
  function updateEditableBalance(updates: Partial<EditableBalance>) {
    if (!editableBalance.value) return
    
    editableBalance.value = {
      ...editableBalance.value,
      ...updates,
    }
    isDirty.value = true
  }

  /**
   * Agregar/quitar asignatura de selección
   */
  function toggleAsignatura(asignaturaId: number) {
    if (!editableBalance.value) return
    
    const idx = editableBalance.value.selectedAsignaturas.indexOf(asignaturaId)
    if (idx >= 0) {
      editableBalance.value.selectedAsignaturas.splice(idx, 1)
    } else {
      editableBalance.value.selectedAsignaturas.push(asignaturaId)
    }
    isDirty.value = true
  }

  /**
   * Crear balance con fragmentos (Leader)
   */
  async function createBalance(): Promise<{ success: boolean; message: string; balanceId?: number }> {
    if (!editableBalance.value) {
      return { success: false, message: 'No hay balance para crear' }
    }

    if (editableBalance.value.selectedAsignaturas.length === 0) {
      return { success: false, message: 'Debe seleccionar al menos una asignatura' }
    }

    isLoading.value = true
    error.value = null

    try {
      const request: CreateBalanceRequest = {
        academic_year: editableBalance.value.academic_year,
        period: editableBalance.value.period,
        academic_year_text: editableBalance.value.academic_year_text,
        start_date: editableBalance.value.start_date,
        weeks: editableBalance.value.weeks,
        deadline: editableBalance.value.deadline || undefined,
        allow_leader_edit: editableBalance.value.allow_leader_edit,
        asignaturas: editableBalance.value.selectedAsignaturas.map(id => ({ asignatura_id: id })),
        non_academic_periods: editableBalance.value.non_academic_periods.length > 0 
          ? editableBalance.value.non_academic_periods 
          : undefined,
      }

      const response = await balancesService.create(request)

      if (response.success && response.data) {
        currentBalance.value = response.data
        editableBalance.value = null
        isDirty.value = false
        
        // Refrescar lista
        await fetchBalances()
        
        return { 
          success: true, 
          message: 'Balance creado exitosamente', 
          balanceId: response.data.id 
        }
      } else {
        error.value = response.message || 'Error al crear el balance'
        return { success: false, message: error.value }
      }
    } catch (e) {
      error.value = 'Error de conexión al crear'
      console.error('Error creating balance:', e)
      return { success: false, message: error.value }
    } finally {
      isLoading.value = false
    }
  }

  /**
   * Actualizar metadatos de balance (Leader)
   */
  async function updateBalance(id: number, updates: UpdateBalanceRequest): Promise<{ success: boolean; message: string }> {
    isLoading.value = true
    error.value = null

    try {
      const response = await balancesService.update(id, updates)

      if (response.success) {
        // Refrescar balance actual si es el mismo
        if (currentBalance.value?.id === id) {
          await loadBalance(id)
        }
        await fetchBalances()
        
        return { success: true, message: 'Balance actualizado exitosamente' }
      } else {
        error.value = response.message || 'Error al actualizar'
        return { success: false, message: error.value }
      }
    } catch (e) {
      error.value = 'Error de conexión al actualizar'
      console.error('Error updating balance:', e)
      return { success: false, message: error.value }
    } finally {
      isLoading.value = false
    }
  }

  /**
   * Eliminar balance (Leader)
   */
  async function deleteBalance(id: number): Promise<{ success: boolean; message: string }> {
    isLoading.value = true
    error.value = null

    try {
      const response = await balancesService.delete(id)
      
      if (response.success) {
        balances.value = balances.value.filter(b => b.id !== id)
        
        if (currentBalance.value?.id === id) {
          currentBalance.value = null
        }
        
        return { success: true, message: 'Balance eliminado exitosamente' }
      } else {
        error.value = response.message || 'Error al eliminar'
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
  // ACTIONS - FRAGMENTS (SubjectLeader)
  // ============================================================================

  /**
   * Cargar fragmentos pendientes (para Dashboard)
   */
  async function fetchPendingFragments(): Promise<boolean> {
    isLoading.value = true
    error.value = null

    try {
      const response = await fragmentsService.getPending()
      if (response.success && response.data) {
        pendingFragments.value = response.data
        return true
      } else {
        error.value = response.message || 'Error al cargar fragmentos pendientes'
        return false
      }
    } catch (e) {
      error.value = 'Error de conexión'
      console.error('Error fetching pending fragments:', e)
      return false
    } finally {
      isLoading.value = false
    }
  }

  /**
   * Cargar fragmento para edición (SubjectLeader)
   */
  async function loadFragment(balanceId: number, asignaturaId: number): Promise<boolean> {
    isLoading.value = true
    error.value = null

    try {
      // Primero obtener el balance para tener el contexto
      const balanceResponse = await balancesService.get(balanceId)
      if (!balanceResponse.success || !balanceResponse.data) {
        error.value = 'Balance no encontrado'
        return false
      }

      const fragmentResponse = await fragmentsService.get(balanceId, asignaturaId)
      if (fragmentResponse.success && fragmentResponse.data) {
        const fragment = fragmentResponse.data
        const balance = balanceResponse.data
        const asig = fragment.asignatura
        
        currentFragment.value = {
          fragmentId: fragment.id,
          balanceId: balance.id,
          balanceName: balance.name,
          asignaturaId: fragment.asignatura_id,
          asignaturaName: asig?.name || 'Asignatura',
          asignaturaHours: asig?.hours || 0,
          asignaturaPlan: {
            c: asig?.c ?? null,
            cp: asig?.cp ?? null,
            s: asig?.s ?? null,
            pl: asig?.pl ?? null,
            te: asig?.te ?? null,
            t: asig?.t ?? null,
            pp: asig?.pp ?? null,
            ec: asig?.ec ?? null,
            tc: asig?.tc ?? null,
            ef: asig?.ef ?? null,
          },
          weeks: balance.weeks,
          startDate: balance.start_date,
          nonAcademicPeriods: balance.non_academic_periods || [],
          status: fragment.status,
          deadline: balance.deadline,
          data: fragment.data || {},
        }
        isDirty.value = false
        return true
      } else {
        error.value = fragmentResponse.message || 'Fragmento no encontrado'
        return false
      }
    } catch (e) {
      error.value = 'Error de conexión al cargar fragmento'
      console.error('Error loading fragment:', e)
      return false
    } finally {
      isLoading.value = false
    }
  }

  /**
   * Actualizar datos del fragmento en memoria
   */
  function updateFragmentData(data: Record<string, unknown>) {
    if (!currentFragment.value) return
    
    currentFragment.value.data = {
      ...currentFragment.value.data,
      ...data,
    }
    isDirty.value = true
  }

  /**
   * Actualizar valor específico en la tabla del fragmento
   */
  function updateFragmentCell(cellIndex: number, value: string) {
    if (!currentFragment.value) return
    
    const values = (currentFragment.value.data.values as string[]) || []
    values[cellIndex] = value
    currentFragment.value.data = {
      ...currentFragment.value.data,
      values,
    }
    isDirty.value = true
  }

  /**
   * Guardar fragmento (SubjectLeader)
   */
  async function saveFragment(markAsCompleted: boolean = false): Promise<{ success: boolean; message: string }> {
    if (!currentFragment.value) {
      return { success: false, message: 'No hay fragmento para guardar' }
    }

    isLoading.value = true
    error.value = null

    try {
      const request: UpdateFragmentRequest = {
        data: currentFragment.value.data,
        status: markAsCompleted ? 'completed' : 'in_progress',
      }

      const response = await fragmentsService.update(
        currentFragment.value.balanceId,
        currentFragment.value.asignaturaId,
        request
      )

      if (response.success) {
        currentFragment.value.status = request.status || 'in_progress'
        isDirty.value = false
        
        // Refrescar pendientes
        await fetchPendingFragments()
        
        return { 
          success: true, 
          message: markAsCompleted ? 'Fragmento completado' : 'Fragmento guardado' 
        }
      } else {
        error.value = response.message || 'Error al guardar'
        return { success: false, message: error.value }
      }
    } catch (e) {
      error.value = 'Error de conexión al guardar'
      console.error('Error saving fragment:', e)
      return { success: false, message: error.value }
    } finally {
      isLoading.value = false
    }
  }

  // ============================================================================
  // ACTIONS - CÁLCULOS
  // ============================================================================

  /**
   * Calcular totales de todos los fragmentos del balance actual
   */
  function calculateAllFragments() {
    if (!currentBalance.value) return

    calculations.value = currentBalance.value.fragments.map(fragment => {
      const values = (fragment.data?.values as string[]) || []
      
      const counts = { C: 0, CP: 0, S: 0, PL: 0, TE: 0, T: 0, PP: 0, EC: 0 }
      let total = 0
      
      for (const val of values) {
        if (typeof val === 'string' && VALID_ACTIVITY_TYPES.has(val)) {
          counts[val as keyof typeof counts]++
          total++
        }
      }
      
      return {
        fragmentId: fragment.id,
        asignaturaName: fragment.asignatura?.name || 'Asignatura',
        status: fragment.status,
        total,
        ...counts,
        coef: Math.round(total * 1.2 * 100) / 100,
      }
    })
  }

  // ============================================================================
  // ACTIONS - UTILIDADES
  // ============================================================================

  /**
   * Reiniciar estado
   */
  function resetState() {
    currentBalance.value = null
    editableBalance.value = null
    currentFragment.value = null
    calculations.value = []
    isDirty.value = false
    error.value = null
  }

  /**
   * Cancelar creación de balance
   */
  function cancelNewBalance() {
    editableBalance.value = null
    isDirty.value = false
  }

  /**
   * Cerrar fragmento actual
   */
  function closeFragment() {
    currentFragment.value = null
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
    editableBalance,
    pendingFragments,
    currentFragment,
    calculations,
    isDirty,
    isLoading,
    error,

    // Getters
    hasUnsavedChanges,
    balancesCount,
    pendingCount,
    isCreatingNew,
    currentProgress,
    orphanedFragments,

    // Actions - Balances
    fetchBalances,
    loadBalance,
    startNewBalance,
    updateEditableBalance,
    toggleAsignatura,
    createBalance,
    updateBalance,
    deleteBalance,

    // Actions - Fragments
    fetchPendingFragments,
    loadFragment,
    updateFragmentData,
    updateFragmentCell,
    saveFragment,

    // Actions - Cálculos
    calculateAllFragments,

    // Actions - Utilidades
    resetState,
    cancelNewBalance,
    closeFragment,
    clearError,
  }
})
