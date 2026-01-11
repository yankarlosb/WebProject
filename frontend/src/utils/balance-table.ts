/**
 * Shared utilities for balance table components
 * Centralizes duplicated logic across BalanceWeekTable, BalanceFinalTable, 
 * BalanceViewTable, and BalanceViewFinalTable components
 */

/**
 * Converts legacy numeric values to string or returns string as-is.
 * Used for rendering cell values in balance tables.
 * 
 * Legacy numeric values (from older data format where 0 meant empty and 
 * positive integers represented activity counts) are converted to empty 
 * strings since the current format uses string codes ('C', 'CP', etc.).
 * 
 * @param value - Cell value that could be number (legacy format) or string (current format)
 * @returns Empty string for falsy/legacy numeric values, or the string value
 */
export function getCellValue(value: number | string | undefined): string {
  if (value === undefined || value === null || value === 0 || value === '') {
    return ''
  }
  // Legacy numeric values (positive integers from old format) are treated as empty
  // Current format uses string codes: 'C', 'CP', 'S', 'PL', 'T', 'TE', 'PP'
  if (typeof value === 'number') {
    return ''
  }
  return value
}

/**
 * Color scheme configuration for balance table components
 * Supports blue, purple, and green color themes
 */
export type ColorScheme = 'blue' | 'purple' | 'green'

export interface ColorClasses {
  border: string
  header: string
  headerBadge: string
  tableHeader: string
  headerText: string
  rowHover?: string
  focusRing?: string
  cellFilled: string
}

/**
 * Color class mappings for editable table components (with hover and focus states)
 */
export const editableColorClasses: Record<ColorScheme, ColorClasses> = {
  blue: {
    border: 'border-blue-200',
    header: 'bg-gradient-to-r from-blue-600 to-blue-500',
    headerBadge: 'text-blue-100',
    tableHeader: 'bg-blue-50',
    headerText: 'text-blue-700',
    rowHover: 'hover:bg-blue-50/50 transition-colors',
    focusRing: 'focus:ring-blue-500 focus:border-blue-500',
    cellFilled: 'bg-blue-50'
  },
  purple: {
    border: 'border-purple-200',
    header: 'bg-gradient-to-r from-purple-600 to-purple-500',
    headerBadge: 'text-purple-100',
    tableHeader: 'bg-purple-50',
    headerText: 'text-purple-700',
    rowHover: 'hover:bg-purple-50/50 transition-colors',
    focusRing: 'focus:ring-purple-500 focus:border-purple-500',
    cellFilled: 'bg-purple-50'
  },
  green: {
    border: 'border-green-200',
    header: 'bg-gradient-to-r from-green-600 to-green-500',
    headerBadge: 'text-green-100',
    tableHeader: 'bg-green-50',
    headerText: 'text-green-700',
    rowHover: 'hover:bg-green-50/50 transition-colors',
    focusRing: 'focus:ring-green-500 focus:border-green-500',
    cellFilled: 'bg-green-50'
  }
} as const

/**
 * Color class mappings for read-only view table components
 */
export const viewColorClasses: Record<ColorScheme, ColorClasses> = {
  blue: {
    border: 'border-blue-200',
    header: 'bg-gradient-to-r from-blue-600 to-blue-500',
    headerBadge: 'text-blue-100',
    tableHeader: 'bg-blue-50',
    headerText: 'text-blue-700',
    cellFilled: 'bg-blue-50 text-blue-800'
  },
  purple: {
    border: 'border-purple-200',
    header: 'bg-gradient-to-r from-purple-600 to-purple-500',
    headerBadge: 'text-purple-100',
    tableHeader: 'bg-purple-50',
    headerText: 'text-purple-700',
    cellFilled: 'bg-purple-50 text-purple-800'
  },
  green: {
    border: 'border-green-200',
    header: 'bg-gradient-to-r from-green-600 to-green-500',
    headerBadge: 'text-green-100',
    tableHeader: 'bg-green-50',
    headerText: 'text-green-700',
    cellFilled: 'bg-green-50 text-green-800'
  }
} as const

/**
 * Interface for week group used in balance tables
 */
export interface WeekGroup {
  start: number
  end: number
  weeks: number[]
  startIndex: number
}

// Cache for generated week groups to avoid redundant calculations
// Key format: `${totalWeeks}-${groupSize}`
const weekGroupsCache = new Map<string, WeekGroup[]>()

/**
 * Generates week groups for balance tables (groups of 4 weeks each)
 * Results are cached for better performance with repeated calls.
 * 
 * @param totalWeeks - Total number of weeks in the balance
 * @param groupSize - Number of weeks per group (default: 4)
 * @returns Array of week groups with start/end indices and week numbers
 */
export function generateWeekGroups(totalWeeks: number, groupSize: number = 4): WeekGroup[] {
  const cacheKey = `${totalWeeks}-${groupSize}`
  
  // Return cached result if available
  const cached = weekGroupsCache.get(cacheKey)
  if (cached) {
    return cached
  }
  
  const groups: WeekGroup[] = []
  
  for (let i = 0; i < totalWeeks; i += groupSize) {
    const start = i + 1
    const end = Math.min(i + groupSize, totalWeeks)
    const weeks: number[] = []
    for (let w = start; w <= end; w++) {
      weeks.push(w)
    }
    groups.push({
      start,
      end,
      weeks,
      startIndex: i * 4, // 4 cells per week (Monday, Tuesday, Wednesday, Thursday)
    })
  }
  
  // Cache the result (limit cache size to prevent memory leaks)
  if (weekGroupsCache.size >= 50) {
    // Clear ~20% of the cache when threshold is reached to reduce cleanup frequency
    const keysToDelete: string[] = []
    const iterator = weekGroupsCache.keys()
    for (let i = 0; i < 10; i++) {
      const key = iterator.next().value
      if (key) {
        keysToDelete.push(key)
      }
    }
    keysToDelete.forEach(key => weekGroupsCache.delete(key))
  }
  weekGroupsCache.set(cacheKey, groups)
  
  return groups
}

// ============================================================================
// Week Dates Calculation
// ============================================================================

/** Período no académico */
export interface NonAcademicPeriod {
  start: string  // 'YYYY-MM-DD'
  end: string    // 'YYYY-MM-DD'
  name: string
}

/** Información de fecha de una semana */
export interface WeekDateInfo {
  weekNumber: number       // Número de semana (1-15)
  startDate: Date          // Fecha de inicio (lunes)
  endDate: Date            // Fecha de fin (jueves, ya que son 4 días lectivos)
  displayStart: string     // Formato corto para mostrar: "12/01"
  displayEnd: string       // Formato corto: "15/01"
  displayRange: string     // Formato completo: "12/01 - 15/01"
}

/**
 * Normaliza una fecha a medianoche local (sin componente de hora)
 * Esto evita problemas de zona horaria al comparar fechas
 */
function normalizeToLocalMidnight(date: Date): Date {
  return new Date(date.getFullYear(), date.getMonth(), date.getDate())
}

/**
 * Parsea una fecha string YYYY-MM-DD a Date local (medianoche local)
 * Evita problemas de UTC al parsear fechas sin hora
 */
function parseLocalDate(dateStr: string): Date {
  const parts = dateStr.split('-').map(Number)
  const year = parts[0] ?? 2025
  const month = parts[1] ?? 1
  const day = parts[2] ?? 1
  return new Date(year, month - 1, day) // month es 0-indexed
}

/**
 * Verifica si una fecha cae dentro de algún período no académico
 * Usa comparación de fechas normalizadas para evitar problemas de zona horaria
 */
function isInNonAcademicPeriod(date: Date, periods: NonAcademicPeriod[]): boolean {
  // Normalizar la fecha a medianoche local para comparación consistente
  const normalizedDate = normalizeToLocalMidnight(date)
  const dateTime = normalizedDate.getTime()
  
  for (const period of periods) {
    // Parsear las fechas del período como fechas locales
    const start = parseLocalDate(period.start).getTime()
    const end = parseLocalDate(period.end).getTime()
    
    // Incluye tanto la fecha de inicio como la de fin en el período no académico
    if (dateTime >= start && dateTime <= end) {
      return true
    }
  }
  return false
}

/**
 * Verifica si una fecha es fin de semana (sábado=6 o domingo=0)
 */
function isWeekend(date: Date): boolean {
  const day = date.getDay()
  return day === 0 || day === 6
}

/**
 * Obtener el siguiente día hábil (no fin de semana, no en período no académico)
 */
function getNextWorkingDay(date: Date, nonAcademicPeriods: NonAcademicPeriod[]): Date {
  const next = new Date(date)
  while (isWeekend(next) || isInNonAcademicPeriod(next, nonAcademicPeriods)) {
    next.setDate(next.getDate() + 1)
  }
  return next
}

/**
 * Calcula las fechas de cada semana académica
 * 
 * @param startDate - Fecha de inicio del semestre (YYYY-MM-DD)
 * @param totalWeeks - Número total de semanas lectivas
 * @param nonAcademicPeriods - Períodos no académicos (vacaciones, feriados)
 * @returns Array con información de fechas para cada semana
 */
export function calculateWeekDates(
  startDate: string,
  totalWeeks: number,
  nonAcademicPeriods: NonAcademicPeriod[] = []
): WeekDateInfo[] {
  const weeks: WeekDateInfo[] = []
  // Usar parseLocalDate para evitar problemas de zona horaria
  let currentDate = parseLocalDate(startDate)
  
  // Asegurar que empezamos en día hábil
  currentDate = getNextWorkingDay(currentDate, nonAcademicPeriods)
  
  for (let weekNum = 1; weekNum <= totalWeeks; weekNum++) {
    // Inicio de la semana (asegurar que es día hábil - lunes)
    const weekStart = getNextWorkingDay(currentDate, nonAcademicPeriods)
    
    // Encontrar el final de la semana (5 días laborables: L, M, X, J, V)
    // Avanzar 4 días desde el inicio (para llegar al viernes)
    let workingDays = 0
    let weekEnd = new Date(weekStart)
    
    while (workingDays < 4) { // 4 porque ya tenemos el día de inicio (total 5 días)
      weekEnd.setDate(weekEnd.getDate() + 1)
      if (!isWeekend(weekEnd) && !isInNonAcademicPeriod(weekEnd, nonAcademicPeriods)) {
        workingDays++
      }
    }
    
    // Formatear fechas para mostrar
    const formatShort = (d: Date) => {
      const day = d.getDate().toString().padStart(2, '0')
      const month = (d.getMonth() + 1).toString().padStart(2, '0')
      return `${day}/${month}`
    }
    
    weeks.push({
      weekNumber: weekNum,
      startDate: weekStart,
      endDate: weekEnd,
      displayStart: formatShort(weekStart),
      displayEnd: formatShort(weekEnd),
      displayRange: `${formatShort(weekStart)} - ${formatShort(weekEnd)}`,
    })
    
    // Avanzar al siguiente lunes (saltar el fin de semana)
    currentDate = new Date(weekEnd)
    currentDate.setDate(currentDate.getDate() + 1)
    currentDate = getNextWorkingDay(currentDate, nonAcademicPeriods)
  }
  
  return weeks
}

/**
 * Calcula las fechas de las semanas finales (consultas y exámenes)
 * Continúa después de las semanas lectivas
 */
export function calculateFinalWeeksDates(
  startDate: string,
  lectiveWeeks: number,
  nonAcademicPeriods: NonAcademicPeriod[] = [],
  finalWeeksCount: number = 2 // Semana de consultas + Semana de exámenes
): WeekDateInfo[] {
  // Primero calculamos las semanas lectivas para saber dónde terminan
  const lectiveWeekDates = calculateWeekDates(startDate, lectiveWeeks, nonAcademicPeriods)
  
  if (lectiveWeekDates.length === 0) return []
  
  // La semana final empieza el lunes después de la última semana lectiva
  const lastLectiveWeek = lectiveWeekDates[lectiveWeekDates.length - 1]
  if (!lastLectiveWeek) return []
  
  let currentDate = new Date(lastLectiveWeek.endDate)
  currentDate.setDate(currentDate.getDate() + 1)
  currentDate = getNextWorkingDay(currentDate, nonAcademicPeriods)
  
  const finalWeeks: WeekDateInfo[] = []
  
  for (let i = 0; i < finalWeeksCount; i++) {
    const weekStart = getNextWorkingDay(currentDate, nonAcademicPeriods)
    
    let workingDays = 0
    let weekEnd = new Date(weekStart)
    
    while (workingDays < 4) {
      weekEnd.setDate(weekEnd.getDate() + 1)
      if (!isWeekend(weekEnd) && !isInNonAcademicPeriod(weekEnd, nonAcademicPeriods)) {
        workingDays++
      }
    }
    
    const formatShort = (d: Date) => {
      const day = d.getDate().toString().padStart(2, '0')
      const month = (d.getMonth() + 1).toString().padStart(2, '0')
      return `${day}/${month}`
    }
    
    finalWeeks.push({
      weekNumber: lectiveWeeks + i + 1, // Continúa la numeración
      startDate: weekStart,
      endDate: weekEnd,
      displayStart: formatShort(weekStart),
      displayEnd: formatShort(weekEnd),
      displayRange: `${formatShort(weekStart)} - ${formatShort(weekEnd)}`,
    })
    
    currentDate = new Date(weekEnd)
    currentDate.setDate(currentDate.getDate() + 1)
    currentDate = getNextWorkingDay(currentDate, nonAcademicPeriods)
  }
  
  return finalWeeks
}

/**
 * Cache para fechas de semanas calculadas
 */
const weekDatesCache = new Map<string, WeekDateInfo[]>()
const finalWeekDatesCache = new Map<string, WeekDateInfo[]>()

/**
 * Versión cacheada de calculateWeekDates
 */
export function getWeekDates(
  startDate: string,
  totalWeeks: number,
  nonAcademicPeriods: NonAcademicPeriod[] = []
): WeekDateInfo[] {
  const cacheKey = `${startDate}-${totalWeeks}-${JSON.stringify(nonAcademicPeriods)}`
  
  const cached = weekDatesCache.get(cacheKey)
  if (cached) {
    return cached
  }
  
  const result = calculateWeekDates(startDate, totalWeeks, nonAcademicPeriods)
  
  // Limitar tamaño del cache
  if (weekDatesCache.size >= 20) {
    const firstKey = weekDatesCache.keys().next().value
    if (firstKey) {
      weekDatesCache.delete(firstKey)
    }
  }
  
  weekDatesCache.set(cacheKey, result)
  return result
}

/**
 * Versión cacheada de calculateFinalWeeksDates
 */
export function getFinalWeeksDates(
  startDate: string,
  lectiveWeeks: number,
  nonAcademicPeriods: NonAcademicPeriod[] = []
): WeekDateInfo[] {
  const cacheKey = `final-${startDate}-${lectiveWeeks}-${JSON.stringify(nonAcademicPeriods)}`
  
  const cached = finalWeekDatesCache.get(cacheKey)
  if (cached) {
    return cached
  }
  
  const result = calculateFinalWeeksDates(startDate, lectiveWeeks, nonAcademicPeriods)
  
  if (finalWeekDatesCache.size >= 20) {
    const firstKey = finalWeekDatesCache.keys().next().value
    if (firstKey) {
      finalWeekDatesCache.delete(firstKey)
    }
  }
  
  finalWeekDatesCache.set(cacheKey, result)
  return result
}
