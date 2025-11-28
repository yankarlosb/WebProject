<!--
  Perfil - Vista de perfil personal del usuario
  Todos los usuarios pueden acceder y editar su información personal
-->
<template>
  <AppLayout>
    <div class="max-w-4xl mx-auto">
      <!-- Header -->
      <div class="mb-8">
        <h1 class="text-3xl font-bold text-blue-700">Mi Perfil</h1>
        <p class="text-gray-600 mt-2">Administra tu información personal</p>
      </div>

      <!-- Card de información del usuario -->
      <AppCard title="Información Personal" class="mb-6">
        <form @submit.prevent="handleSaveProfile" class="space-y-6">
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- Nombre -->
            <AppInput
              v-model="form.name"
              label="Nombre completo"
              placeholder="Ingresa tu nombre"
              required
              :error="errors.name"
            >
              <template #iconLeft>
                <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                </svg>
              </template>
            </AppInput>

            <!-- Email -->
            <AppInput
              v-model="form.email"
              type="email"
              label="Correo electrónico"
              placeholder="correo@ejemplo.com"
              required
              :error="errors.email"
            >
              <template #iconLeft>
                <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
                </svg>
              </template>
            </AppInput>
          </div>

          <!-- Rol (solo lectura) -->
          <div class="p-4 bg-blue-50 rounded-lg border border-blue-200">
            <div class="flex items-center gap-2 text-sm text-blue-800">
              <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              <span class="font-medium">Rol actual:</span>
              <span class="font-bold">{{ authStore.userRole }}</span>
            </div>
          </div>

          <!-- Botones de acción -->
          <div class="flex justify-end gap-3 pt-4 border-t border-gray-200">
            <AppButton
              type="button"
              variant="ghost"
              @click="resetForm"
            >
              Cancelar
            </AppButton>
            
            <AppButton
              type="submit"
              variant="primary"
              :loading="isLoading"
            >
              <template #icon>
                <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                </svg>
              </template>
              Guardar cambios
            </AppButton>
          </div>
        </form>
      </AppCard>

      <!-- Card de cambio de contraseña -->
      <AppCard title="Cambiar Contraseña">
        <form @submit.prevent="handleChangePassword" class="space-y-6">
          <div class="space-y-4">
            <!-- Nueva contraseña -->
            <AppInput
              v-model="passwordForm.newPassword"
              type="password"
              label="Nueva contraseña"
              placeholder="••••••••"
              required
              :error="passwordErrors.newPassword"
              hint="Mínimo 8 caracteres"
            >
              <template #iconLeft>
                <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z" />
                </svg>
              </template>
            </AppInput>

            <!-- Confirmar contraseña -->
            <AppInput
              v-model="passwordForm.confirmPassword"
              type="password"
              label="Confirmar nueva contraseña"
              placeholder="••••••••"
              required
              :error="passwordErrors.confirmPassword"
            >
              <template #iconLeft>
                <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
              </template>
            </AppInput>
          </div>

          <!-- Botones de acción -->
          <div class="flex justify-end gap-3 pt-4 border-t border-gray-200">
            <AppButton
              type="button"
              variant="ghost"
              @click="resetPasswordForm"
            >
              Cancelar
            </AppButton>
            
            <AppButton
              type="submit"
              variant="warning"
              :loading="isChangingPassword"
            >
              <template #icon>
                <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z" />
                </svg>
              </template>
              Cambiar contraseña
            </AppButton>
          </div>
        </form>
      </AppCard>
    </div>
  </AppLayout>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useAuthStore } from '../stores/auth'
import { useUIStore } from '../stores/ui'
import ProfileService from '../services/profile'
import AppLayout from '../components/AppLayout.vue'
import AppCard from '../components/AppCard.vue'
import AppInput from '../components/AppInput.vue'
import AppButton from '../components/AppButton.vue'
import { isValidName, isValidEmail, isValidPassword } from '../utils/validation'

const authStore = useAuthStore()
const uiStore = useUIStore()

const isLoading = ref(false)
const isChangingPassword = ref(false)

// Formulario de perfil
const form = ref({
  name: '',
  email: '',
})

const errors = ref({
  name: '',
  email: '',
})

// Formulario de contraseña
const passwordForm = ref({
  newPassword: '',
  confirmPassword: '',
})

const passwordErrors = ref({
  newPassword: '',
  confirmPassword: '',
})

onMounted(() => {
  // Cargar datos del usuario actual
  if (authStore.user) {
    form.value.name = authStore.user.name
    form.value.email = authStore.user.email
  }
})

function resetForm() {
  if (authStore.user) {
    form.value.name = authStore.user.name
    form.value.email = authStore.user.email
  }
  errors.value = { name: '', email: '' }
}

function resetPasswordForm() {
  passwordForm.value = {
    newPassword: '',
    confirmPassword: '',
  }
  passwordErrors.value = {
    newPassword: '',
    confirmPassword: '',
  }
}

async function handleSaveProfile() {
  // Validación básica
  errors.value = { name: '', email: '' }
  
  const trimmedName = form.value.name.trim()
  const trimmedEmail = form.value.email.trim()
  
  if (!trimmedName) {
    errors.value.name = 'El nombre es requerido'
    return
  }
  
  if (!isValidName(trimmedName)) {
    errors.value.name = 'Nombre inválido'
    return
  }
  
  if (!trimmedEmail) {
    errors.value.email = 'El correo es requerido'
    return
  }
  
  if (!isValidEmail(trimmedEmail)) {
    errors.value.email = 'Email inválido'
    return
  }

  isLoading.value = true

  try {
    // Actualizar perfil en el backend
    const result = await ProfileService.updateProfile({
      name: trimmedName,
      email: trimmedEmail,
    })

    if (result.success) {
      // Actualizar el store local con los nuevos datos
      authStore.updateUser({
        name: form.value.name,
        email: form.value.email,
      })

      uiStore.showSuccess('Perfil actualizado correctamente')
    } else {
      uiStore.showError(result.message || 'Error al actualizar el perfil')
    }
  } catch (error) {
    console.error('Error actualizando perfil:', error)
    uiStore.showError('Error al actualizar el perfil')
  } finally {
    isLoading.value = false
  }
}

async function handleChangePassword() {
  // Validación
  passwordErrors.value = {
    newPassword: '',
    confirmPassword: '',
  }

  const trimmedPassword = passwordForm.value.newPassword.trim()

  if (!trimmedPassword) {
    passwordErrors.value.newPassword = 'La nueva contraseña es requerida'
    return
  }

  if (!isValidPassword(trimmedPassword)) {
    passwordErrors.value.newPassword = 'Contraseña inválida (mínimo 8 caracteres)'
    return
  }

  if (trimmedPassword !== passwordForm.value.confirmPassword.trim()) {
    passwordErrors.value.confirmPassword = 'Las contraseñas no coinciden'
    return
  }

  isChangingPassword.value = true

  try {
    // Cambiar contraseña en el backend
    const result = await ProfileService.changePassword(trimmedPassword)
    
    if (result.success) {
      uiStore.showSuccess('Contraseña cambiada correctamente')
      resetPasswordForm()
    } else {
      uiStore.showError(result.message || 'Error al cambiar la contraseña')
    }
  } catch (error) {
    console.error('Error cambiando contraseña:', error)
    uiStore.showError('Error al cambiar la contraseña')
  } finally {
    isChangingPassword.value = false
  }
}
</script>
