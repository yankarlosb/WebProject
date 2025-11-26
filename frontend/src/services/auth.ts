import { getApiUrl } from '../config/api'
import type { User } from '../types'

// Re-export User type for backward compatibility
export type { User }

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

/**
 * Makes an auth-specific HTTP request with credentials
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

export class AuthService {
  /**
   * Realiza el login del usuario
   */
  static async login(username: string, password: string): Promise<AuthResponse> {
    try {
      const response = await authRequest('/api/login', 'POST', { username, password })
      const data = await response.json()

      // Verificar si el login fue exitoso
      if (response.ok && data.success && data.user) {
        return {
          success: true,
          message: data.message || 'Inicio de sesión exitoso',
          user: data.user,
        }
      }

      // Login falló
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
  }

  /**
   * Verifica autenticación y devuelve datos del usuario desde JWT
   */
  static async checkAuth(): Promise<{ isAuthenticated: boolean; user?: User }> {
    try {
      const response = await authRequest('/api/verify', 'GET')

      if (response.ok) {
        const data: VerifyResponse = await response.json()

        if (data.authenticated && data.user) {
          return {
            isAuthenticated: true,
            user: data.user,
          }
        }
      }

      // Token inválido o expirado
      if (response.status === 401) {
        return { isAuthenticated: false }
      }

      // Otro error
      console.error('Error en checkAuth:', response.status)
      return { isAuthenticated: false }
    } catch (error) {
      console.error('Error verificando autenticación:', error)

      return {
        isAuthenticated: false,
      }
    }
  }

  /**
   * Cierra sesión en frontend y backend
   */
  static async logout(): Promise<{ success: boolean; message?: string }> {
    try {
      // Intentar logout en backend
      const response = await authRequest('/api/logout', 'POST')

      if (!response.ok) {
        console.warn('Logout en backend falló, pero limpiando frontend')
      }
    } catch (error) {
      console.error('Error en logout backend:', error)
    } finally {
      // Intentar limpiar cookie manualmente (fallback)
      this.clearAuthCookie()
    }

    return { success: true }
  }

  /**
   * Limpia la cookie de autenticación (fallback)
   */
  private static clearAuthCookie(): void {
    // Esto es un fallback por si el backend no limpia la cookie
    document.cookie = 'jwt_token=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;'
  }
}

export default AuthService