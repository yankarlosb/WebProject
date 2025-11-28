/**
 * Utilidades de validación y sanitización para formularios
 * Previene inyecciones SQL, XSS y otros ataques
 */

// Patrones de validación
const patterns = {
  // Solo alfanumérico y guión bajo (usernames)
  username: /^[a-zA-Z0-9_]+$/,
  
  // Letras (incluyendo acentos), espacios (nombres de personas)
  name: /^[a-zA-ZáéíóúÁÉÍÓÚüÜñÑ\s]+$/,
  
  // Email estándar
  email: /^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/,
  
  // Nombres de asignaturas: letras, números, espacios, guiones, puntos, paréntesis
  subjectName: /^[a-zA-Z0-9áéíóúÁÉÍÓÚüÜñÑ\s\-\.\(\)]+$/,
  
  // Solo números positivos
  positiveNumber: /^[0-9]+$/,
}

// Caracteres peligrosos para SQL/XSS
const dangerousChars = /[<>'"`;\\{}[\]|&$]/g

/**
 * Valida que un username solo contenga caracteres permitidos
 */
export function isValidUsername(value: string): boolean {
  if (!value || value.length < 3 || value.length > 50) return false
  return patterns.username.test(value)
}

/**
 * Valida que un nombre solo contenga letras y espacios
 */
export function isValidName(value: string): boolean {
  if (!value || value.length < 2 || value.length > 100) return false
  return patterns.name.test(value.trim())
}

/**
 * Valida formato de email
 */
export function isValidEmail(value: string): boolean {
  if (!value || value.length > 254) return false
  return patterns.email.test(value.trim())
}

/**
 * Valida nombre de asignatura
 */
export function isValidSubjectName(value: string): boolean {
  if (!value || value.length < 2 || value.length > 200) return false
  return patterns.subjectName.test(value.trim())
}

/**
 * Valida que sea un número positivo
 */
export function isPositiveNumber(value: number | string): boolean {
  const num = typeof value === 'string' ? parseInt(value, 10) : value
  return !isNaN(num) && num >= 0 && num <= 9999
}

/**
 * Valida contraseña (mínimo 8 caracteres)
 */
export function isValidPassword(value: string): boolean {
  return !!value && value.length >= 8 && value.length <= 128
}

/**
 * Sanitiza HTML para prevenir XSS
 * Escapa caracteres peligrosos
 */
export function sanitizeHtml(value: string): string {
  if (!value) return ''
  return value
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#x27;')
    .replace(/\//g, '&#x2F;')
}

/**
 * Sanitiza texto removiendo caracteres peligrosos
 */
export function sanitizeText(value: string): string {
  if (!value) return ''
  return value.replace(dangerousChars, '').trim()
}

/**
 * Valida y sanitiza un campo según su tipo
 * Retorna { valid: boolean, value: string, error?: string }
 */
export interface ValidationResult {
  valid: boolean
  value: string
  error?: string
}

export function validateField(
  value: string,
  type: 'username' | 'name' | 'email' | 'subject' | 'password' | 'text'
): ValidationResult {
  const trimmed = value?.trim() || ''
  
  switch (type) {
    case 'username':
      return {
        valid: isValidUsername(trimmed),
        value: trimmed,
        error: isValidUsername(trimmed) ? undefined : 'Usuario inválido'
      }
    
    case 'name':
      return {
        valid: isValidName(trimmed),
        value: trimmed,
        error: isValidName(trimmed) ? undefined : 'Nombre inválido'
      }
    
    case 'email':
      return {
        valid: isValidEmail(trimmed),
        value: trimmed,
        error: isValidEmail(trimmed) ? undefined : 'Email inválido'
      }
    
    case 'subject':
      return {
        valid: isValidSubjectName(trimmed),
        value: trimmed,
        error: isValidSubjectName(trimmed) ? undefined : 'Nombre de asignatura inválido'
      }
    
    case 'password':
      return {
        valid: isValidPassword(trimmed),
        value: trimmed,
        error: isValidPassword(trimmed) ? undefined : 'Contraseña inválida (mínimo 8 caracteres)'
      }
    
    case 'text':
    default:
      return {
        valid: true,
        value: sanitizeText(trimmed)
      }
  }
}

export default {
  isValidUsername,
  isValidName,
  isValidEmail,
  isValidSubjectName,
  isPositiveNumber,
  isValidPassword,
  sanitizeHtml,
  sanitizeText,
  validateField,
}
