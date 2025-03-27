<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import Controls from "./lib/Controls.svelte";

  type Data = {
    data: number[];
    guassian: number[];
    min: number;
    max: number;
    mean: number;
    stdev: number;
  };

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

  $effect(() => {
    $inspect(data);
  });
</script>

<main>
  <div class="histogram"></div>
  <Controls onclick={handleClick} />
</main>

<style>
  main {
    display: flex;
    flex-direction: column;
    gap: 5px;
    height: 100vh;
  }

  .histogram {
    background-color: #ccc;
    flex-grow: 1;
  }
</style>
