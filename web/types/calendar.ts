import type { Present } from './present'

export type CalendarStatus = 'draft' | 'ready' | 'published'
export type CalendarDoorState = 'locked' | 'available' | 'opened'

export interface CalendarDoor {
  day: number
  opensAt: string
  state: CalendarDoorState
  present?: Present | null
}

export interface CalendarMeta {
  id: string
  ownerId: string
  name: string
  description?: string
  recipientName?: string
  status: CalendarStatus
  createdAt: string
  updatedAt: string
  publishedAt?: string | null
}

export interface Calendar extends CalendarMeta {
  doors: CalendarDoor[]
}

export interface CalendarSummary extends CalendarMeta {
  doorCount: number
  openDoorCount: number
}
