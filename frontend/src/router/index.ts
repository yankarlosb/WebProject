import { createRouter, createWebHistory } from 'vue-router'
import Login  from '../views/Login.vue'
import BalanceForm from '../views/BalanceForm.vue'  
import Dashboard from '../views/Dashboard.vue'
import Configuracion from '../views/Configuracion.vue'
import Asignaturas from '../views/Asignaturas.vue'
import AuthService from '../services/auth'

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
      component: Login
    },
    {
      path: '/balance',
      name: 'BalanceForm',
      component: BalanceForm,
      meta: { requiresAuth: true }
    },
    {
      path: '/dashboard',
      name: 'Dashboard',
      component: Dashboard,
      meta: { requiresAuth: true }
    },
    {
      path: '/configuracion',
      name: 'Configuracion',
      component: Configuracion,
      meta: { requiresAuth: true }
    },
    {
      path: '/asignaturas',
      name: 'asignaturas',
      component: Asignaturas,
      meta: { requiresAuth: true }
    },
  ],
})

router.beforeEach(async (to, _from, next) => {
  const requiresAuth = to.meta.requiresAuth
  const isLoginPage = to.path === '/login'

  // Si la ruta no requiere autenticación, permitir acceso
  if (!requiresAuth && !isLoginPage) {
    next()
    return
  }

  // Verificación rápida con sessionStorage (evita llamadas innecesarias)
  const hasLocalAuth = AuthService.isLocallyAuthenticated()

  if (requiresAuth) {
    // La ruta requiere autenticación
    if (!hasLocalAuth) {
      // No hay flag local, redirigir directamente a login
      next('/login')
      return
    }

    // Hay flag local, verificar con el backend (valida JWT real)
    const isAuthenticated = await AuthService.checkAuth()
    
    if (isAuthenticated) {
      // JWT válido, permitir acceso
      next()
    } else {
      // JWT inválido o expirado, redirigir a login
      console.log('Token JWT inválido o expirado')
      next('/login')
    }
  } else if (isLoginPage) {
    // Intentando acceder a login
    if (hasLocalAuth) {
      // Verificar si realmente está autenticado
      const isAuthenticated = await AuthService.checkAuth()
      
      if (isAuthenticated) {
        // Ya está autenticado, redirigir a dashboard
        next('/dashboard')
      } else {
        // Token inválido, permitir acceso a login
        next()
      }
    } else {
      // No hay autenticación local, permitir acceso a login
      next()
    }
  } else {
    next()
  }
})

export default router
