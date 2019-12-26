import { Universe } from "wasm-gameoflife";

const pre = document.getElementById('gol-canvas');
const uni = Universe.default();

let prev = 0;
const renderLoop = (ts) => {
    if (ts - prev > 100) {
        prev = ts;
        pre.textContent = uni.render();
        uni.tick();
    }
    requestAnimationFrame(renderLoop);
}

requestAnimationFrame(renderLoop);
