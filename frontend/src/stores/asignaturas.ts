/**
 * Store de Asignaturas
 * Maneja el estado global de las asignaturas del sistema
 */

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export interface Asignatura {
  id: string
  nombre: string
  periodo: string
  fechaInicio?: string
  fechaFin?: string
  semanas: string
  encuentros: number | null
  // Tipos de actividad
  c: number | null
  cp: number | null
  pl: number | null
  t: number | null
  te: number | null
  pp: number | null
  ef: number | null
}

const STORAGE_KEY = 'asignaturas'

export const useAsignaturasStore = defineStore('asignaturas', () => {
  // State
  const asignaturas = ref<Asignatura[]>([])
  const isLoading = ref(false)

  // Getters
  const asignaturasList = computed(() => asignaturas.value)
  const asignaturasCount = computed(() => asignaturas.value.length)
  
  const asignaturasByPeriodo = computed(() => (periodo: string) => {
    return asignaturas.value.filter(a => a.periodo === periodo)
  })

  // Actions
  function loadFromStorage() {
    try {
      const raw = localStorage.getItem(STORAGE_KEY)
      if (raw) {
        asignaturas.value = JSON.parse(raw)
      }
    } catch (error) {
      console.error('Error cargando asignaturas:', error)
      asignaturas.value = []
    }
  }

  function saveToStorage() {
    try {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(asignaturas.value))
    } catch (error) {
      console.error('Error guardando asignaturas:', error)
    }
  }

  function addAsignatura(asignatura: Omit<Asignatura, 'id'>) {
    const newAsignatura: Asignatura = {
      id: Date.now().toString(),
      ...asignatura,
    }
    
    asignaturas.value.unshift(newAsignatura)
    saveToStorage()
    
    return newAsignatura
  }

  function updateAsignatura(id: string, updates: Partial<Omit<Asignatura, 'id'>>) {
    const index = asignaturas.value.findIndex(a => a.id === id)
    
    if (index !== -1 && asignaturas.value[index]) {
      asignaturas.value[index] = {
        ...asignaturas.value[index],
        ...updates,
      } as Asignatura
      saveToStorage()
      return true
    }
    
    return false
  }

  function deleteAsignatura(id: string) {
    const index = asignaturas.value.findIndex(a => a.id === id)
    
    if (index !== -1) {
      asignaturas.value.splice(index, 1)
      saveToStorage()
      return true
    }
    
    return false
  }

  function getAsignaturaById(id: string) {
    return asignaturas.value.find(a => a.id === id)
  }

  function clearAsignaturas() {
    asignaturas.value = []
    saveToStorage()
  }

  // Inicializar cargando desde localStorage
  loadFromStorage()

  return {
    // State
    asignaturas,
    isLoading,
    
    // Getters
    asignaturasList,
    asignaturasCount,
    asignaturasByPeriodo,
    
    // Actions
    loadFromStorage,
    addAsignatura,
    updateAsignatura,
    deleteAsignatura,
    getAsignaturaById,
    clearAsignaturas,
  }
})
