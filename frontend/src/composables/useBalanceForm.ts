/**
 * @deprecated This composable was designed for an older balance architecture
 * that used a local subjects array. The current architecture uses fragments
 * managed by the backend. Use the balance store directly instead.
 * 
 * The new architecture:
 * - Leader creates balance → selects asignaturas → fragments auto-created
 * - SubjectLeaders fill their fragments via BalanceFragmentEdit.vue
 * - Use balanceStore.createBalance(), balanceStore.updateFragment(), etc.
 */

import { ref } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useBalanceStore } from '../stores/balance'
import { useAsignaturasStore } from '../stores/asignaturas'
import { useUIStore } from '../stores/ui'

/**
 * @deprecated Use balance store methods directly
 */
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
   * @deprecated Initialize balance using store methods directly
   */
  async function initializeBalance() {
    balanceStore.isLoading = true
    
    try {
      const balanceId = route.query.id as string
      
      const loadBalancePromise = balanceId
        ? balanceStore.loadBalance(parseInt(balanceId, 10))
        : Promise.resolve(false)
      
      const fetchAsignaturasPromise = asignaturasStore.asignaturas.length === 0
        ? asignaturasStore.fetchAsignaturas()
        : Promise.resolve({ success: true })

      const [loaded] = await Promise.all([loadBalancePromise, fetchAsignaturasPromise])
      
      if (balanceId && !loaded) {
        uiStore.showWarning('No se pudo cargar el balance')
      }
    } catch (error) {
      console.error('Error inicializando balance:', error)
      uiStore.showError('Error al inicializar el balance')
    } finally {
      balanceStore.isLoading = false
    }
  }

  function markDirty() {
    balanceStore.isDirty = true
  }

  /**
   * @deprecated Fragment editing is handled via updateFragment()
   */
  async function saveBalance() {
    isSaving.value = true
    
    try {
      uiStore.showWarning('Esta función está deprecada. Use la vista de fragmentos.')
      router.push('/dashboard')
    } finally {
      isSaving.value = false
    }
  }

  function openAddSubjectModal() {
    customSubjectName.value = ''
    showAddSubjectModal.value = true
  }

  function closeAddSubjectModal() {
    showAddSubjectModal.value = false
    customSubjectName.value = ''
  }

  function setupUnsavedWarning() {
    if (balanceStore.hasUnsavedChanges) {
      window.onbeforeunload = () => true
    }
  }

  return {
    activeTab,
    isSaving,
    showAddSubjectModal,
    customSubjectName,
    tabs,
    balanceStore,
    asignaturasStore,
    uiStore,
    initializeBalance,
    markDirty,
    saveBalance,
    openAddSubjectModal,
    closeAddSubjectModal,
    setupUnsavedWarning,
  }
}
