/**
 * Store de UI
 * Maneja estados globales de la interfaz: modales, notificaciones, loading, etc.
 */

import { defineStore } from 'pinia'
import { ref } from 'vue'

export type NotificationType = 'success' | 'error' | 'warning' | 'info'

export interface Notification {
  id: string
  message: string
  type: NotificationType
  duration?: number
}

export interface ConfirmDialog {
  show: boolean
  title: string
  message: string
  confirmText?: string
  cancelText?: string
  onConfirm?: () => void
  onCancel?: () => void
}

export const useUIStore = defineStore('ui', () => {
  // State
  const notifications = ref<Notification[]>([])
  const confirmDialog = ref<ConfirmDialog>({
    show: false,
    title: '',
    message: '',
    confirmText: 'Confirmar',
    cancelText: 'Cancelar',
  })
  const sidebarCollapsed = ref(false)
  const globalLoading = ref(false)

  // Actions - Notificaciones
  function showNotification(
    message: string,
    type: NotificationType = 'info',
    duration: number = 3000
  ) {
    const id = `notif-${Date.now()}-${Math.random()}`
    
    notifications.value.push({
      id,
      message,
      type,
      duration,
    })

    // Auto-eliminar después de la duración
    if (duration > 0) {
      setTimeout(() => {
        removeNotification(id)
      }, duration)
    }
  }

  function removeNotification(id: string) {
    // Optimized: use filter for cleaner immutable update instead of findIndex + splice
    notifications.value = notifications.value.filter(n => n.id !== id)
  }

  function clearNotifications() {
    notifications.value = []
  }

  // Helpers específicos
  function showSuccess(message: string, duration?: number) {
    showNotification(message, 'success', duration)
  }

  function showError(message: string, duration?: number) {
    showNotification(message, 'error', duration)
  }

  function showWarning(message: string, duration?: number) {
    showNotification(message, 'warning', duration)
  }

  function showInfo(message: string, duration?: number) {
    showNotification(message, 'info', duration)
  }

  // Actions - Dialog de Confirmación
  function openConfirm(config: {
    title?: string
    message: string
    confirmText?: string
    cancelText?: string
    onConfirm?: () => void
    onCancel?: () => void
  }) {
    confirmDialog.value = {
      show: true,
      title: config.title || 'Confirmar acción',
      message: config.message,
      confirmText: config.confirmText || 'Confirmar',
      cancelText: config.cancelText || 'Cancelar',
      onConfirm: config.onConfirm,
      onCancel: config.onCancel,
    }
  }

  function closeConfirm(confirmed: boolean = false) {
    if (confirmed && confirmDialog.value.onConfirm) {
      confirmDialog.value.onConfirm()
    } else if (!confirmed && confirmDialog.value.onCancel) {
      confirmDialog.value.onCancel()
    }

    confirmDialog.value = {
      show: false,
      title: '',
      message: '',
      confirmText: 'Confirmar',
      cancelText: 'Cancelar',
    }
  }

  // Actions - Sidebar
  function toggleSidebar() {
    sidebarCollapsed.value = !sidebarCollapsed.value
  }

  function setSidebarCollapsed(collapsed: boolean) {
    sidebarCollapsed.value = collapsed
  }

  // Actions - Loading Global
  function setGlobalLoading(loading: boolean) {
    globalLoading.value = loading
  }

  return {
    // State
    notifications,
    confirmDialog,
    sidebarCollapsed,
    globalLoading,
    
    // Actions
    showNotification,
    removeNotification,
    clearNotifications,
    showSuccess,
    showError,
    showWarning,
    showInfo,
    openConfirm,
    closeConfirm,
    toggleSidebar,
    setSidebarCollapsed,
    setGlobalLoading,
  }
})
