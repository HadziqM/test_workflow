<script lang="ts">
  import Card from "../lib/Card.svelte";

  export let options = [{id:1,text:"Flow Sensor (In)"}]
  export let selected:{id:number,text:string}
  export let onChange:(id:number)=>void
</script>
  <Card title="Graph Select">
    <p>Select this to change sensor</p>
    <div class="select">
       <select name="format" id="format" bind:value={selected} on:change={()=>{
         onChange(selected.id)
       }}>
        {#each options as opt}
          <option value={opt}>
            {opt.text}
          </option>
        {/each}
       </select>
    </div>
  </Card>
<style>
  p{
  color:#000;
  font-size:0.9rem;
  }
  select {
     -webkit-appearance:none;
     -moz-appearance:none;
     -ms-appearance:none;
     appearance:none;
     outline:0;
     box-shadow:none;
     border:0!important;
     background: var(--accent-light);
     background-image: none;
     flex: 1;
     padding: 0.2em;
     color:var(--gray-0) !important;
     cursor:pointer;
     font-size: 0.8em;
     font-family: 'Open Sans', sans-serif;
  }
  select::-ms-expand {
     display: none;
  }
  .select {
     position: relative;
     display: flex;
     width: 12em;
     height: 2em;
     line-height: 2;
     background: var(--accent-dark);
     overflow: hidden;
     border-radius: .25em;
  }
  .select::after {
     content: '\25BC';
     position: absolute;
     top: 0;
     right: 0;
     padding: 0 1em;
     background: var(--accent-dark);
     color: var(--secondary-color) !important;
     cursor:pointer;
     pointer-events:none;
     transition:.25s all ease;
  }
  .select:hover::after {
     color: #333;
  }
</style>
