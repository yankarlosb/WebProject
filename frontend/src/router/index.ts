import { createRouter, createWebHistory } from 'vue-router'
import Login  from '../views/Login.vue'
import BalanceForm from '../views/BalanceForm.vue'  
import Dashboard from '../views/Dashboard.vue'
import Configuracion from '../views/Configuracion.vue'
import Asignaturas from '../views/Asignaturas.vue'
import Perfil from '../views/Perfil.vue'
import { useAuthStore } from '../stores/auth'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      redirect: '/login'
    },
    {
      path: '/login',
      name: 'Login',
      component: Login,
      meta: { requiresAuth: false }
    },
    {
      path: '/dashboard',
      name: 'Dashboard',
      component: Dashboard,
      meta: { requiresAuth: true }
    },
    {
      path: '/balance',
      name: 'BalanceForm',
      component: BalanceForm,
      meta: { requiresAuth: true }
    },
    {
      path: '/asignaturas',
      name: 'Asignaturas',
      component: Asignaturas,
      meta: { requiresAuth: true }
    },
    {
      path: '/perfil',
      name: 'Perfil',
      component: Perfil,
      meta: { requiresAuth: true }
    },
    {
      path: '/configuracion',
      name: 'Configuracion',
      component: Configuracion,
      meta: { 
        requiresAuth: true,
        requiresAdmin: true  // Solo administradores
      }
    },
  ],
})

router.beforeEach(async (to, _from, next) => {
  const authStore = useAuthStore()
  
  const requiresAuth = to.meta.requiresAuth
  const requiresAdmin = to.meta.requiresAdmin
  const isLoginPage = to.path === '/login'

  // Si la ruta no requiere autenticación, permitir acceso
  if (!requiresAuth && !isLoginPage) {
    next()
    return
  }

  // Intentando acceder a login
  if (isLoginPage) {
    if (authStore.isAuthenticated) {
      // Verificar autenticación real con backend
      const isValid = await authStore.checkAuth()
      
      if (isValid) {
        // Ya está autenticado, redirigir a dashboard
        next('/dashboard')
      } else {
        // Token inválido, permitir acceso a login
        next()
      }
    } else {
      // No autenticado, permitir acceso a login
      next()
    }
    return
  }

  // Ruta requiere autenticación
  if (requiresAuth) {
    if (!authStore.isAuthenticated) {
      // No hay autenticación local, redirigir a login
      next('/login')
      return
    }

    // Verificar con backend
    const isValid = await authStore.checkAuth()
    
    if (!isValid) {
      // JWT inválido o expirado, redirigir a login
      console.log('Token JWT inválido o expirado')
      next('/login')
      return
    }

    // Verificar permisos de administrador si es necesario
    if (requiresAdmin && !authStore.isAdmin) {
      // No tiene permisos de administrador
      console.log('Acceso denegado: se requieren permisos de administrador')
      next('/dashboard') // Redirigir a dashboard
      return
    }

    // Autenticado y con permisos correctos
    next()
  } else {
    next()
  }
})

export default router
