import { API_CONFIG } from '../config/api';

export interface User {
  id: number;
  name: string;
  email: string;
  role: string; // "admin" | "user" | "leader" | "subjectLeader"
}

export interface AuthResponse {
  success: boolean;
  user?: User;
  message?: string;
}

export class AuthService {
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
      
      // Verificar si el login fue exitoso
      if (response.ok && data.success) {
        // Normalizar y guardar datos del usuario
        const user = this.normalizeUser(data.user || data);
        this.setLocalAuth(user);
        
        return { 
          success: true, 
          user,
          message: data.message || 'Inicio de sesión exitoso' 
        };
      }
      
      // Login falló
      return { 
        success: false, 
        message: data.message || 'Credenciales inválidas' 
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
      
      return { 
        isAuthenticated: false,
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
    // Parsear el ID si viene como string
    let userId: number;
    if (typeof userData.id === 'string') {
      userId = parseInt(userData.id, 10);
    } else {
      userId = userData.id;
    }

    return {
      id: userId,
      name: userData.name || userData.username || 'Usuario',
      email: userData.email || '',
      role: userData.role || 'user' // Por defecto "user" si no viene
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
    localStorage.removeItem(this.USER_KEY);
  }

  private static setLocalAuth(user: User): void {
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