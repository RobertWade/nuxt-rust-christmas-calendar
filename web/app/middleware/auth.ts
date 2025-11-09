import { storeToRefs } from 'pinia'
import { useUserStore } from '@/stores/useUserStore'

export default defineNuxtRouteMiddleware((to) => {
  const userStore = useUserStore()
  const { isAuthenticated } = storeToRefs(userStore)

  if (isAuthenticated.value) {
    return
  }

  const redirect = encodeURIComponent(to.fullPath)
  return navigateTo(`/signup?redirect=${redirect}`)
})
