import { NetManager } from './net';
import { Game } from './game';
import { InputManager } from './input';
import { Renderer } from './renderer';

async function init() {
    const canvas = document.getElementById('game') as HTMLCanvasElement;
    const net = new NetManager(
        () => console.log('WS Open'),
        () => console.log('WS Closed')
    );
    const game = new Game(net);
    const input = new InputManager();
    const renderer = new Renderer(canvas, game);

    // 1. Session Claim
    let token = localStorage.getItem('jxwxmk_token');
    if (!token) {
        const player_id = crypto.randomUUID();
        const resp = await fetch('/session/claim', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ player_id })
        });
        const data = await resp.json();
        token = data.token;
        localStorage.setItem('jxwxmk_token', token!);
    }

    // 2. Connect
    net.connect(token!);

    // 3. Hotbar
    window.addEventListener('keydown', (e) => {
        if (e.code === 'KeyE') {
            renderer.toggleInventory();
        }
        if (e.code === 'KeyC') {
            renderer.toggleCrafting();
        }
        if (e.code === 'KeyQ') {
            renderer.toggleQuests();
        }
        if (e.code === 'KeyK') {
            renderer.toggleAchievements();
        }
        if (e.code.startsWith('Digit')) {
            const slot = parseInt(e.code.substring(5)) - 1;
            if (slot >= 0 && slot < 7) {
                net.send({ type: 'slot', data: { slot } });
            }
        }
    });

    window.addEventListener('mousedown', (e) => {
        if (renderer.isCraftingOpen()) {
            const recipe = renderer.getCraftingRecipeAt(e.clientX, e.clientY);
            if (recipe) {
                net.send({ type: 'craft', data: { recipe } });
            }
        }
    });

    // 4. Loops
    setInterval(() => {
        const move = input.getMovement();
        if (move.dx !== 0 || move.dy !== 0 || input.attack || input.interact) {
            net.send({
                type: 'input',
                data: {
                    dx: move.dx,
                    dy: move.dy,
                    attack: input.attack,
                    interact: input.interact,
                    aim: { x: input.mouseX, y: input.mouseY }
                }
            });
        }
    }, 50);

    function loop() {
        game.update();
        renderer.render();
        requestAnimationFrame(loop);
    }
    loop();
}

init();