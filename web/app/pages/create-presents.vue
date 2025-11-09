<script lang="ts" setup>
import { computed, onMounted, ref, watchEffect } from 'vue'
import { storeToRefs } from 'pinia'
import type { NavigationGuard } from 'vue-router'
import { useCalendarStore } from '@/stores/useCalendarStore'
import type { CalendarDoor } from '@/types'

const requireAuth = 'auth' as unknown as NavigationGuard

definePageMeta({
    middleware: requireAuth,
})

const calendarStore = useCalendarStore()
const { doorList, currentDoor } = storeToRefs(calendarStore)
const router = useRouter()
const route = useRoute()

const hasCalendar = computed(() => !!calendarStore.calendar)
const loadingCalendar = ref(true)
const loadError = ref<string | null>(null)

watchEffect(() => {
    if (!loadingCalendar.value && !hasCalendar.value && import.meta.client) {
        router.replace('/new-calendar')
    }
})

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
            'Unable to load the calendar.'
        )
    }
    return 'Unable to load the calendar.'
}

async function ensureCalendar() {
    loadingCalendar.value = true
    loadError.value = null
    try {
        await calendarStore.ensureCalendarLoaded(route.query.calendarId as string | undefined)
    } catch (error) {
        loadError.value = extractErrorMessage(error)
    } finally {
        loadingCalendar.value = false
    }
}

function handleDoorSelect(day: number) {
    calendarStore.setCurrentDoor(day)
    router.push({
        path: '/edit-present',
        query: {
            day,
            calendarId: calendarStore.calendar?.id,
        },
    })
}

function goToPreview() {
    router.push({
        path: '/view-calendar',
        query: { calendarId: calendarStore.calendar?.id },
    })
}

function doorIcon(door: CalendarDoor) {
    const type = door.present?.content.media?.type
    if (!door.present) {
        return ''
    }
    if (door.present.content.tasks && door.present.content.tasks.length !== 0) {
        return '📝'
    }
    switch (type) {
        case 'image':
            return '🖼️'
        case 'video':
            return '🎬'
        case 'audio':
            return '🎧'
        case 'link':
            return '🔗'
        default:
            return '🎁'
    }
}

onMounted(() => {
    ensureCalendar()
})
</script>

<template>
    <CalPageGrid>
        <CalHeader icon="🎁" :title="`Edit: ${calendarStore.calendar?.name ?? ''}`" back-button back-link="/new-calendar" />
        <section class="space-y-2">
            <p v-if="loadingCalendar" class="text-sm text-gray-500">Loading calendar…</p>
            <p v-if="loadError" class="text-sm text-red-500">{{ loadError }}</p>
            <p v-else-if="doorList.length" class="text-sm text-gray-600">
                Select a door to add a present, message, or small mission for that day.
            </p>
        </section>
        <div class="grid grid-cols-4 gap-4">
            <CalDoor
                v-for="door in doorList"
                :key="door.day"
                :title="door.day"
                :icon="doorIcon(door)"
                :active="door.day === currentDoor"
                @select="handleDoorSelect(door.day)"
            />
        </div>
        <CalButton :disabled="!doorList.length" @click="goToPreview">
            Preview calendar
        </CalButton>
    </CalPageGrid>
</template>