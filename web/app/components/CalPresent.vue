<script setup lang="ts">
import CalHeader from './CalHeader.vue';


</script>

<template>
    <div class="bg-white border border-gray-300 p-4 rounded-xl grid gap-4">
      <CalHeader icon="🎁" title="Present Title" description="Opened on 5.Oktober 2025" small />
      <p>
        Here is a cute little text for your present. I hope you like it! Enjoy your day and have a wonderful time unwrapping your gifts!  💌
      </p>
      <div class="w-full h-32 border border-gray-300 rounded-xl" />
      <p class="text-sm text-gray-500">Only today's and past doors are visible. Come back tomorrow for the next surprise!</p>
    </div>
</template>