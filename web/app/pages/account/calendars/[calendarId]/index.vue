<script lang="ts" setup>
import { storeToRefs } from 'pinia'
import { useCalendarStore } from '@/stores/useCalendarStore'

const calendarStore = useCalendarStore()
const { calendar, doorList, currentDoor } = storeToRefs(calendarStore)
const router = useRouter()

const hasCalendar = computed(() => !!calendar.value)

watchEffect(() => {
    if (!hasCalendar.value && import.meta.client) {
        router.replace('/account/calendars/new')
    }
})

const availableDoor = computed(() => doorList.value.find(door => door.state === 'available') ?? null)
const openedDoors = computed(() => doorList.value.filter(door => door.state === 'opened').length)

function handleDoorSelect(day: number, disabled: boolean) {
    if (disabled) {
        return
    }
    calendarStore.setCurrentDoor(day)
    router.push(`/account/calendars/${calendar.value?.id}/doors/${day}/open`)
}

function goToEditor() {
    router.push(`/account/calendars/${calendar.value?.id}/presents`)
}

</script>

<template>
    <CalPageGrid>
        <CalHeader
            back-button
            :title="calendar?.name || 'Your calendar'"
            :description="calendar?.recipientName ? `For ${calendar.recipientName}` : ''"
        />
        <section class="space-y-2">
            <p class="text-sm text-gray-600">
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
                        :icon="door.present && door.present.content.tasks?.length === 0? '🎁' : '📝'"
                        :active="door.day === currentDoor"
                        :disabled="door.state === 'locked'"
                        @select="handleDoorSelect(door.day, door.state === 'locked')"
                    />
        </div>
        <CalButton type="button" alt-style @click="goToEditor">
            Edit calendar
        </CalButton>
    </CalPageGrid>
</template>