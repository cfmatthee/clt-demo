<script setup lang="ts">
import type { Histogram } from "@/types";
import { invoke } from "@tauri-apps/api/core";

const emit = defineEmits<{
  updated: [histogram: Histogram];
}>();

async function handleCommmand(cmd: String) {
  const result = await invoke<Histogram>("command", { cmd: cmd });
  emit("updated", result);
}
</script>

<template>
  <div class="button-container">
    <button @click="() => handleCommmand('clear')" class="secondary">
      Clear
    </button>
    <button @click="() => handleCommmand('rectangular')">Rectangular</button>
    <button @click="() => handleCommmand('ushaped')">U-Shaped</button>
  </div>
</template>

<style lang="css" scoped>
.button-container {
  display: flex;
  flex-direction: row;
  justify-content: center;
  align-items: center;
  gap: 0 1rem;
}

button {
  padding: 0.25rem 1rem;
}
</style>
