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
      redirect: '/dashboard' // Redirect a dashboard, el guard verificará autenticación
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
      meta: { 
        requiresAuth: true,
        requiresLeader: true 
      }
    },
    {
      path: '/asignaturas',
      name: 'Asignaturas',
      component: Asignaturas,
      meta: { 
        requiresAuth: true,
        requiresLeaderOrSubjectLeader: true
      }
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
        requiresAdmin: true
      }
    },
  ],
})

router.beforeEach(async (to, _from, next) => {
  const authStore = useAuthStore()
  
  const requiresAuth = to.meta.requiresAuth
  const requiresAdmin = to.meta.requiresAdmin
  const requiresLeader = to.meta.requiresLeader
  const requiresSubjectLeader = to.meta.requiresSubjectLeader
  const requiresLeaderOrSubjectLeader = to.meta.requiresLeaderOrSubjectLeader
  const isLoginPage = to.path === '/login'

  // If the route doesn't require auth and is not login page, allow access
  if (!requiresAuth && !isLoginPage) {
    next()
    return
  }

  // Check authentication once upfront
  const isValid = await authStore.checkAuth()

  // Attempting to access login page
  if (isLoginPage) {
    if (isValid) {
      // Already authenticated, redirect to dashboard
      console.log('Usuario ya autenticado, redirigiendo a dashboard')
      next('/dashboard')
    } else {
      // No valid session, allow access to login
      next()
    }
    return
  }

  // Route requires authentication
  if (requiresAuth) {
    if (!isValid) {
      // JWT invalid, expired or doesn't exist, redirect to login
      console.log('Token JWT inválido o expirado')
      next('/login')
      return
    }

    // JWT valid, now check specific permissions
    if (requiresAdmin && !authStore.isAdmin) {
      console.log('Acceso denegado: requiere permisos de Admin')
      next('/dashboard')
      return
    }

    if (requiresLeader && !authStore.isLeader) {
      console.log('Acceso denegado: requiere permisos de Leader')
      next('/dashboard')
      return
    }

    if (requiresSubjectLeader && !authStore.isSubjectLeader) {
      console.log('Acceso denegado: requiere permisos de Subject Leader')
      next('/dashboard')
      return
    }

    if (requiresLeaderOrSubjectLeader && !authStore.isLeader && !authStore.isSubjectLeader) {
      console.log('Acceso denegado: requiere permisos de Leader o Subject Leader')
      next('/dashboard')
      return
    }

    // Authenticated and with correct permissions
    next()
  } else {
    next()
  }
})

export default router
