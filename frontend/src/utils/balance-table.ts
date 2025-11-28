/**
 * Shared utilities for balance table components
 * Centralizes duplicated logic across BalanceWeekTable, BalanceFinalTable, 
 * BalanceViewTable, and BalanceViewFinalTable components
 */

/**
 * Converts legacy numeric values to string or returns string as-is.
 * Used for rendering cell values in balance tables.
 * 
 * @param value - Cell value that could be number (legacy) or string (current)
 * @returns Empty string for falsy/legacy values, or the string value
 */
export function getCellValue(value: number | string | undefined): string {
  if (value === undefined || value === null || value === 0 || value === '') {
    return ''
  }
  // Legacy numeric values are treated as empty (migration support)
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

/**
 * Generates week groups for balance tables (groups of 4 weeks each)
 * 
 * @param totalWeeks - Total number of weeks in the balance
 * @param groupSize - Number of weeks per group (default: 4)
 * @returns Array of week groups with start/end indices and week numbers
 */
export function generateWeekGroups(totalWeeks: number, groupSize: number = 4): WeekGroup[] {
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
      startIndex: i * 4, // 4 cells per week (L, M, X, J)
    })
  }
  
  return groups
}
