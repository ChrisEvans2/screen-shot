<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { listen } from '@tauri-apps/api/event'
import ScreenshotOverlay from './components/ScreenshotOverlay.vue'

const overlayRef = ref<InstanceType<typeof ScreenshotOverlay> | null>(null)

onMounted(async () => {
  await listen<string>('show-screenshot', (event) => {
    overlayRef.value?.show(event.payload)
  })
})
</script>

<template>
  <ScreenshotOverlay ref="overlayRef" />
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  overflow: hidden;
}
</style>
