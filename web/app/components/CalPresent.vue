<script setup lang="ts">
import { computed } from 'vue'
import type { CalendarDoor, Present } from '../../types'
import CalHeader from './CalHeader.vue'

const props = defineProps<{ door: CalendarDoor | null }>()

const present = computed<Present | null>(() => props.door?.present ?? null)

const presentTitle = computed(() => present.value?.content.title || `Door ${props.door?.day ?? ''}`.trim())

const presentMessage = computed(() => present.value?.content.message || '')

const media = computed(() => present.value?.content.media ?? null)

const tasks = computed(() => present.value?.content.tasks ?? [])

const mediaBadge = computed(() => {
  if (!media.value) {
    return null
  }
  switch (media.value.type) {
    case 'image':
      return { icon: '🖼️', label: 'Bild' }
    case 'video':
      return { icon: '🎬', label: 'Video' }
    case 'audio':
      return { icon: '🎧', label: 'Audio' }
    case 'link':
    default:
      return { icon: '🔗', label: 'Link' }
  }
})

const openDescription = computed(() => {
  if (!props.door?.opensAt) {
    return ''
  }
  const date = new Date(props.door.opensAt)
  const formatter = Intl.DateTimeFormat(undefined, { dateStyle: 'long' })
  const label = props.door.state === 'opened' ? 'Opened on' : 'Opens on'
  return `${label} ${formatter.format(date)}`
})
</script>

<template>
  <div class="bg-white border border-gray-300 p-4 rounded-xl grid gap-4">
    <CalHeader icon="🎁" :title="presentTitle" :description="openDescription" small />
    <p v-if="presentMessage" class="leading-relaxed">
      {{ presentMessage }}
    </p>
    <p v-else class="italic text-gray-500">This door is still waiting for its story.</p>

    <div v-if="media" class="w-full border border-gray-300 rounded-xl overflow-hidden">
      <div v-if="mediaBadge" class="flex items-center gap-2 bg-primary/5 px-4 py-2 text-sm text-primary">
        <span>{{ mediaBadge.icon }}</span>
        <span>{{ mediaBadge.label }}</span>
      </div>
      <img
        v-if="media.type === 'image'"
        :src="media.url"
        :alt="media.description || presentTitle"
        class="w-full h-48 object-cover"
      >
      <video v-else-if="media.type === 'video'" controls class="w-full h-48 object-cover">
        <source :src="media.url" type="video/mp4">
      </video>
      <audio v-else-if="media.type === 'audio'" controls class="w-full">
        <source :src="media.url">
      </audio>
      <a
        v-else
        :href="media.url"
        target="_blank"
        rel="noopener"
        class="block px-4 py-3 text-primary underline"
      >
        {{ media.description || 'Open link' }}
      </a>
    </div>

    <div v-if="tasks.length" class="space-y-2">
      <p class="text-sm font-semibold text-gray-600">Today's little missions</p>
      <ul class="list-disc list-inside space-y-1 text-sm">
        <li v-for="task in tasks" :key="task">{{ task }}</li>
      </ul>
    </div>

    <p class="text-sm text-gray-500">Only today's and past doors are visible. Come back tomorrow for the next surprise!</p>
  </div>
</template>