import { Universe, Cell } from "wasm-gameoflife";
import { memory } from "wasm-gameoflife/wasm_gameoflife_bg";

const CELL_SIZE = 10; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

let canvas = document.getElementById('gol-canvas');
let uni;
let width;
let height;
let cells;
let targFps = 10;

const initUni = (newuni) => {
    uni = newuni;
    width = uni.width();
    height = uni.height();
    cells = uni.cells();

    canvas.width = (CELL_SIZE + 1) * width + 1;
    canvas.height = (CELL_SIZE + 1) * height + 1;
}

initUni(Universe.default());

const ctx = canvas.getContext('2d');

const getIndex = (x, y) => y * width + x;

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }
    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
}

const drawCells = () => {
    const cellsPtr = uni.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height / 8 + 1);

    ctx.beginPath();

    for (let x = 0; x < width; x++) {
        for (let y = 0; y < height; y++) {
            const idx = getIndex(x, y);
            ctx.fillStyle = ((cells[Math.floor(idx / 8)] & (1 << (idx % 8))) != 0) ? ALIVE_COLOR : DEAD_COLOR;
            ctx.fillRect(
                x * (CELL_SIZE + 1) + 1,
                y * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.stroke();
}

let dragging = false;
let lastX = -1;
let lastY = -1;

const toggleCell = ({offsetX, offsetY}) => {
    const x = Math.floor(offsetX / (CELL_SIZE + 1));
    const y = Math.floor(offsetY / (CELL_SIZE + 1));
    uni.toggle(x, y);
    drawGrid();
    drawCells();
}

const setLast = ({offsetX, offsetY}) => {
    const x = Math.floor(offsetX / (CELL_SIZE + 1));
    const y = Math.floor(offsetY / (CELL_SIZE + 1));
    if (x != lastX || y != lastY) {
        lastX = x;
        lastY = y;
        return true;
    }
    return false;
}

canvas.onclick = toggleCell;

canvas.onmousedown = () => {dragging = true;}
canvas.onmouseup = () => {setLast(-1, -1); dragging = false;}
canvas.onmouseleave = () => {setLast(-1, -1); dragging = false;}
canvas.onmousemove = (e) => {
    if (dragging && setLast(e)) {
        toggleCell(e);
    }
}

let prev = 0;
let animId = null;
let perfPrev = 0;

const renderLoop = (ts) => {
    if (ts - prev > 1000 / targFps) {
        const perf = performance.now();
        const fps = 1000 / (perf - perfPrev);
        perfPrev = perf;
        document.getElementById('fps').innerText = fps.toString().slice(0, 4);
        console.log(fps);
        prev = ts;
        drawGrid();
        drawCells();
        uni.tick();
    }
    animId = requestAnimationFrame(renderLoop);
}

const pause = () => {
    if (animId === null) {
        requestAnimationFrame(renderLoop);
    } else {
        cancelAnimationFrame(animId);
        animId = null;
    }
}

const step = () => {
    uni.tick();
    drawGrid();
    drawCells();
}

const cleargol = () => {
    initUni(Universe.new(64, 64));
    drawGrid();
    drawCells();
}

const randomgol = () => {
    initUni(Universe.new(64, 64));
    for (var x = 0; x < width; x++)
        for (var y = 0; y < width; y++)
            if (Math.random() < 0.5)
                uni.toggle(x, y);
    drawGrid();
    drawCells();
}

const defaultgol = () => {
    initUni(Universe.default());
    drawGrid();
    drawCells();
}

document.getElementById('pause').onclick = pause;
document.getElementById('step').onclick = step;
document.getElementById('clear').onclick = cleargol;
document.getElementById('random').onclick = randomgol;
document.getElementById('default').onclick = defaultgol;

document.getElementById('range').onchange = (e) => {
    targFps = e.target.value;
}

drawGrid();
drawCells();

// requestAnimationFrame(renderLoop);
