<script setup lang="ts">
import type { Histogram } from "@/types";
import { computed, nextTick, ref, watch } from "vue";

const props = defineProps<{
  data: Histogram | undefined;
}>();

type DisplayBars = {
  id: number;
  x: number;
  y: number;
  h: number;
};

const displayBars = ref<DisplayBars[]>([]);

const WIDTH = 600;
const HEIGHT = 300;
const PAD = { top: 20, right: 20, bottom: 30, left: 40 };
const BASELINE = HEIGHT - PAD.bottom;

const barWidth = computed(() => {
  const n = props.data?.data.length ?? 1;
  return (WIDTH - PAD.left - PAD.right) / n;
});

const barHeights = computed(() => {
  if (!props.data) return [];
  return props.data.data;
});

const bars = computed(() => {
  const d = props.data;
  if (!d) return [];
  const data = barHeights.value;
  const maxVal = Math.max(...d.data, ...d.guassian);
  const scaleY = (HEIGHT - PAD.top - PAD.bottom) / maxVal;
  return data.map((v, i) => ({
    id: d.min + i,
    x: PAD.left + i * barWidth.value,
    y: BASELINE - v * scaleY,
    h: v * scaleY,
  }));
});

watch(bars, (newData, prevData) => {
  const oldIds = new Set(prevData.map((v) => v.id));
  displayBars.value = newData.map((v) => {
    const known = oldIds.has(v.id);
    return { ...v, y: known ? v.y : BASELINE, h: known ? v.h : 0 };
  });

  nextTick(() =>
    requestAnimationFrame(() => {
      requestAnimationFrame(() => {
        displayBars.value = newData;
      });
    }),
  );
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
      const y = BASELINE - v * scaleY;
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
      <g>
        <rect
          v-for="bar in displayBars"
          :key="bar.id"
          :style="{
            x: bar.x + 'px',
            y: bar.y + 'px',
            width: barWidth - 1 + 'px',
            height: bar.h + 'px',
          }"
          fill="steelblue"
        />
      </g>
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

rect {
  transition:
    height 0.6s ease,
    y 0.6s ease;
}
</style>
