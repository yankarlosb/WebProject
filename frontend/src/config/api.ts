/**
 * Configuración de la API
 * En desarrollo: Vite hace proxy a localhost:8000
 * En producción: El backend sirve todo desde localhost:8000
 */

const isDevelopment = import.meta.env.DEV;

export const API_CONFIG = {
  // En desarrollo usa el proxy de Vite, en producción usa la variable de entorno
  BASE_URL: isDevelopment ? '' : (import.meta.env.VITE_API_URL || ''),
  ENDPOINTS: {
    // Auth
    LOGIN: '/api/login',
    LOGOUT: '/api/logout',
    VERIFY: '/api/verify',
    // Users (Admin)
    USERS: '/api/users',
    SUBJECT_LEADERS: '/api/users/subject-leaders',
    // Profile
    PROFILE: '/api/profile',
    PROFILE_PASSWORD: '/api/profile/password',
    // Asignaturas
    ASIGNATURAS: '/api/asignaturas',
    // Balances
    BALANCES: '/api/balances',
    FRAGMENTS: '/api/fragments',
  }
};

export function getApiUrl(endpoint: string): string {
  return `${API_CONFIG.BASE_URL}${endpoint}`;
}

export default API_CONFIG;
