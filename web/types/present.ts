export type PresentMediaType = 'image' | 'video' | 'audio' | 'link'

export interface PresentMedia {
  type: PresentMediaType
  url: string
  description?: string
  thumbnailUrl?: string | null
}

export interface PresentContent {
  title: string
  message: string
  media?: PresentMedia | null
  tasks?: string[]
}

export interface Present {
  id: string
  calendarId: string
  doorNumber: number
  releaseDate: string
  content: PresentContent
  createdAt: string
  updatedAt: string
}
