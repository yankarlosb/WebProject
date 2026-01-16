<!--
  Configuración - Vista de administración (Solo Administradores)
  Gestión de usuarios del sistema
-->
<template>
  <AppLayout>
    <!-- Header con advertencia de admin -->
    <div class="mb-8">
      <div class="flex items-center gap-3 mb-4">
        <div class="w-10 h-10 rounded-lg bg-red-100 flex items-center justify-center">
          <svg class="w-6 h-6 text-red-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
          </svg>
        </div>
        <div>
          <h1 class="text-3xl font-bold text-blue-700">Configuración del Sistema</h1>
          <p class="text-sm text-red-600 font-medium mt-1">Panel de Administración - Acceso Restringido</p>
        </div>
      </div>
      <p class="text-gray-600">Gestión de usuarios y configuración general del sistema</p>
    </div>

    <!-- Tabs de configuración -->
    <AppCard padding="none">
      <AppTabs v-model="activeTab" :tabs="tabs">
        <template #default="{ activeTab }">
          <!-- Tab: Gestión de Usuarios -->
          <div v-if="activeTab === 'users'" class="p-6">
            <div class="flex items-center justify-between mb-6">
              <div>
                <h2 class="text-xl font-bold text-blue-700">Usuarios del Sistema</h2>
                <p class="text-sm text-gray-600 mt-1">Gestiona cuentas de usuario y permisos</p>
              </div>
              <AppButton
                variant="primary"
                @click="openCreateUserModal"
              >
                <template #icon>
                  <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z" />
                  </svg>
                </template>
                Nuevo Usuario
              </AppButton>
            </div>

            <!-- Lista de usuarios -->
            <div v-if="usersStore.isLoading" class="text-center py-12">
              <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto"></div>
              <p class="text-gray-600 mt-4">Cargando usuarios...</p>
            </div>

            <div v-else-if="usersStore.users.length === 0" class="text-center py-12 bg-gray-50 rounded-lg">
              <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
              </svg>
              <h3 class="mt-2 text-sm font-medium text-gray-900">No hay usuarios</h3>
              <p class="mt-1 text-sm text-gray-500">Comienza creando el primer usuario</p>
            </div>

            <div v-else class="space-y-4">
              <div
                v-for="user in usersStore.users"
                :key="user.id"
                class="bg-white border border-gray-200 rounded-lg p-4 hover:shadow-md transition-shadow"
              >
                <div class="flex items-start justify-between">
                  <div class="flex items-start gap-4">
                    <!-- Avatar -->
                    <div class="w-12 h-12 rounded-full bg-blue-100 flex items-center justify-center">
                      <svg class="w-6 h-6 text-blue-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                      </svg>
                    </div>

                    <!-- Info -->
                    <div>
                      <h3 class="font-semibold text-gray-900">{{ user.name }}</h3>
                      <p class="text-sm text-gray-600">{{ user.email }}</p>
                      <div class="flex items-center gap-2 mt-2">
                        <span
                          class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium"
                          :class="{
                            'bg-purple-100 text-purple-800': user.role === 'admin',
                            'bg-blue-100 text-blue-800': user.role === 'user',
                            'bg-orange-100 text-orange-800': user.role === 'leader',
                            'bg-teal-100 text-teal-800': user.role === 'subjectLeader'
                          }"
                        >
                          {{ USER_ROLE_OPTIONS.find(r => r.value === user.role)?.label || 'Usuario' }}
                        </span>
                      </div>
                    </div>
                  </div>

                  <!-- Acciones -->
                  <div class="flex gap-2">
                    <AppButton
                      variant="ghost"
                      size="sm"
                      @click="editUser(user)"
                      :disabled="user.id === authStore.user?.id"
                    >
                      Editar
                    </AppButton>
                    <AppButton
                      variant="danger"
                      size="sm"
                      @click="confirmDeleteUser(user)"
                      :disabled="user.id === authStore.user?.id"
                    >
                      Eliminar
                    </AppButton>
                  </div>
                </div>
              </div>
            </div>

            <!-- Error state -->
            <div v-if="usersStore.error" class="mt-6 bg-red-50 border border-red-200 rounded-lg p-4">
              <div class="flex items-start gap-3">
                <svg class="w-5 h-5 text-red-600 mt-0.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
                <div class="text-sm text-red-800">
                  <p class="font-medium">Error al cargar usuarios</p>
                  <p class="mt-1">{{ usersStore.error }}</p>
                  <button
                    @click="usersStore.fetchUsers()"
                    class="mt-2 text-xs font-medium text-red-600 hover:text-red-800 underline"
                  >
                    Reintentar
                  </button>
                </div>
              </div>
            </div>
          </div>

          

          <!-- Tab: Logs de Auditoría -->
          <div v-if="activeTab === 'logs'" class="p-6">
            <div class="flex items-center justify-between mb-6">
              <div>
                <h2 class="text-xl font-bold text-blue-700">Registro de Actividad</h2>
                <p class="text-sm text-gray-600 mt-1">Trazas de auditoría del sistema</p>
              </div>
              <div class="flex gap-2">
                <AppButton variant="ghost" size="sm" @click="loadAuditLogs">
                  <template #icon>
                    <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
                    </svg>
                  </template>
                  Actualizar
                </AppButton>
                <AppButton 
                  variant="danger" 
                  size="sm" 
                  @click="handleCleanupLogs"
                  :disabled="isCleaningLogs"
                >
                  <template #icon>
                    <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                    </svg>
                  </template>
                  {{ isCleaningLogs ? 'Limpiando...' : 'Limpiar antiguos' }}
                </AppButton>
              </div>
            </div>

            <!-- Filtros avanzados -->
            <div class="bg-gray-50 border border-gray-200 rounded-lg p-4 mb-6">
              <div class="flex items-center justify-between mb-3">
                <h3 class="text-sm font-medium text-gray-700">Filtros</h3>
                <button 
                  @click="clearFilters" 
                  class="text-xs text-blue-600 hover:text-blue-800 font-medium"
                >
                  Limpiar filtros
                </button>
              </div>
              <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-3">
                <!-- Categoría -->
                <div>
                  <label class="block text-xs text-gray-600 mb-1">Categoría</label>
                  <select
                    v-model="logFilter"
                    class="w-full px-2 py-1.5 border border-gray-300 rounded text-sm focus:ring-2 focus:ring-blue-500"
                  >
                    <option value="all">Todas</option>
                    <option value="security">Seguridad</option>
                  </select>
                </div>
                
                <!-- Tipo de evento -->
                <div>
                  <label class="block text-xs text-gray-600 mb-1">Tipo de evento</label>
                  <select
                    v-model="eventTypeFilter"
                    class="w-full px-2 py-1.5 border border-gray-300 rounded text-sm focus:ring-2 focus:ring-blue-500"
                  >
                    <option value="all">Todos</option>
                    <option v-for="et in eventTypes" :key="et" :value="et">
                      {{ getEventLabel(et) }}
                    </option>
                  </select>
                </div>
                
                <!-- Usuario -->
                <div>
                  <label class="block text-xs text-gray-600 mb-1">Usuario</label>
                  <select
                    v-model="userFilter"
                    class="w-full px-2 py-1.5 border border-gray-300 rounded text-sm focus:ring-2 focus:ring-blue-500"
                  >
                    <option value="">Todos</option>
                    <option v-for="u in logUsers" :key="u" :value="u">{{ u }}</option>
                  </select>
                </div>
                
                <!-- Resultado -->
                <div>
                  <label class="block text-xs text-gray-600 mb-1">Resultado</label>
                  <select
                    v-model="successFilter"
                    class="w-full px-2 py-1.5 border border-gray-300 rounded text-sm focus:ring-2 focus:ring-blue-500"
                  >
                    <option value="all">Todos</option>
                    <option value="success">Exitosos</option>
                    <option value="failed">Fallidos</option>
                  </select>
                </div>
                
                <!-- Fecha desde -->
                <div>
                  <label class="block text-xs text-gray-600 mb-1">Desde</label>
                  <input
                    type="date"
                    v-model="dateFromFilter"
                    class="w-full px-2 py-1.5 border border-gray-300 rounded text-sm focus:ring-2 focus:ring-blue-500"
                  />
                </div>
                
                <!-- Fecha hasta -->
                <div>
                  <label class="block text-xs text-gray-600 mb-1">Hasta</label>
                  <input
                    type="date"
                    v-model="dateToFilter"
                    class="w-full px-2 py-1.5 border border-gray-300 rounded text-sm focus:ring-2 focus:ring-blue-500"
                  />
                </div>
              </div>
              <div class="mt-3 text-xs text-gray-500">
                Mostrando {{ filteredLogs.length }} de {{ auditLogs.length }} registros
              </div>
            </div>

            <!-- Loading state -->
            <div v-if="isLoadingLogs" class="text-center py-12">
              <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto"></div>
              <p class="text-gray-600 mt-4">Cargando logs...</p>
            </div>

            <!-- Empty state -->
            <div v-else-if="filteredLogs.length === 0" class="text-center py-12 bg-gray-50 rounded-lg">
              <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
              </svg>
              <h3 class="mt-2 text-sm font-medium text-gray-900">No hay registros de auditoría</h3>
              <p class="mt-1 text-sm text-gray-500">
                {{ auditLogs.length > 0 ? 'Intenta ajustar los filtros' : 'Los eventos del sistema aparecerán aquí' }}
              </p>
            </div>

            <!-- Logs list -->
            <div v-else class="space-y-3 max-h-[600px] overflow-y-auto">
              <div
                v-for="log in filteredLogs"
                :key="log.id"
                class="bg-gray-50 border border-gray-200 rounded-lg p-4 hover:bg-gray-100 transition-colors"
              >
                <div class="flex items-start justify-between">
                  <div class="flex items-start gap-3">
                    <div
                      class="w-2 h-2 rounded-full mt-2"
                      :class="getEventBgColor(log.event_type)"
                    ></div>
                    <div>
                      <div class="flex items-center gap-2 flex-wrap">
                        <p class="text-sm font-medium text-gray-900">{{ log.description }}</p>
                        <span
                          class="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium"
                          :class="log.category === 'SECURITY' ? 'bg-red-100 text-red-800' : 'bg-blue-100 text-blue-800'"
                        >
                          {{ log.category === 'SECURITY' ? 'Seguridad' : 'Funcional' }}
                        </span>
                        <span
                          v-if="!log.success"
                          class="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-red-100 text-red-800"
                        >
                          Fallido
                        </span>
                      </div>
                      <p class="text-xs text-gray-600 mt-1">
                        <span v-if="log.user_name">Por {{ log.user_name }} • </span>
                        <span v-if="log.ip_address">IP: {{ log.ip_address }} • </span>
                        {{ formatLogDate(log.created_at) }}
                      </p>
                      <p v-if="log.error_message" class="text-xs text-red-600 mt-1">
                        Error: {{ log.error_message }}
                      </p>
                    </div>
                  </div>
                  <div class="text-right">
                    <span class="text-xs px-2 py-1 rounded" :class="getEventTextColor(log.event_type)">
                      {{ getEventLabel(log.event_type) }}
                    </span>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- Tab: Configuración del Sistema -->
          <div v-if="activeTab === 'settings'" class="p-6">
            <div class="flex items-center justify-between mb-6">
              <div>
                <h2 class="text-xl font-bold text-blue-700">Configuración del Sistema</h2>
                <p class="text-sm text-gray-600 mt-1">Ajustes de seguridad, sesión y políticas</p>
              </div>
              <AppButton 
                variant="primary" 
                @click="saveSettings"
                :disabled="isSavingSettings"
              >
                <template #icon>
                  <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                  </svg>
                </template>
                {{ isSavingSettings ? 'Guardando...' : 'Guardar cambios' }}
              </AppButton>
            </div>

            <!-- Loading state -->
            <div v-if="isLoadingSettings" class="text-center py-12">
              <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto"></div>
              <p class="text-gray-600 mt-4">Cargando configuraciones...</p>
            </div>

            <!-- Settings grid -->
            <div v-else-if="systemSettings" class="grid gap-6 md:grid-cols-2">
              <!-- Seguridad -->
              <div class="bg-gray-50 border border-gray-200 rounded-lg p-4">
                <h3 class="font-semibold text-gray-800 mb-4 flex items-center gap-2">
                  <svg class="w-5 h-5 text-red-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
                  </svg>
                  {{ categoryLabels.security }}
                </h3>
                <div class="space-y-4">
                  <div v-for="setting in systemSettings.security" :key="setting.key">
                    <label class="block text-sm font-medium text-gray-700 mb-1">
                      {{ settingLabels[setting.key] || setting.key }}
                    </label>
                    <template v-if="getInputType(setting.key) === 'boolean'">
                      <select
                        v-model="editedSettings[setting.key]"
                        class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-blue-500"
                      >
                        <option value="true">Sí</option>
                        <option value="false">No</option>
                      </select>
                    </template>
                    <template v-else>
                      <input
                        :type="getInputType(setting.key) === 'number' ? 'number' : 'text'"
                        :min="getInputType(setting.key) === 'number' ? 1 : undefined"
                        v-model="editedSettings[setting.key]"
                        class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-blue-500"
                      />
                    </template>
                    <p v-if="setting.description" class="text-xs text-gray-500 mt-1">{{ setting.description }}</p>
                  </div>
                </div>
              </div>

              <!-- Sesión -->
              <div class="bg-gray-50 border border-gray-200 rounded-lg p-4">
                <h3 class="font-semibold text-gray-800 mb-4 flex items-center gap-2">
                  <svg class="w-5 h-5 text-blue-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
                  </svg>
                  {{ categoryLabels.session }}
                </h3>
                <div class="space-y-4">
                  <div v-for="setting in systemSettings.session" :key="setting.key">
                    <label class="block text-sm font-medium text-gray-700 mb-1">
                      {{ settingLabels[setting.key] || setting.key }}
                    </label>
                    <input
                      :type="getInputType(setting.key) === 'number' ? 'number' : 'text'"
                      :min="getInputType(setting.key) === 'number' ? 1 : undefined"
                      v-model="editedSettings[setting.key]"
                      class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-blue-500"
                    />
                    <p v-if="setting.description" class="text-xs text-gray-500 mt-1">{{ setting.description }}</p>
                  </div>
                </div>
              </div>

              <!-- Contraseñas -->
              <div class="bg-gray-50 border border-gray-200 rounded-lg p-4">
                <h3 class="font-semibold text-gray-800 mb-4 flex items-center gap-2">
                  <svg class="w-5 h-5 text-yellow-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z" />
                  </svg>
                  {{ categoryLabels.password }}
                </h3>
                <div class="space-y-4">
                  <div v-for="setting in systemSettings.password" :key="setting.key">
                    <label class="block text-sm font-medium text-gray-700 mb-1">
                      {{ settingLabels[setting.key] || setting.key }}
                    </label>
                    <template v-if="getInputType(setting.key) === 'boolean'">
                      <select
                        v-model="editedSettings[setting.key]"
                        class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-blue-500"
                      >
                        <option value="true">Sí</option>
                        <option value="false">No</option>
                      </select>
                    </template>
                    <template v-else>
                      <input
                        :type="getInputType(setting.key) === 'number' ? 'number' : 'text'"
                        :min="getInputType(setting.key) === 'number' ? 1 : undefined"
                        v-model="editedSettings[setting.key]"
                        class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-blue-500"
                      />
                    </template>
                    <p v-if="setting.description" class="text-xs text-gray-500 mt-1">{{ setting.description }}</p>
                  </div>
                </div>
              </div>

              <!-- Auditoría -->
              <div class="bg-gray-50 border border-gray-200 rounded-lg p-4">
                <h3 class="font-semibold text-gray-800 mb-4 flex items-center gap-2">
                  <svg class="w-5 h-5 text-purple-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                  </svg>
                  {{ categoryLabels.audit }}
                </h3>
                <div class="space-y-4">
                  <div v-for="setting in systemSettings.audit" :key="setting.key">
                    <label class="block text-sm font-medium text-gray-700 mb-1">
                      {{ settingLabels[setting.key] || setting.key }}
                    </label>
                    <template v-if="getInputType(setting.key) === 'boolean'">
                      <select
                        v-model="editedSettings[setting.key]"
                        class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-blue-500"
                      >
                        <option value="true">Sí</option>
                        <option value="false">No</option>
                      </select>
                    </template>
                    <template v-else>
                      <input
                        :type="getInputType(setting.key) === 'number' ? 'number' : 'text'"
                        :min="getInputType(setting.key) === 'number' ? 90 : undefined"
                        v-model="editedSettings[setting.key]"
                        class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-blue-500"
                      />
                    </template>
                    <p v-if="setting.description" class="text-xs text-gray-500 mt-1">{{ setting.description }}</p>
                  </div>
                </div>
              </div>
            </div>

            <!-- No settings -->
            <div v-else class="text-center py-12 bg-gray-50 rounded-lg">
              <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
              </svg>
              <h3 class="mt-2 text-sm font-medium text-gray-900">No se pudieron cargar las configuraciones</h3>
              <AppButton variant="ghost" size="sm" @click="loadSettings" class="mt-4">
                Reintentar
              </AppButton>
            </div>
          </div>
        </template>
      </AppTabs>
    </AppCard>

    <!-- Modal: Crear/Editar Usuario -->
    <AppModal
      v-model="showUserModal"
      :title="isEditingUser ? 'Editar Usuario' : 'Nuevo Usuario'"
      size="md"
    >
      <form @submit.prevent="saveUser" class="space-y-4">
        <AppInput
          v-model="userForm.user_name"
          label="Nombre de usuario"
          placeholder="juanperez"
          required
        />
        
        <AppInput
          v-model="userForm.name"
          label="Nombre completo"
          placeholder="Juan Pérez"
          required
        />

        <AppInput
          v-model="userForm.email"
          type="email"
          label="Correo electrónico"
          placeholder="usuario@ejemplo.com"
          required
        />

        <div v-if="!isEditingUser">
          <AppInput
            v-model="userForm.password"
            type="password"
            label="Contraseña"
            placeholder="••••••••"
            required
            hint="Mínimo 8 caracteres, mayúsculas, minúsculas y caracteres especiales"
          />
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">Rol</label>
          <select
            v-model="userForm.role"
            class="w-full px-4 py-2.5 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
          >
            <option
              v-for="roleOption in USER_ROLE_OPTIONS"
              :key="roleOption.value"
              :value="roleOption.value"
            >
              {{ roleOption.label }}
            </option>
          </select>
        </div>


      </form>

      <template #footer>
        <AppButton variant="ghost" @click="closeUserModal">
          Cancelar
        </AppButton>
        <AppButton variant="primary" @click="saveUser">
          {{ isEditingUser ? 'Actualizar' : 'Crear' }}
        </AppButton>
      </template>
    </AppModal>
  </AppLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { useRoute } from 'vue-router'
import { useAuthStore } from '../stores/auth'
import { useUIStore } from '../stores/ui'
import { useUsersStore } from '../stores/users'
import { 
  getAuditLogs, 
  getSecurityLogs, 
  formatLogDate,
  getEventConfig,
  cleanupOldLogs,
  type AuditLog 
} from '../services/audit'
import settingsService, { type SettingsGrouped } from '../services/settings'
import AppLayout from '../components/AppLayout.vue'
import AppCard from '../components/AppCard.vue'
import AppButton from '../components/AppButton.vue'
import AppInput from '../components/AppInput.vue'
import AppModal from '../components/AppModal.vue'
import AppTabs from '../components/AppTabs.vue'
import { USER_ROLES, USER_ROLE_OPTIONS } from '../utils/constants'
import { isValidUsername, isValidName, isValidEmail, isValidPassword } from '../utils/validation'

const authStore = useAuthStore()
const uiStore = useUIStore()
const usersStore = useUsersStore()
const route = useRoute()

// Estado
const activeTab = ref('users')
const showUserModal = ref(false)
const isEditingUser = ref(false)
const editingUserId = ref<number | null>(null)

const tabs = [
  { id: 'users', label: 'Usuarios' },
  { id: 'logs', label: 'Auditoría' },
  { id: 'settings', label: 'Sistema' },
]

// Cargar usuarios al montar el componente
onMounted(async () => {
  // Verificar si hay tab en la URL
  const tabParam = route.query.tab as string
  if (tabParam && ['users', 'logs', 'settings'].includes(tabParam)) {
    activeTab.value = tabParam
  }
  
  await usersStore.fetchUsers()
  // Cargar logs de auditoría
  await loadAuditLogs()
  // Cargar configuraciones del sistema
  await loadSettings()
})

// ============================================================================
// AUDITORÍA
// ============================================================================

// Estado de auditoría
const logFilter = ref<'all' | 'security'>('all')
const isLoadingLogs = ref(false)
const auditLogs = ref<AuditLog[]>([])

// Filtros avanzados
const eventTypeFilter = ref<string>('all')
const userFilter = ref<string>('')
const successFilter = ref<'all' | 'success' | 'failed'>('all')
const dateFromFilter = ref<string>('')
const dateToFilter = ref<string>('')
const isCleaningLogs = ref(false)

// Lista de tipos de evento únicos
const eventTypes = computed(() => {
  const types = new Set(auditLogs.value.map(l => l.event_type))
  return Array.from(types).sort()
})

// Lista de usuarios únicos en los logs
const logUsers = computed(() => {
  const users = new Set(auditLogs.value.filter(l => l.user_name).map(l => l.user_name!))
  return Array.from(users).sort()
})

// Logs filtrados
const filteredLogs = computed(() => {
  return auditLogs.value.filter(log => {
    // Filtro por tipo de evento
    if (eventTypeFilter.value !== 'all' && log.event_type !== eventTypeFilter.value) {
      return false
    }
    
    // Filtro por usuario
    if (userFilter.value && log.user_name !== userFilter.value) {
      return false
    }
    
    // Filtro por éxito/fallo
    if (successFilter.value === 'success' && !log.success) return false
    if (successFilter.value === 'failed' && log.success) return false
    
    // Filtro por fecha desde
    if (dateFromFilter.value && log.created_at) {
      const logDate = new Date(log.created_at)
      const fromDate = new Date(dateFromFilter.value)
      if (logDate < fromDate) return false
    }
    
    // Filtro por fecha hasta
    if (dateToFilter.value && log.created_at) {
      const logDate = new Date(log.created_at)
      const toDate = new Date(dateToFilter.value)
      toDate.setHours(23, 59, 59, 999)
      if (logDate > toDate) return false
    }
    
    return true
  })
})

// Cargar logs de auditoría
async function loadAuditLogs() {
  isLoadingLogs.value = true
  try {
    const result = logFilter.value === 'security' 
      ? await getSecurityLogs()
      : await getAuditLogs()
    
    if (result.success && result.data) {
      auditLogs.value = result.data
    }
  } finally {
    isLoadingLogs.value = false
  }
}

// Limpiar logs antiguos
async function handleCleanupLogs() {
  uiStore.openConfirm({
    title: 'Limpiar logs antiguos',
    message: '¿Estás seguro de que deseas eliminar los logs con más de 90 días? Esta acción no se puede deshacer.',
    confirmText: 'Sí, limpiar',
    cancelText: 'Cancelar',
    onConfirm: async () => {
      isCleaningLogs.value = true
      try {
        const result = await cleanupOldLogs()
        if (result.success) {
          uiStore.showSuccess(result.message || 'Logs antiguos eliminados')
          await loadAuditLogs()
        } else {
          uiStore.showError(result.message || 'Error al limpiar logs')
        }
      } finally {
        isCleaningLogs.value = false
      }
    },
  })
}

// Limpiar filtros
function clearFilters() {
  eventTypeFilter.value = 'all'
  userFilter.value = ''
  successFilter.value = 'all'
  dateFromFilter.value = ''
  dateToFilter.value = ''
}

// ============================================================================
// CONFIGURACIÓN DEL SISTEMA
// ============================================================================

const systemSettings = ref<SettingsGrouped | null>(null)
const editedSettings = ref<Record<string, string>>({})
const isLoadingSettings = ref(false)
const isSavingSettings = ref(false)

// Descripciones legibles de las configuraciones
const settingLabels: Record<string, string> = {
  // Security
  max_login_attempts: 'Intentos máximos de login',
  block_duration_minutes: 'Duración de bloqueo (minutos)',
  attempt_window_minutes: 'Ventana de intentos (minutos)',
  require_ip_validation: 'Validar IP en JWT',
  token_expiration_hours: 'Expiración de token (horas)',
  // Session
  session_timeout_minutes: 'Timeout de sesión (minutos)',
  // Password
  password_min_length: 'Longitud mínima de contraseña',
  password_require_uppercase: 'Requerir mayúsculas',
  password_require_lowercase: 'Requerir minúsculas',
  password_require_special: 'Requerir caracteres especiales',
  // Audit
  audit_log_ip: 'Registrar IP en auditoría',
  audit_retention_days: 'Retención de logs (días)',
}

// Nombres de categorías
const categoryLabels: Record<string, string> = {
  security: 'Seguridad',
  session: 'Sesión',
  password: 'Contraseñas',
  audit: 'Auditoría',
}

// Cargar configuraciones
async function loadSettings() {
  isLoadingSettings.value = true
  try {
    const result = await settingsService.list()
    if (result.success && result.data) {
      systemSettings.value = result.data
      // Inicializar valores editados
      editedSettings.value = {}
      for (const category of Object.keys(result.data) as (keyof SettingsGrouped)[]) {
        for (const setting of result.data[category]) {
          editedSettings.value[setting.key] = setting.value
        }
      }
    } else {
      uiStore.showError(result.message || 'Error al cargar configuraciones')
    }
  } finally {
    isLoadingSettings.value = false
  }
}

// Guardar configuraciones
async function saveSettings() {
  isSavingSettings.value = true
  try {
    const result = await settingsService.update(editedSettings.value)
    if (result.success) {
      uiStore.showSuccess('Configuraciones guardadas correctamente')
      await loadSettings()
    } else {
      uiStore.showError(result.message || 'Error al guardar configuraciones')
    }
  } finally {
    isSavingSettings.value = false
  }
}

// Determinar tipo de input para cada setting
function getInputType(key: string): 'number' | 'boolean' | 'text' {
  // Booleanos: claves que empiezan con 'require', 'password_require', o son 'audit_log_ip'
  if (key.startsWith('require') || key.startsWith('password_require') || key === 'audit_log_ip') {
    return 'boolean'
  }
  // Numéricos: cualquier clave que contenga estas palabras
  if (key.includes('minutes') || key.includes('hours') || key.includes('attempts') || 
      key.includes('length') || key.includes('days')) {
    return 'number'
  }
  return 'text'
}

// Helpers para colores de eventos
function getEventBgColor(eventType: string): string {
  const config = getEventConfig(eventType)
  return config.bgColor
}

function getEventTextColor(eventType: string): string {
  const config = getEventConfig(eventType)
  return config.color
}

function getEventLabel(eventType: string): string {
  const config = getEventConfig(eventType)
  return config.label
}

// Recargar logs cuando cambia el filtro
watch(logFilter, () => {
  loadAuditLogs()
})

// ============================================================================
// GESTIÓN DE USUARIOS
// ============================================================================

// Formulario de usuario
const emptyUserForm = () => ({
  user_name: '',
  name: '',
  email: '',
  password: '',
  role: USER_ROLES.USER,
})

const userForm = ref(emptyUserForm())

// Métodos
function openCreateUserModal() {
  isEditingUser.value = false
  editingUserId.value = null
  userForm.value = emptyUserForm()
  showUserModal.value = true
}

function editUser(user: any) {
  isEditingUser.value = true
  editingUserId.value = user.id
  userForm.value = {
    user_name: user.user_name,
    name: user.name,
    email: user.email,
    password: '',
    role: user.role,
  }
  showUserModal.value = true
}

function closeUserModal() {
  showUserModal.value = false
  setTimeout(() => {
    userForm.value = emptyUserForm()
    isEditingUser.value = false
    editingUserId.value = null
  }, 200)
}

async function saveUser() {
  const trimmedUserName = userForm.value.user_name.trim()
  const trimmedName = userForm.value.name.trim()
  const trimmedEmail = userForm.value.email.trim()
  const trimmedPassword = userForm.value.password.trim()

  // Validación básica de campos requeridos
  if (!trimmedUserName || !trimmedName || !trimmedEmail) {
    uiStore.showWarning('Por favor completa todos los campos requeridos')
    return
  }

  // Validar formato de username
  if (!isValidUsername(trimmedUserName)) {
    uiStore.showError('Nombre de usuario inválido (solo letras, números y guión bajo)')
    return
  }

  // Validar formato de nombre
  if (!isValidName(trimmedName)) {
    uiStore.showError('Nombre inválido (solo letras y espacios)')
    return
  }

  // Validar formato de email
  if (!isValidEmail(trimmedEmail)) {
    uiStore.showError('Email inválido')
    return
  }

  // Validar contraseña para nuevos usuarios
  if (!isEditingUser.value) {
    if (!trimmedPassword) {
      uiStore.showWarning('La contraseña es requerida para nuevos usuarios')
      return
    }
    if (!isValidPassword(trimmedPassword)) {
      uiStore.showError('Contraseña inválida (mínimo 8 caracteres, mayúsculas, minúsculas y caracteres especiales)')
      return
    }
  }

  // Guardar en backend
  let result
  
  if (isEditingUser.value && editingUserId.value !== null) {
    // Actualizar usuario existente
    // Buscar el usuario actual para obtener todos sus datos
    const currentUser = usersStore.getUserById(editingUserId.value)
    
    if (!currentUser) {
      uiStore.showError('Usuario no encontrado')
      return
    }
    
    // Preparar el objeto completo que espera el backend (usuarios::Model)
    result = await usersStore.updateUser(editingUserId.value, {
      id: currentUser.id,
      user_name: userForm.value.user_name,
      name: userForm.value.name,
      email: userForm.value.email,
      token: userForm.value.password || currentUser.token, // Si no hay nueva password, mantener la actual
      role: userForm.value.role,
      created_at: currentUser.created_at,
    })
    
    if (result.success) {
      uiStore.showSuccess('Usuario actualizado correctamente')
    } else {
      uiStore.showError(result.message || 'Error al actualizar usuario')
      return
    }
  } else {
    // Crear nuevo usuario
    result = await usersStore.createUser({
      user_name: userForm.value.user_name,
      name: userForm.value.name,
      email: userForm.value.email,
      password: userForm.value.password,
      role: userForm.value.role,
    })
    
    if (result.success) {
      uiStore.showSuccess('Usuario creado correctamente')
    } else {
      uiStore.showError(result.message || 'Error al crear usuario')
      return
    }
  }

  closeUserModal()
}

function confirmDeleteUser(user: any) {
  uiStore.openConfirm({
    title: 'Eliminar Usuario',
    message: `¿Estás seguro de que deseas eliminar a "${user.name}"? Esta acción no se puede deshacer.`,
    confirmText: 'Sí, eliminar',
    cancelText: 'Cancelar',
    onConfirm: async () => {
      const result = await usersStore.deleteUser(user.id)
      
      if (result.success) {
        uiStore.showSuccess('Usuario eliminado correctamente')
      } else {
        uiStore.showError(result.message || 'Error al eliminar usuario')
      }
    },
  })
}
</script>
