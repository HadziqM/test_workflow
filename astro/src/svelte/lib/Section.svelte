<script lang="ts">
  export let title = "cards"
  export let lock = false
  export let clickable = true

  export let show = true
</script>
<div class="section-container">
  <div class={show?"section-title":"section-alter"} on:click={()=>{
    if (clickable){
      show = !show
    }
  }}><p>{title}</p></div>
  <div class="section-body" class:shown={show}>
    <div class="lock-screen" class:shown={lock}>
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M144 144v48H304V144c0-44.2-35.8-80-80-80s-80 35.8-80 80zM80 192V144C80 64.5 144.5 0 224 0s144 64.5 144 144v48h16c35.3 0 64 28.7 64 64V448c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V256c0-35.3 28.7-64 64-64H80z"/></svg>
    </div>
    <slot/>
  </div>
</div>
<style>
  .lock-screen{
  display: none;
  position: absolute;
  left: 0;
  top: 0;
  width: 100%;
  height: 100%;
  justify-content: center;
  align-items: center;
  background-color: rgba(50, 50, 50, .5);
  z-index: 1;
  }
  svg{
  height: 100px;
  width: 100px;
  }
  .section-body{
  position: relative;
  display: none;
  width: 750px;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  background-color: var(--gray-500);
  }
  .section-container{
  display: flex;
  flex-direction: column;
  align-items: center;
  margin: 2rem 0;
  }
  .shown{
  display: flex;
  }
  .section-title {
    padding: 0;
    cursor: pointer;
    align-items: center;
    justify-content: center;
    position: relative;
    display: flex;
    width: 750px;
    line-height: 2;
    background: var(--accent-light);
    border-radius: .25em;
    text-align: center;
  }
  .section-title::after {
    content: '\25B2';
    position: absolute;
    top: 0;
    right: 0;
    padding: 0 1em;
    background: #000;
    cursor:pointer;
    pointer-events:none;
    transition:.25s all ease;
  }
  .section-title:hover::after {
    color: #555;
  }
  .section-alter {
    padding: 0;
    cursor: pointer;
    align-items: center;
    justify-content: center;
    position: relative;
    display: flex;
    width: 750px;
    line-height: 2;
    background: #111;
    border-radius: .25em;
    text-align: center;
  }
  .section-alter::after {
    content: '\25BC';
    position: absolute;
    top: 0;
    right: 0;
    padding: 0 1em;
    background: #000;
    cursor:pointer;
    pointer-events:none;
    transition:.25s all ease;
  }
  .section-alter:hover::after {
    color: #555;
  }
  p{
  padding: 0;
  margin: 0;
  }
  @media only screen and (max-width: 768px) {
    p{
      font-size: 0.8rem;
    }
    .section-title{
      width: calc(100% - 20px);
    }
    .section-title::after{
      font-size: 0.8rem;
    }
    .section-alter{
      width: calc(100% - 20px);
    }
    .section-alter::after{
      font-size: 0.8rem;
    }
    .section-body{
      width: calc(100% - 20px);
    }
  }
</style>
