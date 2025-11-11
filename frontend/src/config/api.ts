/**
 * Configuración de la API
 * En desarrollo: Vite hace proxy a localhost:8000
 * En producción: El backend sirve todo desde localhost:8000
 */

const isDevelopment = import.meta.env.DEV;

export const API_CONFIG = {
  // En desarrollo usa el proxy de Vite, en producción usa la misma URL
  BASE_URL: isDevelopment ? '' : '',
  ENDPOINTS: {
    LOGIN: '/api/login',
    LOGOUT: '/api/logout',
    VERIFY: '/api/verify',
    BALANCE: '/api/balance',
    ASIGNATURAS: '/api/asignaturas',
    USUARIOS: '/api/usuarios',
    ADMINISTRAR: '/api/manager',
  }
};

export function getApiUrl(endpoint: string): string {
  return `${API_CONFIG.BASE_URL}${endpoint}`;
}

export default API_CONFIG;
