<script setup lang="ts">
import type { Histogram } from "@/types";
import { computed, nextTick, ref, watch } from "vue";

const props = defineProps<{
  data: Histogram | undefined;
}>();

const WIDTH = 600;
const HEIGHT = 300;
const PAD = { top: 20, right: 20, bottom: 30, left: 40 };

const ghostHeights = ref<number[] | null>(null);
const ghostActive = ref(false);
const instant = ref(false);
let prevHeights: number[] = [];

function buildSplitGhost(old: number[], nNew: number): number[] {
  if (old.length === 0) return new Array(nNew).fill(0);
  if (nNew <= old.length) return [...old];
  const m = Math.floor((old.length - 1) / 2);
  const gap = nNew - old.length;
  return [
    ...old.slice(0, m + 1),
    ...new Array(gap).fill(old[m]),
    ...old.slice(m + 1),
  ];
}

watch(
  () => props.data,
  (d, prev) => {
    const prevN = prev?.data.length ?? 0;
    const newN = d?.data.length ?? 0;
    if (newN > prevN && newN > 0) {
      const maxOld = prev ? Math.max(...prev.data, ...prev.guassian) : 0;
      const maxNew = d ? Math.max(...d.data, ...d.guassian) : 0;
      const ratio = maxOld > 0 ? maxNew / maxOld : 1;
      ghostHeights.value = buildSplitGhost(prevHeights, newN).map(
        (v) => v * ratio
      );
      ghostActive.value = true;
      instant.value = true;
    } else {
      ghostActive.value = false;
    }
    prevHeights = d ? [...d.data] : [];
    nextTick(() => {
      requestAnimationFrame(() => {
        requestAnimationFrame(() => {
          instant.value = false;
          ghostActive.value = false;
        });
      });
    });
  },
  { immediate: true }
);

const barWidth = computed(() => {
  const n = props.data?.data.length ?? 1;
  return (WIDTH - PAD.left - PAD.right) / n;
});

const barHeights = computed(() => {
  if (!props.data) return [];
  if (ghostActive.value && ghostHeights.value) return ghostHeights.value;
  return props.data.data;
});

const bars = computed(() => {
  const d = props.data;
  if (!d) return [];
  const data = barHeights.value;
  const maxVal = Math.max(...d.data, ...d.guassian);
  const scaleY = (HEIGHT - PAD.top - PAD.bottom) / maxVal;
  return data.map((v, i) => ({
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
      <g :class="{ instant }">
        <rect
          v-for="(bar, i) in bars"
          :key="i"
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
  transition: height 0.6s ease, y 0.6s ease;
}

.instant rect {
  transition: none !important;
}
</style>
