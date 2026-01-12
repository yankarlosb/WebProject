// Shared UI constants
export const yearOptions = [
  { value: '1ro', label: '1ro.ICS' },
  { value: '2do', label: '2do.ICS' },
  { value: '3ro', label: '3ro.ICS' },
  { value: '4to', label: '4to.ICS' },
]

export const tiposActividad = [
  { id: 'c', label: 'C' },
  { id: 'cp', label: 'CP' },
  { id: 's', label: 'S' },
  { id: 'pl', label: 'PL' },
  { id: 'te', label: 'TE' },
  { id: 't', label: 'T' },
  { id: 'pp', label: 'PP' },
  { id: 'ec', label: 'EC' },
]

// Tipos de actividad para el balance de carga (selectores en la tabla)
export const tiposActividadBalance = [
  { value: '', label: '-' },
  { value: 'C', label: 'C' },
  { value: 'CP', label: 'CP' },
  { value: 'S', label: 'S' },
  { value: 'PL', label: 'PL' },
  { value: 'TE', label: 'TE' },
  { value: 'T', label: 'T' },
  { value: 'PP', label: 'PP' },
  { value: 'EC', label: 'EC' },
]

// Tipos críticos (se colorean en rojo y no puede haber 3+ en la misma semana)
// T - Taller, TE - Tarea Extraclase, PP - Prueba Parcial
export const TIPOS_CRITICOS = ['T', 'TE', 'PP']

// Horas por tipo de actividad
// Cada clase regular = 2 horas
export const HORAS_POR_TIPO: Record<string, number> = {
  'C': 2,   // Conferencia
  'CP': 2,  // Clase Práctica
  'S': 2,   // Seminario
  'PL': 2,  // Práctica de Laboratorio
  'TE': 0,  // Tarea Extraclase (no cuenta como horas presenciales)
  'T': 2,   // Taller
  'PP': 2,  // Prueba Parcial
  'EC': 2,  // Examen Comprobatorio
}

// Tipos de actividad que cuentan como horas de clase
export const TIPOS_HORAS_CLASE = ['C', 'CP', 'S', 'PL', 'T', 'PP', 'EC']

// Tipos de exámenes para detección de conflictos (deprecated - usar TIPOS_CRITICOS)
export const TIPOS_EXAMENES = ['PP']  // Solo pruebas parciales para conflictos semanales

// Roles de usuario (sincronizados con backend jwt.rs)
export const USER_ROLES = {
  ADMIN: 'admin',
  USER: 'user',
  LEADER: 'leader',
  SUBJECT_LEADER: 'subjectLeader',
} as const

export type UserRole = typeof USER_ROLES[keyof typeof USER_ROLES]

export const USER_ROLE_LABELS: Record<UserRole, string> = {
  [USER_ROLES.ADMIN]: 'Administrador',
  [USER_ROLES.USER]: 'Usuario',
  [USER_ROLES.LEADER]: 'Líder',
  [USER_ROLES.SUBJECT_LEADER]: 'Jefe de Asignatura',
}

export const USER_ROLE_OPTIONS = [
  { value: USER_ROLES.ADMIN, label: USER_ROLE_LABELS[USER_ROLES.ADMIN] },
  { value: USER_ROLES.USER, label: USER_ROLE_LABELS[USER_ROLES.USER] },
  { value: USER_ROLES.LEADER, label: USER_ROLE_LABELS[USER_ROLES.LEADER] },
  { value: USER_ROLES.SUBJECT_LEADER, label: USER_ROLE_LABELS[USER_ROLES.SUBJECT_LEADER] },
]

export default {
  yearOptions,
  tiposActividad,
  USER_ROLES,
  USER_ROLE_LABELS,
  USER_ROLE_OPTIONS,
}
