<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";

async function handleCommmand(cmd: String) {
  const result = await invoke("command", { cmd: cmd });
  console.log(cmd, result);
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
