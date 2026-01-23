// Minimal JS client (converted from TS) to run in browser without build step
const canvas = document.getElementById('game');
const ctx = canvas.getContext('2d');
const id = String(Math.floor(Math.random()*1e9));
const ws = new WebSocket((location.protocol === 'https:' ? 'wss' : 'ws') + '://' + location.host + '/ws/');
let state = {};

ws.addEventListener('open', ()=>{
  console.log('ws open');
});
ws.addEventListener('message', (ev)=>{
  try{ const d = JSON.parse(ev.data); /* not used yet */ }catch(_){}
});

window.addEventListener('keydown', (e)=>{
  if(e.key==='ArrowUp') move(0,-5);
  if(e.key==='ArrowDown') move(0,5);
  if(e.key==='ArrowLeft') move(-5,0);
  if(e.key==='ArrowRight') move(5,0);
});

let me = { id: id, x: 400, y: 300 };

function move(dx, dy){
  me.x += dx; me.y += dy;
  sendState();
}

function sendState(){
  if(ws.readyState === WebSocket.OPEN) ws.send(JSON.stringify(me));
}

function draw(){
  ctx.clearRect(0,0,canvas.width,canvas.height);
  // draw me
  ctx.fillStyle = '#2a9d8f';
  ctx.beginPath(); ctx.arc(me.x, me.y, 10, 0, Math.PI*2); ctx.fill();
  // placeholder resources
  ctx.fillStyle = '#e76f51';
  ctx.fillRect(100,100,12,12);
  requestAnimationFrame(draw);
}

draw();
