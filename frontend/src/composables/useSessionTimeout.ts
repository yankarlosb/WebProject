/**
 * Composable: Session Timeout Management
 * 
 * Handles automatic session timeout after inactivity.
 * Monitors user activity and logs out when timeout is reached.
 */

import { ref, onMounted, onUnmounted, watch } from 'vue'
import { useAuthStore } from '../stores/auth'
import { useUIStore } from '../stores/ui'
import { settingsService } from '../services/settings'

// Default timeout in minutes (fallback if API fails)
const DEFAULT_TIMEOUT_MINUTES = 30

// Activity events to monitor
const ACTIVITY_EVENTS = [
  'mousedown',
  'mousemove',
  'keypress',
  'scroll',
  'touchstart',
  'click',
]

export function useSessionTimeout() {
  const authStore = useAuthStore()
  const uiStore = useUIStore()
  
  const timeoutMinutes = ref(DEFAULT_TIMEOUT_MINUTES)
  const lastActivityTime = ref(Date.now())
  const isInitialized = ref(false)
  let timeoutCheckInterval: ReturnType<typeof setInterval> | null = null

  /**
   * Update last activity timestamp
   */
  const updateActivity = () => {
    lastActivityTime.value = Date.now()
  }

  /**
   * Check if session has timed out
   */
  const checkTimeout = () => {
    if (!authStore.isAuthenticated) return
    
    const now = Date.now()
    const timeoutMs = timeoutMinutes.value * 60 * 1000
    const timeSinceLastActivity = now - lastActivityTime.value
    
    if (timeSinceLastActivity >= timeoutMs) {
      // Session has timed out due to inactivity
      handleSessionTimeout()
    }
  }

  /**
   * Handle session timeout
   */
  const handleSessionTimeout = async () => {
    if (!authStore.isAuthenticated) return
    
    // Show notification
    uiStore.showWarning('Tu sesión ha expirado por inactividad', 5000)
    
    // Logout
    await authStore.logout()
  }

  /**
   * Fetch timeout setting from server
   */
  const fetchTimeoutSetting = async () => {
    try {
      const response = await settingsService.getPublic()
      if (response.success && response.data) {
        timeoutMinutes.value = response.data.session_timeout_minutes
      }
    } catch (error) {
      console.warn('Failed to fetch session timeout setting, using default:', DEFAULT_TIMEOUT_MINUTES)
    }
  }

  /**
   * Start monitoring activity
   */
  const startMonitoring = () => {
    // Add activity event listeners
    ACTIVITY_EVENTS.forEach(event => {
      document.addEventListener(event, updateActivity, { passive: true })
    })

    // Check timeout every minute
    timeoutCheckInterval = setInterval(checkTimeout, 60 * 1000)
    
    // Reset activity timestamp
    updateActivity()
  }

  /**
   * Stop monitoring activity
   */
  const stopMonitoring = () => {
    // Remove activity event listeners
    ACTIVITY_EVENTS.forEach(event => {
      document.removeEventListener(event, updateActivity)
    })

    // Clear interval
    if (timeoutCheckInterval) {
      clearInterval(timeoutCheckInterval)
      timeoutCheckInterval = null
    }
  }

  /**
   * Initialize session timeout monitoring
   */
  const initialize = async () => {
    if (isInitialized.value) return
    
    await fetchTimeoutSetting()
    
    if (authStore.isAuthenticated) {
      startMonitoring()
    }
    
    isInitialized.value = true
  }

  /**
   * Cleanup on unmount
   */
  const cleanup = () => {
    stopMonitoring()
    isInitialized.value = false
  }

  // Watch authentication state
  watch(() => authStore.isAuthenticated, (isAuth) => {
    if (isAuth) {
      startMonitoring()
    } else {
      stopMonitoring()
    }
  })

  // Lifecycle hooks for component usage
  onMounted(() => {
    initialize()
  })

  onUnmounted(() => {
    cleanup()
  })

  return {
    timeoutMinutes,
    lastActivityTime,
    initialize,
    cleanup,
    updateActivity,
  }
}

export default useSessionTimeout
