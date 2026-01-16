// Servicio de auditoría para el frontend

import { httpGet, httpPost, type ServiceResponse } from './http'

// Tipos para logs de auditoría
export interface AuditLog {
  id: number
  user_id: number | null
  user_name: string | null
  event_type: string
  category: string
  entity_type: string | null
  entity_id: number | null
  description: string
  ip_address: string | null
  success: boolean
  error_message: string | null
  created_at: string | null
}

export interface AuditStats {
  total_logs: number
  recent_logins: number
  recent_errors: number
}

export interface CleanupResponse {
  deleted_count: number
}

// Mapeo de tipos de eventos a iconos/colores para el frontend
export const EVENT_TYPE_CONFIG: Record<string, { label: string; color: string; bgColor: string }> = {
  LOGIN: { label: 'Inicio de sesión', color: 'text-green-600', bgColor: 'bg-green-500' },
  LOGOUT: { label: 'Cierre de sesión', color: 'text-gray-600', bgColor: 'bg-gray-500' },
  LOGIN_FAILED: { label: 'Login fallido', color: 'text-red-600', bgColor: 'bg-red-500' },
  CREATE: { label: 'Creación', color: 'text-blue-600', bgColor: 'bg-blue-500' },
  UPDATE: { label: 'Actualización', color: 'text-yellow-600', bgColor: 'bg-yellow-500' },
  DELETE: { label: 'Eliminación', color: 'text-red-600', bgColor: 'bg-red-500' },
  ERROR: { label: 'Error', color: 'text-red-600', bgColor: 'bg-red-500' },
  ACCESS_DENIED: { label: 'Acceso denegado', color: 'text-orange-600', bgColor: 'bg-orange-500' },
  SETTINGS_UPDATED: { label: 'Config. actualizada', color: 'text-purple-600', bgColor: 'bg-purple-500' },
  EXPORT: { label: 'Exportación', color: 'text-teal-600', bgColor: 'bg-teal-500' },
}

export const CATEGORY_CONFIG: Record<string, { label: string; color: string }> = {
  SECURITY: { label: 'Seguridad', color: 'text-red-600' },
  FUNCTIONAL: { label: 'Funcional', color: 'text-blue-600' },
}

/**
 * Obtiene los logs de auditoría recientes
 */
export async function getAuditLogs(): Promise<ServiceResponse<AuditLog[]>> {
  return httpGet<AuditLog[]>('/api/audit/logs', 'Error al obtener logs de auditoría')
}

/**
 * Obtiene solo los logs de seguridad
 */
export async function getSecurityLogs(): Promise<ServiceResponse<AuditLog[]>> {
  return httpGet<AuditLog[]>('/api/audit/logs/security', 'Error al obtener logs de seguridad')
}

/**
 * Obtiene estadísticas de auditoría
 */
export async function getAuditStats(): Promise<ServiceResponse<AuditStats>> {
  return httpGet<AuditStats>('/api/audit/stats', 'Error al obtener estadísticas')
}

/**
 * Formatea una fecha de log para mostrar
 */
export function formatLogDate(dateStr: string | null): string {
  if (!dateStr) return 'N/A'
  
  try {
    const date = new Date(dateStr)
    return new Intl.DateTimeFormat('es-ES', {
      day: '2-digit',
      month: '2-digit',
      year: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
    }).format(date)
  } catch {
    return dateStr
  }
}

/**
 * Obtiene la configuración visual para un tipo de evento
 */
export function getEventConfig(eventType: string) {
  return EVENT_TYPE_CONFIG[eventType] || { 
    label: eventType, 
    color: 'text-gray-600', 
    bgColor: 'bg-gray-500' 
  }
}

/**
 * Obtiene la configuración visual para una categoría
 */
export function getCategoryConfig(category: string) {
  return CATEGORY_CONFIG[category] || { 
    label: category, 
    color: 'text-gray-600' 
  }
}

/**
 * Ejecuta la limpieza de logs antiguos según la configuración de retención
 */
export async function cleanupOldLogs(): Promise<ServiceResponse<CleanupResponse>> {
  return httpPost<CleanupResponse>('/api/audit/cleanup', {}, 'Error al limpiar logs antiguos')
}

/**
 * Exporta las trazas de auditoría a partir de todos los registros disponibles
 * Devuelve un blob para descargar el archivo .log
 */
export async function downloadLogs(): Promise<Blob | null> {
  try {
    // Necesitamos usar fetch directamente porque los helpers http* asumen JSON
    // y queremos recibir un blob/archivo
    const response = await fetch('/api/audit/export', {
      method: 'GET',
    })

    if (!response.ok) {
      throw new Error('Error al descargar los logs')
    }

    return await response.blob()
  } catch (error) {
    console.error('Error downloading audit logs:', error)
    return null
  }
}

const auditService = {
  getAuditLogs,
  getSecurityLogs,
  getAuditStats,
  formatLogDate,
  getEventConfig,
  getCategoryConfig,
  cleanupOldLogs,
  downloadLogs
}

export default auditService
