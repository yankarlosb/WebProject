/**
 * Store de Asignaturas
 * Maneja el estado global de las asignaturas del sistema
 * Conectado a la API del backend
 */

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { 
  asignaturasService,
  type Asignatura,
  type CreateAsignaturaData,
  type UpdateAsignaturaData,
  type SubjectLeader
} from '../services/asignaturas'

// Re-export Asignatura type for consumers of this store
export type { Asignatura }

export const useAsignaturasStore = defineStore('asignaturas', () => {
  // State
  const asignaturas = ref<Asignatura[]>([])
  const subjectLeaders = ref<SubjectLeader[]>([])
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  // Getters
  const asignaturasList = computed(() => asignaturas.value)
  const asignaturasCount = computed(() => asignaturas.value.length)
  
  const asignaturasByPeriodo = computed(() => (periodo: string) => {
    return asignaturas.value.filter(a => a.semester === periodo)
  })

  // Actions
  
  /**
   * Cargar asignaturas desde la API
   * El backend filtra automáticamente según el rol del usuario
   */
  async function fetchAsignaturas() {
    isLoading.value = true
    error.value = null
    
    try {
      const response = await asignaturasService.list()
      
      if (response.success && response.data) {
        asignaturas.value = response.data
        return { success: true }
      }
      
      error.value = response.message || 'Error al cargar asignaturas'
      return { success: false, message: error.value }
    } catch (err) {
      error.value = 'Error de conexión'
      return { success: false, message: error.value }
    } finally {
      isLoading.value = false
    }
  }

  /**
   * Crear nueva asignatura (solo Leaders)
   */
  async function createAsignatura(data: CreateAsignaturaData) {
    isLoading.value = true
    error.value = null
    
    try {
      const response = await asignaturasService.create(data)
      
      if (response.success) {
        await fetchAsignaturas()
        return { success: true, message: response.message }
      }
      
      error.value = response.message || 'Error al crear asignatura'
      return { success: false, message: error.value }
    } catch (err) {
      error.value = 'Error de conexión'
      return { success: false, message: error.value }
    } finally {
      isLoading.value = false
    }
  }

  /**
   * Actualizar asignatura
   */
  async function updateAsignatura(id: number, data: UpdateAsignaturaData) {
    isLoading.value = true
    error.value = null
    
    try {
      const response = await asignaturasService.update(id, data)
      
      if (response.success) {
        await fetchAsignaturas()
        return { success: true, message: response.message }
      }
      
      error.value = response.message || 'Error al actualizar asignatura'
      return { success: false, message: error.value }
    } catch (err) {
      error.value = 'Error de conexión'
      return { success: false, message: error.value }
    } finally {
      isLoading.value = false
    }
  }

  /**
   * Eliminar asignatura (solo Leaders)
   */
  async function deleteAsignatura(id: number) {
    isLoading.value = true
    error.value = null
    
    try {
      const response = await asignaturasService.delete(id)
      
      if (response.success) {
        asignaturas.value = asignaturas.value.filter(a => a.id !== id)
        return { success: true }
      }
      
      error.value = response.message || 'Error al eliminar asignatura'
      return { success: false, message: error.value }
    } catch (err) {
      error.value = 'Error de conexión'
      return { success: false, message: error.value }
    } finally {
      isLoading.value = false
    }
  }

  /**
   * Cargar lista de jefes de asignatura (solo Leaders)
   */
  async function fetchSubjectLeaders() {
    isLoading.value = true
    error.value = null
    
    try {
      const response = await asignaturasService.getSubjectLeaders()
      
      if (response.success && response.data) {
        subjectLeaders.value = response.data
        return { success: true }
      }
      
      error.value = response.message || 'Error al cargar jefes de asignatura'
      return { success: false, message: error.value }
    } catch (err) {
      error.value = 'Error de conexión'
      return { success: false, message: error.value }
    } finally {
      isLoading.value = false
    }
  }

  function getAsignaturaById(id: number) {
    return asignaturas.value.find(a => a.id === id)
  }

  function clearAsignaturas() {
    asignaturas.value = []
    subjectLeaders.value = []
    error.value = null
  }

  return {
    // State
    asignaturas,
    subjectLeaders,
    isLoading,
    error,
    
    // Getters
    asignaturasList,
    asignaturasCount,
    asignaturasByPeriodo,
    
    // Actions
    fetchAsignaturas,
    createAsignatura,
    updateAsignatura,
    deleteAsignatura,
    fetchSubjectLeaders,
    getAsignaturaById,
    clearAsignaturas,
  }
})
