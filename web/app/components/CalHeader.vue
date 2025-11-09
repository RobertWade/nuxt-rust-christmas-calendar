<script lang="ts" setup>
import { useRouter } from 'vue-router'
const router = useRouter()

const props = defineProps({
    title: {
        type: String,
        default: ''
    },
    icon: {
        type: String,
        default: ''
    },
    small: {
        type: Boolean,
        default: false
    },
    description: {
        type: String,
        default: ''
    },
    backButton: {
        type: Boolean,
        default: false
    },
    backLink: {
        type: String,
        default: ''
    }
})

function goBack() {
    if (props.backLink) {
        router.push(props.backLink)
    } else {
        window.history.back()
    }
}
</script>

<template>
    <div v-if="!small" class="flex gap-2 items-center">
        <div 
        v-if="backButton"
            class="py-1 px-4 text-sm flex justify-center items-center border border-gray-300 rounded-full box-border cursor-pointer" @click="goBack">
            <!-- Ios back icon -->
            Back
        </div>
        <div v-else class="h-12 w-12 flex justify-center items-center bg-primary rounded-xl text-2xl box-border">
            <!-- Ios present icon -->
            🎁
        </div>
        <div>
            <h1 class="text-2xl font-bold">{{title}}</h1>
            <p class="text-sm text-gray-500">{{description}}</p>
        </div>
    </div>
    <div v-else class="flex gap-2 items-center">
        <div class="h-12 w-12 flex justify-center items-center bg-primary rounded-xl text-lg box-border">
            <!-- Ios present icon -->
            🎁
        </div>
        <div class="grid">
            <h1 class="text-lg font-bold">{{title}}</h1>
            <p class="text-sm text-gray-500">{{description}}</p>
        </div>
    </div>
</template>