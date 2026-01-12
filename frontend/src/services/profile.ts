/**
 * Servicio de Perfil de Usuario
 * Maneja actualización de perfil y cambio de contraseña
 */

import { httpPut, type ServiceResponse } from './http'

// ============================================================================
// TIPOS
// ============================================================================

export interface UpdateProfileData {
  name: string
  email: string
}

export interface ChangePasswordData {
  new_password: string
}

// ============================================================================
// SERVICIO (Object literal pattern - standardized)
// ============================================================================

export const profileService = {
  /**
   * PUT /profile - Update current user's profile
   */
  async update(data: UpdateProfileData): Promise<ServiceResponse<void>> {
    return httpPut('/api/profile', data, 'Error al actualizar el perfil')
  },

  /**
   * PUT /profile/password - Change current user's password
   */
  async changePassword(newPassword: string): Promise<ServiceResponse<void>> {
    return httpPut('/api/profile/password', { new_password: newPassword }, 'Error al cambiar la contraseña')
  },
}

export default profileService
