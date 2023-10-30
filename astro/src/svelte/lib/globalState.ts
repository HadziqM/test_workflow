import { writable, type Writable } from "svelte/store";
import type { Ship,CommonData,Users,SensorList } from "../type";

export const domain = window.location.origin
// no class is black
export let theme = writable("")

//the state for client side pages
export let page = writable("main")

//the state for client side pages
export let sens = writable("gps")

//the state for client side sidebar
export let side = writable(false)

//the state for user data
export let user:Writable<Users> = writable({name:"dummy",id:0,ships:[{name:"dummy",id:0}]})

//the state of selected ship
export let selectedShip:Writable<{id:number,name:string}> = writable({id:0,name:"dummy"})

//the state for sensor list
export let sensorList:Writable<SensorList> = writable({flow:[{id:0,text:"dummy0"}],sonic:[{id:1,text:"dummy1"}]})

export const defaultData:CommonData[] =[
    { "date": 1682560242, "data": 0 },
    { "date": 1682560243, "data": 0 },
    { "date": 1682560244, "data": 0 },
    { "date": 1682560245, "data": 0 },
    { "date": 1682560246, "data": 0 },
    { "date": 1682560247, "data": 0 },
    { "date": 1682560248, "data": 0 },
    { "date": 1682560249, "data": 0 },
    { "date": 1682560250, "data": 0 },
    { "date": 1682560251, "data": 0 },
    { "date": 1682560252, "data": 0 },
    { "date": 1682560253, "data": 0 },
    { "date": 1682560254, "data": 0 }
  ]


//the ship data state
export let ship:Writable<Ship> = writable({
  name:"dummy",
  id:0,
  status:[
    {name:"dummy0",status:false,key:0},
    {name:"dummy1",status:false,key:1},
    {name:"dummy2",status:false,key:2},
  ],
  flow:[
    {
      fall_id:0,
      fall_name:"dummy1",
      fall:defaultData,
      rise:defaultData,
      rise_id:0,
      rise_name:"dummy2",
      rate:{
        rate:0,
        total:0
      }
    }
  ],
  ultrasonic:[
    {
      main:defaultData,
      id:2,
      max:1000,
      name:"dummy3",
      rate:{
        rate:0
      }
    }
  ],
  notif:[
    {message:"dummy added",date:1682560254}
  ]
})
