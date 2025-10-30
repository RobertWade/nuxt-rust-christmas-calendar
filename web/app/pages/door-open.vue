<script lang="ts" setup>
import { storeToRefs } from 'pinia'
import { useCalendarStore } from '../../stores/useCalendarStore'
import type { CalendarDoor } from '../../types'

const calendarStore = useCalendarStore()
const { activeDoor, doorList } = storeToRefs(calendarStore)

const route = useRoute()
const router = useRouter()

const dayParam = computed(() => Number(route.query.day ?? calendarStore.currentDoor))

watchEffect(() => {
   if (Number.isFinite(dayParam.value) && dayParam.value > 0) {
      calendarStore.setCurrentDoor(dayParam.value)
   }
})

watch(
   activeDoor,
   door => {
      if (!door) {
         return
      }

         if (door.state !== 'opened') {
            const updated: CalendarDoor = { ...door, state: 'opened' }
            calendarStore.upsertDoor(updated)

            const nextDoor = doorList.value.find(item => item.day === door.day + 1)
            if (nextDoor && nextDoor.state === 'locked') {
               calendarStore.upsertDoor({ ...nextDoor, state: 'available' })
            }
         }
   },
   { immediate: true }
)

function goBackToCalendar() {
   router.push('/view-calendar')
}
</script>

<template>
   <CalPageGrid>
      <CalHeader icon="🎁" :title="`Door ${activeDoor?.day ?? dayParam}`" back-button />
      <CalPresent :door="activeDoor" />
      <CalButton type="button" alt-style @click="goBackToCalendar">
         Back to calendar
      </CalButton>
   </CalPageGrid>
</template>