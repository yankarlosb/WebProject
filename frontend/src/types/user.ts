/**
 * Shared User type definitions
 * Single source of truth for user-related interfaces
 */

/**
 * Base user interface used across the application
 */
export interface User {
  id: number
  user_name: string
  name: string
  email: string
  role: string
}

/**
 * Extended user interface with token (used by admin operations)
 */
export interface UserWithToken extends User {
  token: string
  created_at?: string
}
