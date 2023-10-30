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
  height: 170px;
  justify-content: center;
  align-items: center;
  box-sizing: border-box;
  padding: 0 2rem;
  color: #fff !important;
  }
  .card-container{
  background-color: #eee;
  width: 200px;
  height: 200px;
  }
  .card-title{
  background-color: var(--accent-light);
  display: flex;
  align-items: center;
  justify-content: center;
  }
  .card-title h1{
  color: var(--gray-0) !important;
  padding: 0;
  margin: 0.3rem 0;
  font-size: 1.2rem;
  font-family: Arial, "Helvetica Neue", Helvetica, sans-serif;
  font-weight: normal;
  }
  @media only screen and (max-width: 768px) {
  .card-container{
  width: 300px;
  }
  }
</style>
