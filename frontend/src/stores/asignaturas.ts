/**
 * Store de Asignaturas
 * Maneja el estado global de las asignaturas del sistema
 * Conectado a la API del backend
 */

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import AsignaturasService, { 
  type CreateAsignaturaData,
  type UpdateAsignaturaData,
  type SubjectLeader
} from '../services/asignaturas'

// Tipo de asignatura para el frontend (compatible con backend)
export interface Asignatura {
  id: number
  leader_id: number
  name: string
  year: string
  semester: string
  c: number | null
  cp: number | null
  s: number | null
  pl: number | null
  te: number | null
  t: number | null
  pp: number | null
  ec: number | null
  tc: number | null
  ef: number | null
  hours: number
  weeks: number | null
  date_start: string
  date_end: string
}

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
  async function loadAsignaturas() {
    isLoading.value = true
    error.value = null
    
    try {
      const data = await AsignaturasService.getAsignaturas()
      asignaturas.value = data
      return { success: true }
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Error al cargar asignaturas'
      console.error('Error cargando asignaturas:', err)
      return { success: false, error: error.value }
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
      await AsignaturasService.createAsignatura(data)
      // Recargar lista después de crear
      await loadAsignaturas()
      return { success: true }
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Error al crear asignatura'
      console.error('Error creando asignatura:', err)
      return { success: false, error: error.value }
    } finally {
      isLoading.value = false
    }
  }

  /**
   * Actualizar asignatura (solo SubjectLeaders - sus asignaturas)
   */
  async function updateAsignatura(id: number, data: UpdateAsignaturaData) {
    isLoading.value = true
    error.value = null
    
    try {
      await AsignaturasService.updateAsignatura(id, data)
      // Recargar lista después de actualizar
      await loadAsignaturas()
      return { success: true }
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Error al actualizar asignatura'
      console.error('Error actualizando asignatura:', err)
      return { success: false, error: error.value }
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
      await AsignaturasService.deleteAsignatura(id)
      // Recargar lista después de eliminar
      await loadAsignaturas()
      return { success: true }
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Error al eliminar asignatura'
      console.error('Error eliminando asignatura:', err)
      return { success: false, error: error.value }
    } finally {
      isLoading.value = false
    }
  }

  /**
   * Cargar lista de jefes de asignatura (solo Leaders)
   */
  async function loadSubjectLeaders() {
    isLoading.value = true
    error.value = null
    
    try {
      const data = await AsignaturasService.getSubjectLeaders()
      subjectLeaders.value = data
      return { success: true }
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Error al cargar jefes de asignatura'
      console.error('Error cargando jefes de asignatura:', err)
      return { success: false, error: error.value }
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
    loadAsignaturas,
    createAsignatura,
    updateAsignatura,
    deleteAsignatura,
    loadSubjectLeaders,
    getAsignaturaById,
    clearAsignaturas,
  }
})
