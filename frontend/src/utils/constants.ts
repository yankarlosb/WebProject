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
  { id: 't', label: 'T' },
  { id: 'te', label: 'TE' },
  { id: 'pp', label: 'PP' },
  { id: 'ef', label: 'EF' },
]

// Tipos de actividad para el balance de carga (selectores en la tabla)
export const tiposActividadBalance = [
  { value: '', label: '-' },
  { value: 'C', label: 'C' },
  { value: 'CP', label: 'CP' },
  { value: 'S', label: 'S' },
  { value: 'PL', label: 'PL' },
  { value: 'T', label: 'T' },
  { value: 'TE', label: 'TE' },
  { value: 'PP', label: 'PP' },
]

// Horas por tipo de actividad
// Cada clase regular = 2 horas
// EF (Examen Final) y TE (Trabajo Extraclase) tienen su propio cálculo
export const HORAS_POR_TIPO: Record<string, number> = {
  'C': 2,   // Conferencia
  'CP': 2,  // Clase Práctica
  'S': 2,   // Seminario
  'PL': 2,  // Práctica de Laboratorio
  'T': 2,   // Tutoría
  'TE': 0,  // Trabajo Extraclase (no cuenta como horas presenciales)
  'PP': 2,  // Prueba Parcial
  'EF': 0,  // Examen Final (no cuenta como horas presenciales regulares)
}

// Tipos de actividad que cuentan como horas de clase
export const TIPOS_HORAS_CLASE = ['C', 'CP', 'S', 'PL', 'T', 'PP']

// Tipos de exámenes para detección de conflictos
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
