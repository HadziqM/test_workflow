<script lang="ts">
  export let loading:boolean = true
</script>

<div class="page-margin">
  <div class="page">
    <div class="spinner" class:shown={loading}>
      <div class="lds-dual-ring"/>
    </div>
    <slot/>
  </div>
</div>

<style>
.spinner{
  display: none;
  height: 100%;
  width: 100%;
  justify-content: center;
  align-items: center;
  position: absolute;
  left: 0;
  top: 0;
  z-index: 10;
  background: var(--page-color)
}
.shown{
  display: flex;
}
.lds-dual-ring {
  display: inline-block;
  width: 80px;
  height: 80px;
}
.lds-dual-ring:after {
  content: " ";
  display: block;
  width: 64px;
  height: 64px;
  margin: 8px;
  border-radius: 50%;
  border: 6px solid #fff;
  border-color: #fff transparent #fff transparent;
  animation: lds-dual-ring 1.2s linear infinite;
}
@keyframes lds-dual-ring {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}

</style>
