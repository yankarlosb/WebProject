/**
 * Servicio de autenticación - Versión Mejorada
 * Maneja la verificación de autenticación con el backend JWT
 */

import { API_CONFIG } from '../config/api';

export interface User {
  id: number;
  name: string;
  email: string;
  isAdmin: boolean;
}

export interface AuthResponse {
  success: boolean;
  user?: User;
  message?: string;
}

export class AuthService {
  private static readonly AUTH_KEY = 'loggedIn';
  private static readonly USER_KEY = 'currentUser';

  /**
   * Realiza el login del usuario
   */
  static async login(username: string, password: string): Promise<AuthResponse> {
    try {
      const response = await fetch(API_CONFIG.ENDPOINTS.LOGIN, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        credentials: 'include', // Para recibir la cookie HttpOnly
        body: JSON.stringify({ username, password })
      });

      const data = await response.json();
      
      if (data.success && data.user) {
        const user: User = this.normalizeUser(data.user);
        
        // Guardar datos locales
        this.setLocalAuth(user);
        
        return { success: true, user };
      }
      
      return { 
        success: false, 
        message: data.message || 'Error al iniciar sesión' 
      };
    } catch (error) {
      console.error('Error en login:', error);
      return { 
        success: false, 
        message: 'Error de conexión con el servidor' 
      };
    }
  }

  /**
   * Verifica autenticación y sincroniza datos del usuario
   */
  static async checkAuth(): Promise<{ isAuthenticated: boolean; user?: User }> {
    // Primero verificar localmente para UX rápida
    if (!this.isLocallyAuthenticated()) {
      return { isAuthenticated: false };
    }

    try {
      const response = await fetch(API_CONFIG.ENDPOINTS.VERIFY, {
        method: 'GET',
        credentials: 'include',
        headers: {
          'Content-Type': 'application/json',
        }
      });

      if (response.ok) {
        const data = await response.json();
        
        // Si el endpoint retorna datos del usuario, actualizar
        if (data.user) {
          const user = this.normalizeUser(data.user);
          this.setLocalAuth(user);
          return { isAuthenticated: true, user };
        }
        
        // Si no retorna usuario, mantener el actual
        const currentUser = this.getCurrentUser();
        return { 
          isAuthenticated: true, 
          user: currentUser || undefined 
        };
      }

      // Token inválido o expirado
      if (response.status === 401) {
        this.clearLocalAuth();
        return { isAuthenticated: false };
      }

      // Otro error
      console.error('Error en checkAuth:', response.status);
      return { isAuthenticated: false };

    } catch (error) {
      console.error('Error verificando autenticación:', error);
      
      // En caso de error de red, mantener estado local para mejor UX
      // pero marcar como posiblemente desconectado
      return { 
        isAuthenticated: this.isLocallyAuthenticated(),
        user: this.getCurrentUser() || undefined
      };
    }
  }

  /**
   * Cierra sesión en frontend y backend
   */
  static async logout(): Promise<{ success: boolean; message?: string }> {
    try {
      // Intentar logout en backend
      const response = await fetch(API_CONFIG.ENDPOINTS.LOGOUT, {
        method: 'POST',
        credentials: 'include',
        headers: {
          'Content-Type': 'application/json',
        }
      });

      if (!response.ok) {
        console.warn('Logout en backend falló, pero limpiando frontend');
      }
    } catch (error) {
      console.error('Error en logout backend:', error);
    } finally {
      // Siempre limpiar frontend
      this.clearLocalAuth();
      
      // Intentar limpiar cookie manualmente (fallback)
      this.clearAuthCookie();
    }

    return { success: true };
  }

  /**
   * Normaliza datos del usuario desde el backend
   */
  private static normalizeUser(userData: any): User {
    return {
      id: userData.id,
      name: userData.name || userData.username,
      email: userData.email,
      isAdmin: userData.isAdmin || userData.is_admin || false
    };
  }

  /**
   * Limpia la cookie de autenticación (fallback)
   */
  private static clearAuthCookie(): void {
    // Esto es un fallback por si el backend no limpia la cookie
    document.cookie = 'jwt_token=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;';
  }

  // --- Métodos de persistencia local ---

  static isLocallyAuthenticated(): boolean {
    return localStorage.getItem(this.AUTH_KEY) === 'true';
  }

  static getCurrentUser(): User | null {
    const userStr = localStorage.getItem(this.USER_KEY);
    if (!userStr) return null;
    
    try {
      return JSON.parse(userStr);
    } catch {
      return null;
    }
  }

  private static clearLocalAuth(): void {
    localStorage.removeItem(this.AUTH_KEY);
    localStorage.removeItem(this.USER_KEY);
  }

  private static setLocalAuth(user: User): void {
    localStorage.setItem(this.AUTH_KEY, 'true');
    localStorage.setItem(this.USER_KEY, JSON.stringify(user));
  }

  /**
   * Actualiza datos locales del usuario (útil para edición de perfil)
   */
  static updateLocalUser(userUpdates: Partial<User>): void {
    const currentUser = this.getCurrentUser();
    if (currentUser) {
      const updatedUser = { ...currentUser, ...userUpdates };
      localStorage.setItem(this.USER_KEY, JSON.stringify(updatedUser));
    }
  }
}

export default AuthService;