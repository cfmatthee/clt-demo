<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import Controls from "./lib/Controls.svelte";
  import Histogram from "./lib/Histogram.svelte";
  import type { Data } from "./types";

  let data = $state<Data>({
    data: [],
    guassian: [],
    min: 0,
    max: 0,
    mean: 0,
    stdev: 0,
  });

  const handleClick = (cmd: string) => {
    invoke<Data>("run_command", { command: cmd || "" }).then((new_data) => {
      data = new_data;
    });
  };
</script>

<main>
  <Histogram {data} />
  <Controls onclick={handleClick} />
</main>

<style>
  main {
    display: flex;
    flex-direction: column;
    gap: 5px;
    height: 100vh;
    max-height: 100vh;
  }
</style>
