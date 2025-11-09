import { computed, ref } from 'vue'
import { defineStore } from 'pinia'
import type {
  Calendar,
  CalendarDoor,
  CalendarDoorState,
  CalendarStatus,
  Present,
} from '@/types'
import { useUserStore } from './useUserStore'

type ApiCalendarDoor = {
  day: number
  title?: string | null
  mediaUrl?: string | null
  opensAt?: string | null
  state?: string | null
  present?: unknown
}

type ApiCalendar = {
  id: string | number
  ownerId?: string | null
  name: string
  description?: string | null
  recipientName?: string | null
  status?: string | null
  createdAt?: string | null
  updatedAt?: string | null
  publishedAt?: string | null
  doors?: Array<ApiCalendarDoor | CalendarDoor>
}

function isCalendarDoor(entry: ApiCalendarDoor | CalendarDoor): entry is CalendarDoor {
  return typeof (entry as CalendarDoor).opensAt === 'string'
}

function normalizeDoorState(raw: string | CalendarDoorState | null | undefined): CalendarDoorState {
  if (raw === 'available' || raw === 'opened') {
    return raw
  }
  return 'locked'
}

function fallbackOpensAt(day: number): string {
  const date = new Date()
  date.setMonth(11, day)
  date.setHours(6, 0, 0, 0)
  return date.toISOString()
}

function normalizePresent(
  raw: unknown,
  calendarId: string,
  day: number,
  releaseDate: string,
  fallbackTitle?: string | null,
  mediaUrl?: string | null,
): Present | null {
  if (!raw || typeof raw !== 'object') {
    if (!fallbackTitle && !mediaUrl) {
      return null
    }

    return {
      id: `present-${day}`,
      calendarId,
      doorNumber: day,
      releaseDate,
      content: {
        title: fallbackTitle || `Door ${day}`,
        message: '',
        media: mediaUrl
          ? {
              type: 'link',
              url: mediaUrl,
            }
          : null,
        tasks: [],
      },
      createdAt: releaseDate,
      updatedAt: releaseDate,
    }
  }

  const candidate = raw as Partial<Present>
  const content = candidate.content ?? {
    title: fallbackTitle || `Door ${day}`,
    message: '',
    media: mediaUrl
      ? {
          type: 'link' as const,
          url: mediaUrl,
        }
      : null,
    tasks: [],
  }

  return {
    id: candidate.id ?? `present-${day}`,
    calendarId: candidate.calendarId ?? calendarId,
    doorNumber: candidate.doorNumber ?? day,
    releaseDate: candidate.releaseDate ?? releaseDate,
    content: {
      title: content.title ?? fallbackTitle ?? `Door ${day}`,
      message: content.message ?? '',
      media: content.media ?? null,
      tasks: content.tasks ?? [],
    },
    createdAt: candidate.createdAt ?? releaseDate,
    updatedAt: candidate.updatedAt ?? releaseDate,
  }
}

function normalizeDoor(input: ApiCalendarDoor | CalendarDoor, calendarId: string): CalendarDoor {
  if (isCalendarDoor(input)) {
    return {
      ...input,
      state: normalizeDoorState(input.state),
    }
  }

  const apiDoor = input as ApiCalendarDoor
  const day = Number(apiDoor.day)
  const opensAt = typeof apiDoor.opensAt === 'string' ? apiDoor.opensAt : fallbackOpensAt(day)
  const normalized: CalendarDoor = {
    day,
    opensAt,
    state: normalizeDoorState(apiDoor.state ?? null),
    present: normalizePresent(apiDoor.present, calendarId, day, opensAt, apiDoor.title, apiDoor.mediaUrl),
  }
  return normalized
}

function normalizeCalendar(input: ApiCalendar | Calendar): Calendar {
  const now = new Date().toISOString()
  const id = String(input.id ?? 'temp')
  const doors = Array.isArray(input.doors)
    ? input.doors
        .map((door) => normalizeDoor(door, id))
        .sort((a, b) => a.day - b.day)
    : []

  return {
    id,
    ownerId: input.ownerId ?? '',
    name: input.name ?? '',
    description: input.description ?? undefined,
    recipientName: input.recipientName ?? undefined,
    status: (input.status ?? 'draft') as CalendarStatus,
    createdAt: input.createdAt ?? now,
    updatedAt: input.updatedAt ?? now,
    publishedAt: input.publishedAt ?? null,
    doors,
  }
}

function buildDoorPayload(door: CalendarDoor) {
  const payload: Record<string, unknown> = {
    title: door.present?.content.title ?? `Door ${door.day}`,
    opensAt: door.opensAt,
    state: door.state,
  }

  if (door.present) {
    payload.present = {
      id: door.present.id,
      content: {
        title: door.present.content.title,
        message: door.present.content.message ?? '',
        media: door.present.content.media
          ? {
              type: door.present.content.media.type ?? undefined,
              url: door.present.content.media.url,
              description: door.present.content.media.description,
              thumbnailUrl: door.present.content.media.thumbnailUrl ?? undefined,
            }
          : null,
        tasks: door.present.content.tasks ?? [],
      },
    }
  } else {
    payload.present = null
  }

  return payload
}

function createEmptyCalendarPlaceholder(): Calendar {
  const now = new Date().toISOString()
  return {
    id: 'temp',
    ownerId: '',
    name: '',
    description: undefined,
    recipientName: undefined,
    status: 'draft',
    createdAt: now,
    updatedAt: now,
    publishedAt: null,
    doors: [],
  }
}

export const useCalendarStore = defineStore(
  "calendar",
  () => {
    const calendar = ref<Calendar | null>(null)
    const calendars = ref<Calendar[]>([])
    const currentDoor = ref<CalendarDoor["day"]>(1)

    const doorList = computed<CalendarDoor[]>(
      () => calendar.value?.doors ?? []
    );
    const activeDoor = computed<CalendarDoor | null>(
      () =>
        doorList.value.find((door) => door.day === currentDoor.value) ?? null
    );

    function setCalendar(payload: ApiCalendar | Calendar) {
      const normalized = normalizeCalendar(payload)
      calendar.value = normalized
      const nextDoor = normalized.doors.find((door) => door.state === "available")
      currentDoor.value = nextDoor?.day ?? 1
    }

    function clearCalendar() {
      calendar.value = null
      currentDoor.value = 1
    }

    function setCalendars(list: Array<ApiCalendar | Calendar>) {
      calendars.value = list.map((item) => normalizeCalendar(item)).sort(
        (a, b) => new Date(a.createdAt).getTime() - new Date(b.createdAt).getTime(),
      )
    }

    function setCurrentDoor(day: number) {
      currentDoor.value = day
    }

    function upsertDoor(door: CalendarDoor) {
      if (!calendar.value) {
        calendar.value = { ...createEmptyCalendarPlaceholder(), doors: [door] }
        currentDoor.value = door.day
        return
      }

      const index = calendar.value.doors.findIndex(
        (existing) => existing.day === door.day
      );
      if (index === -1) {
        calendar.value.doors.push(door)
        calendar.value.doors.sort((a, b) => a.day - b.day)
        return
      }

      calendar.value.doors[index] = door
    }

    async function saveDoor(door: CalendarDoor) {
      if (!calendar.value) {
        throw new Error('No calendar is loaded yet.')
      }

      const calendarId = calendar.value.id
      const userStore = useUserStore()

      const result = await $fetch<ApiCalendarDoor>(`/api/calendars/${calendarId}/doors/${door.day}`, {
        method: 'PUT',
        headers: userStore.getAuthHeaders(),
        body: buildDoorPayload(door),
      })

      const normalized = normalizeDoor(result, calendarId)
      upsertDoor(normalized)

      const listIndex = calendars.value.findIndex((item) => item.id === calendarId)
      if (listIndex !== -1) {
        const nextList = calendars.value.slice()
        const existingCalendar = nextList[listIndex]
        if (existingCalendar) {
          const updatedDoors = existingCalendar.doors.some((item) => item.day === normalized.day)
            ? existingCalendar.doors.map((item) => (item.day === normalized.day ? normalized : item))
            : [...existingCalendar.doors, normalized].sort((a, b) => a.day - b.day)

          nextList[listIndex] = {
            ...existingCalendar,
            doors: updatedDoors,
          }
          calendars.value = nextList
        }
      }

      return normalized
    }

    async function fetchCalendars() {
      const userStore = useUserStore()
      const result = await $fetch<ApiCalendar[]>('/api/calendars', {
        headers: userStore.getAuthHeaders(),
      })

      const normalized = result.map((item) => normalizeCalendar(item))
      setCalendars(normalized)
      return normalized
    }

    async function fetchCalendar(calendarId: string | number) {
      const userStore = useUserStore()
      const result = await $fetch<ApiCalendar>(`/api/calendars/${calendarId}`, {
        headers: userStore.getAuthHeaders(),
      })

      const normalized = normalizeCalendar(result)
      setCalendar(normalized)
      return normalized
    }

    async function createCalendar(payload: { name: string }) {
      const userStore = useUserStore()
      const ownerId = userStore.user?.id ?? undefined

      const result = await $fetch<ApiCalendar>('/api/calendars', {
        method: 'POST',
        headers: userStore.getAuthHeaders(),
        body: {
          name: payload.name,
          ownerId,
        },
      })

      const normalized = normalizeCalendar(result)
      setCalendar(normalized)
      setCalendars([
        ...calendars.value.filter((item) => item.id !== normalized.id),
        normalized,
      ])

      return normalized
    }

    async function ensureCalendarLoaded(calendarId?: string | number | null) {
      if (calendarId) {
        if (calendar.value && calendar.value.id === String(calendarId)) {
          return calendar.value
        }
        return await fetchCalendar(calendarId)
      }

      if (calendar.value) {
        return calendar.value
      }

      if (!calendars.value.length) {
        const list = await fetchCalendars()
        if (!list.length) {
          return null
        }
      }

      const fallback = calendars.value[0]
      if (!fallback) {
        return null
      }

      return await fetchCalendar(fallback.id)
    }

    return {
      calendar,
      calendars,
      doorList,
      activeDoor,
      currentDoor,
      setCalendar,
      setCalendars,
      setCurrentDoor,
      upsertDoor,
  saveDoor,
      clearCalendar,
      fetchCalendars,
      fetchCalendar,
      createCalendar,
      ensureCalendarLoaded,
    };
  },
  {
    persist: true,
  }
);