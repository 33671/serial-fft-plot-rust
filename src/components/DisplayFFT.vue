<script setup lang="ts">
import { onMounted, reactive, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import {
  Chart,
  ChartConfiguration,
  ChartData,
  ChartOptions,
  DecimationOptions,
} from "chart.js/auto";
let selected_baud = ref(115200);
let isSelected = false;
let ports = ref([]);
onMounted(async () => {
  ports.value = await invoke("list_ports");
  console.log(ports);
});

let selected_port = ref<String>();
async function listen() {
  if (selected_port.value == "" || selected_port.value == null) return;
  await invoke("listen_port", {
    port: selected_port.value,
    baudRate: selected_baud.value,
  });
  isSelected = true;
}
async function refresh() {
  ports.value = await invoke("list_ports");
}
let chart: Chart;
let charty: Chart;
let chartxyz: Chart;
let chartz: Chart;

const MAX_FREQ = 728.0;
const POINTS_NUM = 2048.0;
const SHOW_NUM = 128;
const labels = Array.from(Array(SHOW_NUM).keys()).map((x) =>
  Math.round((x * MAX_FREQ) / POINTS_NUM)
);
let xAxis = Array.from(Array(SHOW_NUM).keys()).map((x) => 0);
const data: ChartData = {
  labels: labels,
  datasets: [
    {
      pointStyle: false,
      label: "X ACC",
      data: xAxis,
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
      pointStyle: false,
      label: "Y ACC",
      data: xAxis,
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
      pointStyle: false,
      label: "Z ACC",
      data: xAxis,
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
      pointStyle: false,
      label: "XYZ ACC",
      data: xAxis,
      fill: "start",
      borderColor: "rgb(75, 192, 192)",
      tension: 0.4,
    },
  ],
};
const decimation: DecimationOptions = {
  enabled: false,
  algorithm: "min-max",
};
// borderColor: 'rgb(75, 192, 192)',
let chart_config: any = {
  type: "line",
  options: {
    // plugins: {
    //   decimation: decimation,
    // },
    scales: {
      y: {
        min: 0,
        max: 3,
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
let sample_rate = ref(0);
setInterval(async () => {
  if (!isSelected) return;
  let fft_batch: Array<Array<number>> = await invoke("get_fft_result");
  console.log(fft_batch);
  sample_rate.value = await invoke("get_sample_rate");

  data.datasets[0].data = fft_batch[0];
  datay.datasets[0].data = fft_batch[1];
  dataz.datasets[0].data = fft_batch[2];
  dataxyz.datasets[0].data = fft_batch[3];
  charty.update();
  chart.update();
  chartxyz.update();
  chartz.update();
  let getFreq = (idx: number) => ((idx / POINTS_NUM) * MAX_FREQ).toFixed(2);
  const amp_limit = 0.3;
  setTimeout(() => {
    let index = indexOfMax(fft_batch[0].slice(3)) + 3;
    if (fft_batch[0][index] <= amp_limit) {
      max.x.amp = "--";
      max.x.freq = "--";
    } else {
      max.x.amp = fft_batch[0][index].toFixed(2);
      max.x.freq = getFreq(index);
    }

    index = indexOfMax(fft_batch[1].slice(3)) + 3;
    if (fft_batch[1][index] <= amp_limit) {
      max.y.amp = "--";
      max.y.freq = "--";
    } else {
      max.y.amp = fft_batch[1][index].toFixed(2);
      max.y.freq = getFreq(index);
    }

    index = indexOfMax(fft_batch[2].slice(3)) + 3;
    if (fft_batch[2][index] <= 0.5) {
      max.z.amp = "--";
      max.z.freq = "--";
    } else {
      max.z.amp = fft_batch[2][index].toFixed(2);
      max.z.freq = getFreq(index);
    }

    index = indexOfMax(fft_batch[3].slice(3)) + 3;
    if (fft_batch[3][index] <= 0.5) {
      max.xyz.amp = "--";
      max.xyz.freq = "--";
    } else {
      max.xyz.amp = fft_batch[3][index].toFixed(2);
      max.xyz.freq = getFreq(index);
    }
  });
}, 100);
</script>

<template>
  <div style="width: 100%;font-size: smaller;">

    <label for="ports">端口:</label>
  
    <select name="ports" id="ports" v-model="selected_port" style="width: 150px;">
      <option v-for="item in ports" :value="item">{{ item }}</option>
    </select>
  
    <label for="ports" style="margin-left: 20px">波特率:</label>
  
    <select name="ports" id="ports" v-model="selected_baud">
      <option
        v-for="item in [1200, 2400, 4800, 9600, 19200, 38400, 57600, 115200]"
        :value="item"
      >
        {{ item }}
      </option>
    </select>
  
    <button
      @click="refresh"
      style="margin-left: 20px; padding-top: 5px; padding-bottom: 5px"
    >
      刷新
    </button>
    <button
      @click="listen"
      style="margin-left: 20px; padding-top: 5px; padding-bottom: 5px"
    >
      监听
    </button>
  
    <span style="margin-left: 20px"
      >当前采样率：{{ sample_rate }} Hz 分辨率：{{
        (MAX_FREQ / POINTS_NUM).toFixed(2)
      }}Hz</span
    >
  </div>

  <div style="display: flex; width: 100%;height: 100%;">
    <div style="flex: 1; position: relative">
      <canvas class="my-chart"></canvas>
      <div class="text-small">
        频率：{{ max.x.freq }}Hz 幅度：{{ max.x.amp }}m/s^2
      </div>
    </div>
    <div style="flex: 1; position: relative">
      <canvas class="my-charty"></canvas>
      <div class="text-small">
        频率：{{ max.y.freq }}Hz 幅度：{{ max.y.amp }}m/s^2
      </div>
    </div>
  </div>
  <div style="display: flex; width: 100%">
    <div style="flex: 1; position: relative; width: 100%">
      <canvas class="my-chartz"></canvas>
      <div class="text-small">
        频率：{{ max.z.freq }}Hz 幅度：{{ max.z.amp }}m/s^2
      </div>
    </div>
    <div style="flex: 1; position: relative; width: 100%">
      <canvas class="my-chartxyz"></canvas>
      <div class="text-small">
        频率：{{ max.xyz.freq }}Hz 幅度：{{ max.xyz.amp }}m/s^2
      </div>
    </div>
  </div>
</template>
<style>
.text-small {
  font-size: smaller;
}
</style>
