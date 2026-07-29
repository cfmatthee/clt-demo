<script setup lang="ts">
import { onMounted, ref } from "vue";
import Chart from "./components/Chart.vue";
import ControlButtonGroup from "./components/ControlButtonGroup.vue";
import type { Histogram } from "./types";
import { invoke } from "@tauri-apps/api/core";

const histogram = ref<Histogram>();

onMounted(async () => {
  const result = await invoke<Histogram>("command", { cmd: "clear" });
  histogram.value = result;
});

function handleUpdated(data: Histogram) {
  histogram.value = data;
}
</script>

<template>
  <Chart :data="histogram" />
  <ControlButtonGroup @updated="handleUpdated" />
</template>

<style>
html,
body {
  height: 100%;
  margin: 0;
  padding: 0;
  overflow: hidden;
  background-color: #ccc;
}

#app {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  height: 100%;
  overflow: hidden;
}
</style>
