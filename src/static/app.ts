// Minimal TypeScript client. Compiles to app.js using tsc outside this repo.
interface Player {
  id: string;
  x: number;
  y: number;
}

const canvas = document.getElementById('game') as HTMLCanvasElement;
const ctx = canvas.getContext('2d')!;

const player: Player = { id: 'local', x: 400, y: 300 };

function draw() {
  ctx.fillStyle = '#061';
  ctx.fillRect(0, 0, canvas.width, canvas.height);

  // draw player
  ctx.fillStyle = '#ff0';
  ctx.beginPath();
  ctx.arc(player.x, player.y, 10, 0, Math.PI * 2);
  ctx.fill();
}

window.addEventListener('keydown', (e) => {
  const speed = 5;
  if (e.key === 'ArrowUp') player.y -= speed;
  if (e.key === 'ArrowDown') player.y += speed;
  if (e.key === 'ArrowLeft') player.x -= speed;
  if (e.key === 'ArrowRight') player.x += speed;
});

function loop() {
  draw();
  requestAnimationFrame(loop);
}

loop();
