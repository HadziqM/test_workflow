export interface CommonData{
  data:number
  date:number
}
export interface Notif{
  message:string
  date:number
}
export interface Status{
  name:string
  key:number
  status:bool
}
export interface Ultrasonic{
  id:number
  name:string
  rate:{
    rate:number
  }
  main:CommonData[]
  max:number
}
export interface Flow{
  fall_id:number
  fall_name:string
  fall:CommonData[]
  rise_id?:number
  rise_name?:string
  rise?:CommonData[]
  rate:{
    rate:number
    total:number
  }
}
export interface Ship{
  name:string
  id:number
  flow:Flow[]
  ultrasonic:Ultrasonic[]
  status:Status[]
  notif:Notif[]
}
export interface Users{
  name:string,
  id:number,
  ships:{
    name:string,
    id:number
  }[]
}

export interface Dropdown{
  id:number
  text:string
}

export interface SensorList{
  flow:Dropdown[]
  sonic:Dropdown[]
}

export interface AgregateData{
  start:number
  stop:number
  id:number
  sens:string
}
