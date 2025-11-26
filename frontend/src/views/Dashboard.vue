/**
 * Dashboard - Vista principal rediseñada
 * Usa el nuevo sistema de componentes y stores
 * Balances cargados desde la API (base de datos)
 */
<template>
  <AppLayout>
    <!-- Welcome Section -->
    <AppCard class="mb-8">
      <div class="flex flex-col md:flex-row md:items-center md:justify-between gap-4">
        <div>
          <h2 class="text-3xl font-bold text-blue-700">
            ¡Bienvenido, {{ authStore.userName }}!
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
    <AppCard title="Mis Balances">
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
        <p class="mt-1 text-sm text-gray-500">Comienza creando tu primer balance de carga docente.</p>
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
      <div v-else class="overflow-x-auto">
        <table class="min-w-full divide-y divide-blue-100">
          <thead class="bg-blue-50">
            <tr>
              <th class="px-6 py-3 text-left text-xs font-bold text-blue-700 uppercase tracking-wider">
                Nombre
              </th>
              <th class="px-6 py-3 text-left text-xs font-bold text-blue-700 uppercase tracking-wider">
                Año Académico
              </th>
              <th class="px-6 py-3 text-left text-xs font-bold text-blue-700 uppercase tracking-wider">
                Período
              </th>
              <th class="px-6 py-3 text-left text-xs font-bold text-blue-700 uppercase tracking-wider">
                Asignaturas
              </th>
              <th class="px-6 py-3 text-left text-xs font-bold text-blue-700 uppercase tracking-wider">
                Fecha
              </th>
              <th class="px-6 py-3 text-center text-xs font-bold text-blue-700 uppercase tracking-wider">
                Acciones
              </th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-blue-50">
            <tr
              v-for="balance in balanceStore.balances"
              :key="balance.id"
              class="hover:bg-blue-50 transition"
            >
              <td class="px-6 py-4 whitespace-nowrap text-sm font-semibold text-blue-900">
                {{ balance.name }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-blue-800">
                {{ balance.academic_year_text }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-blue-800">
                {{ balance.period }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-blue-800">
                <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800">
                  {{ balance.subjects.length }} asig.
                </span>
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                {{ formatDate(balance.updated_at || balance.created_at) }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-center">
                <div class="flex justify-center gap-2">
                  <AppButton
                    variant="primary"
                    size="sm"
                    @click="editBalance(balance.id)"
                  >
                    Editar
                  </AppButton>
                  <AppButton
                    variant="danger"
                    size="sm"
                    @click="confirmDeleteBalance(balance)"
                  >
                    Eliminar
                  </AppButton>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </AppCard>
  </AppLayout>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
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

onMounted(async () => {
  await loadBalances()
})

async function loadBalances() {
  await balanceStore.fetchBalances()
}

function createNewBalance() {
  balanceStore.resetBalance()
  router.push('/balance')
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
