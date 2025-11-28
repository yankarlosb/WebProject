<!--
  AppLayout - Layout principal de la aplicación
  Incluye sidebar, header y área de contenido
-->
<template>
  <div class="min-h-screen bg-gradient-to-br from-blue-50 to-blue-100">
    <!-- Header -->
    <header class="bg-gradient-to-r from-blue-600 via-blue-500 to-blue-400 shadow-lg sticky top-0 z-40">
      <div class="px-4 sm:px-6 lg:px-8 py-4">
        <div class="flex items-center justify-between">
          <!-- Logo y título -->
          <div class="flex items-center space-x-4">
            <!-- Botón de menú móvil -->
            <button
              class="lg:hidden p-2 rounded-lg text-white hover:bg-white/20 transition-colors"
              @click="toggleMobileSidebar"
            >915
              <svg class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
              </svg>
            </button>

            <div class="flex items-center space-x-3">
              <div class="w-10 h-10 flex items-center justify-center bg-white/20 rounded-full shadow-lg">
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                  <rect x="3" y="3" width="18" height="18" rx="4" fill="url(#gradient)" />
                  <defs>
                    <linearGradient id="gradient" x1="0" x2="1">
                      <stop offset="0" stop-color="#4fa6ff" />
                      <stop offset="1" stop-color="#0b5fff" />
                    </linearGradient>
                  </defs>
                </svg>
              </div>
              <div class="hidden sm:block">
                <h1 class="text-xl font-bold text-white tracking-wide drop-shadow">
                  CiberBalance
                </h1>
                <p class="text-xs text-blue-100">
                  Balance de Carga Docente
                </p>
              </div>
            </div>
          </div>

          <!-- User info y acciones -->
          <div class="flex items-center gap-4">
            <div class="hidden md:block text-right">
              <div class="font-semibold text-white text-sm">
                {{ authStore.userName }}
              </div>
              <div class="text-xs text-blue-100">
                {{ authStore.userRole }}
              </div>
            </div>
            
            <AppButton
              variant="danger"
              size="sm"
              @click="handleLogout"
              :loading="authStore.isLoading"
            >
              <template #icon>
                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" />
                </svg>
              </template>
              Salir
            </AppButton>
          </div>
        </div>
      </div>
    </header>

    <!-- Container principal -->
    <div class="flex">
      <!-- Sidebar Desktop -->
      <aside 
        class="hidden lg:block w-64 bg-white/80 backdrop-blur border-r border-blue-100 min-h-screen sticky top-[72px] self-start"
      >
        <nav class="p-4 space-y-2">
          <router-link
            v-for="item in visibleNavigationItems"
            :key="item.path"
            :to="item.path"
            v-slot="{ isActive }"
            custom
          >
            <button
              @click="navigateTo(item.path)"
              class="w-full flex items-center space-x-3 px-4 py-3 rounded-lg transition-all duration-200 font-semibold text-left"
              :class="isActive 
                ? 'bg-blue-600 text-white shadow-lg scale-105' 
                : 'text-gray-700 hover:bg-blue-50 hover:scale-102'
              "
            >
              <span class="w-6 h-6 flex-shrink-0" v-html="item.icon"></span>
              <span>{{ item.label }}</span>
            </button>
          </router-link>
        </nav>
      </aside>

      <!-- Sidebar Móvil (Overlay) -->
      <Transition
        enter-active-class="transition-opacity duration-200"
        enter-from-class="opacity-0"
        enter-to-class="opacity-100"
        leave-active-class="transition-opacity duration-200"
        leave-from-class="opacity-100"
        leave-to-class="opacity-0"
      >
        <div
          v-if="mobileSidebarOpen"
          class="fixed inset-0 bg-black/50 z-50 lg:hidden"
          @click="closeMobileSidebar"
        >
          <Transition
            enter-active-class="transition-transform duration-200"
            enter-from-class="-translate-x-full"
            enter-to-class="translate-x-0"
            leave-active-class="transition-transform duration-200"
            leave-from-class="translate-x-0"
            leave-to-class="-translate-x-full"
          >
            <aside
              v-if="mobileSidebarOpen"
              class="w-64 bg-white h-full shadow-xl"
              @click.stop
            >
              <div class="p-4">
                <div class="flex items-center justify-between mb-6">
                  <h2 class="text-lg font-bold text-blue-700">Menú</h2>
                  <button
                    class="p-2 rounded-lg text-gray-400 hover:text-gray-600 hover:bg-gray-100"
                    @click="closeMobileSidebar"
                  >
                    <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                    </svg>
                  </button>
                </div>

                <nav class="space-y-2">
                  <router-link
                    v-for="item in visibleNavigationItems"
                    :key="item.path"
                    :to="item.path"
                    v-slot="{ isActive }"
                    custom
                  >
                    <button
                      @click="navigateTo(item.path)"
                      class="w-full flex items-center space-x-3 px-4 py-3 rounded-lg transition-all duration-200 font-semibold text-left"
                      :class="isActive 
                        ? 'bg-blue-600 text-white shadow-lg' 
                        : 'text-gray-700 hover:bg-blue-50'
                      "
                    >
                      <span class="w-6 h-6 flex-shrink-0" v-html="item.icon"></span>
                      <span>{{ item.label }}</span>
                    </button>
                  </router-link>
                </nav>
              </div>
            </aside>
          </Transition>
        </div>
      </Transition>

      <!-- Contenido principal -->
      <main class="flex-1 p-4 sm:p-6 lg:p-8">
        <slot></slot>
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'
import { useUIStore } from '../stores/ui'
import AppButton from './AppButton.vue'

const router = useRouter()
const authStore = useAuthStore()
const uiStore = useUIStore()

const mobileSidebarOpen = ref(false)

// Definición de items de navegación con control de acceso
const navigationItems = [
  {
    path: '/dashboard',
    label: 'Dashboard',
    icon: `<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M3 13h8V3H3v10zM3 21h8v-6H3v6zM13 21h8V11h-8v10zM13 3v6h8V3h-8z" />
    </svg>`,
  },
  {
    path: '/balance/view',
    label: 'Ver Balances',
    icon: `<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
    </svg>`,
  },
  {
    path: '/balance',
    label: 'Balance de Carga',
    icon: `<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M3 10h18M5 6h14M7 14h10M9 18h6" />
    </svg>`,
    requiresLeader: true,
  },
  {
    path: '/asignaturas',
    label: 'Asignaturas',
    icon: `<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
    </svg>`,
    requiresSubjectLeaderOrLeader: true,
  },
  {
    path: '/perfil',
    label: 'Mi Perfil',
    icon: `<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
    </svg>`,
  },
  {
    path: '/configuracion',
    label: 'Configuración',
    icon: `<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
    </svg>`,
    requiresAdmin: true,
  },
]

// Filtrar items de navegación según permisos
const visibleNavigationItems = computed(() => {
  return navigationItems.filter(item => {
    if (item.requiresAdmin) {
      return authStore.isAdmin
    }
    if (item.requiresLeader) {
      return authStore.isLeader
    }
    if (item.requiresSubjectLeaderOrLeader) {
      return authStore.isSubjectLeader || authStore.isLeader
    }
    return true
  })
})

function toggleMobileSidebar() {
  mobileSidebarOpen.value = !mobileSidebarOpen.value
}

function closeMobileSidebar() {
  mobileSidebarOpen.value = false
}

function navigateTo(path: string) {
  router.push(path)
  closeMobileSidebar()
}

async function handleLogout() {
  uiStore.openConfirm({
    title: 'Cerrar Sesión',
    message: '¿Estás seguro de que deseas cerrar sesión?',
    confirmText: 'Sí, cerrar sesión',
    cancelText: 'Cancelar',
    onConfirm: async () => {
      await authStore.logout()
      router.push('/login')
      uiStore.showSuccess('Sesión cerrada correctamente')
    },
  })
}
</script>
