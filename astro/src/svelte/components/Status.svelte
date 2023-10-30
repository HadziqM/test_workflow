<script lang="ts">
  import Card2 from "../lib/Card2.svelte";
  import type { Status } from "../type.d.ts";

  export let status:Status[] = []
</script>
<Card2 title="Status">
  {#each status as stat}
  <label class="toggle">
    <input on:click={(e)=>e.preventDefault()} class="toggle-checkbox" type="checkbox" checked={stat.status}>
    <div class="toggle-switch"/>
    <span class="toggle-label">{stat.name}</span>
  </label>
  {/each}
</Card2>
<style>
  .toggle {
  cursor: pointer;
  display: inline-block;
  color: var(--primary-color) !important;
  margin-left: 0.5rem;
  margin-bottom: 0.3rem;
  font-size: 0.8rem;
}

.toggle-switch {
  display: inline-block;
  background: #a00;
  border-radius: 16px;
  width: 58px;
  height: 32px;
  position: relative;
  vertical-align: middle;
  transition: background 0.25s;
}
.toggle-switch:before, .toggle-switch:after {
  content: "";
}
.toggle-switch:before {
  display: block;
  background: linear-gradient(to bottom, #fff 0%, #eee 100%);
  border-radius: 50%;
  box-shadow: 0 0 0 1px rgba(0, 0, 0, 0.25);
  width: 24px;
  height: 24px;
  position: absolute;
  top: 4px;
  left: 4px;
  transition: left 0.25s;
}
.toggle:hover .toggle-switch:before {
  background: linear-gradient(to bottom, #fff 0%, #fff 100%);
  box-shadow: 0 0 0 1px rgba(0, 0, 0, 0.5);
}
.toggle-checkbox:checked + .toggle-switch {
  background: #56c080;
}
.toggle-checkbox:checked + .toggle-switch:before {
  left: 30px;
}

.toggle-checkbox {
  position: absolute;
  visibility: hidden;
}

.toggle-label {
  margin-left: 5px;
  position: relative;
  top: 2px;
}
</style>
