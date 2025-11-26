import { httpPost } from './http'

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
   * Actualizar perfil del usuario
   */
  static async updateProfile(data: UpdateProfileRequest): Promise<ProfileResponse> {
    const result = await httpPost('/api/update_profile', data, 'Error de conexión al actualizar el perfil')
    return {
      success: result.success,
      message: result.message || '',
      alert: result.success ? 'success' : 'error',
    }
  }

  /**
   * Cambiar contraseña del usuario
   */
  static async changePassword(newPassword: string): Promise<ProfileResponse> {
    const result = await httpPost(
      '/api/change_password',
      { new_password: newPassword },
      'Error de conexión al cambiar la contraseña'
    )
    return {
      success: result.success,
      message: result.message || '',
      alert: result.success ? 'success' : 'error',
    }
  }
}

export default ProfileService
