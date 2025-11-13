<template>
  <AppLayout>
    <!-- Loading state -->
    <div v-if="balanceStore.isLoading" class="flex items-center justify-center min-h-[60vh]">
      <div class="text-center">
        <div class="animate-spin rounded-full h-16 w-16 border-b-2 border-blue-600 mx-auto mb-4"></div>
        <p class="text-gray-600">Cargando balance...</p>
      </div>
    </div>

    <!-- Contenido principal -->
    <div v-else>
      <!-- Header con breadcrumb -->
      <header class="mb-6">
        <nav class="flex items-center text-sm text-blue-600 mb-2">
          <router-link to="/dashboard" class="hover:underline">Dashboard</router-link>
          <svg class="w-4 h-4 mx-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
          </svg>
          <span class="text-gray-700">Balance de Carga</span>
        </nav>
        <h1 class="text-3xl font-bold text-blue-700">Balance de Carga Docente</h1>
      </header>

      <!-- Card de configuración -->
      <BalanceConfigCard
        v-if="balanceStore.currentBalance"
        :config="balanceConfig"
        :is-saving="isSaving"
        :has-unsaved-changes="balanceStore.hasUnsavedChanges"
        @update:config="handleConfigUpdate"
        @calculate="calculateBalance"
        @save="saveBalance"
      />

      <!-- Tabs -->
      <AppCard v-if="balanceStore.currentBalance" padding="none">
        <AppTabs v-model="activeTab" :tabs="tabs">
          <template #default="{ activeTab: currentTab }">
            <!-- Tab: Tabla de Balance -->
            <div v-if="currentTab === 'table'" class="p-4">
              <!-- Header de tabla -->
              <div class="mb-3 flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3">
                <div>
                  <h3 class="text-base font-bold text-blue-700">
                    Balance {{ balanceStore.currentBalance.academicYearText }} - {{ balanceStore.currentBalance.period }}
                  </h3>
                  <p class="text-xs text-gray-600 mt-0.5">
                    {{ balanceStore.currentBalance.subjects.length }} asignatura(s)
                  </p>
                </div>
                <AppButton
                  variant="primary"
                  size="sm"
                  @click="openAddSubjectModal"
                >
                  <template #icon>
                    <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                    </svg>
                  </template>
                  Agregar Asignatura
                </AppButton>
              </div>

              <!-- Estado vacío -->
              <div v-if="!balanceStore.hasSubjects" class="text-center py-12 bg-gray-50 rounded-lg">
                <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
                </svg>
                <h3 class="mt-2 text-sm font-medium text-gray-900">No hay asignaturas</h3>
                <p class="mt-1 text-sm text-gray-500">Agrega asignaturas para comenzar a llenar el balance</p>
              </div>

              <!-- Tablas de balance -->
              <div v-else class="space-y-6">
                <!-- Semanas 1-4 -->
                <BalanceWeekTable
                  :subjects="balanceStore.currentBalance.subjects"
                  title="Semanas 1 - 4"
                  :weeks="[1, 2, 3, 4]"
                  :start-cell-index="0"
                  color-scheme="blue"
                  @update-value="updateCellValue"
                />

                <!-- Semanas 5-8 -->
                <BalanceWeekTable
                  :subjects="balanceStore.currentBalance.subjects"
                  title="Semanas 5 - 8"
                  :weeks="[5, 6, 7, 8]"
                  :start-cell-index="16"
                  color-scheme="blue"
                  @update-value="updateCellValue"
                />

                <!-- Semanas 9-12 -->
                <BalanceWeekTable
                  :subjects="balanceStore.currentBalance.subjects"
                  title="Semanas 9 - 12"
                  :weeks="[9, 10, 11, 12]"
                  :start-cell-index="32"
                  color-scheme="blue"
                  @update-value="updateCellValue"
                />

                <!-- Semanas 13-15 -->
                <BalanceWeekTable
                  :subjects="balanceStore.currentBalance.subjects"
                  title="Semanas 13 - 15"
                  :weeks="[13, 14, 15]"
                  :start-cell-index="48"
                  color-scheme="purple"
                  @update-value="updateCellValue"
                />

                <!-- Consultas y Exámenes Finales -->
                <BalanceFinalTable
                  :subjects="balanceStore.currentBalance.subjects"
                  @update-value="updateCellValue"
                  @delete-subject="confirmDeleteSubject"
                />
              </div>
            </div>

            <!-- Tab: Cálculos -->
            <CalculationsTable
              v-if="currentTab === 'calculations'"
              :calculations="balanceStore.calculations"
            />
          </template>
        </AppTabs>
      </AppCard>

      <!-- Modal: Agregar asignatura -->
      <AppModal
        v-model="showAddSubjectModal"
        title="Agregar Asignatura al Balance"
        size="md"
      >
        <div class="space-y-4">
          <p class="text-sm text-gray-600">Selecciona una asignatura existente o ingresa un nombre personalizado</p>
          
          <!-- Selector de asignaturas existentes -->
          <div v-if="asignaturasStore.asignaturasCount > 0">
            <label class="block text-sm font-medium text-gray-700 mb-2">Asignaturas disponibles</label>
            <div class="max-h-48 overflow-y-auto border border-gray-200 rounded-lg divide-y">
              <button
                v-for="asignatura in asignaturasStore.asignaturasList"
                :key="asignatura.id"
                @click="addExistingSubject(asignatura.nombre)"
                class="w-full px-4 py-3 text-left hover:bg-blue-50 transition flex items-center justify-between"
              >
                <div>
                  <div class="font-medium text-gray-900">{{ asignatura.nombre }}</div>
                  <div class="text-xs text-gray-500">{{ asignatura.periodo }}</div>
                </div>
                <svg class="w-5 h-5 text-blue-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                </svg>
              </button>
            </div>
            
            <div class="relative my-4">
              <div class="absolute inset-0 flex items-center">
                <div class="w-full border-t border-gray-300"></div>
              </div>
              <div class="relative flex justify-center text-sm">
                <span class="px-2 bg-white text-gray-500">o</span>
              </div>
            </div>
          </div>
        </div>
      </AppModal>
    </div>
  </AppLayout>
</template>

<script setup lang="ts">
import { onMounted, onBeforeUnmount, computed } from 'vue'
import { useBalanceForm } from '../composables/useBalanceForm'
import AppLayout from '../components/AppLayout.vue'
import AppCard from '../components/AppCard.vue'
import AppButton from '../components/AppButton.vue'
import AppModal from '../components/AppModal.vue'
import AppTabs from '../components/AppTabs.vue'
import BalanceConfigCard from '../components/BalanceConfigCard.vue'
import BalanceWeekTable from '../components/BalanceWeekTable.vue'
import BalanceFinalTable from '../components/BalanceFinalTable.vue'
import CalculationsTable from '../components/CalculationsTable.vue'

// Composable con toda la lógica
const {
  activeTab,
  isSaving,
  showAddSubjectModal,
  tabs,
  balanceStore,
  asignaturasStore,
  initializeBalance,
  markDirty,
  updateCellValue,
  calculateBalance,
  saveBalance,
  openAddSubjectModal,
  addExistingSubject,
  confirmDeleteSubject,
  setupUnsavedWarning,
} = useBalanceForm()

// Computed para configuración del balance
const balanceConfig = computed(() => {
  if (!balanceStore.currentBalance) {
    return {
      academicYear: '',
      period: '1ero',
      academicYearText: '',
      startDate: '',
    }
  }
  return {
    academicYear: balanceStore.currentBalance.academicYear,
    period: balanceStore.currentBalance.period,
    academicYearText: balanceStore.currentBalance.academicYearText,
    startDate: balanceStore.currentBalance.startDate,
  }
})

// Manejador de actualización de configuración
function handleConfigUpdate(field: string, value: string) {
  if (!balanceStore.currentBalance) return
  
  // Type-safe update
  const validFields = ['academicYear', 'period', 'academicYearText', 'startDate'] as const
  type ValidField = typeof validFields[number]
  
  if (validFields.includes(field as ValidField)) {
    (balanceStore.currentBalance as any)[field] = value
  }
  
  markDirty()
}

// Lifecycle
onMounted(initializeBalance)
onBeforeUnmount(setupUnsavedWarning)
</script>
