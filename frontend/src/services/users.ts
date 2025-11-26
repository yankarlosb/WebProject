/**
 * Servicio de Gestión de Usuarios
 * Maneja todas las operaciones CRUD de usuarios con el backend
 */

import { httpGet, httpPost } from './http'
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

export interface UsersResponse {
  success: boolean
  users?: User[]
  user?: User
  message?: string
}

export class UsersService {
  /**
   * Obtener todos los usuarios
   */
  static async getAll(): Promise<UsersResponse> {
    const result = await httpGet<User[]>('/api/list_users', 'Error al cargar los usuarios')
    return {
      success: result.success,
      users: result.data,
      message: result.message,
    }
  }

  /**
   * Crear un nuevo usuario
   */
  static async create(userData: CreateUserRequest): Promise<UsersResponse> {
    const result = await httpPost('/api/create_user', userData, 'Error al crear usuario')
    return {
      success: result.success,
      message: result.message,
    }
  }

  /**
   * Actualizar un usuario existente
   */
  static async update(id: number, userData: UpdateUserRequest): Promise<UsersResponse> {
    const result = await httpPost(`/api/modify_user/${id}`, userData, 'Error al actualizar usuario')
    return {
      success: result.success,
      message: result.message,
    }
  }

  /**
   * Eliminar un usuario
   */
  static async delete(id: number): Promise<UsersResponse> {
    const result = await httpPost(`/api/delete_user/${id}`, undefined, 'Error al eliminar usuario')
    return {
      success: result.success,
      message: result.message,
    }
  }
}

export default UsersService
