/**
 * Settings Service
 * 
 * Provides API calls for system settings management (admin only).
 */

import { httpGet, httpPut, type ServiceResponse } from './http'

/** Single setting response */
export interface SettingResponse {
  key: string
  value: string
  description: string | null
  category: string
}

/** Settings grouped by category */
export interface SettingsGrouped {
  security: SettingResponse[]
  session: SettingResponse[]
  password: SettingResponse[]
  audit: SettingResponse[]
}

/** Public settings response */
export interface PublicSettingsResponse {
  session_timeout_minutes: number
}

/** Update settings request */
export interface UpdateSettingsRequest {
  settings: Record<string, string>
}

export const settingsService = {
  /**
   * Get public settings (session timeout, etc.) - accessible to authenticated users
   */
  async getPublic(): Promise<ServiceResponse<PublicSettingsResponse>> {
    return httpGet<PublicSettingsResponse>('/api/settings/public', 'Error al obtener configuraciones')
  },

  /**
   * Get all system settings grouped by category (admin only)
   */
  async list(): Promise<ServiceResponse<SettingsGrouped>> {
    return httpGet<SettingsGrouped>('/api/settings', 'Error al obtener configuraciones')
  },

  /**
   * Update system settings (admin only)
   */
  async update(settings: Record<string, string>): Promise<ServiceResponse<void>> {
    return httpPut<void>(
      '/api/settings',
      { settings },
      'Error al actualizar configuraciones'
    )
  },
}

export default settingsService
