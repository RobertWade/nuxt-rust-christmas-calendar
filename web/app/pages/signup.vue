<script lang="ts" setup>
import { computed, ref, watch } from 'vue'
import { storeToRefs } from 'pinia'
import { useUserStore } from '@/stores/useUserStore'
import { useCalendarStore } from '@/stores/useCalendarStore'

const mode = ref<'login' | 'signup'>(
    (useRoute().query.mode as 'login' | 'signup' | undefined) === 'signup' ? 'signup' : 'login',
)
const name = ref('')
const email = ref('')
const password = ref('')
const loading = ref(false)
const errorMessage = ref<string | null>(null)
const resetMessage = ref<string | null>(null)

const userStore = useUserStore()
const calendarStore = useCalendarStore()
const router = useRouter()
const route = useRoute()
const { isAuthenticated } = storeToRefs(userStore)

const redirectTo = computed(() => (route.query.redirect as string | undefined) || '/new-calendar')
const title = computed(() => (mode.value === 'login' ? 'Welcome back' : 'Create your account'))
const primaryCta = computed(() => (mode.value === 'login' ? 'Sign In' : 'Sign Up'))
const secondaryCta = computed(() =>
    mode.value === 'login' ? 'Need an account? Sign up' : 'Already registered? Sign in',
)
const showNameField = computed(() => mode.value === 'signup')

watch(
    isAuthenticated,
    (value) => {
        if (value) {
            router.replace(redirectTo.value)
        }
    },
    { immediate: true },
)

function deriveName(address: string) {
    return address.split('@')[0] || 'Guest'
}

function extractErrorMessage(error: unknown) {
    if (error && typeof error === 'object') {
        const payload = error as {
            data?: { message?: string; error?: string }
            statusMessage?: string
            message?: string
        }
        return (
            payload.data?.message ||
            payload.data?.error ||
            payload.statusMessage ||
            payload.message ||
            'Unable to process your request right now.'
        )
    }
    return 'Unable to process your request right now.'
}

function toggleMode() {
    mode.value = mode.value === 'login' ? 'signup' : 'login'
    errorMessage.value = null
    resetMessage.value = null
}

async function handleSubmit() {
    if (!email.value.trim() || !password.value.trim()) {
        errorMessage.value = 'Please provide both email and password.'
        return
    }

    if (showNameField.value && !name.value.trim()) {
        name.value = deriveName(email.value)
    }

    errorMessage.value = null
    resetMessage.value = null
    loading.value = true

    try {
        if (mode.value === 'signup') {
            await userStore.register({
                name: name.value.trim() || deriveName(email.value),
                email: email.value.trim(),
                password: password.value,
            })
        } else {
            await userStore.login({
                email: email.value.trim(),
                password: password.value,
            })
        }

        await calendarStore.fetchCalendars().catch(() => undefined)
    } catch (error) {
        errorMessage.value = extractErrorMessage(error)
    } finally {
        loading.value = false
    }
}

async function handlePasswordReset() {
    if (!email.value.trim()) {
        errorMessage.value = 'Please enter your email first.'
        return
    }

    errorMessage.value = null
    resetMessage.value = null
    loading.value = true

    try {
        const result = await userStore.requestPasswordReset(email.value.trim())
        resetMessage.value = result.resetToken
            ? `Temporary reset token: ${result.resetToken}`
            : 'If this address exists, a reset email is on its way.'
    } catch (error) {
        errorMessage.value = extractErrorMessage(error)
    } finally {
        loading.value = false
    }
}
</script>

<template>
    <CalPageGrid>
        <CalHeader icon="🎁" :title="title" />
        <section>
            <form class="flex flex-col gap-4 max-w-sm" @submit.prevent="handleSubmit">
                <div v-if="showNameField" class="flex flex-col">
                    <label for="name" class="mb-1">Display name</label>
                    <CalTextBox
                        id="name"
                        v-model="name"
                        type="text"
                        autocomplete="name"
                        placeholder="How should we call you?"
                    />
                </div>
                <div class="flex flex-col">
                    <label for="email" class="mb-1">E-mail</label>
                    <CalTextBox
                        id="email"
                        v-model="email"
                        type="email"
                        autocomplete="email"
                        placeholder="Enter your email"
                    />
                </div>
                <div class="flex flex-col">
                    <label for="password" class="mb-1">Password</label>
                    <CalTextBox
                        id="password"
                        v-model="password"
                        type="password"
                        autocomplete="current-password"
                        placeholder="Enter your password"
                    />
                </div>
                <p v-if="errorMessage" class="text-sm text-red-500">{{ errorMessage }}</p>
                <p v-if="resetMessage" class="text-sm text-emerald-600">{{ resetMessage }}</p>
                <CalButton type="submit" :disabled="loading">
                    {{ loading ? 'Please wait…' : primaryCta }}
                </CalButton>
                <CalButton type="button" alt-style :disabled="loading" @click="toggleMode">
                    {{ secondaryCta }}
                </CalButton>
                <CalButton type="button" alt-style :disabled="loading" @click="handlePasswordReset">
                    Forgot password?
                </CalButton>
            </form>
        </section>
    </CalPageGrid>
</template>
