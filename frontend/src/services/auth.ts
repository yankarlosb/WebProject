/**
 * Servicio de Autenticación
 * Maneja login, logout y verificación de sesión
 */

import { getApiUrl, API_CONFIG } from '../config/api'
import type { User } from '../types'

// Re-export User type for backward compatibility
export type { User }

// ============================================================================
// TIPOS
// ============================================================================

export interface AuthResponse {
  success: boolean
  message?: string
  user?: User
}

export interface VerifyResponse {
  success: boolean
  authenticated: boolean
  user?: User
}

// ============================================================================
// HELPER INTERNO
// ============================================================================

/**
 * Makes an auth-specific HTTP request with credentials included
 */
async function authRequest(
  endpoint: string,
  method: 'GET' | 'POST' = 'GET',
  body?: unknown
): Promise<Response> {
  const url = getApiUrl(endpoint)
  const options: RequestInit = {
    method,
    credentials: 'include',
    headers: {
      'Content-Type': 'application/json',
    },
  }

  if (body !== undefined) {
    options.body = JSON.stringify(body)
  }

  return fetch(url, options)
}

/**
 * Clear auth cookie (fallback)
 */
function clearAuthCookie(): void {
  document.cookie = 'jwt_token=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;'
}

// ============================================================================
// SERVICIO (Object literal pattern - standardized)
// ============================================================================

export const authService = {
  /**
   * POST /login - Authenticate user
   */
  async login(username: string, password: string): Promise<AuthResponse> {
    try {
      const response = await authRequest(API_CONFIG.ENDPOINTS.LOGIN, 'POST', { username, password })
      const data = await response.json()

      if (response.ok && data.success && data.user) {
        return {
          success: true,
          message: data.message || 'Inicio de sesión exitoso',
          user: data.user,
        }
      }

      return {
        success: false,
        message: data.message || 'Credenciales inválidas',
      }
    } catch (error) {
      console.error('Error en login:', error)
      return {
        success: false,
        message: 'Error de conexión con el servidor',
      }
    }
  },

  /**
   * GET /verify - Check authentication status
   */
  async checkAuth(): Promise<{ isAuthenticated: boolean; user?: User }> {
    try {
      const response = await authRequest(API_CONFIG.ENDPOINTS.VERIFY, 'GET')

      if (response.ok) {
        const data: VerifyResponse = await response.json()

        if (data.authenticated && data.user) {
          return {
            isAuthenticated: true,
            user: data.user,
          }
        }
      }

      if (response.status === 401) {
        return { isAuthenticated: false }
      }

      console.error('Error en checkAuth:', response.status)
      return { isAuthenticated: false }
    } catch (error) {
      console.error('Error verificando autenticación:', error)
      return { isAuthenticated: false }
    }
  },

  /**
   * POST /logout - End session
   */
  async logout(): Promise<{ success: boolean; message?: string }> {
    try {
      const response = await authRequest(API_CONFIG.ENDPOINTS.LOGOUT, 'POST')

      if (!response.ok) {
        console.warn('Logout en backend falló, pero limpiando frontend')
      }
    } catch (error) {
      console.error('Error en logout backend:', error)
    } finally {
      clearAuthCookie()
    }

    return { success: true }
  },
}

// Export class for backward compatibility (deprecated)
export const AuthService = authService

export default authService