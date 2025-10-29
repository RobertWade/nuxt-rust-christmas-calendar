<script lang="ts" setup>
defineProps({
  placeholder: {
    type: String,
    default: ''
  },
  type: {
    type: String,
    default: 'text'
  },
  autocomplete: {
    type: String,
    default: ''
  },
  id: {
    type: String,
    default: ''
  },
})

const input = defineModel<string>({
  type: String,
  default: ''
})
</script>

<template>
  <input
    :id="id"
    v-model="input"
    :autocomplete="autocomplete"
    :type="type"
    class="px-6 py-4 border border-gray-300 rounded-xl focus:outline-none focus:ring-2 focus:ring-primary"
    :placeholder="placeholder"
  />
</template>