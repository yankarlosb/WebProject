import { ref } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useBalanceStore } from '../stores/balance'
import { useAsignaturasStore } from '../stores/asignaturas'
import { useUIStore } from '../stores/ui'

export function useBalanceForm() {
  const router = useRouter()
  const route = useRoute()
  const balanceStore = useBalanceStore()
  const asignaturasStore = useAsignaturasStore()
  const uiStore = useUIStore()

  const activeTab = ref('table')
  const isSaving = ref(false)
  const showAddSubjectModal = ref(false)
  const customSubjectName = ref('')

  const tabs = [
    { id: 'table', label: 'Tabla de Balance' },
    { id: 'calculations', label: 'Cálculos' },
  ]

  /**
   * Inicializa el balance cargando uno existente desde API o creando uno nuevo
   */
  async function initializeBalance() {
    balanceStore.isLoading = true
    
    try {
      const balanceId = route.query.id as string
      
      if (balanceId) {
        // Cargar balance existente desde la API
        const loaded = await balanceStore.loadBalance(parseInt(balanceId, 10))
        if (!loaded) {
          uiStore.showWarning('No se pudo cargar el balance')
          balanceStore.createNewBalance()
        }
      } else {
        // Crear nuevo balance vacío
        balanceStore.createNewBalance()
      }
    } catch (error) {
      console.error('Error inicializando balance:', error)
      uiStore.showError('Error al inicializar el balance')
      balanceStore.createNewBalance()
    } finally {
      balanceStore.isLoading = false
    }
  }

  /**
   * Marca el formulario como modificado
   */
  function markDirty() {
    balanceStore.isDirty = true
  }

  /**
   * Actualiza el valor de una celda específica (ahora con string para tipo de actividad)
   */
  function updateCellValue(subjectId: string, cellIndex: number, event: Event) {
    const target = event.target as HTMLSelectElement
    const value = target.value // String del tipo de actividad: 'C', 'CP', 'S', etc.
    balanceStore.updateSubjectValue(subjectId, cellIndex, value)
  }

  /**
   * Ejecuta los cálculos del balance
   */
  function calculateBalance() {
    balanceStore.calculateAll()
    uiStore.showSuccess('Cálculos actualizados')
    activeTab.value = 'calculations'
  }

  /**
   * Guarda el balance actual en la base de datos
   */
  async function saveBalance() {
    isSaving.value = true
    
    try {
      const result = await balanceStore.saveBalance()
      
      if (result.success) {
        uiStore.showSuccess(result.message)
        setTimeout(() => {
          router.push('/dashboard')
        }, 1000)
      } else {
        uiStore.showError(result.message)
      }
    } catch (error) {
      console.error('Error guardando:', error)
      uiStore.showError('Error al guardar el balance')
    } finally {
      isSaving.value = false
    }
  }

  /**
   * Abre el modal para agregar asignatura
   */
  function openAddSubjectModal() {
    customSubjectName.value = ''
    showAddSubjectModal.value = true
  }

  /**
   * Cierra el modal de agregar asignatura
   */
  function closeAddSubjectModal() {
    showAddSubjectModal.value = false
    customSubjectName.value = ''
  }

  /**
   * Agrega una asignatura existente al balance
   */
  function addExistingSubject(name: string) {
    balanceStore.addSubject(name)
    uiStore.showSuccess(`Asignatura "${name}" agregada`)
    closeAddSubjectModal()
  }

  /**
   * Agrega una asignatura personalizada al balance
   */
  function addCustomSubject() {
    if (!customSubjectName.value.trim()) return
    
    balanceStore.addSubject(customSubjectName.value.trim())
    uiStore.showSuccess('Asignatura agregada al balance')
    closeAddSubjectModal()
  }

  /**
   * Confirma y elimina una asignatura del balance
   */
  function confirmDeleteSubject(subjectId: string) {
    const subject = balanceStore.currentBalance?.subjects.find(s => s.id === subjectId)
    if (!subject) return

    uiStore.openConfirm({
      title: 'Eliminar Asignatura',
      message: `¿Estás seguro de que deseas eliminar "${subject.name}" del balance?`,
      confirmText: 'Sí, eliminar',
      cancelText: 'Cancelar',
      onConfirm: () => {
        balanceStore.removeSubject(subjectId)
        uiStore.showSuccess('Asignatura eliminada del balance')
      },
    })
  }

  /**
   * Configura advertencia de salida si hay cambios sin guardar
   */
  function setupUnsavedWarning() {
    if (balanceStore.hasUnsavedChanges) {
      window.onbeforeunload = () => true
    }
  }

  return {
    // State
    activeTab,
    isSaving,
    showAddSubjectModal,
    customSubjectName,
    tabs,
    
    // Stores
    balanceStore,
    asignaturasStore,
    uiStore,
    
    // Methods
    initializeBalance,
    markDirty,
    updateCellValue,
    calculateBalance,
    saveBalance,
    openAddSubjectModal,
    closeAddSubjectModal,
    addExistingSubject,
    addCustomSubject,
    confirmDeleteSubject,
    setupUnsavedWarning,
  }
}
