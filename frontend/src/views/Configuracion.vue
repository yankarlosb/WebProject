<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'

interface User {
  id: number
  name: string
  password?: string
  email: string
  role: string
  status: 'active' | 'inactive'
  date: string
}

interface Activity {
  id: number
  user: string
  action: string
  date: string
}

const currentSection = ref<'dashboard' | 'user-list' | 'add-user' | 'user-profile' | 'settings'>('dashboard')
const router = useRouter()

const originalUsersSnapshot = ref('')
const originalNewUserSnapshot = ref('')
const originalSelectedUserSnapshot = ref('')
const originalSettingsSnapshot = ref('')

const takeSnapshotUsers = () => { originalUsersSnapshot.value = JSON.stringify(users.value || []) }
const takeSnapshotNewUser = () => { originalNewUserSnapshot.value = JSON.stringify(newUser.value || {}) }
const takeSnapshotSelectedUser = () => { originalSelectedUserSnapshot.value = JSON.stringify(selectedUser.value || {}) }
const takeSnapshotSettings = () => { originalSettingsSnapshot.value = JSON.stringify(settings.value || {}) }

const hasUnsavedChanges = (): boolean => {

  if (JSON.stringify(users.value || []) !== originalUsersSnapshot.value) return true
  switch (currentSection.value) {
    case 'add-user':
      return JSON.stringify(newUser.value || {}) !== originalNewUserSnapshot.value
    case 'user-profile':
      return JSON.stringify(selectedUser.value || {}) !== originalSelectedUserSnapshot.value
    case 'settings':
      return JSON.stringify(settings.value || {}) !== originalSettingsSnapshot.value
    default:
      return false
  }
}
const navigationHistory = ref<string[]>(['dashboard'])

const users = ref<User[]>([])

const activities = ref<Activity[]>([])

const newUser = ref({
  name: '',
  password: '',
  email: '',
  role: '',
  status: 'active' as 'active' | 'inactive'
})

const selectedUser = ref<User>({
  id: 0,
  name: '',
  email: '',
  role: '',
  status: 'active',
  date: ''
})

const settings = ref({
  itemsPerPage: '10',
  emailNotifications: 'enabled' as 'enabled' | 'disabled'
})

const searchQuery = ref('')
const filteredUsers = ref<User[]>([...users.value])

const editingProfile = ref(false)

const notification = ref({
  show: false,
  message: '',
  type: 'success' as 'success' | 'error'
})

const activeUsersCount = computed(() => users.value.filter(u => u.status === 'active').length)
const inactiveUsersCount = computed(() => users.value.filter(u => u.status === 'inactive').length)

const showNotification = (message: string, type: 'success' | 'error') => {
  notification.value = { show: true, message, type }
  setTimeout(() => {
    notification.value.show = false
  }, 5000)
}

const showSection = (section: typeof currentSection.value) => {
  currentSection.value = section
  if (navigationHistory.value[navigationHistory.value.length - 1] !== section) {
    navigationHistory.value.push(section)
  }
  if (section === 'user-profile' && selectedUser.value.id === 0 && users.value.length > 0) {
    const first = users.value[0]
    if (first) viewUser(first.id)
  }
}

const goBack = () => {
  if (hasUnsavedChanges()) {
    if (!confirm('Hay cambios sin guardar. ¿Deseas salir?')) return
  }
  router.back()
}

const handleEsc = (e: KeyboardEvent) => {
  if (e.key === 'Escape') goBack()
}

const resetData = () => {
  if (confirm('¿Está seguro de que desea restablecer todos los datos? Esta acción no se puede deshacer.')) {
    users.value = [
      { id: 1, name: 'Juan Díaz', password: 'juan123', email: 'juan.diaz@ejemplo.com', role: 'admin', status: 'active', date: '2023-01-15' },
      { id: 2, name: 'María López', password: 'maria123', email: 'maria.lopez@ejemplo.com', role: 'user', status: 'active', date: '2023-02-20' }
    ]
    filteredUsers.value = [...users.value]
  persistUsers()
    showNotification('Datos restablecidos correctamente', 'success')
  }
}

const addUser = () => {
  if (!newUser.value.name || !newUser.value.email || !newUser.value.role) {
    showNotification('Por favor, complete todos los campos obligatorios', 'error')
    return
  }

  const nextId = users.value.length > 0 ? Math.max(...users.value.map(u => u.id)) + 1 : 1
  const user: User = {
    id: nextId,
    name: newUser.value.name,
    password: newUser.value.password,
    email: newUser.value.email,
    role: newUser.value.role,
    status: newUser.value.status,
    date: (new Date().toISOString().split('T')[0]) || ''
  }

  users.value.push(user)
  filteredUsers.value = [...users.value]
  clearForm()
  persistUsers()
  takeSnapshotUsers()
  takeSnapshotNewUser()
  showNotification('Usuario agregado correctamente', 'success')

  activities.value.unshift({
    id: activities.value.length + 1,
    user: 'Administrador',
    action: `Agregó al usuario ${user.name}`,
    date: new Date().toLocaleString()
  })
}

const clearForm = () => {
  newUser.value = {
    name: '',
    password: '',
    email: '',
    role: '',
    status: 'active'
  }
  takeSnapshotNewUser()
}

const searchUsers = () => {
  if (!searchQuery.value.trim()) {
    filteredUsers.value = [...users.value]
    return
  }
  const q = searchQuery.value.toLowerCase()
  filteredUsers.value = users.value.filter(u =>
    u.name.toLowerCase().includes(q) ||
    u.email.toLowerCase().includes(q) ||
    u.role.toLowerCase().includes(q)
  )
}

const viewUser = (id: number) => {
  const user = users.value.find(u => u.id === id)
  if (user) {
    selectedUser.value = { ...user }
    showSection('user-profile')
  }
}

const editUser = (id: number) => {
  showNotification(`Editando usuario con ID: ${id}`, 'success')
}

const deleteUser = (id: number) => {
  if (confirm('¿Está seguro de que desea eliminar este usuario?')) {
    users.value = users.value.filter(u => u.id !== id)
    filteredUsers.value = filteredUsers.value.filter(u => u.id !== id)
  persistUsers()
  takeSnapshotUsers()
    showNotification('Usuario eliminado correctamente', 'success')
    activities.value.unshift({
      id: activities.value.length + 1,
      user: 'Administrador',
      action: `Eliminó un usuario (ID: ${id})`,
      date: new Date().toLocaleString()
    })
  }
}

const toggleEditProfile = () => {
  if (editingProfile.value) {
    const idx = users.value.findIndex(u => u.id === selectedUser.value.id)
    if (idx !== -1) {
  users.value[idx] = { ...selectedUser.value }
  persistUsers()
  takeSnapshotUsers()
  takeSnapshotSelectedUser()
      showNotification('Perfil actualizado correctamente', 'success')
    }
  }
  editingProfile.value = !editingProfile.value
}

const deactivateUser = () => {
  if (confirm('¿Está seguro de que desea desactivar este usuario?')) {
    selectedUser.value.status = 'inactive'
    const idx = users.value.findIndex(u => u.id === selectedUser.value.id)
      if (idx !== -1 && users.value[idx]) {
        users.value[idx].status = 'inactive'
      }
  persistUsers()
  takeSnapshotUsers()
  takeSnapshotSelectedUser()
    showNotification('Usuario desactivado correctamente', 'success')
  }
}

const saveSettings = () => {
  showNotification('Configuración guardada correctamente', 'success')
}

const resetSettings = () => {
  settings.value = {
    itemsPerPage: '10',
    emailNotifications: 'enabled'
  }
  showNotification('Configuración restablecida', 'success')
}

const getInitials = (name: string): string => {
  return name
    .split(' ')
    .map(n => n[0])
    .join('')
    .toUpperCase()
}

onMounted(() => {
  try {
    const raw = localStorage.getItem('app_users')
    if (raw) {
      users.value = JSON.parse(raw)
    } else {
      users.value = [
        { id: 1, name: 'Juan Díaz', password: 'juan123', email: 'juan.diaz@ejemplo.com', role: 'admin', status: 'active', date: '2023-01-15' },
        { id: 2, name: 'María López', password: 'maria123', email: 'maria.lopez@ejemplo.com', role: 'user', status: 'active', date: '2023-02-20' }
      ]
      localStorage.setItem('app_users', JSON.stringify(users.value))
    }
  } catch (e) {
    // ignorar errores
  }
  filteredUsers.value = [...users.value]
  takeSnapshotUsers()
  takeSnapshotNewUser()
  takeSnapshotSelectedUser()
  takeSnapshotSettings()
  
  // Agregar listener para ESC
  window.addEventListener('keydown', handleEsc)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleEsc)
})

const persistUsers = () => {
  try {
    localStorage.setItem('app_users', JSON.stringify(users.value))
    takeSnapshotUsers()
  } catch (e) {
  }
}
</script>

<template>
  <div class="min-h-screen bg-gradient-to-br from-blue-50 to-blue-100 p-6">
    <div class="max-w-7xl mx-auto">
      <!-- Encabezado -->
      <header class="mb-8">
        <div class="flex items-center justify-between mb-4">
          <div>
            <h1 class="text-3xl font-bold text-blue-700">Gestión de Usuarios</h1>
            <p class="text-blue-600 mt-1 font-medium">Administración completa de usuarios del sistema</p>
          </div>
          <button
            @click="resetData"
            class="px-4 py-2 bg-yellow-500 hover:bg-yellow-600 text-white rounded-lg shadow transition font-semibold"
          >
            Restablecer Datos
          </button>
        </div>
      </header>

      <!-- Notificación -->
      <div
        v-if="notification.show"
        :class="{
          'bg-green-100 border-green-500 text-green-700': notification.type === 'success',
          'bg-red-100 border-red-500 text-red-700': notification.type === 'error'
        }"
        class="mb-6 p-4 border-l-4 rounded"
      >
        {{ notification.message }}
      </div>

      <div class="flex gap-6">
  <!-- Barra lateral -->
        <aside class="w-64 flex-shrink-0">
          <div
            v-for="section in [
              { key: 'dashboard', label: 'Inicio' },
              { key: 'user-list', label: 'Lista de Usuarios' },
              { key: 'add-user', label: 'Agregar Usuario' },
              { key: 'user-profile', label: 'Perfil de Usuario' },
              { key: 'settings', label: 'Configuración' }
            ]"
            :key="section.key"
            @click="showSection(section.key as any)"
            :class="[
              'p-3 cursor-pointer rounded-md mb-2 transition',
              currentSection === section.key
                ? 'bg-blue-100 text-blue-700 font-medium'
                : 'text-gray-700 hover:bg-gray-100'
            ]"
          >
            {{ section.label }}
          </div>
        </aside>

        <!-- Contenido principal -->
        <main class="flex-1">
          <!-- Dashboard -->
          <div v-show="currentSection === 'dashboard'" class="space-y-6">
            <div class="bg-white p-6 rounded-lg shadow">
              <h2 class="text-xl font-semibold mb-4">Resumen del Sistema</h2>
              <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                <div class="text-center p-4 bg-blue-50 rounded">
                  <div class="text-3xl font-bold text-blue-700">{{ users.length }}</div>
                  <div class="text-gray-600">Usuarios Totales</div>
                </div>
                <div class="text-center p-4 bg-green-50 rounded">
                  <div class="text-3xl font-bold text-green-700">{{ activeUsersCount }}</div>
                  <div class="text-gray-600">Usuarios Activos</div>
                </div>
                <div class="text-center p-4 bg-red-50 rounded">
                  <div class="text-3xl font-bold text-red-700">{{ inactiveUsersCount }}</div>
                  <div class="text-gray-600">Usuarios Inactivos</div>
                </div>
              </div>
            </div>

            <div class="bg-white p-6 rounded-lg shadow">
              <h2 class="text-xl font-semibold mb-4">Actividad Reciente</h2>
              <table class="w-full">
                <thead class="border-b">
                  <tr>
                    <th class="text-left py-2">Usuario</th>
                    <th class="text-left py-2">Acción</th>
                    <th class="text-left py-2">Fecha</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="activity in activities" :key="activity.id" class="border-b hover:bg-gray-50">
                    <td class="py-2">{{ activity.user }}</td>
                    <td class="py-2">{{ activity.action }}</td>
                    <td class="py-2 text-gray-500">{{ activity.date }}</td>
                  </tr>
                  <tr v-if="activities.length === 0">
                    <td colspan="3" class="py-4 text-center text-gray-500">No hay actividad reciente</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>

          <!-- Lista de Usuarios -->
          <div v-show="currentSection === 'user-list'" class="bg-white p-6 rounded-lg shadow">
            <h2 class="text-xl font-semibold mb-4">Lista de Usuarios</h2>
            <div class="mb-4 flex gap-2">
              <input
                v-model="searchQuery"
                type="text"
                placeholder="Buscar usuario..."
                class="flex-1 px-4 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                @keyup.enter="searchUsers"
              />
              <button
                @click="searchUsers"
                class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
              >
                Buscar
              </button>
            </div>
            <div class="overflow-x-auto">
              <table class="w-full">
                <thead class="bg-gray-100">
                  <tr>
                    <th class="py-2 px-4 text-left">ID</th>
                    <th class="py-2 px-4 text-left">Nombre</th>
                    <th class="py-2 px-4 text-left">Email</th>
                    <th class="py-2 px-4 text-left">Rol</th>
                    <th class="py-2 px-4 text-left">Estado</th>
                    <th class="py-2 px-4 text-left">Acciones</th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="user in filteredUsers" :key="user.id" class="border-b hover:bg-gray-50">
                    <td class="py-2 px-4">{{ user.id }}</td>
                    <td class="py-2 px-4">{{ user.name }}</td>
                    <td class="py-2 px-4">{{ user.email }}</td>
                    <td class="py-2 px-4">{{ user.role }}</td>
                    <td class="py-2 px-4">
                      <span
                        :class="{
                          'bg-green-100 text-green-800': user.status === 'active',
                          'bg-red-100 text-red-800': user.status === 'inactive'
                        }"
                        class="px-2 py-1 rounded-full text-xs font-medium"
                      >
                        {{ user.status === 'active' ? 'Activo' : 'Inactivo' }}
                      </span>
                    </td>
                    <td class="py-2 px-4">
                      <div class="flex gap-2">
                        <button
                          @click="viewUser(user.id)"
                          class="text-blue-600 hover:text-blue-800 text-sm"
                        >
                          Ver
                        </button>
                        <button
                          @click="editUser(user.id)"
                          class="text-yellow-600 hover:text-yellow-800 text-sm"
                        >
                          Editar
                        </button>
                        <button
                          @click="deleteUser(user.id)"
                          class="text-red-600 hover:text-red-800 text-sm"
                        >
                          Eliminar
                        </button>
                      </div>
                    </td>
                  </tr>
                  <tr v-if="filteredUsers.length === 0">
                    <td colspan="6" class="py-4 text-center text-gray-500">No se encontraron usuarios</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>

          <!-- Agregar Usuario -->
          <div v-show="currentSection === 'add-user'" class="bg-white p-6 rounded-lg shadow">
            <h2 class="text-xl font-semibold mb-4">Agregar Nuevo Usuario</h2>
            <form @submit.prevent="addUser" class="space-y-4">
              <div>
                <label class="block text-gray-700 mb-1">Nombre Completo</label>
                <input
                  v-model="newUser.name"
                  type="text"
                  class="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                  required
                />
              </div>
              <div>
                <label class="block text-gray-700 mb-1">Correo Electrónico</label>
                <input
                  v-model="newUser.email"
                  type="email"
                  class="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                  required
                />
              </div>
              <div>
                <label class="block text-gray-700 mb-1">Contraseña</label>
                <input
                  v-model="newUser.password"
                  type="password"
                  class="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                  required
                />
              </div>
              <div>
                <label class="block text-gray-700 mb-1">Rol</label>
                <select
                  v-model="newUser.role"
                  class="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                  required
                >
                  <option value="">Seleccione un rol</option>
                  <option value="Administrador">Administrador</option>
                  <option value="Usuario">Usuario</option>
                  <option value="Editor">Editor</option>
                </select>
              </div>
              <div>
                <label class="block text-gray-700 mb-1">Estado</label>
                <select
                  v-model="newUser.status"
                  class="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                >
                  <option value="active">Activo</option>
                  <option value="inactive">Inactivo</option>
                </select>
              </div>
              <div class="flex gap-3 pt-2">
                <button type="submit" class="px-4 py-2 bg-green-600 text-white rounded-md hover:bg-green-700">
                  Guardar Usuario
                </button>
                <button
                  type="button"
                  @click="clearForm"
                  class="px-4 py-2 bg-gray-200 text-gray-800 rounded-md hover:bg-gray-300"
                >
                  Limpiar Formulario
                </button>
              </div>
            </form>
          </div>

          <!-- Perfil de Usuario -->
          <div v-show="currentSection === 'user-profile'" class="bg-white p-6 rounded-lg shadow">
            <h2 class="text-xl font-semibold mb-6">Perfil de Usuario</h2>
            <div class="flex items-center gap-4 mb-6">
              <div
                class="w-16 h-16 rounded-full bg-blue-500 flex items-center justify-center text-white text-xl font-bold"
              >
                {{ getInitials(selectedUser.name) }}
              </div>
              <div>
                <h3 class="text-2xl font-bold">{{ selectedUser.name }}</h3>
                <p class="text-gray-600">{{ selectedUser.email }}</p>
              </div>
            </div>

            <div class="space-y-4">
              <div>
                <label class="block text-gray-700 mb-1">Rol</label>
                <input
                  v-model="selectedUser.role"
                  type="text"
                  :readonly="!editingProfile"
                  class="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white"
                />
              </div>
              <div>
                <label class="block text-gray-700 mb-1">Estado</label>
                <input
                  v-model="selectedUser.status"
                  type="text"
                  :readonly="!editingProfile"
                  class="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white"
                />
              </div>
              <div class="flex gap-3 pt-2">
                <button
                  v-if="!editingProfile"
                  @click="toggleEditProfile"
                  class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
                >
                  Editar Perfil
                </button>
                <button
                  v-else
                  @click="toggleEditProfile"
                  class="px-4 py-2 bg-green-600 text-white rounded-md hover:bg-green-700"
                >
                  Guardar Cambios
                </button>
                <button
                  v-if="selectedUser.status === 'active'"
                  @click="deactivateUser"
                  class="px-4 py-2 bg-red-600 text-white rounded-md hover:bg-red-700"
                >
                  Desactivar Usuario
                </button>
              </div>
            </div>
          </div>

          <!-- Configuración -->
          <div v-show="currentSection === 'settings'" class="bg-white p-6 rounded-lg shadow">
            <h2 class="text-xl font-semibold mb-4">Configuración</h2>
            <div class="space-y-4">
              <div>
                <label class="block text-gray-700 mb-1">Elementos por página</label>
                <select
                  v-model="settings.itemsPerPage"
                  class="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                >
                  <option value="10">10</option>
                  <option value="25">25</option>
                  <option value="50">50</option>
                </select>
              </div>
              <div>
                <label class="block text-gray-700 mb-1">Notificaciones por email</label>
                <select
                  v-model="settings.emailNotifications"
                  class="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                >
                  <option value="enabled">Habilitadas</option>
                  <option value="disabled">Deshabilitadas</option>
                </select>
              </div>
              <div class="flex gap-3 pt-2">
                <button
                  @click="saveSettings"
                  class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
                >
                  Guardar Configuración
                </button>
                <button
                  @click="resetSettings"
                  class="px-4 py-2 bg-gray-200 text-gray-800 rounded-md hover:bg-gray-300"
                >
                  Restablecer
                </button>
              </div>
            </div>
          </div>
        </main>
      </div>
    </div>
  </div>
</template>