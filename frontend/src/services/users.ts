/**
 * Servicio de Gestión de Usuarios
 * Maneja todas las operaciones CRUD de usuarios con el backend
 */

import { API_CONFIG } from '../config/api'

export interface User {
  id: number
  user_name: string
  name: string
  email: string
  token: string 
  role: string
  created_at?: string
}

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

// Respuesta del backend
interface BackendResponse {
  message: string
  alert: 'success' | 'error'
  data?: User[] | null
}

export class UsersService {
  private static readonly BASE_URL = `${API_CONFIG.BASE_URL}/api`

  /**
   * Helper genérico para peticiones HTTP
   */
  private static async request(
    endpoint: string,
    method: 'GET' | 'POST' = 'GET',
    body?: any,
    errorMessage: string = 'Error en la operación'
  ): Promise<UsersResponse> {
    try {
      const options: RequestInit = {
        method,
        credentials: 'include',
        headers: {
          'Content-Type': 'application/json',
        },
      }

      if (body) {
        options.body = JSON.stringify(body)
      }

      const response = await fetch(`${this.BASE_URL}${endpoint}`, options)
      const data: BackendResponse = await response.json()

      if (data.alert === 'success') {
        return {
          success: true,
          message: data.message,
          users: data.data || undefined,
        }
      } else {
        return {
          success: false,
          message: data.message || errorMessage,
        }
      }
    } catch (error) {
      console.error(`Error en ${endpoint}:`, error)
      return {
        success: false,
        message: `Error de conexión: ${errorMessage}`,
      }
    }
  }

  /**
   * Obtener todos los usuarios
   */
  static async getAll(): Promise<UsersResponse> {
    return this.request('/list_users', 'GET', undefined, 'Error al cargar los usuarios')
  }

  /**
   * Crear un nuevo usuario
   */
  static async create(userData: CreateUserRequest): Promise<UsersResponse> {
    return this.request('/create_user', 'POST', userData, 'Error al crear usuario')
  }

  /**
   * Actualizar un usuario existente
   */
  static async update(id: number, userData: UpdateUserRequest): Promise<UsersResponse> {
    return this.request(`/modify_user/${id}`, 'POST', userData, 'Error al actualizar usuario')
  }

  /**
   * Eliminar un usuario
   */
  static async delete(id: number): Promise<UsersResponse> {
    return this.request(`/delete_user/${id}`, 'POST', undefined, 'Error al eliminar usuario')
  }
}

export default UsersService
