import { computed, ref } from 'vue'
import { defineStore } from 'pinia'
import type { AuthSession, SignInPayload, SignUpPayload, User } from '@/types'

interface PasswordResetResult {
  success: boolean
  resetToken?: string
  expiresAt?: string
}

export const useUserStore = defineStore(
  "user",
  () => {
    const user = ref<User | null>(null)
    const session = ref<AuthSession | null>(null)

    const isAuthenticated = computed(() => Boolean(session.value))

    function setSession(payload: AuthSession) {
      session.value = payload
      user.value = payload.user
    }

    function setUser(payload: User | null) {
      user.value = payload
    }

    function clearUser() {
      user.value = null
    }

    function clearSession() {
      session.value = null
      clearUser()
    }

    function logout() {
      clearSession()
    }

    async function register(payload: SignUpPayload) {
      const response = await $fetch<AuthSession>('/api/auth/register', {
        method: 'POST',
        body: payload,
      })

      setSession(response)
      return response
    }

    async function login(payload: SignInPayload) {
      const response = await $fetch<AuthSession>('/api/auth/login', {
        method: 'POST',
        body: payload,
      })

      setSession(response)
      return response
    }

    async function requestPasswordReset(email: string) {
      const result = await $fetch<PasswordResetResult>('/api/auth/password/request', {
        method: 'POST',
        body: { email },
      })

      return result
    }

    async function resetPassword(token: string, password: string) {
      await $fetch('/api/auth/password/reset', {
        method: 'POST',
        body: { token, password },
      })
    }

    function getAuthHeaders(): Record<string, string> {
      if (!session.value) {
        return {}
      }

      return {
        Authorization: `Bearer ${session.value.token}`,
      }
    }

    return {
      user,
      session,
      isAuthenticated,
      register,
      login,
      requestPasswordReset,
      resetPassword,
      setSession,
      setUser,
      clearSession,
      clearUser,
      getAuthHeaders,
      logout,
    }
  },
  {
    persist: true,
  }
);