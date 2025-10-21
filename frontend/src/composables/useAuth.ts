/**
 * Composable para gestionar la autenticación en componentes Vue
 * 
 * Uso:
 * ```ts
 * import { useAuth } from '@/composables/useAuth'
 * 
 * const { user, logout, isAuthenticated } = useAuth()
 * ```
 */

import { computed } from 'vue'
import { useRouter } from 'vue-router'
import AuthService from '../services/auth'

export function useAuth() {
  const router = useRouter()

  const user = computed(() => AuthService.getCurrentUser())
  
  const isAuthenticated = computed(() => AuthService.isLocallyAuthenticated())

  const logout = async () => {
    await AuthService.logout()
    router.push('/login')
  }

  return {
    user,
    isAuthenticated,
    logout
  }
}
