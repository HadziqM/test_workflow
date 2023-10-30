<script lang="ts">
  import Card3 from "../lib/Card3.svelte";
</script>
<Card3 title="Compass">
  <div class="compass">
<div class="compass-inner"><div class="north">N</div><div class="east">E</div><div class="west">W</div><div class="south">S</div><div class="main-arrow"><div class="arrow-up"></div><div class="arrow-down"></div></div></div>
</div>
</Card3>
<style>
.compass {
	width: 150px;
	height: 150px;
	background-color: #555;
	border-radius: 100%;
	background-image: -webkit-linear-gradient(top, #555, #666);
  position: relative;
  margin: 0 auto;
}

.compass-inner {
	width: 120px;
	height: 120px;
	background-color: #3D3D3D;
	border-radius: 100%;
	position: relative;
	left: 12px;
	top: 12px;
	border: 3px solid #C5C5C5;
}

.main-arrow {
	height: 100%;
	width: 10px;
	left: 57px;
	position: relative;
	padding-top: 5px;
	box-sizing:border-box;
  transform: rotate(20deg);
	animation: spin 2.0s alternate infinite;
	-ms-transform: rotate(20deg);
	-o-transform: rotate(20deg);
	-o-animation: spin 2.0s alternate infinite;
	-moz-transform: rotate(20deg);
	-moz-animation: spin 2.0s alternate infinite;
	-webkit-transform: rotate(20deg);
	-webkit-animation: spin 2.0s alternate infinite;
	
}

.arrow-up {
	width: 0; 
	height: 0;
	border-left: 5px solid transparent;
	border-right: 5px solid transparent;
	border-bottom: 60px solid #EF5052;
	position: relative;
}

.arrow-down {
	-webkit-transform: rotate(180deg); /* Safari and Chrome */
	width: 0; 
  transform: rotate(180deg);
  -ms-transform: rotate(180deg);
	height: 0;
	border-left: 5px solid transparent;
	border-right: 5px solid transparent;
	border-bottom: 60px solid #F3F3F3;
	position: relative;
}

.north {
	font-size: 1rem;
	font-family: 'Lobster Two', cursive;
	color: #FFF;
	position: absolute;
	left: 55px;
	top: 10px;
}

.east {
	font-size: 1rem;
	font-family: 'Lobster Two', cursive;
	color: #FFF;
	position: absolute;
	left: 100px;
	top: 60px;
}

.west {
	font-size: 1rem;
	font-family: 'Lobster Two', cursive;
	color: #FFF;
	position: absolute;
	left: 10px;
	top: 60px;
}

.south {
	font-size: 1rem;
	font-family: 'Lobster Two', cursive;
	color: #FFF;
	position: absolute;
	left: 55px;
	top: 100px;
}
</style>
