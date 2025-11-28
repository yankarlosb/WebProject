<!--
  Dashboard - Vista principal rediseñada
  Usa el nuevo sistema de componentes y stores
  Balances cargados desde la API (base de datos)
-->
<template>
  <AppLayout>
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
          v-if="authStore.isLeaderOrSubjectLeader"
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
        title="Semanas"
        :value="stats.weeks"
        icon="clock"
        color="yellow"
      />
      <StatsCard
        title="Balances Guardados"
        :value="balanceStore.balancesCount"
        icon="document"
        color="purple"
      />
    </div>

    <!-- Balances Recientes -->
    <AppCard title="Balances Recientes">
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
        <div class="mt-6" v-if="authStore.isLeaderOrSubjectLeader">
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
                Período
              </th>
              <th class="hidden md:table-cell px-3 sm:px-4 py-3 text-left text-xs font-bold text-blue-700 uppercase tracking-wider">
                Asig.
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
              <td class="px-3 sm:px-4 py-3 whitespace-nowrap text-sm text-blue-800">
                {{ balance.period === '1ero' ? '1er Sem.' : '2do Sem.' }}
              </td>
              <td class="hidden md:table-cell px-3 sm:px-4 py-3 whitespace-nowrap text-sm text-blue-800">
                <span class="inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800">
                  {{ balance.subjects.length }}
                </span>
              </td>
              <td class="hidden lg:table-cell px-3 sm:px-4 py-3 whitespace-nowrap text-sm text-gray-500">
                {{ formatDate(balance.updated_at || balance.created_at) }}
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
                    v-if="authStore.isLeader"
                    @click="editBalance(balance.id)"
                    class="p-1.5 text-green-600 hover:bg-green-100 rounded-lg transition"
                    title="Editar balance"
                  >
                    <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                    </svg>
                  </button>
                  <button
                    v-if="authStore.isLeader"
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
  </AppLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'
import { useAsignaturasStore } from '../stores/asignaturas'
import { useBalanceStore, type Balance } from '../stores/balance'
import { useUIStore } from '../stores/ui'
import AppLayout from '../components/AppLayout.vue'
import AppCard from '../components/AppCard.vue'
import AppButton from '../components/AppButton.vue'
import StatsCard from '../components/StatsCard.vue'

const router = useRouter()
const authStore = useAuthStore()
const asignaturasStore = useAsignaturasStore()
const balanceStore = useBalanceStore()
const uiStore = useUIStore()

// Estadísticas
const stats = ref({
  academicYears: 4,
  weeks: 15,
})

// Últimos 3 balances (ordenados por fecha de actualización/creación)
const recentBalances = computed(() => {
  return [...balanceStore.balances]
    .sort((a, b) => {
      const dateA = new Date(a.updated_at || a.created_at || 0).getTime()
      const dateB = new Date(b.updated_at || b.created_at || 0).getTime()
      return dateB - dateA
    })
    .slice(0, 3)
})

onMounted(async () => {
  // Load balances and asignaturas in parallel for better performance
  await Promise.all([
    loadBalances(),
    asignaturasStore.asignaturas.length === 0 ? asignaturasStore.loadAsignaturas() : Promise.resolve()
  ])
})

async function loadBalances() {
  await balanceStore.fetchBalances()
}

function createNewBalance() {
  balanceStore.resetBalance()
  router.push('/balance')
}

function viewBalance(id: number) {
  router.push({ path: '/balance/view', query: { id: id.toString() } })
}

function editBalance(id: number) {
  router.push({ path: '/balance', query: { id: id.toString() } })
}

function confirmDeleteBalance(balance: Balance) {
  uiStore.openConfirm({
    title: 'Eliminar Balance',
    message: `¿Estás seguro de que deseas eliminar "${balance.name}"? Esta acción no se puede deshacer.`,
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

function formatDate(dateString: string | null): string {
  if (!dateString) return '-'
  try {
    const date = new Date(dateString)
    return date.toLocaleDateString('es-ES', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    })
  } catch {
    return dateString
  }
}
</script>
