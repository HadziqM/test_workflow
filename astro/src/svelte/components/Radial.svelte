<script lang="ts">
import Card3 from "../lib/Card3.svelte";
export let title = "progress"
export let percentage = 90
export let value = 200
export let unit = "Hz"

$: color = percentage > 80 ? "#500" : "#050"
$: degree = percentage *3.6
$: text = `${percentage} %`
</script>
<Card3 title={title}>
  <div class="text-div">
    <p>Value</p>
    <h1>{value}</h1>
    <p>{unit}</p>
  </div>
  <div style={`background:conic-gradient(${color} ${degree}deg,#aaa 0deg)`} class="circular-progress">
    <span class="progress-value">{text}</span>
  </div>
</Card3>
<style>
  .text-div{
  color: var(--primary-color) !important;
  display: flex;
  flex-direction: column;
  align-items:start;
  justify-content: start;
  }
  p,h1{
  color: #000 !important;
  margin: 0.3rem 0;
  padding: 0;
  text-align: left;
  }
  h1{
  padding-left: 1rem;
  }
  .circular-progress{
    color: var(--primary-color) !important;
    position: relative;
    width: 120px;
    height: 120px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .circular-progress::before{
    content: "";
    width: 100px;
    height: 100px;
    position: absolute;
    background-color: #eee;
    border-radius: 50%;
  }
  .progress-value{
  position: relative;
  }
</style>
