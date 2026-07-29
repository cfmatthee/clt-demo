<script setup lang="ts">
import type { Histogram } from "@/types";
import { computed, watchEffect } from "vue";

const props = defineProps<{
  data: Histogram | undefined;
}>();

watchEffect(() => console.log("Chart:", props.data));

const WIDTH = 600;
const HEIGHT = 300;
const PAD = { top: 20, right: 20, bottom: 30, left: 40 };

const barWidth = computed(() => {
  const n = props.data?.data.length ?? 1;
  return (WIDTH - PAD.left - PAD.right) / n;
});

const bars = computed(() => {
  const d = props.data;
  if (!d) return [];
  const maxVal = Math.max(...d.data, ...d.guassian);
  const scaleY = (HEIGHT - PAD.top - PAD.bottom) / maxVal;
  return d.data.map((v, i) => ({
    x: PAD.left + i * barWidth.value,
    y: HEIGHT - PAD.bottom - v * scaleY,
    h: v * scaleY,
  }));
});

const curvePath = computed(() => {
  const d = props.data;
  if (!d?.guassian?.length) return "";
  if (d?.fit > 0.01) return "";
  const maxVal = Math.max(...d.data, ...d.guassian);
  const scaleY = (HEIGHT - PAD.top - PAD.bottom) / maxVal;
  const step = (WIDTH - PAD.left - PAD.right) / d.guassian.length;
  return d.guassian
    .map((v, i) => {
      const x = PAD.left + (i + 0.5) * step;
      const y = HEIGHT - PAD.bottom - v * scaleY;
      return `${i === 0 ? "M" : "L"}${x},${y}`;
    })
    .join(" ");
});
</script>

<template>
  <section>
    <svg
      width="100%"
      height="100%"
      :viewBox="`0 0 ${WIDTH} ${HEIGHT}`"
      preserve-aspect-ratio="none"
      xmlns="http://www.w3.org/2000/svg"
    >
      <!-- Bars -->
      <rect
        v-for="(bar, i) in bars"
        :key="i"
        :x="bar.x"
        :y="bar.y"
        :width="barWidth - 1"
        :height="bar.h"
        fill="steelblue"
      />
      <!-- Gaussian curve -->
      <path
        :d="curvePath"
        fill="none"
        stroke="red"
        stroke-width="2"
        stroke-dasharray="5 5"
      />
    </svg>
  </section>
</template>

<style lang="css" scoped>
section {
  flex-grow: 1;
  overflow: hidden;
}
</style>
