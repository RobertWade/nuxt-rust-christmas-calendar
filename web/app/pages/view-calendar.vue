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
        <div class="grid grid-cols-4 gap-4">
            <CalDoor v-for="i in 24" :key="i" :title="i" :active="currentDoor === i" :disabled="i > currentDoor" />
        </div>
   </CalPageGrid>
</template>