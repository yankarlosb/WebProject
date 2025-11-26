/**
 * Servicio de Balances
 * Maneja las llamadas API para CRUD de balances de carga docente
 */

import { httpGet, httpPost, httpPut, httpDelete, type ServiceResponse } from './http'

// Tipos para las asignaturas dentro de un balance
export interface BalanceSubject {
  id: string
  name: string
  values: string[] // Array de valores: 'C', 'CP', 'S', 'PL', 'T', 'TE', 'PP' o ''
}

// Tipo para el balance completo desde/hacia la API
export interface Balance {
  id: number
  user_id: number
  name: string
  academic_year: string      // '1ro', '2do', '3ro', '4to'
  period: string             // '1ero', '2do'
  academic_year_text: string // '2025-2026'
  start_date: string         // 'YYYY-MM-DD'
  weeks: number
  subjects: BalanceSubject[]
  created_at: string | null
  updated_at: string | null
}

// Tipo para crear/actualizar un balance (name es auto-generado en backend)
export interface BalanceRequest {
  academic_year: string
  period: string
  academic_year_text: string
  start_date: string
  weeks: number
  subjects: BalanceSubject[]
}

/**
 * Servicio de balances - CRUD completo
 */
export const balancesService = {
  /**
   * Listar todos los balances del usuario autenticado
   */
  async list(): Promise<ServiceResponse<Balance[]>> {
    return httpGet<Balance[]>('/api/balances', 'Error al obtener los balances')
  },

  /**
   * Obtener un balance específico por ID
   */
  async get(id: number): Promise<ServiceResponse<Balance>> {
    return httpGet<Balance>(`/api/balances/${id}`, 'Error al obtener el balance')
  },

  /**
   * Crear un nuevo balance
   */
  async create(data: BalanceRequest): Promise<ServiceResponse<Balance>> {
    return httpPost<Balance>('/api/balances', data, 'Error al crear el balance')
  },

  /**
   * Actualizar un balance existente
   */
  async update(id: number, data: BalanceRequest): Promise<ServiceResponse<Balance>> {
    return httpPut<Balance>(`/api/balances/${id}`, data, 'Error al actualizar el balance')
  },

  /**
   * Eliminar un balance
   */
  async delete(id: number): Promise<ServiceResponse> {
    return httpDelete(`/api/balances/${id}`, 'Error al eliminar el balance')
  },
}

export default balancesService
