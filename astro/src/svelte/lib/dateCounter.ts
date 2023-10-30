const monthNames = ["January", "February", "March", "April", "May", "June",
  "July", "August", "September", "October", "November", "December"];
const add_zero = (num:number) => num<10 ? `0${num}` : num;

export function dateConversion(timestamp:number){
  let date = new Date(timestamp * 1000)
  let now = new Date()
  let now_stamp = now.getTime()/1000
  let yesterday = (48 - now.getHours())*60*60

  if (date.toDateString() == now.toDateString()) {
    return "Today"
  }else if (now_stamp - timestamp < yesterday) {
    return "Yesterday"
  }else{
    return `${add_zero(date.getDate())}/${add_zero(date.getMonth()+1)}/${date.getFullYear()}`
  }
}

export function dateCalender(timestamp:number){
  let date = new Date(timestamp * 1000)
  let now = new Date()
  let now_stamp = now.getTime()/1000
  let yesterday = (48 - now.getHours())*60*60

  if (date.toDateString() == now.toDateString()) {
    return "Today"
  }else if (now_stamp - timestamp < yesterday) {
    return "Yesterday"
  }else{
    return `${date.getDate()}-${monthNames[date.getMonth()]}-${date.getFullYear()}`
  }
}

export function timeConversion(timestamp:number) {
  let date = new Date(timestamp*1000)
  return `${add_zero(date.getHours())}:${add_zero(date.getMinutes())}`
}

export function timeCalender(timestamp:number) {
  let date = new Date(timestamp*1000)
  return `${add_zero(date.getHours())}:${add_zero(date.getMinutes())}:${add_zero(date.getSeconds())}`
}
