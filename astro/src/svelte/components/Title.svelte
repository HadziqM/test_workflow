<script lang="ts">
  import { onMount } from "svelte";

  export let title = "Dashboard"

  let add_zero = (num:number) => num<10 ? `0${num}` : num;
  let date = {
    date:"today",
    time:"now"
  }

  function recursiveClock(){
    let tick = new Date();
    date.time = `${add_zero(tick.getHours())}:${add_zero(tick.getMinutes())}:${add_zero(tick.getSeconds())}`;
    date.date = tick.toDateString()
    setTimeout(() => {
      recursiveClock()
    }, 1000);
  }

  onMount(()=>{
    recursiveClock()
  })
</script>

<div class="title">
  <h1>{title}</h1>
  <div>
    <p>{date.date}</p>
    <p>{date.time}</p>
  </div>
</div>

<style>
  .title{
  background-color: #333;
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
  }
  .title h1{
  font-size: 1.5rem;
  }
  p{
  padding: 0.3rem;
  margin: 0;
  text-align: right;
  }
    @media only screen and (max-width: 768px) {
  .title h1{
  font-size: 1.2rem;
  }
  .title p{
  font-size: 0.8rem;
  }
  }
</style>
