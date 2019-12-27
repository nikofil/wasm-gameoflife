import { Universe, Cell } from "wasm-gameoflife";
import { memory } from "wasm-gameoflife/wasm_gameoflife_bg";

const CELL_SIZE = 10; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const canvas = document.getElementById('gol-canvas');
// const uni = Universe.default();
const uni = Universe.new(64, 64);
const width = uni.width();
const height = uni.height();
for (var x = 0; x < width; x++)
    for (var y = 0; y < width; y++)
        if (Math.random() < 0.5)
            uni.toggle(x, y);
const cells = uni.cells();

canvas.width = (CELL_SIZE + 1) * width + 1;
canvas.height = (CELL_SIZE + 1) * height + 1;

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

let prev = 0;
const renderLoop = (ts) => {
    if (ts - prev > 100) {
        prev = ts;
        drawGrid();
        drawCells();
        uni.tick();
    }
    requestAnimationFrame(renderLoop);
}

requestAnimationFrame(renderLoop);
