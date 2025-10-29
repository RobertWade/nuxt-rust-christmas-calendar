<script lang="ts" setup>
const currentDoor = 3;
</script>

<template>
   <CalPageGrid>
        <CalHeader icon="🎁" title="Your calendar" />
        <section>
            <!-- Content for creating presents goes here -->
            <p>This is the view calendar page.</p>
        </section>
        <NuxtLink to="/door-open" class="grid grid-cols-4 gap-2">
            <div 
            v-for="i in 24" 
            :key="i"
            class="relative p-4 border text-xl border-gray-300 rounded-xl flex justify-center items-center aspect-square"
            :class="currentDoor === i ? 'border-primary' : null"
            >
            <span v-if="i>currentDoor" class="absolute left-1 top-1">🔒</span>
            {{ i }}</div>
        </NuxtLink>
   </CalPageGrid>
</template>