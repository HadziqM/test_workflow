<script lang="ts">
  import Page from "../lib/Page.svelte";
  import AreaCharts from "../components/AreaCharts.svelte";
  import GraphSelect from "../components/GraphSelect.svelte";
  import Card from "../lib/Card.svelte";
  import Section from "../lib/Section.svelte";
  import Tank from "../components/Tank.svelte";
  import Notification from "../components/Notification.svelte";
  import Status from "../components/Status.svelte";
  import Radial from "../components/Radial.svelte";
  import Compass from "../components/Compass.svelte";
  import Card3 from "../lib/Card3.svelte";
  import { ship,defaultData,sensorList,domain,selectedShip } from "../lib/globalState";
  import type { Ship,CommonData,Dropdown,SensorList } from "../type";
  import { onMount } from 'svelte';


  let sens:{flow:CommonData[],sonic:CommonData[]} = {
    flow:defaultData,
    sonic:defaultData
  }

  let sens_selected:{flow:Dropdown,sonic:Dropdown} = {
    flow:{text:"Flow (in) (Main)",id:0},
    sonic:{text:"Ultra (main)",id:0}
  }

  interface SensorData{
    data:CommonData[]
    rate:{
      rate:number
      total:number
    }
  }

  const dummyData:SensorData = {
    data:defaultData,
    rate:{
      rate:103,
      total:1002
    }
  } 

  let sens_list:SensorList;
  let all_data:Ship;
  let allGraphData = new Map<number,SensorData>()
  let flowRate = {rate:102,total:1103}
  let sonicRate = {rate:10,total:1000}


  let onChangeData = {
    flow:(id:number) => {
      const data = allGraphData.get(id)
      sens.flow = data.data.length != 0? data.data:[{data:0,date:new Date().getTime()/1000}]
      flowRate = data.rate
    },
    sonic:(id:number) =>{
      const data = allGraphData.get(id)
      sens.sonic = data.data.length != 0? data.data:[{data:0,date:new Date().getTime()/1000}]
      sonicRate = data.rate
    }
  }
  ship.subscribe(e=>{
    console.log("begin update graph")
    all_data = e
    allGraphData = new Map()

    e.flow.map(x=>{
      allGraphData.set(x.fall_id,{data:x.fall,rate:x.rate})
    })
    e.flow.filter(x=>typeof x.rise_name == "string")
      .map(y=>allGraphData.set(y.rise_id,{data:y.rise,rate:y.rate}))
    e.ultrasonic.map(x=>allGraphData.set(x.id,{data:x.main,rate:{rate:x.rate.rate,total:x.max}}))

  })

  sensorList.subscribe(e=>{
    sens_list = e
  })

  let globalShown = {
    x:true,
    y:true,
    z:true,
    a:true,
    b:false
  };

  let loading = false

  allGraphData.set(0,dummyData)
  allGraphData.set(1,dummyData)


  function updateGraph(){
    const fData = allGraphData.get(0)

    sens = {
      flow:fData.data.length != 0? fData.data : [{data:0,date:new Date().getTime()/1000}],
      sonic:sens.sonic
    }
    // flowRate = fData.rate
    // sonicRate = sData.rate
  }

  //simulating

  function add_random(x:number,y:number){
    let data = allGraphData.get(x);
    data?.data.shift();
    data?.data.push({data: 5 + Math.random()*y,date:new Date().getTime()/1000})
  }
  function add_rate(){
    flowRate = {total:flowRate.total + Math.random()*5,rate:flowRate.rate}
  }

  function rand_rec(){
    add_random(0,5)
    add_rate()
    updateGraph()

    setTimeout(() => {
      rand_rec()
    }, 2000);
  }

  onMount(()=>{
    rand_rec()
  })

  // async function getShip(id:number,first:boolean){
  //   let res = await fetch(`${domain}/api/ship/${id}`)
  //   let e = await res.json() as Ship
  //   if (first) {
  //     //update sensor list only for first time
  //     const sonicList = e.ultrasonic.map(x=>{
  //       return {id:x.id,text:x.name}
  //     })
  //     const flowlist = e.flow.map(x=>{
  //       return {id:x.fall_id,text:x.fall_name}
  //     })
  //     const added = e.flow.filter(y=>typeof y.rise_name == "string")
  //     .map(x=>{
  //       return {id:x.rise_id,text:x.rise_name}
  //     })
  //     sensorList.set({flow:[...flowlist,...added],sonic:sonicList})
  //     sens_selected = {
  //       flow:sens_list.flow[0],
  //       sonic:sens_list.sonic[0]
  //     }
  //     console.log("finish update list")
  //
  //     ship.set(e)
  //     updateGraph()
  //
  //     loading = false
  //     globalShown = {x:true,y:true,z:true,a:true,b:true}
  //   }else {
  //     ship.set(e)
  //     updateGraph()
  //   }
  // }
  // async function getShipRec(id:number){
  //   await getShip(id,false)
  //
  //   setTimeout(async () => {
  //     if (shipId==id) {
  //       await getShipRec(id)
  //     }
  //   }, 10000);
  // }
  // async function recursive(id:number){
  //   await getShip(id,true)
  //
  //   setTimeout(async () => {
  //     if (shipId==id) {
  //       await getShipRec(id)
  //     }
  //   }, 10000);
  // }

  let shipId:number = 0
  let title:string

  // selectedShip.subscribe(async (e) =>{
  //   if (e.id != 0){
  //     shipId = e.id
  //     title = e.name
  //     await recursive(e.id)
  //   }
  // })

  let dummy = 0


</script>
<Page bind:loading={loading}>
  <Section title="Flow Sensor Fuel Section" bind:show={globalShown.x}>
    <div class="section-wrap">
      <AreaCharts title={sens_selected.flow.text||"idk"} bind:graphData={sens.flow} mean={`${flowRate.rate.toFixed(2)} mL/s`}/>
      <div class="pair-card">
        <GraphSelect bind:selected={sens_selected.flow} options={sens_list.flow} onChange={onChangeData.flow}/>
        <Card title= "Total (In a Day)">
          <h1 class="total-x">{`${flowRate.total.toFixed(2)} mL`}</h1>
        </Card>
      </div>
    </div>
  </Section>
  <Section title="Ultrasonic Sensor Fuel Section" bind:show={globalShown.y}>
    <div class="section-wrap">
        <AreaCharts title={sens_selected.sonic.text||"idk"} meanTitle="Average" bind:graphData={sens.sonic} 
          mean={`${sonicRate.rate.toFixed(2)} mL/s`}/>
      <div class="pair-card">
        <GraphSelect options={sens_list.sonic} bind:selected={sens_selected.sonic} onChange={onChangeData.sonic}/>
        <Tank value={sens.sonic[sens.sonic.length -1].data.toFixed(2)} percentage={sens.sonic[sens.sonic.length - 1].data*100/sonicRate.total}/>
      </div>
    </div>
  </Section>
  <Section title="Gps and Gyroscope Motion" lock bind:show={globalShown.z}>
    <div class="section-wrap">
      <div class="pair-card">
        <Compass/>
        <Card3 title="Coordinates">
          <div class="coordinates">
            <h1 class="coord">{`${dummy} N`}</h1>
            <h1 class="coord">{`${dummy} W`}</h1>
          </div>
        </Card3>
      </div>
      <div class="pair-card">
        <Card3 title="Speed from Motion">
          <h1 style="font-weight: lighter;width: 100%;text-align: center;">{`${dummy} m/s`}</h1>
        </Card3>
        <Card3 title="Distance (In a Day)">
          <h1 style="font-weight: lighter;width: 100%;text-align: center;">{`${dummy} m`}</h1>
        </Card3>
      </div>
    </div>
  </Section>
  <Section title="CAN-BUS Comminucation Binding" lock bind:show={globalShown.a}>
    <div class="section-wrap">
      <div class="pair-card">
        <Radial title="Temperature" value={27} percentage={9} unit="Celcius"/>
        <Radial title="Oil Pressure" value={300} percentage={50} unit="Newton"/>
      </div>
      <div class="pair-card">
        <Radial title="Vibration" value={103} percentage={90} unit="Hz"/>
        <Radial title="Rpm" value={500} percentage={30} unit="rpm"/>
      </div>
    </div>
  </Section>
  <Section title="Sensor Status" bind:show={globalShown.b}>
    <div class="section-wrap">
      <Status status={all_data.status}/>
      <Notification notif={all_data.notif}/>
    </div>
  </Section>
</Page>
<style>
  .coord{
    font-size: 2 rem;
    font-weight: lighter;
    text-align: center;
  }
  .coordinates{
  display: flex;
  height: 100%;
  width: 100%;
  flex-direction: column;
  }
  .section-wrap{
  display: flex;
  align-items: start;
  justify-content: center;
  gap: 0.5rem;
  padding: 0.5rem;
  }
  .pair-card{
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: column;
  gap: 0.5rem;
  }
  h1{
  color: #000;
  font-size: 2rem;
  }
  @media only screen and (max-width: 768px) {
  .section-wrap{
  flex-direction: column;
  gap: 0.2rem;
  }
  .pair-card{
  gap: 0.2rem;
  }
  }
</style>
