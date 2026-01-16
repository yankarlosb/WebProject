/**
 * Store de Autenticación
 * Maneja el estado global del usuario autenticado y sus permisos
 */

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { authService } from '../services/auth'
import type { User } from '../types'
import { USER_ROLES, USER_ROLE_LABELS } from '../utils/constants'

export const useAuthStore = defineStore('auth', () => {
  // State
  const user = ref<User | null>(null)
  const isAuthenticated = ref(false)
  const isLoading = ref(false)

  // Getters
  const isAdmin = computed(() => user.value?.role === USER_ROLES.ADMIN)
  const isLeader = computed(() => user.value?.role === USER_ROLES.LEADER)
  const isSubjectLeader = computed(() => user.value?.role === USER_ROLES.SUBJECT_LEADER)
  const isLeaderOrSubjectLeader = computed(() => 
    user.value?.role === USER_ROLES.LEADER || user.value?.role === USER_ROLES.SUBJECT_LEADER
  )
  const isRegularUser = computed(() => user.value?.role === USER_ROLES.USER)
  const userName = computed(() => user.value?.name || 'Usuario')
  const userEmail = computed(() => user.value?.email || '')
  const userRole = computed(() => {
    const role = user.value?.role || USER_ROLES.USER
    return USER_ROLE_LABELS[role as keyof typeof USER_ROLE_LABELS] || 'Usuario'
  })
  const userRoleRaw = computed(() => user.value?.role || USER_ROLES.USER)
  const mustChangePassword = computed(() => user.value?.must_change_password ?? false)

  // Actions
  async function login(username: string, password: string) {
    isLoading.value = true
    try {
      const response = await authService.login(username, password)
      
      if (response.success && response.user) {
        user.value = response.user
        isAuthenticated.value = true
        return { success: true }
      }
      
      return { 
        success: false, 
        message: response.message || 'Error al iniciar sesión' 
      }
    } catch (error) {
      console.error('Error en login:', error)
      return { 
        success: false, 
        message: 'Error de conexión' 
      }
    } finally {
      isLoading.value = false
    }
  }

  async function checkAuth() {
    isLoading.value = true
    try {
      const result = await authService.checkAuth()
      
      if (result.isAuthenticated && result.user) {
        user.value = result.user
        isAuthenticated.value = true
        return true
      }
      
      // No autenticado, limpiar estado
      clearAuth()
      return false
    } catch (error) {
      console.error('Error verificando autenticación:', error)
      return false
    } finally {
      isLoading.value = false
    }
  }

  async function logout() {
    isLoading.value = true
    try {
      await authService.logout()
    } finally {
      clearAuth()
      isLoading.value = false
    }
  }

  function updateUser(updates: Partial<User>) {
    if (user.value) {
      user.value = { ...user.value, ...updates }
    }
  }

  function clearAuth() {
    user.value = null
    isAuthenticated.value = false
  }

  return {
    // State
    user,
    isAuthenticated,
    isLoading,
    
    // Getters
    isAdmin,
    isLeader,
    isSubjectLeader,
    isLeaderOrSubjectLeader,
    isRegularUser,
    userName,
    userEmail,
    userRole,
    userRoleRaw,
    mustChangePassword,
    
    // Actions
    login,
    checkAuth,
    logout,
    updateUser,
    clearAuth,
  }
})
