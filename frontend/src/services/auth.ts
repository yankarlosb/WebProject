/**
 * Servicio de autenticación
 * Maneja la verificación de autenticación con el backend JWT
 */

import { API_CONFIG } from '../config/api';

interface User {
  id: number;
  name: string;
  email: string;
  isAdmin: boolean;
}

export class AuthService {
  /**
   * Realiza el login del usuario
   * @param email Email del usuario
   * @param password Contraseña del usuario
   * @returns Objeto con success, user opcional y message opcional
   */
  static async login(email: string, password: string): Promise<{ success: boolean; user?: User; message?: string }> {
    try {
      const response = await fetch(API_CONFIG.ENDPOINTS.LOGIN, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        credentials: 'include', // Importante: para recibir la cookie JWT
        body: JSON.stringify({ email, password })
      });

      const data = await response.json();
      
      if (data.success && data.user) {
        const user: User = {
          id: data.user.id,
          name: data.user.name,
          email: data.user.email,
          isAdmin: data.user.is_admin
        };
        
        // Guardar datos locales para mejorar UX
        this.setLocalAuth(user);
        
        return { success: true, user };
      }
      
      return { success: false, message: data.message || 'Error al iniciar sesión' };
    } catch (error) {
      console.error('Error al conectar con el backend:', error);
      return { 
        success: false, 
        message: 'Error al conectar con el servidor. Asegúrese de que el backend esté corriendo en el puerto 8000.' 
      };
    }
  }

  static async checkAuth(): Promise<boolean> {
    try {
      const response = await fetch(API_CONFIG.ENDPOINTS.VERIFY, {
        method: 'GET',
        credentials: 'include', // Importante: incluye las cookies
        headers: {
          'Content-Type': 'application/json',
        }
      });

      if (response.ok) {
        // Si el backend responde OK, el JWT es válido
        return true;
      }
      
      // Si es 401 Unauthorized, el token no es válido o expiró
      if (response.status === 401) {
        this.clearLocalAuth();
        return false;
      }

      return false;
    } catch (error) {
      console.error('Error verificando autenticación:', error);
      return false;
    }
  }

  static isLocallyAuthenticated(): boolean {
    return sessionStorage.getItem('loggedIn') === 'true';
  }

  static getCurrentUser(): User | null {
    const userStr = sessionStorage.getItem('currentUser');
    if (!userStr) return null;
    
    try {
      return JSON.parse(userStr);
    } catch {
      return null;
    }
  }

  static clearLocalAuth(): void {
    sessionStorage.removeItem('loggedIn');
    sessionStorage.removeItem('currentUser');
  }

  static setLocalAuth(user: User): void {
    sessionStorage.setItem('loggedIn', 'true');
    sessionStorage.setItem('currentUser', JSON.stringify(user));
  }

  static async logout(): Promise<void> {
    try {
      // Llamar al endpoint de logout para invalidar el JWT en el backend
      await fetch(API_CONFIG.ENDPOINTS.LOGOUT, {
        method: 'POST',
        credentials: 'include',
        headers: {
          'Content-Type': 'application/json',
        }
      });
    } catch (error) {
      console.error('Error al hacer logout:', error);
    } finally {
      // Siempre limpiar los datos locales
      this.clearLocalAuth();
    }
  }
}

export default AuthService;
