/**
 * Servicio de Gestión de Usuarios
 * Maneja todas las operaciones CRUD de usuarios con el backend
 */

import { httpGet, httpPost, httpPut, httpDelete, type ServiceResponse } from './http'
import type { UserWithToken } from '../types'

// In admin context, users always include token from database
export type User = UserWithToken

export interface CreateUserRequest {
  user_name: string
  name: string
  email: string
  password: string
  role: string
}

export interface UpdateUserRequest {
  id: number
  user_name: string
  name: string
  email: string
  token: string
  role: string
  created_at?: string
}

// ============================================================================
// SERVICIO (Object literal pattern - standardized)
// ============================================================================

export const usersService = {
  /**
   * GET /users - List all users (Admin only)
   */
  async list(): Promise<ServiceResponse<User[]>> {
    return httpGet<User[]>('/api/users', 'Error al cargar los usuarios')
  },

  /**
   * POST /users - Create a new user (Admin only)
   */
  async create(userData: CreateUserRequest): Promise<ServiceResponse<void>> {
    return httpPost('/api/users', userData, 'Error al crear usuario')
  },

  /**
   * PUT /users/<id> - Update a user (Admin only)
   */
  async update(id: number, userData: UpdateUserRequest): Promise<ServiceResponse<void>> {
    return httpPut(`/api/users/${id}`, userData, 'Error al actualizar usuario')
  },

  /**
   * DELETE /users/<id> - Delete a user (Admin only)
   */
  async delete(id: number): Promise<ServiceResponse<void>> {
    return httpDelete(`/api/users/${id}`, 'Error al eliminar usuario')
  },
}

export default usersService
