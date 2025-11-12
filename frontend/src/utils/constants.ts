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
  { id: 'pl', label: 'PL' },
  { id: 't', label: 'T' },
  { id: 'te', label: 'TE' },
  { id: 'pp', label: 'PP' },
  { id: 'ef', label: 'EF' },
]

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
