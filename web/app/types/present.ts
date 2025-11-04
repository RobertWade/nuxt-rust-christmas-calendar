export type PresentMediaType = 'image' | 'video' | 'audio' | 'link' | null;

export interface PresentMedia {
  type?: PresentMediaType
  url: string
  description?: string
  thumbnailUrl?: string | null
}

export interface PresentContent {
  title: string
  message: string
  media?: PresentMedia | null
  tasks?: string[]
  audio?: string
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
