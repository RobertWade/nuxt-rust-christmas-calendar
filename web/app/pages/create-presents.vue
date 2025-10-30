<script lang="ts" setup>
import { storeToRefs } from 'pinia'
import { useCalendarStore } from '@/stores/useCalendarStore'
import type { CalendarDoor } from '@/types'

const calendarStore = useCalendarStore()
const { doorList, currentDoor } = storeToRefs(calendarStore)
const router = useRouter()

const hasCalendar = computed(() => !!calendarStore.calendar)

watchEffect(() => {
    if (!hasCalendar.value && import.meta.client) {
        router.replace('/new-calendar')
    }
})

function handleDoorSelect(day: number) {
    calendarStore.setCurrentDoor(day)
    router.push({ path: '/edit-present', query: { day } })
}

function goToPreview() {
    router.push('/view-calendar')
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
</script>

<template>
    <CalPageGrid>
        <CalHeader icon="🎁" :title="`Edit: ${calendarStore.calendar?.name}`" back-button />
        <section class="space-y-2">
            <p v-if="doorList.length" class="text-sm text-gray-600">
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
        <CalButton
            :disabled="!doorList.length"
            @click="goToPreview"
        >
            Preview calendar
        </CalButton>
    </CalPageGrid>
</template>