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
        requiresSubjectLeader: true
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
        requiresAdmin: true  // Solo administradores
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
  const isLoginPage = to.path === '/login'

  // Si la ruta no requiere autenticación, permitir acceso
  if (!requiresAuth && !isLoginPage) {
    next()
    return
  }

  // Intentando acceder a login
  if (isLoginPage) {
    // Verificar si hay sesión activa (JWT válido en cookie)
    const isValid = await authStore.checkAuth()
    
    if (isValid) {
      // Ya está autenticado, redirigir a dashboard
      console.log('Usuario ya autenticado, redirigiendo a dashboard')
      next('/dashboard')
    } else {
      // No hay sesión válida, permitir acceso a login
      next()
    }
    return
  }

  // Ruta requiere autenticación
  if (requiresAuth) {
    // Siempre verificar con backend primero (valida JWT en cookie)
    const isValid = await authStore.checkAuth()
    
    if (!isValid) {
      // JWT inválido, expirado o no existe, redirigir a login
      console.log('Token JWT inválido o expirado')
      next('/login')
      return
    }

    // JWT válido, ahora verificar permisos específicos
    if ((requiresAdmin && !authStore.isAdmin) || 
        (requiresLeader && !authStore.isLeader) || 
        (requiresSubjectLeader && !authStore.isSubjectLeader)) {
      // No tiene los permisos requeridos
      console.log('Acceso denegado: permisos insuficientes')
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
