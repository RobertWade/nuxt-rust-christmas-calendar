import { useUserStore } from '@/stores/useUserStore'

export default defineNuxtRouteMiddleware((to) => {
  const path = typeof to.path === 'string' ? to.path : `${to.fullPath}`
  if (!path.startsWith('/account')) { return }
  const userStore = useUserStore()
  if (!userStore.user) {
    return navigateTo('/signup')
  }
})
