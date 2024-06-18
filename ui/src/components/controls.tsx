import { For } from "solid-js";

const commands = {
  clear: "Clear chart",
  rectangular: "Rectangular",
  ushaped: "U-shaped",
};

export default function Controls(props: { onClick?: (e: MouseEvent) => void }) {
  return (
    <div class="flex flex-row gap-2 justify-center p-1">
      <For each={Object.entries(commands)}>
        {([command, description]) => (
          <button
            id={command}
            classList={{
              btn: true,
              "btn-primary": command !== "clear",
              "text-white": command !== "clear",
              "btn-outline": command === "clear",
            }}
            onClick={(e) => props.onClick?.(e)}
          >
            {description}
          </button>
        )}
      </For>
    </div>
  );
}
