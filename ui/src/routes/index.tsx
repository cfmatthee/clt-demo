import { invoke } from "@tauri-apps/api";
import { createSignal } from "solid-js";

import Histogram from "~/components/histogram";
import Controls from "~/components/controls";

export type Data = {
  data: number[];
  guassian: number[];
  min: number;
  max: number;
  mean: number;
  stdev: number;
};

export default function Home() {
  const [data, setData] = createSignal<Data>({
    data: [],
    guassian: [],
    min: 0,
    max: 0,
    mean: 0,
    stdev: 0,
  });

  function handleClick(e: MouseEvent) {
    const btn = e.target as HTMLButtonElement;
    console.time("requesting data");
    invoke<Data>("run_command", { command: btn.id || "" }).then((data) => {
      console.timeEnd("requesting data");
      setData(data);
    });
  }

  return (
    <>
      <Histogram data={data} />
      <Controls onClick={handleClick} />
    </>
  );
}
