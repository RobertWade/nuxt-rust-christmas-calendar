<script lang="ts" setup>
import { storeToRefs } from 'pinia'
import { useCalendarStore } from '@/stores/useCalendarStore'
import type { CalendarDoor, Present, PresentMedia, PresentMediaType } from '@/types'

const calendarStore = useCalendarStore()
const { activeDoor, calendar } = storeToRefs(calendarStore)

const router = useRouter()
const route = useRoute()

const dayParam = computed(() => {
    const doorParam = route.params.door
    const day = Number(doorParam)
    return Number.isFinite(day) && day > 0 ? day : calendarStore.currentDoor
})

watchEffect(() => {
    if (Number.isFinite(dayParam.value) && dayParam.value > 0) {
        calendarStore.setCurrentDoor(dayParam.value)
    }
})

const form = reactive({
    title: '',
    message: '',
    mediaUrl: '',
    mediaDescription: '',
    mediaType: null as PresentMediaType,
    tasksText: ''
})

const errorMessage = ref<string | null>(null)
const saving = ref(false)

const mediaTypeOptions: Array<{ value: PresentMediaType; label: string; icon: string; hint: string }> = [
    { value: 'link', label: 'Link', icon: '🔗', hint: 'Use any external resource' },
    { value: 'image', label: 'Bild', icon: '🖼️', hint: 'Display a picture' },
    { value: 'video', label: 'Video', icon: '🎬', hint: 'Embed a short clip' },
    { value: 'audio', label: 'Audio', icon: '🎧', hint: 'Share a voice note' }
]

const mediaUrlPlaceholder = computed(() => {
    switch (form.mediaType) {
        case null:
            return 'Media URL (optional)'
        case 'image':
            return 'Image URL (optional)'
        case 'video':
            return 'Video URL (optional)'
        case 'audio':
            return 'Audio URL (optional)'
        default:
            return 'Link URL (optional)'
    }
})

const mediaDescriptionPlaceholder = computed(() => {
    switch (form.mediaType) {
        case null:
            return 'Description (optional)'
        case 'image':
            return 'Short caption for the image (optional)'
        case 'video':
            return 'Add a note about the video (optional)'
        case 'audio':
            return 'Describe the audio message (optional)'
        default:
            return 'Description (optional)'
    }
})

watch(
    activeDoor,
    door => {
        if (!door) {
            form.title = ''
            form.message = ''
            form.mediaUrl = ''
            form.mediaDescription = ''
            form.mediaType = null
            form.tasksText = ''
            return
        }

        form.title = door.present?.content.title ?? ''
        form.message = door.present?.content.message ?? ''
        form.mediaUrl = door.present?.content.media?.url ?? ''
        form.mediaDescription = door.present?.content.media?.description ?? ''
        form.mediaType = door.present?.content.media?.type ?? null
        form.tasksText = (door.present?.content.tasks ?? []).join('\n')
    },
    { immediate: true }
)

function ensureDoor(day: number): CalendarDoor {
    const existing = calendar.value?.doors.find(item => item.day === day)
    if (existing) {
        return existing
    }

    const fallbackDate = new Date()
    fallbackDate.setMonth(11)
    fallbackDate.setDate(day)
    fallbackDate.setHours(6, 0, 0, 0)

    return {
        day,
        opensAt: fallbackDate.toISOString(),
        state: 'available',
        present: null
    }
}

function buildMedia(): PresentMedia | null {
    const url = form.mediaUrl.trim()
    if (!url) {
        return null
    }

    return {
        type: form.mediaType,
        url,
        description: form.mediaDescription || undefined,
        thumbnailUrl: null
    }
}

function buildPresent(baseDoor: CalendarDoor): Present {
    const now = new Date().toISOString()
    return {
        id: baseDoor.present?.id ?? `present-${baseDoor.day}`,
        calendarId: calendar.value?.id ?? 'temp',
        doorNumber: baseDoor.day,
        releaseDate: baseDoor.opensAt,
        content: {
            title: form.title || `Door ${baseDoor.day}`,
            message: form.message,
            media: buildMedia(),
            tasks: form.tasksText
                .split('\n')
                .map(task => task.trim())
                .filter(Boolean)
        },
        createdAt: baseDoor.present?.createdAt ?? now,
        updatedAt: now
    }
}

async function handleSave() {
    if (!calendar.value) {
        router.push('/account/calendars/new')
        return
    }

    if (!form.message.trim() && !form.title.trim()) {
        errorMessage.value = 'Give this door at least a title or a message.'
        return
    }

    errorMessage.value = null
    saving.value = true

    try {
        const door = ensureDoor(dayParam.value || 1)
        const updatedDoor: CalendarDoor = {
            ...door,
            state: door.state === 'locked' ? 'available' : door.state,
            present: buildPresent(door)
        }

        calendarStore.upsertDoor(updatedDoor)
        calendarStore.setCurrentDoor(updatedDoor.day)
        await router.push(`/account/calendars/${calendar.value.id}/presents`)
    } finally {
        saving.value = false
    }
}

function handleCancel() {
    router.back()
}

function selectMediaType(type: PresentMediaType) {
    form.mediaType = form.mediaType === type ? null : type
}
</script>

<template>
    <CalPageGrid>
        <CalHeader
            icon="🎁"
            :title="`Edit Door ${dayParam || ''}`"
            back-button
        />
        <section class="flex flex-col gap-4">
            <p class="text-sm text-gray-600">
                Make this day memorable with a personal message, a shared activity, or a special hint.
            </p>
            <form class="grid gap-4" @submit.prevent="handleSave">
                
                <CalTextBox v-model="form.title" placeholder="Title" />
                <CalTextArea v-model="form.message" placeholder="Description" />
                <div class="grid gap-2">
                    <p class="text-sm font-semibold text-gray-600">Media type</p>
                    <div class="grid grid-cols-2 gap-3 sm:grid-cols-4">
                        <button
                            v-for="option in mediaTypeOptions"
                            :key="option.value || ''"
                            type="button"
                            class="grid gap-1 rounded-xl border px-4 py-3 text-center transition-colors"
                            :class="form.mediaType === option.value ? 'border-primary bg-primary/10 text-primary' : 'border-gray-300 text-gray-700'"
                            @click="selectMediaType(option.value)"
                        >
                            <span class="text-2xl">{{ option.icon }}</span>
                            <span class="text-sm font-semibold">{{ option.label }}</span>
                            <span class="text-xs text-gray-500">{{ option.hint }}</span>
                        </button>
                    </div>
                </div>
                <CalTextBox
                    v-if="form.mediaType"
                    v-model="form.mediaUrl"
                    :placeholder="mediaUrlPlaceholder"
                />
                <CalTextBox
                    v-if="form.mediaType"
                    v-model="form.mediaDescription"
                    :placeholder="mediaDescriptionPlaceholder"
                />
                <CalTextArea
                    v-model="form.tasksText"
                    placeholder="Optional missions (one per line)"
                    :rows="3"
                />
                <p v-if="errorMessage" class="text-sm text-red-500">{{ errorMessage }}</p>
                <div class="flex gap-3">
                    <CalButton type="submit" :disabled="saving">
                        {{ saving ? 'Saving…' : 'Save' }}
                    </CalButton>
                    <CalButton type="button" alt-style @click="handleCancel">Cancel</CalButton>
                </div>
            </form>
        </section>
    </CalPageGrid>
</template>