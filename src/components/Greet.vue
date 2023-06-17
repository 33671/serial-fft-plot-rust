<script setup lang="ts">
import { onMounted, reactive, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
// import {Chart} from "chart.js";
import { Chart, ChartItem } from "chart.js/auto";
// import {Point} from "chart.js"
const name = ref("");
async function greet() {
  console.log(await invoke("get_fft_result"));
}
let chart: Chart;
let charty: Chart;
let chartxyz: Chart;
let chartz: Chart;
const labels = Array.from(Array(65).keys()).map((x) =>
  Math.round((x / 65.0) * 94.5)
);
const data = {
  labels: labels,
  datasets: [
    {
      label: "X ACC",
      data: Array.from(Array(65).keys()).map((x) => 1),
      fill: "start",
      borderColor: "rgb(75, 192, 192)",
      tension: 0.4,
    },
  ],
};
let datay = {
  labels: labels,
  datasets: [
    {
      label: "Y ACC",
      data: Array.from(Array(65).keys()).map((x) => 1),
      fill: "start",
      borderColor: "rgb(75, 192, 192)",
      tension: 0.4,
    },
  ],
};
let dataz = {
  labels: labels,
  datasets: [
    {
      label: "Z ACC",
      data: Array.from(Array(65).keys()).map((x) => 1),
      fill: "start",
      borderColor: "rgb(75, 192, 192)",
      tension: 0.4,
    },
  ],
};
let dataxyz = {
  labels: labels,
  datasets: [
    {
      label: "XYZ ACC",
      data: Array.from(Array(65).keys()).map((x) => 1),
      fill: "start",
      borderColor: "rgb(75, 192, 192)",
      tension: 0.4,
    },
  ],
};
// borderColor: 'rgb(75, 192, 192)',
let chart_config: any = {
  type: "line",

  options: {
    scales: {
      y: {
        min: 0,
        max: 4,
      },
    },
    responsive: true,
  },
};
interface FreqAndAmp {
  freq: string;
  amp: string;
}
let max = reactive<{
  x: FreqAndAmp;
  y: FreqAndAmp;
  z: FreqAndAmp;
  xyz: FreqAndAmp;
}>({
  x: { freq: "", amp: "" },
  y: { freq: "", amp: "" },
  z: { freq: "", amp: "" },
  xyz: { freq: "", amp: "" },
});
onMounted(() => {
  let ctx = document.querySelector(".my-chart") as HTMLCanvasElement;
  let ctxy = document.querySelector(".my-charty") as HTMLCanvasElement;
  let ctz = document.querySelector(".my-chartz") as HTMLCanvasElement;
  let ctxyz = document.querySelector(".my-chartxyz") as HTMLCanvasElement;
  chart = new Chart(ctx, { ...chart_config, data: data });
  charty = new Chart(ctxy, {
    ...chart_config,
    data: datay,
  });
  chartz = new Chart(ctz, {
    ...chart_config,
    data: dataz,
  });
  chartxyz = new Chart(ctxyz, {
    ...chart_config,
    data: dataxyz,
    options: {
      scales: {
        y: {
          min: 0,
          max: 4,
        },
      },
    },
  });
});

addEventListener("resize", (event) => {
  console.log("resized");

  chart.resize();
  charty.resize();
  chartz.resize();
  chartxyz.resize();
});
function indexOfMax(arr: Array<number>): number {
  if (arr.length === 0) {
    return -1;
  }

  var max = arr[0];
  var maxIndex = 0;

  for (var i = 1; i < arr.length; i++) {
    if (arr[i] > max) {
      maxIndex = i;
      max = arr[i];
    }
  }

  return maxIndex;
}

setInterval(async () => {
  let fft_batch: Array<Array<number>> = await invoke("get_fft_result");
  console.log(fft_batch);

  data.datasets[0].data = fft_batch[0];
  datay.datasets[0].data = fft_batch[1];
  dataz.datasets[0].data = fft_batch[2];
  dataxyz.datasets[0].data = fft_batch[3];
  charty.update();
  chart.update();
  chartxyz.update();
  chartz.update();
  setTimeout(() => {
    let index = indexOfMax(fft_batch[0].slice(1)) + 1;
    if (fft_batch[0][index] <= 0.5) 
    {
      max.x.amp = "--"
      max.x.freq = '--'
    }
    else{
      max.x.amp = fft_batch[0][index].toFixed(2);
      max.x.freq = ((index / 65) * 94.5).toFixed(1);
    }
    
    index = indexOfMax(fft_batch[1].slice(1)) + 1;
    if (fft_batch[1][index] <= 0.5) 
    {
      max.y.amp = "--"
      max.y.freq = '--'
    }
    else{
      max.y.amp = fft_batch[1][index].toFixed(2) ;
      max.y.freq = ((index / 65) * 94.5).toFixed(1) ;
    }

    index = indexOfMax(fft_batch[2].slice(1)) + 1;
    if (fft_batch[2][index] <= 0.5) {
      max.z.amp = "--"
      max.z.freq = '--'
    }
    else{
      max.z.amp = fft_batch[2][index].toFixed(2);
      max.z.freq = ((index / 65) * 94.5).toFixed(1);
    }

    index = indexOfMax(fft_batch[3].slice(1)) + 1;
    if (fft_batch[3][index] <= 0.5) {
      max.xyz.amp = "--"
      max.xyz.freq = '--'
    }
    else{
      max.xyz.amp = fft_batch[3][index].toFixed(2);
      max.xyz.freq = ((index / 65) * 94.5).toFixed(1);
    }

  });
  // charty.tooltip?.setActiveElements([0,1]);
  // chart.resize();
}, 100);
</script>

<template>
  <!-- <form class="row" @submit.prevent="greet">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="submit">Greet</button>
  </form> -->
  <div style="display: flex; width: 100vw">
    <div style="flex: 1; position: relative">
      <canvas class="my-chart"></canvas>
      <div>频率：{{ max.x.freq }}Hz 幅度：{{ max.x.amp }}m/s^2</div>
    </div>
    <div style="flex: 1; position: relative">
      <canvas class="my-charty"></canvas>
      <div>频率：{{ max.y.freq }}Hz 幅度：{{ max.y.amp }}m/s^2</div>
    </div>
  </div>
  <div style="display: flex; width: 100vw">
    <div style="flex: 1; position: relative; width: 100%">
      <canvas class="my-chartz"></canvas>
      <div>频率：{{ max.z.freq }}Hz 幅度：{{ max.z.amp }}m/s^2</div>
    </div>
    <div style="flex: 1; position: relative; width: 100%">
      <canvas class="my-chartxyz"></canvas>
      <div>频率：{{ max.xyz.freq }}Hz 幅度：{{ max.xyz.amp }}m/s^2</div>
    </div>
  </div>
</template>
<style></style>
