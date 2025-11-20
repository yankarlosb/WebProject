import { API_CONFIG } from '../config/api'

export interface UpdateProfileRequest {
  name: string
  email: string
}

export interface ChangePasswordRequest {
  newPassword: string
}

export interface ProfileResponse {
  success: boolean
  message: string
  alert: 'success' | 'error'
}

export class ProfileService {
  /**
   * Helper genérico para peticiones del perfil
   */
  private static async request(
    endpoint: string,
    body: any,
    errorMessage: string
  ): Promise<ProfileResponse> {
    try {
      const response = await fetch(`${API_CONFIG.BASE_URL}/api/${endpoint}`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        credentials: 'include',
        body: JSON.stringify(body)
      })

      const result = await response.json()
      
      return {
        success: result.alert === 'success',
        message: result.message,
        alert: result.alert
      }
    } catch (error) {
      console.error(`Error en ${endpoint}:`, error)
      return {
        success: false,
        message: errorMessage,
        alert: 'error'
      }
    }
  }

  /**
   * Actualizar perfil del usuario
   */
  static async updateProfile(data: UpdateProfileRequest): Promise<ProfileResponse> {
    return this.request('update_profile', data, 'Error de conexión al actualizar el perfil')
  }

  /**
   * Cambiar contraseña del usuario
   */
  static async changePassword(newPassword: string): Promise<ProfileResponse> {
    return this.request('change_password', { new_password: newPassword }, 'Error de conexión al cambiar la contraseña')
  }
}

export default ProfileService
