/**
 * Store de Usuarios
 * Maneja el estado global de los usuarios del sistema (admin)
 */

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { usersService, type User, type CreateUserRequest, type UpdateUserRequest } from '../services/users'

export const useUsersStore = defineStore('users', () => {
  // State
  const users = ref<User[]>([])
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  // Getters
  const usersCount = computed(() => users.value.length)
  const adminUsers = computed(() => users.value.filter(u => u.role === 'admin'))
  
  const getUserById = computed(() => {
    return (id: number) => users.value.find(u => u.id === id)
  })

  const usersByRole = computed(() => {
    return (role: string) => users.value.filter(u => u.role === role)
  })

  // Actions

  /**
   * Cargar todos los usuarios desde el backend
   */
  async function fetchUsers() {
    isLoading.value = true
    error.value = null
    
    try {
      const response = await usersService.list()
      
      if (response.success && response.data) {
        users.value = response.data
        return { success: true }
      }
      
      error.value = response.message || 'Error al cargar usuarios'
      return { success: false, message: error.value }
    } catch (err) {
      error.value = 'Error de conexión'
      return { success: false, message: error.value }
    } finally {
      isLoading.value = false
    }
  }

  /**
   * Crear un nuevo usuario
   */
  async function createUser(userData: CreateUserRequest) {
    isLoading.value = true
    error.value = null
    
    try {
      const response = await usersService.create(userData)
      
      if (response.success) {
        await fetchUsers()
        return { success: true, message: response.message }
      }
      
      error.value = response.message || 'Error al crear usuario'
      return { success: false, message: error.value }
    } catch (err) {
      error.value = 'Error de conexión'
      return { success: false, message: error.value }
    } finally {
      isLoading.value = false
    }
  }

  /**
   * Actualizar un usuario existente
   */
  async function updateUser(id: number, userData: UpdateUserRequest) {
    isLoading.value = true
    error.value = null
    
    try {
      const response = await usersService.update(id, userData)
      
      if (response.success) {
        await fetchUsers()
        return { success: true, message: response.message }
      }
      
      error.value = response.message || 'Error al actualizar usuario'
      return { success: false, message: error.value }
    } catch (err) {
      error.value = 'Error de conexión'
      return { success: false, message: error.value }
    } finally {
      isLoading.value = false
    }
  }

  /**
   * Eliminar un usuario
   */
  async function deleteUser(id: number) {
    isLoading.value = true
    error.value = null
    
    try {
      const response = await usersService.delete(id)
      
      if (response.success) {
        users.value = users.value.filter(u => u.id !== id)
        return { success: true }
      }
      
      error.value = response.message || 'Error al eliminar usuario'
      return { success: false, message: error.value }
    } catch (err) {
      error.value = 'Error de conexión'
      return { success: false, message: error.value }
    } finally {
      isLoading.value = false
    }
  }

  /**
   * Limpiar errores
   */
  function clearError() {
    error.value = null
  }

  /**
   * Reiniciar el store
   */
  function reset() {
    users.value = []
    error.value = null
    isLoading.value = false
  }

  return {
    // State
    users,
    isLoading,
    error,
    
    // Getters
    usersCount,
    adminUsers,
    getUserById,
    usersByRole,
    
    // Actions
    fetchUsers,
    createUser,
    updateUser,
    deleteUser,
    clearError,
    reset,
  }
})
