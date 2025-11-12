/**
 * Servicio de Gestión de Usuarios
 * Maneja todas las operaciones CRUD de usuarios con el backend
 */

import { API_CONFIG } from '../config/api'

export interface User {
  id: number
  name: string
  email: string
  token: string // Contraseña hasheada
  role: string // 'admin' | 'user' | 'leader' | 'subjectLeader'
  created_at?: string
}

export interface CreateUserRequest {
  name: string
  email: string
  password: string
  role: string
}

export interface UpdateUserRequest {
  id: number
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
  private static readonly BASE_URL = `${API_CONFIG.BASE_URL}/api/`

  /**
   * Obtener todos los usuarios
   */
  static async getAll(): Promise<UsersResponse> {
    try {
      const response = await fetch(`${this.BASE_URL}/list_users`, {
        method: 'GET',
        credentials: 'include',
        headers: {
          'Content-Type': 'application/json',
        },
      })

      if (!response.ok) {
        throw new Error('Error al obtener usuarios')
      }

      const data: BackendResponse = await response.json()
      
      if (data.alert === 'success') {
        return { 
          success: true, 
          users: data.data || [],
          message: data.message 
        }
      } else {
        return {
          success: false,
          message: data.message || 'Error al cargar los usuarios',
        }
      }
    } catch (error) {
      console.error('Error en getAll:', error)
      return {
        success: false,
        message: 'Error de conexión al cargar los usuarios',
      }
    }
  }

//   /**
//    * Obtener un usuario por ID
//    */
//   static async getById(id: number): Promise<UsersResponse> {
//     try {
//       const response = await fetch(`${this.BASE_URL}/users/${id}`, {
//         method: 'GET',
//         credentials: 'include',
//         headers: {
//           'Content-Type': 'application/json',
//         },
//       })

//       if (!response.ok) {
//         throw new Error('Error al obtener usuario')
//       }

//       const data = await response.json()
//       return { success: true, user: data.user || data }
//     } catch (error) {
//       console.error('Error en getById:', error)
//       return {
//         success: false,
//         message: 'Error al cargar el usuario',
//       }
//     }
//   }

  /**
   * Crear un nuevo usuario
   */
  static async create(userData: CreateUserRequest): Promise<UsersResponse> {
    try {
      const response = await fetch(`${this.BASE_URL}/create_user`, {
        method: 'POST',
        credentials: 'include',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(userData),
      })

      const data: BackendResponse = await response.json()

      if (data.alert === 'success') {
        return { 
          success: true, 
          message: data.message 
        }
      } else {
        return {
          success: false,
          message: data.message || 'Error al crear usuario',
        }
      }
    } catch (error) {
      console.error('Error en create:', error)
      return {
        success: false,
        message: 'Error de conexión al crear usuario',
      }
    }
  }

  /**
   * Actualizar un usuario existente
   * Nota: El backend espera el modelo completo de usuario, no actualizaciones parciales
   */
  static async update(
    id: number,
    userData: UpdateUserRequest
  ): Promise<UsersResponse> {
    try {
      const response = await fetch(`${this.BASE_URL}/modify_user/${id}`, {
        method: 'POST',
        credentials: 'include',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(userData),
      })

      const data: BackendResponse = await response.json()

      if (data.alert === 'success') {
        return { 
          success: true, 
          message: data.message 
        }
      } else {
        return {
          success: false,
          message: data.message || 'Error al actualizar usuario',
        }
      }
    } catch (error) {
      console.error('Error en update:', error)
      return {
        success: false,
        message: 'Error de conexión al actualizar usuario',
      }
    }
  }

  /**
   * Eliminar un usuario
   */
  static async delete(id: number): Promise<UsersResponse> {
    try {
      const response = await fetch(`${this.BASE_URL}/delete_user/${id}`, {
        method: 'POST',
        credentials: 'include',
        headers: {
          'Content-Type': 'application/json',
        },
      })

      const data: BackendResponse = await response.json()

      if (data.alert === 'success') {
        return { 
          success: true, 
          message: data.message 
        }
      } else {
        return {
          success: false,
          message: data.message || 'Error al eliminar usuario',
        }
      }
    } catch (error) {
      console.error('Error en delete:', error)
      return {
        success: false,
        message: 'Error de conexión al eliminar usuario',
      }
    }
  }
}

export default UsersService
