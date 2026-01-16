<!--
  Dashboard - Vista principal rediseñada
  Usa el nuevo sistema de componentes y stores
  Balances cargados desde la API (base de datos)
  SubjectLeaders ven sus fragmentos pendientes
  Admin ve panel de administración con accesos rápidos
-->
<template>
  <AppLayout>
    <!-- ========== DASHBOARD ADMIN ========== -->
    <template v-if="authStore.isAdmin">
      <!-- Welcome Section Admin -->
      <AppCard class="mb-8">
        <div class="flex flex-col md:flex-row md:items-center md:justify-between gap-4">
          <div>
            <h2 class="text-3xl font-bold text-red-700">
              Panel de Administración
            </h2>
            <p class="text-gray-600 mt-2">
              Bienvenido, {{ authStore.user?.name || authStore.userName }}. Gestiona el sistema desde aquí.
            </p>
          </div>
          <div class="flex items-center gap-2 text-sm text-red-600 bg-red-50 px-4 py-2 rounded-lg">
            <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
            </svg>
            <span class="font-medium">Acceso de administrador</span>
          </div>
        </div>
      </AppCard>

      <!-- Estadísticas Admin -->
      <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
        <StatsCard
          title="Usuarios"
          :value="adminStats.totalUsers"
          icon="users"
          color="blue"
        />
        <StatsCard
          title="Asignaturas"
          :value="asignaturasStore.asignaturasCount"
          icon="book"
          color="green"
        />
        <StatsCard
          title="Balances"
          :value="balanceStore.balancesCount"
          icon="document"
          color="purple"
        />
        <StatsCard
          title="Eventos Hoy"
          :value="adminStats.eventsToday"
          icon="clock"
          color="yellow"
        />
      </div>

      <!-- Accesos Rápidos Admin -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mb-8">
        <QuickAccessCard
          title="Gestión de Usuarios"
          description="Crear, editar y administrar cuentas de usuario"
          icon="users"
          @click="$router.push('/configuracion')"
        />
        <QuickAccessCard
          title="Auditoría"
          description="Revisar logs de actividad y seguridad"
          icon="shield"
          @click="goToAudit"
        />
        <QuickAccessCard
          title="Configuración"
          description="Ajustes de seguridad, sesión y políticas"
          icon="settings"
          @click="goToSettings"
        />
      </div>

      <!-- Actividad Reciente -->
      <AppCard title="Actividad Reciente">
        <div v-if="isLoadingAudit" class="text-center py-8">
          <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600 mx-auto"></div>
          <p class="text-gray-600 mt-2 text-sm">Cargando actividad...</p>
        </div>
        
        <div v-else-if="recentAuditLogs.length === 0" class="text-center py-8 text-gray-500">
          No hay actividad reciente
        </div>
        
        <div v-else class="space-y-3">
          <div
            v-for="log in recentAuditLogs"
            :key="log.id"
            class="flex items-start gap-3 p-3 bg-gray-50 rounded-lg"
          >
            <div
              class="w-2 h-2 rounded-full mt-2 flex-shrink-0"
              :class="log.success ? 'bg-green-500' : 'bg-red-500'"
            ></div>
            <div class="flex-1 min-w-0">
              <div class="text-sm font-medium text-gray-900">
                <p class="whitespace-pre-wrap break-words">
                  {{ log.description }}
                </p>
              </div>
              <p class="text-xs text-gray-500 mt-1">
                <span v-if="log.user_name">{{ log.user_name }} • </span>
                {{ formatLogDate(log.created_at) }}
              </p>
            </div>
            <span
              class="text-xs px-2 py-0.5 rounded flex-shrink-0"
              :class="log.category === 'SECURITY' ? 'bg-red-100 text-red-700' : 'bg-blue-100 text-blue-700'"
            >
              {{ log.category === 'SECURITY' ? 'Seguridad' : 'Funcional' }}
            </span>
          </div>
        </div>
        
        <div class="mt-4 pt-4 border-t border-gray-200 text-center">
          <button
            @click="goToAudit"
            class="text-blue-600 hover:text-blue-800 text-sm font-medium"
          >
            Ver todos los logs →
          </button>
        </div>
      </AppCard>
    </template>

    <!-- ========== DASHBOARD LEADER/USER ========== -->
    <template v-else>
    <!-- Welcome Section -->
    <AppCard class="mb-8">
      <div class="flex flex-col md:flex-row md:items-center md:justify-between gap-4">
        <div>
          <h2 class="text-3xl font-bold text-blue-700">
            ¡Bienvenido, {{ authStore.user?.name || authStore.userName }}!
          </h2>
          <p class="text-gray-600 mt-2">
            Sistema de Balance de Carga Docente - Facultad de Ciberseguridad
          </p>
        </div>
        <AppButton
          v-if="authStore.isLeader"
          variant="primary"
          size="lg"
          @click="createNewBalance"
        >
          <template #icon>
            <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
            </svg>
          </template>
          Nuevo Balance
        </AppButton>
      </div>
    </AppCard>

    <!-- Estadísticas -->
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
      <StatsCard
        title="Años Académicos"
        :value="stats.academicYears"
        icon="calendar"
        color="blue"
      />
      <StatsCard
        title="Asignaturas"
        :value="asignaturasStore.asignaturasCount"
        icon="book"
        color="green"
      />
      <StatsCard
        title="Balances"
        :value="balanceStore.balancesCount"
        icon="document"
        color="purple"
      />
      <StatsCard
        v-if="authStore.isSubjectLeader"
        title="Pendientes"
        :value="balanceStore.pendingCount"
        icon="clock"
        color="yellow"
      />
      <StatsCard
        v-else
        title="Semanas"
        :value="stats.weeks"
        icon="clock"
        color="yellow"
      />
    </div>

    <!-- Fragmentos Pendientes (solo SubjectLeaders) -->
    <AppCard 
      v-if="authStore.isSubjectLeader && balanceStore.pendingFragments.length > 0" 
      title="Fragmentos Pendientes" 
      class="mb-8"
    >
      <p class="text-sm text-gray-600 mb-4">
        Tienes {{ balanceStore.pendingCount }} fragmento(s) pendiente(s) de completar
      </p>
      
      <div class="space-y-3">
        <div
          v-for="fragment in balanceStore.pendingFragments"
          :key="fragment.fragment_id"
          @click="editFragment(fragment)"
          class="border rounded-lg p-4 cursor-pointer transition-all hover:shadow-md hover:border-blue-300"
          :class="fragment.status === 'pending' ? 'border-yellow-200 bg-yellow-50' : 'border-blue-200 bg-blue-50'"
        >
          <div class="flex items-start justify-between">
            <div class="flex-1 min-w-0">
              <h4 class="font-medium text-gray-900">{{ fragment.asignatura_name }}</h4>
              <p class="text-sm text-gray-600 mt-0.5">Balance: {{ fragment.balance_name }}</p>
              <p v-if="fragment.deadline" class="text-xs text-amber-600 mt-1">
                Límite: {{ formatDate(fragment.deadline) }}
              </p>
            </div>
            <div class="ml-4 flex items-center gap-2">
              <span 
                class="px-2 py-1 text-xs font-medium rounded-full"
                :class="fragment.status === 'pending' ? 'bg-yellow-100 text-yellow-700' : 'bg-blue-100 text-blue-700'"
              >
                {{ fragment.status === 'pending' ? 'Pendiente' : 'En Progreso' }}
              </span>
              <svg class="w-5 h-5 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
              </svg>
            </div>
          </div>
        </div>
      </div>
    </AppCard>

    <!-- Balances (para Leaders) -->
    <AppCard v-if="authStore.isLeader" title="Balances Recientes">
      <!-- Loading state -->
      <div v-if="balanceStore.isLoading" class="text-center py-12">
        <div class="animate-spin rounded-full h-10 w-10 border-b-2 border-blue-600 mx-auto mb-4"></div>
        <p class="text-gray-600">Cargando balances...</p>
      </div>

      <!-- Error state -->
      <div v-else-if="balanceStore.error" class="text-center py-12">
        <svg class="mx-auto h-12 w-12 text-red-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
        </svg>
        <h3 class="mt-2 text-sm font-medium text-red-900">Error al cargar balances</h3>
        <p class="mt-1 text-sm text-red-600">{{ balanceStore.error }}</p>
        <div class="mt-4">
          <AppButton variant="primary" size="sm" @click="loadBalances">
            Reintentar
          </AppButton>
        </div>
      </div>

      <!-- Empty state -->
      <div v-else-if="balanceStore.balances.length === 0" class="text-center py-12">
        <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
        </svg>
        <h3 class="mt-2 text-sm font-medium text-gray-900">No hay balances</h3>
        <p class="mt-1 text-sm text-gray-500">Crea un nuevo balance para comenzar</p>
        <div class="mt-6">
          <AppButton variant="primary" @click="createNewBalance">
            <template #icon>
              <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
              </svg>
            </template>
            Crear Balance
          </AppButton>
        </div>
      </div>

      <!-- Balances list -->
      <div v-else class="overflow-x-auto -mx-4 sm:mx-0">
        <table class="min-w-full divide-y divide-blue-100">
          <thead class="bg-blue-50">
            <tr>
              <th class="px-3 sm:px-4 py-3 text-left text-xs font-bold text-blue-700 uppercase tracking-wider">
                Nombre
              </th>
              <th class="hidden sm:table-cell px-3 sm:px-4 py-3 text-left text-xs font-bold text-blue-700 uppercase tracking-wider">
                Curso
              </th>
              <th class="px-3 sm:px-4 py-3 text-left text-xs font-bold text-blue-700 uppercase tracking-wider">
                Estado
              </th>
              <th class="hidden md:table-cell px-3 sm:px-4 py-3 text-left text-xs font-bold text-blue-700 uppercase tracking-wider">
                Progreso
              </th>
              <th class="hidden lg:table-cell px-3 sm:px-4 py-3 text-left text-xs font-bold text-blue-700 uppercase tracking-wider">
                Fecha
              </th>
              <th class="px-3 sm:px-4 py-3 text-center text-xs font-bold text-blue-700 uppercase tracking-wider">
                Acciones
              </th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-blue-50">
            <tr
              v-for="balance in recentBalances"
              :key="balance.id"
              class="hover:bg-blue-50 transition"
            >
              <td class="px-3 sm:px-4 py-3 text-sm font-semibold text-blue-900">
                <div class="max-w-[120px] sm:max-w-[200px] truncate" :title="balance.name">
                  {{ balance.name }}
                </div>
              </td>
              <td class="hidden sm:table-cell px-3 sm:px-4 py-3 whitespace-nowrap text-sm text-blue-800">
                {{ balance.academic_year_text }}
              </td>
              <td class="px-3 sm:px-4 py-3 whitespace-nowrap text-sm">
                <span 
                  class="inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium"
                  :class="statusClass(balance.status)"
                >
                  {{ statusLabel(balance.status) }}
                </span>
              </td>
              <td class="hidden md:table-cell px-3 sm:px-4 py-3 whitespace-nowrap text-sm">
                <div class="flex items-center gap-2">
                  <div class="w-20 bg-gray-200 rounded-full h-2">
                    <div 
                      class="bg-green-600 h-2 rounded-full transition-all"
                      :style="{ width: `${balance.progress.percentage}%` }"
                    ></div>
                  </div>
                  <span class="text-xs text-gray-500">{{ balance.progress.percentage.toFixed(0) }}%</span>
                </div>
              </td>
              <td class="hidden lg:table-cell px-3 sm:px-4 py-3 whitespace-nowrap text-sm text-gray-500">
                {{ formatDate(balance.created_at) }}
              </td>
              <td class="px-3 sm:px-4 py-3 whitespace-nowrap text-center">
                <div class="flex justify-center gap-1">
                  <button
                    @click="viewBalance(balance.id)"
                    class="p-1.5 text-blue-600 hover:bg-blue-100 rounded-lg transition"
                    title="Ver balance"
                  >
                    <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                    </svg>
                  </button>
                  <button
                    @click="editBalance(balance.id)"
                    class="p-1.5 text-green-600 hover:bg-green-100 rounded-lg transition"
                    title="Editar balance"
                  >
                    <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                    </svg>
                  </button>
                  <button
                    @click="confirmDeleteBalance(balance)"
                    class="p-1.5 text-red-600 hover:bg-red-100 rounded-lg transition"
                    title="Eliminar balance"
                  >
                    <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                    </svg>
                  </button>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
        
        <!-- Link a ver todos -->
        <div v-if="balanceStore.balances.length > 3" class="mt-4 text-center">
          <router-link 
            to="/balance/view" 
            class="text-blue-600 hover:text-blue-800 text-sm font-medium"
          >
            Ver todos los balances ({{ balanceStore.balances.length }}) →
          </router-link>
        </div>
      </div>
    </AppCard>

    <!-- Vista para SubjectLeaders sin fragmentos pendientes -->
    <AppCard 
      v-if="authStore.isSubjectLeader && balanceStore.pendingFragments.length === 0 && !balanceStore.isLoading"
      title="Sin Tareas Pendientes"
    >
      <div class="text-center py-8">
        <svg class="mx-auto h-12 w-12 text-green-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        <h3 class="mt-2 text-sm font-medium text-gray-900">¡Todo al día!</h3>
        <p class="mt-1 text-sm text-gray-500">No tienes fragmentos pendientes de completar</p>
      </div>
    </AppCard>
    </template>
  </AppLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'
import { useAsignaturasStore } from '../stores/asignaturas'
import { useBalanceStore, type BalanceListItem, type PendingFragment } from '../stores/balance'
import { useUsersStore } from '../stores/users'
import { useUIStore } from '../stores/ui'
import { getAuditLogs, formatLogDate, type AuditLog } from '../services/audit'
import AppLayout from '../components/AppLayout.vue'
import AppCard from '../components/AppCard.vue'
import AppButton from '../components/AppButton.vue'
import StatsCard from '../components/StatsCard.vue'
import QuickAccessCard from '../components/QuickAccessCard.vue'

const router = useRouter()
const authStore = useAuthStore()
const asignaturasStore = useAsignaturasStore()
const balanceStore = useBalanceStore()
const usersStore = useUsersStore()
const uiStore = useUIStore()

// ============================================================================
// ESTADO ADMIN DASHBOARD
// ============================================================================

const isLoadingAudit = ref(false)
const auditLogs = ref<AuditLog[]>([])

const adminStats = ref({
  totalUsers: 0,
  eventsToday: 0,
})

const recentAuditLogs = computed(() => auditLogs.value.slice(0, 5))

async function loadAdminData() {
  // Cargar usuarios
  await usersStore.fetchUsers()
  adminStats.value.totalUsers = usersStore.users.length
  
  // Cargar logs de auditoría
  isLoadingAudit.value = true
  try {
    const result = await getAuditLogs()
    if (result.success && result.data) {
      auditLogs.value = result.data
      // Contar eventos de hoy
      const today = new Date()
      today.setHours(0, 0, 0, 0)
      adminStats.value.eventsToday = auditLogs.value.filter(log => {
        if (!log.created_at) return false
        const logDate = new Date(log.created_at)
        return logDate >= today
      }).length
    }
  } finally {
    isLoadingAudit.value = false
  }
}

function goToAudit() {
  router.push({ path: '/configuracion', query: { tab: 'logs' } })
}

function goToSettings() {
  router.push({ path: '/configuracion', query: { tab: 'settings' } })
}

// Estadísticas
const stats = ref({
  academicYears: 4,
  weeks: 15,
})

// Últimos 3 balances (ordenados por fecha de creación)
const recentBalances = computed(() => {
  return [...balanceStore.balances]
    .sort((a, b) => {
      const dateA = new Date(a.created_at || 0).getTime()
      const dateB = new Date(b.created_at || 0).getTime()
      return dateB - dateA
    })
    .slice(0, 3)
})

onMounted(async () => {
  // Admin tiene su propio flujo de datos
  if (authStore.isAdmin) {
    await loadAdminData()
    return
  }
  
  // Load data in parallel for better performance
  const promises: Promise<unknown>[] = []
  
  // Cargar asignaturas si no están cargadas
  if (asignaturasStore.asignaturas.length === 0) {
    promises.push(asignaturasStore.fetchAsignaturas())
  }
  
  // Cargar balances para Leaders
  if (authStore.isLeader) {
    promises.push(balanceStore.fetchBalances())
  }
  
  // Cargar fragmentos pendientes para SubjectLeaders
  if (authStore.isSubjectLeader) {
    promises.push(balanceStore.fetchPendingFragments())
  }
  
  await Promise.all(promises)
})

async function loadBalances() {
  await balanceStore.fetchBalances()
}

function createNewBalance() {
  balanceStore.startNewBalance()
  router.push('/balance')
}

function viewBalance(id: number) {
  router.push({ path: '/balance/view', query: { id: id.toString() } })
}

function editBalance(id: number) {
  router.push({ path: '/balance', query: { id: id.toString() } })
}

function editFragment(fragment: PendingFragment) {
  router.push({ 
    path: '/balance/fragment', 
    query: { 
      balanceId: fragment.balance_id.toString(), 
      asignaturaId: fragment.asignatura_id.toString() 
    } 
  })
}

function confirmDeleteBalance(balance: BalanceListItem) {
  uiStore.openConfirm({
    title: 'Eliminar Balance',
    message: `¿Estás seguro de que deseas eliminar "${balance.name}"? Esta acción eliminará todos los fragmentos asociados.`,
    confirmText: 'Sí, eliminar',
    cancelText: 'Cancelar',
    onConfirm: async () => {
      const result = await balanceStore.deleteBalance(balance.id)
      if (result.success) {
        uiStore.showSuccess(result.message)
      } else {
        uiStore.showError(result.message)
      }
    },
  })
}

function statusClass(status: string): string {
  switch (status) {
    case 'draft': return 'bg-gray-100 text-gray-700'
    case 'in_progress': return 'bg-blue-100 text-blue-700'
    case 'completed': return 'bg-green-100 text-green-700'
    default: return 'bg-gray-100 text-gray-700'
  }
}

function statusLabel(status: string): string {
  switch (status) {
    case 'draft': return 'Borrador'
    case 'in_progress': return 'En Progreso'
    case 'completed': return 'Completado'
    default: return status
  }
}

function formatDate(dateString: string | null): string {
  if (!dateString) return '-'
  try {
    const date = new Date(dateString)
    return date.toLocaleDateString('es-ES', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
    })
  } catch {
    return dateString
  }
}
</script>
