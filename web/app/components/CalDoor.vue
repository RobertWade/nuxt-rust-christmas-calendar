<script lang="ts" setup>
defineProps({
    active:{
        type: Boolean,
        default: false
    },
    title:{
        type: Number,
        required: true
    },
    icon:{
        type: String,
        default: ''
    },
    disabled:{
        type: Boolean,
        default: false
    },
})
</script>

<template>
    <NuxtLink to="/door-open"
        class="relative p-4 border text-xl border-gray-300 rounded-xl flex justify-center items-center aspect-square"
        :class="active ? 'border-primary' : null">
        <span v-if="disabled" class="absolute left-1 top-1">🔒</span>
        {{ title }}
    </NuxtLink>
</template>