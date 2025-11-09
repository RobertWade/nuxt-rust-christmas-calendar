<script lang="ts" setup>
import { computed, onMounted, ref, watch, watchEffect } from 'vue'
import { storeToRefs } from 'pinia'
import type { NavigationGuard } from 'vue-router'
import { useCalendarStore } from '@/stores/useCalendarStore'
import type { CalendarDoor } from '@/types'

const requireAuth = 'auth' as unknown as NavigationGuard

definePageMeta({
   middleware: requireAuth,
})

const calendarStore = useCalendarStore()
const { activeDoor, doorList } = storeToRefs(calendarStore)

const route = useRoute()
const router = useRouter()

const loadingCalendar = ref(true)
const loadError = ref<string | null>(null)

const dayParam = computed(() => Number(route.query.day ?? calendarStore.currentDoor))

watchEffect(() => {
   if (Number.isFinite(dayParam.value) && dayParam.value > 0) {
      calendarStore.setCurrentDoor(dayParam.value)
   }
})

watch(
   activeDoor,
   (door) => {
      if (!door) {
         return
      }

      if (door.state !== 'opened') {
         const updated: CalendarDoor = { ...door, state: 'opened' }
         calendarStore.upsertDoor(updated)

         const nextDoor = doorList.value.find((item) => item.day === door.day + 1)
         if (nextDoor && nextDoor.state === 'locked') {
            calendarStore.upsertDoor({ ...nextDoor, state: 'available' })
         }
      }
   },
   { immediate: true },
)

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

function goBackToCalendar() {
   router.push({
      path: '/view-calendar',
      query: {
         calendarId: calendarStore.calendar?.id,
      },
   })
}

onMounted(() => {
   ensureCalendar()
})
</script>

<template>
   <CalPageGrid>
      <CalHeader icon="🎁" :title="`Door ${activeDoor?.day ?? dayParam}`" back-button />
      <p v-if="loadingCalendar" class="text-sm text-gray-500">Loading calendar…</p>
      <p v-if="loadError" class="text-sm text-red-500">{{ loadError }}</p>
      <CalPresent v-else :door="activeDoor" />
      <CalButton type="button" alt-style @click="goBackToCalendar">
         Back to calendar
      </CalButton>
   </CalPageGrid>
</template>