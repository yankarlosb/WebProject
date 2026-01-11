/**
 * Servicio de Balances y Fragmentos
 * Maneja las llamadas API para CRUD de balances de carga docente
 * 
 * Nueva arquitectura:
 * - Leader crea balance y selecciona asignaturas
 * - Se crean fragmentos automáticamente para cada asignatura
 * - SubjectLeaders llenan sus fragmentos correspondientes
 */

import { httpGet, httpPost, httpPut, httpDelete, type ServiceResponse } from './http'

// ============================================================================
// TIPOS
// ============================================================================

/** Información del SubjectLeader */
export interface SubjectLeaderInfo {
  id: number
  user_name: string
  name: string
  email: string
}

/** Información de la asignatura */
export interface AsignaturaInfo {
  id: number
  name: string
  year: string
  semester: string
  hours: number
  // Cantidades planificadas por tipo de clase
  c: number | null   // Conferencias
  cp: number | null  // Clases Prácticas
  s: number | null   // Seminarios
  pl: number | null  // Prácticas de Laboratorio
  te: number | null  // Trabajo Extraclase
  t: number | null   // Tutorías
  pp: number | null  // Pruebas Parciales
  ec: number | null  // Examen de Culminación
  tc: number | null  // Trabajo de Curso
  ef: number | null  // Examen Final
}

/** Progreso del balance */
export interface BalanceProgress {
  total: number
  pending: number
  in_progress: number
  completed: number
  percentage: number
}

/** Fragmento con información completa */
export interface Fragment {
  id: number
  balance_id: number
  asignatura_id: number
  asignatura: AsignaturaInfo | null
  subject_leader_id: number | null
  subject_leader: SubjectLeaderInfo | null
  status: 'pending' | 'in_progress' | 'completed'
  data: Record<string, unknown>
  completed_at: string | null
  created_at: string | null
  updated_at: string | null
}

/** Período no académico (vacaciones, feriados, etc.) */
export interface NonAcademicPeriod {
  start: string  // 'YYYY-MM-DD'
  end: string    // 'YYYY-MM-DD'
  name: string   // Descripción del período
}

/** Balance con fragmentos (respuesta detallada) */
export interface Balance {
  id: number
  user_id: number
  name: string
  academic_year: string
  period: string
  academic_year_text: string
  start_date: string
  weeks: number
  status: 'draft' | 'in_progress' | 'completed'
  deadline: string | null
  allow_leader_edit: boolean
  non_academic_periods: NonAcademicPeriod[]
  created_at: string | null
  updated_at: string | null
  fragments: Fragment[]
  progress: BalanceProgress
}

/** Balance para listado (sin fragmentos) */
export interface BalanceListItem {
  id: number
  user_id: number
  name: string
  academic_year: string
  period: string
  academic_year_text: string
  start_date: string
  weeks: number
  status: 'draft' | 'in_progress' | 'completed'
  deadline: string | null
  non_academic_periods: NonAcademicPeriod[]
  created_at: string | null
  progress: BalanceProgress
}

/** Fragmento pendiente (para dashboard) */
export interface PendingFragment {
  fragment_id: number
  balance_id: number
  balance_name: string
  asignatura_id: number
  asignatura_name: string
  status: 'pending' | 'in_progress'
  deadline: string | null
}

/** Asignatura seleccionada para crear balance */
export interface SelectedSubject {
  asignatura_id: number
}

/** Request para crear un balance (Leader) */
export interface CreateBalanceRequest {
  academic_year: string
  period: string
  academic_year_text: string
  start_date: string
  weeks: number
  deadline?: string
  allow_leader_edit?: boolean
  asignaturas: SelectedSubject[]
  non_academic_periods?: NonAcademicPeriod[]
}

/** Request para actualizar metadatos del balance (Leader) */
export interface UpdateBalanceRequest {
  academic_year?: string
  period?: string
  academic_year_text?: string
  start_date?: string
  weeks?: number
  deadline?: string
  allow_leader_edit?: boolean
  status?: 'draft' | 'in_progress' | 'completed'
  non_academic_periods?: NonAcademicPeriod[]
}

/** Request para actualizar un fragmento (SubjectLeader) */
export interface UpdateFragmentRequest {
  data: Record<string, unknown>
  status?: 'pending' | 'in_progress' | 'completed'
}

// ============================================================================
// SERVICIO DE BALANCES
// ============================================================================

export const balancesService = {
  /**
   * Listar todos los balances
   * - Leader: ve todos
   * - SubjectLeader: ve donde tiene fragmentos asignados
   */
  async list(): Promise<ServiceResponse<BalanceListItem[]>> {
    return httpGet<BalanceListItem[]>('/api/balances', 'Error al obtener los balances')
  },

  /**
   * Obtener un balance con todos sus fragmentos
   */
  async get(id: number): Promise<ServiceResponse<Balance>> {
    return httpGet<Balance>(`/api/balances/${id}`, 'Error al obtener el balance')
  },

  /**
   * Crear un nuevo balance con sus fragmentos (Solo Leader)
   */
  async create(data: CreateBalanceRequest): Promise<ServiceResponse<Balance>> {
    return httpPost<Balance>('/api/balances', data, 'Error al crear el balance')
  },

  /**
   * Actualizar metadatos de un balance (Solo Leader)
   */
  async update(id: number, data: UpdateBalanceRequest): Promise<ServiceResponse<void>> {
    return httpPut<void>(`/api/balances/${id}`, data, 'Error al actualizar el balance')
  },

  /**
   * Eliminar un balance y sus fragmentos (Solo Leader)
   */
  async delete(id: number): Promise<ServiceResponse<void>> {
    return httpDelete(`/api/balances/${id}`, 'Error al eliminar el balance')
  },
}

// ============================================================================
// SERVICIO DE FRAGMENTOS
// ============================================================================

export const fragmentsService = {
  /**
   * Obtener fragmentos pendientes del usuario actual (para Dashboard)
   */
  async getPending(): Promise<ServiceResponse<PendingFragment[]>> {
    return httpGet<PendingFragment[]>('/api/fragments/pending', 'Error al obtener fragmentos pendientes')
  },

  /**
   * Obtener un fragmento específico
   */
  async get(balanceId: number, asignaturaId: number): Promise<ServiceResponse<Fragment>> {
    return httpGet<Fragment>(
      `/api/balances/${balanceId}/fragments/${asignaturaId}`,
      'Error al obtener el fragmento'
    )
  },

  /**
   * Actualizar un fragmento (SubjectLeader edita su parte)
   */
  async update(
    balanceId: number,
    asignaturaId: number,
    data: UpdateFragmentRequest
  ): Promise<ServiceResponse<void>> {
    return httpPut<void>(
      `/api/balances/${balanceId}/fragments/${asignaturaId}`,
      data,
      'Error al actualizar el fragmento'
    )
  },
}

export default balancesService
