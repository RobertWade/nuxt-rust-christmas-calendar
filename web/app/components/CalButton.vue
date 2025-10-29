<script lang="ts" setup>
const props = defineProps({
  altStyle: {
    type: Boolean,
    default: false,
  },
  link: {
    type: String,
    default: '',
  },
  to: {
    type: String,
    default: '',
  },
  disabled: {
    type: Boolean,
    default: false,
  }
})

defineEmits(['click'])

const computedClass = computed(() => {
  let baseClasses = 'px-6 py-4 font-semibold rounded-xl transition-colors border border-primary inline-block text-center cursor-pointer';
  if (props.altStyle) {
    baseClasses += ' bg-transparent text-primary';
  } else {
    baseClasses += ' bg-primary text-white';
  }
  if (props.disabled) {
    baseClasses += ' opacity-50 cursor-not-allowed';
  }
  return baseClasses;
});
</script>

<template>
  <button v-if="!link && !to" :class="computedClass" @click="$emit('click')">
    <slot />
  </button>
  <a v-else-if="link" :href="link" :class="computedClass">
    <slot />
  </a>
  <NuxtLink v-else-if="to" :to="to" :class="computedClass">
    <slot />
  </NuxtLink>
</template>