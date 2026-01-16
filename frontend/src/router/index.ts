import { createRouter, createWebHistory } from 'vue-router'
import Login  from '../views/Login.vue'
import BalanceForm from '../views/BalanceForm.vue'  
import BalanceView from '../views/BalanceView.vue'
import BalanceFragmentEdit from '../views/BalanceFragmentEdit.vue'
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
      path: '/balance/view',
      name: 'BalanceView',
      component: BalanceView,
      meta: { requiresAuth: true }
    },
    {
      path: '/balance/fragment',
      name: 'BalanceFragmentEdit',
      component: BalanceFragmentEdit,
      meta: { 
        requiresAuth: true,
        // SubjectLeader o Leader (si allow_leader_edit está habilitado)
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

  // Check auth when:
  // 1. Not currently marked as authenticated (need to verify)
  // 2. Accessing login page (need to redirect if already logged in)
  // 3. Accessing protected route while not authenticated in store
  const shouldVerifyAuth = !authStore.isAuthenticated || isLoginPage
  
  let isValid: boolean
  if (shouldVerifyAuth) {
    isValid = await authStore.checkAuth()
  } else {
    // Already authenticated in store, trust the cached state for navigation
    // The JWT validation happens on the backend for actual API calls
    isValid = true
  }

  // Attempting to access login page
  if (isLoginPage) {
    if (isValid) {
      // Already authenticated, redirect to dashboard
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
      next('/login')
      return
    }

    // Force password change if required
    if (authStore.mustChangePassword && to.path !== '/perfil') {
      next('/perfil')
      return
    }

    // JWT valid, now check specific permissions
    if (requiresAdmin && !authStore.isAdmin) {
      next('/dashboard')
      return
    }

    if (requiresLeader && !authStore.isLeader) {
      next('/dashboard')
      return
    }

    if (requiresSubjectLeader && !authStore.isSubjectLeader) {
      next('/dashboard')
      return
    }

    if (requiresLeaderOrSubjectLeader && !authStore.isLeader && !authStore.isSubjectLeader) {
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
