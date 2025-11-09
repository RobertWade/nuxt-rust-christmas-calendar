<script lang="ts" setup>
import { computed, onMounted, reactive, ref, watchEffect } from 'vue'
import type { NavigationGuard } from 'vue-router'
import { useCalendarStore } from '@/stores/useCalendarStore'
import { useUserStore } from '@/stores/useUserStore'
import type { Calendar, CalendarDoor } from '@/types'

const requireAuth = 'auth' as unknown as NavigationGuard

definePageMeta({
    middleware: requireAuth,
})

const calendarStore = useCalendarStore()
const userStore = useUserStore()
const router = useRouter()
const route = useRoute()

const form = reactive({
    name: '',
    recipientName: '',
    description: '',
})

const loading = ref(false)
const initializing = ref(true)
const errorMessage = ref<string | null>(null)
const loadError = ref<string | null>(null)
const navigationError = ref<string | null>(null)
const navigating = ref(false)

const calendarList = computed(() => calendarStore.calendars)

let prefilled = false

watchEffect(() => {
    if (prefilled || !calendarStore.calendar) {
        return
    }

    const existing = calendarStore.calendar
    form.name = existing.name
    form.recipientName = existing.recipientName ?? ''
    form.description = existing.description ?? ''
    prefilled = true
})

function createDoors(): CalendarDoor[] {
    const now = new Date()
    const year = now.getFullYear()
    return Array.from({ length: 24 }, (_, index) => {
        const day = index + 1
        const openDate = new Date(year, 11, day, 6)
        return {
            day,
            opensAt: openDate.toISOString(),
            state: day === 1 ? 'available' : 'locked',
            present: null,
        }
    })
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
            'Unable to load your calendars right now.'
        )
    }
    return 'Unable to load your calendars right now.'
}

async function ensureCalendar() {
    initializing.value = true
    loadError.value = null
    try {
        await calendarStore.ensureCalendarLoaded(route.query.calendarId as string | undefined)
    } catch (error) {
        loadError.value = extractErrorMessage(error)
    } finally {
        initializing.value = false
    }
}

async function handleSubmit() {
    if (!form.name.trim()) {
        errorMessage.value = 'Please provide a calendar name.'
        return
    }

    errorMessage.value = null
    loading.value = true

    try {
        const created = await calendarStore.createCalendar({
            name: form.name.trim(),
        })

        const withDetails: Calendar = {
            ...created,
            description: form.description || created.description,
            recipientName: form.recipientName || created.recipientName,
            doors: created.doors.length ? created.doors : createDoors(),
        }

        calendarStore.setCalendar(withDetails)
            calendarStore.setCalendars([
                ...calendarList.value.filter((item) => item.id !== withDetails.id),
                withDetails,
            ])
        await router.push('/create-presents')
    } catch (error) {
        errorMessage.value = extractErrorMessage(error)
    } finally {
        loading.value = false
    }
}

async function openCalendarEditor(targetId: string) {
    navigationError.value = null
    navigating.value = true
    try {
        await calendarStore.ensureCalendarLoaded(targetId)
        await router.push({ path: '/create-presents', query: { calendarId: targetId } })
    } catch (error) {
        navigationError.value = extractErrorMessage(error)
    } finally {
        navigating.value = false
    }
}

onMounted(() => {
    if (!userStore.session) {
        return
    }
    ensureCalendar()
})
</script>

<template>
    <CalPageGrid>
        <CalHeader title="Create New Calendar" icon="🎄" />
        <div v-if="initializing" class="text-sm text-gray-500">Loading your calendars…</div>
        <p v-if="loadError" class="text-sm text-red-500">{{ loadError }}</p>
        <form class="flex flex-col gap-4 max-w-sm" @submit.prevent="handleSubmit">
            <div class="flex flex-col">
                <label for="calendar-name" class="mb-1">Calendar Name</label>
                <CalTextBox id="calendar-name" v-model="form.name" placeholder="Enter calendar name" />
            </div>
            <div class="flex flex-col">
                <label for="recipient" class="mb-1">Recipient</label>
                <CalTextBox id="recipient" v-model="form.recipientName" placeholder="Enter recipient name" />
            </div>
            <div class="flex flex-col">
                <label for="description" class="mb-1">Description (optional)</label>
                <CalTextBox id="description" v-model="form.description" placeholder="Enter calendar description" />
            </div>
            <p v-if="errorMessage" class="text-sm text-red-500">{{ errorMessage }}</p>
            <CalButton type="submit" :disabled="loading">{{ loading ? 'Creating…' : 'Next' }}</CalButton>
        </form>
        <section v-if="calendarList.length" class="mt-8 flex flex-col gap-2">
            <p class="text-sm text-gray-600">Your calendars</p>
            <p v-if="navigationError" class="text-sm text-red-500">{{ navigationError }}</p>
            <CalButton
                v-for="item in calendarList"
                :key="item.id"
                type="button"
                alt-style
                :disabled="navigating"
                @click="openCalendarEditor(item.id)"
            >
                {{ item.name }}
            </CalButton>
        </section>
    </CalPageGrid>
</template>