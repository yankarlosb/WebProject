<template>
  <div class="min-h-screen bg-gradient-to-br from-blue-50 to-blue-100 flex flex-col items-center justify-center p-4">
    <!-- Logotipo -->
    <div class="text-center mb-10">
      <div class="flex justify-center mb-4">
        <div class="w-20 h-20 flex items-center justify-center bg-blue-600 rounded-full shadow-2xl">
          <span class="text-5xl">🔐</span>
        </div>
      </div>
      <h1 class="text-5xl font-extrabold text-blue-700 drop-shadow-lg">CiberBalance</h1>
      <p class="text-xl text-blue-600 mt-3 font-medium">Facultad de Ciberseguridad</p>
    </div>

    <!-- Formulario -->
    <form
      @submit.prevent="handleSubmit"
      class="w-full max-w-md bg-white p-10 rounded-2xl shadow-2xl border border-blue-200"
    >
      <div class="mb-6">
        <label for="username" class="block text-sm font-semibold text-gray-700 mb-2">
          Usuario
        </label>
        <input
          id="username"
          v-model="username"
          type="text"
          placeholder="Ingrese su usuario"
          required
          class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition"
        />
      </div>

      <div class="mb-8">
        <label for="password" class="block text-sm font-semibold text-gray-700 mb-2">
          Contraseña
        </label>
        <input
          id="password"
          v-model="password"
          type="password"
          placeholder="Ingrese su contraseña"
          required
          class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition"
        />
      </div>

      <button
        type="submit"
        class="w-full bg-blue-600 hover:bg-blue-700 text-white font-bold py-3 px-6 rounded-lg shadow-lg transition duration-200 transform hover:scale-105"
      >
        Iniciar Sesión
      </button>
    </form>

    <!-- Pie de página -->
    <div class="mt-10 text-center text-blue-600">
      <p class="font-medium">Sistema de Balance de Carga Docente <span class="font-bold">v1.0</span></p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'
import { useUIStore } from '../stores/ui'

const username = ref<string>('')
const password = ref<string>('')
const router = useRouter()
const authStore = useAuthStore()
const uiStore = useUIStore()

const handleSubmit = async () => {
  if (!username.value.trim() || !password.value.trim()) {
    uiStore.showWarning('Por favor ingrese usuario y contraseña')
    return
  }

  // Usar el authStore para login
  const result = await authStore.login(username.value.trim(), password.value.trim())
  
  if (result.success) {
    // Login exitoso, mostrar notificación y redirigir
    uiStore.showSuccess('¡Bienvenido! Inicio de sesión exitoso')
    router.push('/dashboard')
  } else {
    // Mostrar mensaje de error con notificación
    uiStore.showError(result.message || 'Error al iniciar sesión')
  }
}
</script>