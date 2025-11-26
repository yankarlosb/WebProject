/**
 * Dashboard - Vista principal rediseñada
 * Usa el nuevo sistema de componentes y stores
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
        :value="recentBalances.length"
        icon="document"
        color="purple"
      />
    </div>

    <!-- Balances Recientes -->
    <AppCard title="Balances Recientes">
      <div v-if="recentBalances.length === 0" class="text-center py-12">
        <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
        </svg>
        <h3 class="mt-2 text-sm font-medium text-gray-900">No hay balances</h3>
      </div>

      <div v-else class="overflow-x-auto">
        <table class="min-w-full divide-y divide-blue-100">
          <thead class="bg-blue-50">
            <tr>
              <th class="px-6 py-3 text-left text-xs font-bold text-blue-700 uppercase tracking-wider">
                Año Académico
              </th>
              <th class="px-6 py-3 text-left text-xs font-bold text-blue-700 uppercase tracking-wider">
                Período
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
              v-for="balance in recentBalances"
              :key="balance.id"
              class="hover:bg-blue-50 transition"
            >
              <td class="px-6 py-4 whitespace-nowrap text-sm font-semibold text-blue-900">
                {{ balance.year }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-blue-800">
                {{ balance.period }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-blue-800">
                {{ balance.date }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-center">
                <div class="flex justify-center gap-2">
                  <AppButton
                    variant="primary"
                    size="sm"
                    @click="viewBalance(balance.id)"
                  >
                    Ver
                  </AppButton>
                  <AppButton
                    v-if="authStore.isLeader"
                    variant="primary"
                    size="sm"
                    @click="editBalance(balance.id)"
                  >
                    Editar
                  </AppButton>
                  <AppButton
                    variant="secondary"
                    size="sm"
                    disabled
                  >
                    Exportar
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
import AppLayout from '../components/AppLayout.vue'
import AppCard from '../components/AppCard.vue'
import AppButton from '../components/AppButton.vue'
import StatsCard from '../components/StatsCard.vue'

const router = useRouter()
const authStore = useAuthStore()
const asignaturasStore = useAsignaturasStore()

// Estadísticas
const stats = ref({
  academicYears: 4,
  weeks: 15,
})

// Balances recientes
const recentBalances = ref<Array<{ id: string; year: string; period: string; date: string }>>([])

onMounted(() => {
  loadRecentBalances()
})

function loadRecentBalances() {
  try {
    const raw = localStorage.getItem('recentBalances')
    if (raw) {
      const allBalances = JSON.parse(raw)
      // Mostrar solo los últimos 5 balances
      recentBalances.value = allBalances.slice(0, 5)
    }
  } catch (error) {
    console.error('Error cargando balances recientes:', error)
  }
}

function editBalance(id: string) {
  router.push({ path: '/balance', query: { id } })
}
function viewBalance(id: string) {
  router.push({ path: '/balance/view', query: { id } })
}
</script>
