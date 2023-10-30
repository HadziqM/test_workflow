<script lang="ts">
  import { onMount,afterUpdate } from 'svelte';
  import { Chart, registerables } from 'chart.js';
  import { dateCalender,timeCalender } from "../lib/dateCounter";
  import type { CommonData } from "../type";

  export let graphData:CommonData[] =[
    { "date": 1682560242, "data": 1.2 },
    { "date": 1682560243, "data": 2.1 },
    { "date": 1682560244, "data": 3.4 },
    { "date": 1682560245, "data": 4.2 },
    { "date": 1682560246, "data": 5.7 },
    { "date": 1682560247, "data": 2.8 },
    { "date": 1682560248, "data": 3.9 },
    { "date": 1682560249, "data": 2.4 },
    { "date": 1682560250, "data": 1.2 },
    { "date": 1682560251, "data": 0.8 },
    { "date": 1682560252, "data": 3.7 },
    { "date": 1682560253, "data": 5.5 },
    { "date": 1682560254, "data": 4.1 }
  ]
 
   Chart.register(...registerables);


  export let meanTitle = "Flow Rate"
  export let mean = "0 mL/min"
  export let title = "Flow Sensor (In)"
  let displayDate = dateCalender(graphData[0].date)
  $: width = (window.innerWidth > 0) ? window.innerWidth : screen.width;
  $: width_c =width>768?400:268
  $: height_c= width>768?280:150


   let barChartElement: HTMLCanvasElement;
   const chartData = {
     labels: graphData.map(({ date }) => timeCalender(date)),
     datasets: [
       {
         label: 'Volume (mL)',
         data: graphData.map(({ data }) => data),
         backgroundColor: ['#fff'],
         borderColor: ['#000'],
         borderRadius: 4,
         borderWidth: 2,
       },
     ],
   };
  let chart:Chart;
   onMount(() => {
      chart = new Chart(barChartElement, {
         type: 'line',
         data: chartData,
         options: {
          responsive:false,
           plugins: {
             legend: {
               display: false,
             },
           },
          animation: {
            duration: 500, // animation duration in milliseconds
            easing: 'easeInOutQuad' // easing function used for the animation
          },
           scales: {
             x: {
               grid: {
                 color: 'transparent',
               },
               ticks: { color: '#000' },
             },
             y: {
               beginAtZero: false,
               ticks: { color: '#000' },
               grid: {
                 color: '#000',
               },
               title: {
                 display: false,
                 text: 'Volume (mL)',
                 color: '#000',
                 font: { size: 20},
               },
             },
           },
         },
       });
   });

  function updateChart(data:CommonData[]) {
    chart.data.labels = data.map(e=>timeCalender(e.date));
    chart.data.datasets![0].data = data.map(e=>e.data);
    chart.update();
    displayDate = dateCalender(data[0].date)
  }

  afterUpdate(()=>{
    console.log("graph updated")
    updateChart(graphData)
  })
</script>
  <div class="canvas-container">
    <div class="canvas-head">
      <div class="canvas-title"><h1>{title}</h1><p>{displayDate}</p></div>
      <div class="canvas-title"><h1>{meanTitle}</h1><p>{mean}</p></div>
    </div>
    <div class="canvas-body">
      <canvas width={width_c} height={height_c} bind:this={barChartElement} />
    </div>
  </div>
<style>
  .canvas-container{
    display: flex;
    flex-direction:column;
    overflow: scroll;
    width: fit-content;
  }
  .canvas-body{
    padding: 1rem;
    background-color: #eee;
  }
  .canvas-head{
    background-color: var(--accent-light);
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
  .canvas-title{
    padding: 0.5rem;
    display: flex;
    flex-direction: column;
    align-items: start;
    justify-content: center;
    color: var(--gray-0);
  }
  .canvas-title h1{
    color: var(--gray-0);
    font-family: 'Courier New', Courier, monospace;
    font-size: 1.2rem;
    font-weight: normal;
    margin: 0;
    margin-bottom: 0.2rem;
    padding: 0;
  }
  .canvas-title p{
    font-size: 0.8rem;
    margin: 0;
    padding: 0;
  }
  @media (max-width: 50em) {
    .canvas-title h1{
      font-size: 1rem;
    }
    .canvas-title p{
      font-size: 0.6rem;
    }
    .canvas-body {
      padding: 0.3rem;
    }
    .canvas-container {
      width: 300px;
    }
  }
</style>
