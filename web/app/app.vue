<script setup lang="ts">
</script>

<template>
  <div>
    <SnowEffect 
      :density="0.26" 
      :speed="55"
      :wind="10" 
      :size="{ min: 0.7, max: 3 }" 
      :opacity=".9"
      color="rgba(255,255,255,0.95)" />
    <NuxtPage />
  </div>
</template>