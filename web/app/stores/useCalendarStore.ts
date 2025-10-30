import { defineStore } from 'pinia'
import type { Calendar, CalendarDoor } from '@/types'

export const useCalendarStore = defineStore(
  "calendar",
  () => {
    const calendar = ref<Calendar | null>(null);
    const currentDoor = ref<CalendarDoor["day"]>(1);

    const doorList = computed<CalendarDoor[]>(
      () => calendar.value?.doors ?? []
    );
    const activeDoor = computed<CalendarDoor | null>(
      () =>
        doorList.value.find((door) => door.day === currentDoor.value) ?? null
    );

    function setCalendar(payload: Calendar) {
      calendar.value = payload;
      const nextDoor = payload.doors.find((door) => door.state === "available");
      currentDoor.value = nextDoor?.day ?? 1;
    }

    function clearCalendar() {
      calendar.value = null;
      currentDoor.value = 1;
    }

    function setCurrentDoor(day: number) {
      currentDoor.value = day;
    }

    function upsertDoor(door: CalendarDoor) {
      if (!calendar.value) {
        calendar.value = { ...createEmptyCalendarPlaceholder(), doors: [door] };
        currentDoor.value = door.day;
        return;
      }

      const index = calendar.value.doors.findIndex(
        (existing) => existing.day === door.day
      );
      if (index === -1) {
        calendar.value.doors.push(door);
        calendar.value.doors.sort((a, b) => a.day - b.day);
        return;
      }

      calendar.value.doors[index] = door;
    }

    function createEmptyCalendarPlaceholder(): Calendar {
      return {
        id: "temp",
        ownerId: "temp",
        name: "",
        status: "draft",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
        doors: [],
      };
    }

    return {
      calendar,
      doorList,
      activeDoor,
      currentDoor,
      setCalendar,
      setCurrentDoor,
      upsertDoor,
      clearCalendar,
    };
  },
  {
    persist: true,
  }
);