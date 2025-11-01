<script lang="ts" setup>
import { useUserStore } from '@/stores/useUserStore'

const email = ref('')
const password = ref('')
const loading = ref(false)
const errorMessage = ref<string | null>(null)

const userStore = useUserStore()
const router = useRouter()

function generateId() {
    if (typeof crypto !== 'undefined' && 'randomUUID' in crypto) {
        return crypto.randomUUID()
    }
    return `user-${Date.now()}`
}

function deriveName(address: string) {
    return address.split('@')[0] || 'Guest'
}

async function handleSignIn() {
    if (!email.value || !password.value) {
        errorMessage.value = 'Please enter your email and password.'
        return
    }

    errorMessage.value = null
    loading.value = true

    try {
        userStore.setUser({
            id: generateId(),
            name: deriveName(email.value),
            email: email.value,
            avatarUrl: null
        })

        await router.push('/account/calendars/new')
    } finally {
        loading.value = false
    }
}
</script>

<template>
    <CalPageGrid>
        <CalHeader icon="🎁" title="Christmas Calendar" />
        <section>
            <form class="flex flex-col gap-4 max-w-sm" @submit.prevent="handleSignIn">
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
                        placeholder="Enter your password"
                    />
                </div>
                <p v-if="errorMessage" class="text-sm text-red-500">{{ errorMessage }}</p>
                <CalButton type="submit" :disabled="loading">{{ loading ? 'Signing In…' : 'Sign In' }}</CalButton>
                <CalButton type="button" alt-style disabled>Sign up (soon)</CalButton>
            </form>
        </section>
    </CalPageGrid>
</template>
