<script lang="ts" setup>
import { computed, onMounted, ref, watchEffect } from 'vue'
import { storeToRefs } from 'pinia'
import type { NavigationGuard } from 'vue-router'
import { useCalendarStore } from '@/stores/useCalendarStore'

const requireAuth = 'auth' as unknown as NavigationGuard

definePageMeta({
    middleware: requireAuth,
})

const calendarStore = useCalendarStore()
const { calendar, doorList, currentDoor } = storeToRefs(calendarStore)
const router = useRouter()
const route = useRoute()

const loadingCalendar = ref(true)
const loadError = ref<string | null>(null)
const navigationError = ref<string | null>(null)
const navigating = ref(false)

const hasCalendar = computed(() => !!calendar.value)

watchEffect(() => {
    if (!loadingCalendar.value && !hasCalendar.value && import.meta.client) {
        router.replace('/new-calendar')
    }
})

const availableDoor = computed(() => doorList.value.find((door) => door.state === 'available') ?? null)
const openedDoors = computed(() => doorList.value.filter((door) => door.state === 'opened').length)

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

function handleDoorSelect(day: number, disabled: boolean) {
    if (disabled) {
        return
    }
    calendarStore.setCurrentDoor(day)
    router.push({
        path: '/door-open',
        query: {
            day,
            calendarId: calendar.value?.id,
        },
    })
}

async function goToEditor() {
    if (!calendar.value) {
        router.push('/new-calendar')
        return
    }

    navigationError.value = null
    navigating.value = true
    try {
        await calendarStore.ensureCalendarLoaded(calendar.value.id)
        await router.push({ path: '/create-presents', query: { calendarId: calendar.value.id } })
    } catch (error) {
        navigationError.value = extractErrorMessage(error)
    } finally {
        navigating.value = false
    }
}

onMounted(() => {
    ensureCalendar()
})
</script>

<template>
    <CalPageGrid>
        <CalHeader
            back-button
            :title="calendar?.name || 'Your calendar'"
            :description="calendar?.recipientName ? `For ${calendar.recipientName}` : ''"
        />
        <section class="space-y-2">
            <p v-if="loadingCalendar" class="text-sm text-gray-500">Loading calendar…</p>
            <p v-if="loadError" class="text-sm text-red-500">{{ loadError }}</p>
            <p v-else class="text-sm text-gray-600">
                {{ calendar?.description || 'Keep the magic alive by opening one door per day.' }}
            </p>
            <p v-if="availableDoor" class="text-sm text-primary">
                Next surprise: Door {{ availableDoor.day }}
            </p>
            <p class="text-xs text-gray-500">
                {{ openedDoors }} doors already opened · {{ doorList.length - openedDoors }} waiting
            </p>
        </section>
        <div class="grid grid-cols-4 gap-4">
            <CalDoor
                v-for="door in doorList"
                :key="door.day"
                :title="door.day"
                :icon="door.present && door.present.content.tasks?.length ? '📝' : (door.present ? '🎁' : '')"
                :active="door.day === currentDoor"
                :disabled="door.state === 'locked'"
                @select="handleDoorSelect(door.day, door.state === 'locked')"
            />
        </div>
        <p v-if="navigationError" class="text-sm text-red-500">{{ navigationError }}</p>
        <CalButton type="button" alt-style :disabled="navigating" @click="goToEditor">
            Edit calendar
        </CalButton>
    </CalPageGrid>
</template>