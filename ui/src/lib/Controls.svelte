<script lang="ts">
  interface Props {
    onclick?: (cmd: string) => void;
  }
  let { onclick }: Props = $props();

  const commands = {
    clear: "Clear chart",
    rectangular: "Rectangular",
    ushaped: "U-shaped",
  };

  const handleClick = (cmd: string) => {
    if (onclick) onclick(cmd);
  };
</script>

<div class="container">
  {#each Object.entries(commands) as command}
    <button
      id={command[0]}
      class={[command[0] != "clear" && "primary"]}
      onclick={() => handleClick(command[0])}
    >
      {command[1]}
    </button>
  {/each}
</div>

<style>
  .container {
    display: flex;
    flex-direction: row;
    gap: 15px;
    justify-content: center;
    padding: 5px;
  }

  button {
    border: 1px solid black;
    border-radius: 10px;
    padding: 7px 20px;
  }

  .primary {
    background-color: var(--primary-color);
    color: white;
  }
</style>
