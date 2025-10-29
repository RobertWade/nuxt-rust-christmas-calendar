<script lang="ts" setup>
</script>

<template>
   <CalPageGrid>
        <CalHeader icon="🎁" title="Edit Present 1" back-button/>
        <section>
            <!-- Content for creating presents goes here -->
            <p>This is the edit presents page.</p>
        </section>
        <CalTextArea :rows="2" placeholder="Title" />

        <CalTextArea placeholder="Description" />
        <CalButton to="/view-calendar">Save</CalButton>
        <CalButton to="/new-calendar" alt-style>Cancel</CalButton>

   </CalPageGrid>
</template>