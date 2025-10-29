<template>
  <canvas
    ref="canvas"
    class="pointer-events-none fixed inset-0 z-[9999]"
    aria-hidden="true"
  />
</template>

<script setup lang="ts">
/**
 * SnowEffect.vue — Canvas-based particle snow for Nuxt 4
 * Props let you tweak density, speed, wind, size and color.
 * Automatically handles resize & devicePixelRatio.
 */
import { onMounted, onBeforeUnmount, ref, watchEffect } from 'vue'

const props = withDefaults(defineProps<{
  /** particles per 10,000 px² (e.g. 0.15 ≈ ~150 flakes on 1080p) */
  density?: number
  /** base fall speed in px/s */
  speed?: number
  /** horizontal wind, px/s (negative = left, positive = right) */
  wind?: number
  /** min/max flake radius in px */
  size?: { min: number; max: number }
  /** global alpha 0..1 */
  opacity?: number
  /** flake color (any canvas fillStyle) */
  color?: string
}>(), {
  density: 0.16,
  speed: 55,
  wind: 8,
  size: () => ({ min: 0.7, max: 3.0 }),
  opacity: 0.9,
  color: 'white'
})

type Flake = {
  x: number; y: number; r: number;
  vy: number; vx: number;
  swayPhase: number; swayAmp: number; swayFreq: number;
}

const canvas = ref<HTMLCanvasElement | null>(null)

let ctx: CanvasRenderingContext2D | null = null
let flakes: Flake[] = []
let raf = 0
let lastTime = 0

// Fixing type mismatch for mediaReduced
const mediaReduced = typeof window !== 'undefined'
  ? window.matchMedia('(prefers-reduced-motion: reduce)')
  : ({ matches: false, addEventListener() {}, removeEventListener() {} } as unknown as MediaQueryList);

function rand(min: number, max: number) {
  return Math.random() * (max - min) + min
}

function targetFlakeCount(w: number, h: number) {
  // particles per 10,000 px²
  return Math.round((w * h) / 10000 * props.density)
}

function resize() {
  if (!canvas.value) return
  const dpr = Math.max(1, window.devicePixelRatio || 1)
  const { innerWidth: w, innerHeight: h } = window
  canvas.value.width = Math.floor(w * dpr)
  canvas.value.height = Math.floor(h * dpr)
  canvas.value.style.width = w + 'px'
  canvas.value.style.height = h + 'px'
  ctx = canvas.value.getContext('2d')
  ctx?.scale(dpr, dpr)

  // Rebalance flakes to new target
  const want = targetFlakeCount(w, h)
  if (flakes.length > want) {
    flakes.length = want
  } else {
    for (let i = flakes.length; i < want; i++) {
      flakes.push(makeFlake(true))
    }
  }
}

function makeFlake(spawnAtRandomY = false): Flake {
  const w = window.innerWidth
  const h = window.innerHeight
  const r = rand(props.size.min, props.size.max)
  const speedFactor = r / props.size.max // small flakes fall slower
  const vy = rand(props.speed * 0.5, props.speed * 1.2) * (0.4 + speedFactor * 0.6) / 60 // px/frame
  const vx = (props.wind / 60) + rand(-0.3, 0.3)
  return {
    x: rand(0, w),
    y: spawnAtRandomY ? rand(0, h) : rand(-h * 0.2, -r * 4),
    r,
    vy,
    vx,
    swayPhase: rand(0, Math.PI * 2),
    swayAmp: rand(0.2, 1.2),
    swayFreq: rand(0.008, 0.02)
  }
}

function tick(t: number) {
  raf = requestAnimationFrame(tick)
  if (!ctx || !canvas.value) return
  const dt = lastTime ? Math.min(33, t - lastTime) : 16.7
  lastTime = t

  const w = window.innerWidth
  const h = window.innerHeight

  ctx.clearRect(0, 0, w, h)
  ctx.globalAlpha = props.opacity
  ctx.fillStyle = props.color

  // maintain target count (in case props changed)
  const want = targetFlakeCount(w, h)
  if (flakes.length < want && !mediaReduced.matches) {
    for (let i = flakes.length; i < want; i++) flakes.push(makeFlake(false))
  } else if (flakes.length > want) {
    flakes.length = want
  }

  const dtFrames = dt // already tuned to “per-frame” velocities
  for (let i = 0; i < flakes.length; i++) {
    const f = flakes[i]
    if (!f) continue; // Ensure `f` is defined

    // gentle sway
    f.swayPhase += f.swayFreq * dtFrames
    const sway = Math.sin(f.swayPhase) * f.swayAmp

    // integrate
    f.x += (f.vx + sway) * (dtFrames / 16.7)
    f.y += f.vy * (dtFrames / 16.7)

    // wrap
    if (f.y - f.r > h) {
      flakes[i] = makeFlake(false)
    } else if (f.x < -10) {
      f.x = w + 10
    } else if (f.x > w + 10) {
      f.x = -10
    }

    // draw
    ctx.beginPath()
    ctx.arc(f.x, f.y, f.r, 0, Math.PI * 2)
    ctx.fill()
  }
}

function start() {
  stop()
  lastTime = 0
  raf = requestAnimationFrame(tick)
}

function stop() {
  if (raf) cancelAnimationFrame(raf)
  raf = 0
}

function rebuild() {
  // rebuild flake set (when key props change)
  flakes = []
  const want = targetFlakeCount(window.innerWidth, window.innerHeight)
  for (let i = 0; i < want; i++) flakes.push(makeFlake(true))
}

onMounted(() => {
  if (!canvas.value) return
  resize()
  rebuild()
  if (!mediaReduced.matches) start()

  window.addEventListener('resize', resize)
  mediaReduced.addEventListener?.('change', (e) => {
    if (e.matches) stop()
    else start()
  })
})

onBeforeUnmount(() => {
  stop()
  window.removeEventListener('resize', resize)
})

watchEffect(() => {
  // any prop changes rebuild snow instantly
  if (typeof window === 'undefined') return
  rebuild()
})
</script>

<style scoped>
/* keep it above app content but below modals if you want; adjust z-index as needed */
</style>
