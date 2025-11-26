/**
 * Servicio de Asignaturas
 * Maneja las operaciones CRUD de asignaturas con el backend
 */

import { httpGet, httpPost, httpPut, httpDelete } from './http'

// ============================================================================
// TIPOS
// ============================================================================

export interface Asignatura {
  id: number
  leader_id: number
  name: string
  year: string
  semester: string
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
  hours: number
  weeks: number | null
  date_start: string
  date_end: string
}

export interface CreateAsignaturaData {
  leader_user_name: string
  name: string
  year: string
  semester: string
}

export interface UpdateAsignaturaData {
  name: string
  year: string
  semester: string
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
  hours: number
  weeks: number | null
}

export interface SubjectLeader {
  id: number
  user_name: string
  name: string
  email: string
  role: string
}

// ============================================================================
// SERVICIO
// ============================================================================

const AsignaturasService = {
  /**
   * Obtener todas las asignaturas
   * El backend filtra automáticamente según el rol del usuario
   */
  async getAsignaturas(): Promise<Asignatura[]> {
    const result = await httpGet<Asignatura[]>(
      '/api/asignaturas/list',
      'Error al obtener las asignaturas'
    )
    
    if (!result.success || !result.data) {
      throw new Error(result.message || 'Error al obtener las asignaturas')
    }

    return result.data
  },

  /**
   * Crear nueva asignatura (solo Leaders)
   */
  async createAsignatura(asignaturaData: CreateAsignaturaData): Promise<void> {
    const result = await httpPost(
      '/api/asignaturas/create',
      asignaturaData,
      'Error al crear la asignatura'
    )

    if (!result.success) {
      throw new Error(result.message || 'Error al crear la asignatura')
    }
  },

  /**
   * Actualizar asignatura (solo SubjectLeaders - sus asignaturas)
   */
  async updateAsignatura(id: number, asignaturaData: UpdateAsignaturaData): Promise<void> {
    const result = await httpPut(
      `/api/asignaturas/update/${id}`,
      asignaturaData,
      'Error al actualizar la asignatura'
    )

    if (!result.success) {
      throw new Error(result.message || 'Error al actualizar la asignatura')
    }
  },

  /**
   * Eliminar asignatura (solo Leaders)
   */
  async deleteAsignatura(id: number): Promise<void> {
    const result = await httpDelete(
      `/api/asignaturas/delete/${id}`,
      'Error al eliminar la asignatura'
    )

    if (!result.success) {
      throw new Error(result.message || 'Error al eliminar la asignatura')
    }
  },

  /**
   * Obtener lista de jefes de asignatura (solo Leaders)
   */
  async getSubjectLeaders(): Promise<SubjectLeader[]> {
    const result = await httpGet<SubjectLeader[]>(
      '/api/users/subject_leaders',
      'Error al obtener los jefes de asignatura'
    )
    
    if (!result.success || !result.data) {
      throw new Error(result.message || 'Error al obtener los jefes de asignatura')
    }

    return result.data
  },
}

export default AsignaturasService
