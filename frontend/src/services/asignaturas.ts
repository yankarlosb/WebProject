/**
 * Servicio de Asignaturas
 * Maneja las operaciones CRUD de asignaturas con el backend
 */

import { getApiUrl } from '../config/api'

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

interface ApiResponse {
  message: string
  alert: 'success' | 'error'
}

interface ApiResponseWithData<T> extends ApiResponse {
  data: T | null
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
    const url = getApiUrl('/api/asignaturas/list')
    console.log('[AsignaturasService] Llamando a:', url)
    
    const response = await fetch(url, {
      method: 'GET',
      credentials: 'include',
      headers: {
        'Content-Type': 'application/json',
      },
    })

    console.log('[AsignaturasService] Response status:', response.status)
    console.log('[AsignaturasService] Response headers:', response.headers)
    
    if (!response.ok) {
      const text = await response.text()
      console.error('[AsignaturasService] Error response:', text)
      throw new Error('Error al obtener las asignaturas')
    }

    const data: ApiResponseWithData<Asignatura[]> = await response.json()
    
    if (data.alert === 'error' || !data.data) {
      throw new Error(data.message)
    }

    return data.data
  },

  /**
   * Crear nueva asignatura (solo Leaders)
   */
  async createAsignatura(asignaturaData: CreateAsignaturaData): Promise<void> {
    const response = await fetch(getApiUrl('/api/asignaturas/create'), {
      method: 'POST',
      credentials: 'include',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(asignaturaData),
    })

    if (!response.ok) {
      throw new Error('Error al crear la asignatura')
    }

    const data: ApiResponse = await response.json()
    
    if (data.alert === 'error') {
      throw new Error(data.message)
    }
  },

  /**
   * Actualizar asignatura (solo SubjectLeaders - sus asignaturas)
   */
  async updateAsignatura(id: number, asignaturaData: UpdateAsignaturaData): Promise<void> {
    const response = await fetch(getApiUrl(`/api/asignaturas/update/${id}`), {
      method: 'PUT',
      credentials: 'include',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(asignaturaData),
    })

    if (!response.ok) {
      throw new Error('Error al actualizar la asignatura')
    }

    const data: ApiResponse = await response.json()
    
    if (data.alert === 'error') {
      throw new Error(data.message)
    }
  },

  /**
   * Eliminar asignatura (solo Leaders)
   */
  async deleteAsignatura(id: number): Promise<void> {
    const response = await fetch(getApiUrl(`/api/asignaturas/delete/${id}`), {
      method: 'DELETE',
      credentials: 'include',
      headers: {
        'Content-Type': 'application/json',
      },
    })

    if (!response.ok) {
      throw new Error('Error al eliminar la asignatura')
    }

    const data: ApiResponse = await response.json()
    
    if (data.alert === 'error') {
      throw new Error(data.message)
    }
  },

  /**
   * Obtener lista de jefes de asignatura (solo Leaders)
   */
  async getSubjectLeaders(): Promise<SubjectLeader[]> {
    const url = getApiUrl('/api/users/subject_leaders')
    console.log('[AsignaturasService] Llamando a subject_leaders:', url)
    
    const response = await fetch(url, {
      method: 'GET',
      credentials: 'include',
      headers: {
        'Content-Type': 'application/json',
      },
    })

    console.log('[AsignaturasService] Subject leaders response status:', response.status)
    
    if (!response.ok) {
      const text = await response.text()
      console.error('[AsignaturasService] Subject leaders error:', text)
      throw new Error('Error al obtener los jefes de asignatura')
    }

    const data: ApiResponseWithData<SubjectLeader[]> = await response.json()
    
    if (data.alert === 'error' || !data.data) {
      throw new Error(data.message)
    }

    return data.data
  },
}

export default AsignaturasService
