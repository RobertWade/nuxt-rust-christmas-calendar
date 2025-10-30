<script lang="ts" setup>
import { computed } from 'vue'
import type { PropType } from 'vue'

const props = defineProps({
    active: {
        type: Boolean,
        default: false
    },
    title: {
        type: [Number, String] as PropType<number | string>,
        required: true
    },
    icon: {
        type: String,
        default: ''
    },
    disabled: {
        type: Boolean,
        default: false
    },
    to: {
        type: String,
        default: ''
    }
})

const emit = defineEmits(['select'])

const baseClasses = 'relative p-4 border text-xl border-gray-300 rounded-xl flex justify-center items-center aspect-square transition-colors duration-150'

const computedClass = computed(() => {
    let classes = baseClasses
    if (props.active) {
        classes += ' border-primary ring-2 ring-primary/30'
    }
    if (props.disabled) {
        classes += ' opacity-50 cursor-not-allowed pointer-events-none'
    } else {
        classes += ' hover:border-primary cursor-pointer'
    }
    return classes
})

function handleSelect() {
    if (props.disabled) {
        return
    }
    emit('select')
}
</script>

<template>
    <NuxtLink
        v-if="to && !disabled"
        :to="to"
        :class="computedClass"
        @click="handleSelect"
    >
        <span v-if="disabled" class="absolute left-1 top-1">🔒</span>
        <span v-else-if="icon" class="absolute left-1 top-1">{{ icon }}</span>
        {{ title }}
    </NuxtLink>
    <button
        v-else
        type="button"
        :class="computedClass"
        :disabled="disabled"
        @click="handleSelect"
    >
        <span v-if="disabled" class="absolute left-1 top-1">🔒</span>
        <span v-else-if="icon" class="absolute left-1 top-1">{{ icon }}</span>
        {{ title }}
    </button>
</template>