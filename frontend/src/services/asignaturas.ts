/**
 * Servicio de Asignaturas
 * Maneja las operaciones CRUD de asignaturas con el backend
 */

import { httpGet, httpPost, httpPut, httpDelete, type ServiceResponse } from './http'

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
// SERVICIO (Object literal pattern - standardized)
// ============================================================================

export const asignaturasService = {
  /**
   * GET /asignaturas - List subjects
   * Backend filters automatically based on user role
   */
  async list(): Promise<ServiceResponse<Asignatura[]>> {
    return httpGet<Asignatura[]>('/api/asignaturas', 'Error al obtener las asignaturas')
  },

  /**
   * POST /asignaturas - Create a new subject (Leader only)
   */
  async create(data: CreateAsignaturaData): Promise<ServiceResponse<void>> {
    return httpPost('/api/asignaturas', data, 'Error al crear la asignatura')
  },

  /**
   * PUT /asignaturas/<id> - Update a subject (Leader only)
   */
  async update(id: number, data: UpdateAsignaturaData): Promise<ServiceResponse<void>> {
    return httpPut(`/api/asignaturas/${id}`, data, 'Error al actualizar la asignatura')
  },

  /**
   * DELETE /asignaturas/<id> - Delete a subject (Leader only)
   */
  async delete(id: number): Promise<ServiceResponse<void>> {
    return httpDelete(`/api/asignaturas/${id}`, 'Error al eliminar la asignatura')
  },

  /**
   * GET /users/subject-leaders - List subject leaders (for selector)
   */
  async getSubjectLeaders(): Promise<ServiceResponse<SubjectLeader[]>> {
    return httpGet<SubjectLeader[]>('/api/users/subject-leaders', 'Error al obtener jefes de asignatura')
  },
}

export default asignaturasService
