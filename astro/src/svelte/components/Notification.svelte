<script lang="ts">
  import Card2 from "../lib/Card2.svelte";
  import { dateConversion,timeConversion } from "../lib/dateCounter";
  import type { Notif } from "../type";

  export let notif:Notif[] = [
   {date:1682560242,message:"sensor x activated"},{date:1682560241,message:"sensor z deactivated"}
  ,{date:1682560240,message:"sensor x activated"},{date:1682560239,message:"sensor z deactivated"}
  ,{date:1682560238,message:"sensor x activated"},{date:1682473842,message:"sensor z deactivated"}
  ,{date:1681560236,message:"sensor x activated"}]
</script>
<Card2 title="Notification">
  <div class="notif-container">
    {#each notif as note}
      <div class="notif">
        <p class="notif-text">{note.message}</p>
        <div class="notif-date">
          <p>{dateConversion(note.date)}</p>
          <p>{timeConversion(note.date)}</p>
        </div>
      </div>
    {/each}
  </div>
</Card2>
<style>
  .notif-container{
  box-sizing: border-box;
  width: 100%;
  height: 100%;
  display: flex;
  padding: 0.5rem 1rem;
  flex-direction: column;
  justify-content: start;
  align-items: start;
  }
  .notif{
  display: flex;
  color: black !important;
  align-items: end;
  }
  .notif-text{
  border-radius: 1.15rem;
  line-height: 1.25;
  max-width: 75%;
  padding: 0.5rem .875rem;
  position: relative;
  word-wrap: break-word;
  align-items: flex-start;
  background-color: #e5e5ea;
  color: #000;
  margin: 0.2rem 0;
  }
  .notif-text::before,
  .notif-text::after {
  bottom: -0.1rem;
  content: "";
  height: 0.5rem;
  position: absolute;
  }
  .notif-text::before {
  border-bottom-right-radius: 0.7rem 0.8rem;
  border-left: 1rem solid #e5e5ea;
  left: -0.35rem;
  transform: translate(0, -0.1rem);
  }
  .notif-text::after {
  background-color: #fff;
  border-bottom-right-radius: 0.5rem;
  left: 20px;
  transform: translate(-30px, -2px);
  width: 10px;
  }
  .notif-date{
  font-size: 0.6rem;
  padding-left: 0.3rem;
  display: flex;
  flex-direction: column;
  gap: 0.2rem;
  justify-content: start;
  align-items: start;
  }
  .notif-date p{
  margin: 0;
  padding: 0;
  }
</style>
