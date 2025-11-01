<script lang="ts" setup>
import { useCalendarStore } from '@/stores/useCalendarStore'
import { useUserStore } from '@/stores/useUserStore'
import type { Calendar, CalendarDoor } from '@/types'

const calendarStore = useCalendarStore()
const userStore = useUserStore()
const router = useRouter()

const form = reactive({
    name: '',
    recipientName: '',
    description: ''
})

const loading = ref(false)
const errorMessage = ref<string | null>(null)

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
            present: null
        }
    })
}

async function handleSubmit() {
    if (!form.name.trim()) {
        errorMessage.value = 'Please provide a calendar name.'
        return
    }

    errorMessage.value = null
    loading.value = true

    try {
        const now = new Date().toISOString()

        const calendar: Calendar = {
            id: calendarStore.calendar?.id ?? `cal-${Date.now()}`,
            ownerId: userStore.user?.id ?? 'guest',
            name: form.name,
            description: form.description || undefined,
            recipientName: form.recipientName || undefined,
            status: calendarStore.calendar?.status ?? 'draft',
            createdAt: calendarStore.calendar?.createdAt ?? now,
            updatedAt: now,
            publishedAt: calendarStore.calendar?.publishedAt ?? null,
            doors: calendarStore.calendar?.doors.length ? calendarStore.calendar.doors : createDoors()
        }

        calendarStore.setCalendar(calendar)
        await router.push(`/account/calendars/${calendar.id}/presents`)
    } finally {
        loading.value = false
    }
}
</script>

<template>
    <CalPageGrid v-if="!calendarStore.calendar">
        <CalHeader title="Create New Calendar" icon="🎄" />
        <form v-if="!calendarStore.calendar" class="flex flex-col gap-4 max-w-sm" @submit.prevent="handleSubmit">
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
    </CalPageGrid>
    <CalPageGrid v-else>
        <CalHeader title="Your Calendars" icon="🎄" />
        <CalButton :to="`/account/calendars/${calendarStore.calendar.id}/presents`" alt-style>{{ calendarStore.calendar.name }}</CalButton>
        <CalButton disabled @click="calendarStore.clearCalendar">Create More Calendars (coming soon)</CalButton>
    </CalPageGrid>
</template>