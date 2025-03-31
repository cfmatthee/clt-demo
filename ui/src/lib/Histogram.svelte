<script lang="ts">
  import type { Data } from "../types";
  import {
    BarController,
    BarElement,
    Chart,
    CategoryScale,
    LineController,
    LineElement,
    LinearScale,
    PointElement,
  } from "chart.js";

  interface Props {
    data: Data;
  }
  const { data }: Props = $props();

  let chartObject: Chart | null = null;
  Chart.register(
    BarController,
    BarElement,
    CategoryScale,
    LineController,
    LineElement,
    LinearScale,
    PointElement,
  );

  function chart(node: HTMLCanvasElement, data: Data) {
    function setupChart(_data: Data) {
      const labels = _data.data.length
        ? Array.from({ length: _data.data.length }, (_, i) => i + _data.min)
        : Array(11).fill(" ");

      Chart.defaults.backgroundColor = "var(--background-color)";
      Chart.defaults.color = "var(--text-color)";

      chartObject = new Chart(node, {
        data: {
          datasets: [
            {
              type: "line",
              label: "Gaussian",
              data:
                _data.guassian.length && _data.max > 10
                  ? _data.guassian.map((i) => (i * 100).toFixed(2))
                  : Array(_data.guassian.length || 11).fill(null),
              borderColor: "red",
              borderDash: [5, 10],
              pointStyle: false,
            },
            {
              type: "bar",
              label: "Data",
              data: _data.data.length
                ? _data.data.map((i) => (i * 100).toFixed(2))
                : Array(11).fill(null),
              backgroundColor: "blue",
              borderColor: "black",
              borderWidth: 1,
            },
          ],
          labels: labels,
        },
        options: {
          animation: false,
          maintainAspectRatio: false,
          plugins: {
            legend: { display: false },
          },
          scales: {
            y: { display: false },
          },
        },
      });
    }
    setupChart(data);

    return {
      update(data: Data) {
        chartObject?.destroy();
        setupChart(data);
      },
      destroy() {
        chartObject?.destroy();
      },
    };
  }
</script>

<section>
  <canvas use:chart={$state.snapshot(data)}></canvas>
</section>

<style>
  section {
    background-color: var(--background-color);
    display: flex;
    flex-direction: column;
    flex-grow: 1;
    justify-content: stretch;
    max-height: calc(100vh - 50px);
    padding: 15px;
  }
</style>
