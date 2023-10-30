<script lang="ts">
  export let title = "cards"
</script>
<div class="card-container">
  <div class="card-title"><h1>{title}</h1></div>
  <div class="card-body">
    <slot/>
  </div>
</div>
<style>
  .card-body{
  display: flex;
  height: 149.4px;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  color: #000 !important;
  }
  .card-container{
  background-color: #eee;
  width: 300px;
  height: 180px;
  }
  .card-title{
  background-color: var(--accent-light);
  display: flex;
  align-items: center;
  justify-content: center;
  }
  .card-title h1{
  color: var(--gray-0);
  padding: 0;
  margin: 0.3rem 0;
  font-size: 1.2rem;
  font-family: Arial, "Helvetica Neue", Helvetica, sans-serif;
  font-weight: normal;
  }
</style>
