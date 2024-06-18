import { ApexOptions } from "apexcharts";
import { SolidApexCharts } from "solid-apexcharts";
import { Accessor, createEffect, createSignal, on } from "solid-js";
import { Data } from "~/routes";

export default function Histogram(props: { data: Accessor<Data> }) {
  const data = () => props.data();
  const [series, setSeries] = createSignal<ApexAxisChartSeries>([]);
  const [options, setOptions] = createSignal<ApexOptions>({});

  createEffect(
    on(data, (data) => {
      setSeries([
        {
          type: "column",
          data: data.data.length
            ? data.data.map((i) => (i * 100).toFixed(2))
            : Array(11).fill(null),
          color: "#00F",
        },
        {
          type: "line",
          data:
            data.guassian.length && data.max > 10
              ? data.guassian.map((i) => (i * 100).toFixed(2))
              : Array(data.guassian.length || 11).fill(null),
          color: "#F00",
        },
      ]);
      setOptions({
        xaxis: {
          categories: data.data.length
            ? Array.from({ length: data.data.length }, (_, i) => i + data.min)
            : Array(11).fill(" "),
        },
        yaxis: { labels: { show: false } },
        dataLabels: { enabled: false },
        stroke: {
          width: 3,
          curve: "straight",
          dashArray: [0, 10],
        },
        legend: { show: false },
      });
    })
  );

  return (
    <main class="grow p-2">
      <SolidApexCharts
        type="bar"
        width="100%"
        height="100%"
        series={series()}
        options={options()}
      />
    </main>
  );
}
