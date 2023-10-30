<script lang="ts">
    import { afterUpdate } from "svelte";
  import Card from "../lib/Card.svelte";
  export let title = "Fuel Tank"
  export let value = 200
  export let percentage = 50

  $: style = `height: ${percentage*1.2}px;`

  afterUpdate(()=>console.log(percentage))
</script>

<Card title={title}>
  <div class="tank-card">
    <div class="tank-data">
      <p class="tank-text">Value</p>
      <h1 class="tank-value">{value}</h1>
      <p class="tank-unit">mL</p>
    </div>
    <div class="tank-pill">
      <div class="tank-level">
        <div class="tank-liquid" style={style}/>
      </div>
    </div>
  </div>
</Card>

<style>
  .tank-card{
    box-sizing: border-box;
    position: relative;
    width: 300px;
    height: 149.8px;
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    color: var(--primary-color) !important;
    align-items: center;
    padding: 0 2rem;
  }
  p{
  padding: 0;
  margin: 0 1rem;
  }
  h1{
  padding: 0;
  margin: 0.5rem;
  font-size: 2rem;
  text-align: center;
  color: #000 !important;
  }
  .tank-pill{
  position:relative;
  width: 80px;
  height: 120px;
  background-color: #111;
  box-shadow: inset 10px 0 48px hsl(0, 0%, 16%),
              inset -4px 12px 48px hsl(0, 0%, 56%);
  border-radius: 2rem;
  justify-self: flex-end;
  }
  .tank-level{
  position:absolute;
  inset: 2px;
  border-radius: 2rem;
  overflow: hidden;
  }
  .tank-liquid{
  position: absolute;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgb(219,79,26);
  background: linear-gradient(90deg, rgba(219,79,26,1) 0%, rgba(191,131,23,1) 34%, rgba(255,220,0,1) 86%);
  }
</style>
