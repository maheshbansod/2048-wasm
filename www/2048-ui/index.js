import { Direction } from "2048-rs";
import { Game } from "2048-rs";
import { memory } from "2048-rs/twentyfortyeight_bg";

const GRID_SIZE = 4; // number of rows = cols
const CELL_SIZE = 80; // cell size in px
const CELL_PADDING = 4; // cell padding on both sides in px

const TOTAL_CELL_SIZE = CELL_SIZE + 2 * CELL_PADDING;

const theme = {
    LINE_COLOR: 'grey', // grid bg color
    CELL_COLOR: 'green',
    TEXT_COLOR: 'white',
}

const canvas = document.getElementById("game-canvas");
const ctx = canvas.getContext("2d");
canvas.width = TOTAL_CELL_SIZE * GRID_SIZE;
canvas.height = TOTAL_CELL_SIZE * GRID_SIZE;
ctx.textBaseline = 'middle';
ctx.textAlign = 'center';
ctx.font = `${0.75 * CELL_SIZE}px Arial`;

let game = Game.new();

const getIndex = (x, y) => x * GRID_SIZE + y;

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = theme.LINE_COLOR;

    // vertical lines
    for (let i = 0; i <= GRID_SIZE; i++) {
        ctx.moveTo(i * TOTAL_CELL_SIZE, 0);
        ctx.lineTo(i * TOTAL_CELL_SIZE, TOTAL_CELL_SIZE * GRID_SIZE);
    }

    // horizontal lines
    for (let i = 0; i <= GRID_SIZE; i++) {
        ctx.moveTo(0, i * TOTAL_CELL_SIZE);
        ctx.lineTo(TOTAL_CELL_SIZE * GRID_SIZE, i * TOTAL_CELL_SIZE);
    }

    ctx.stroke();
}

const fillGrid = () => {
    const cellsPtr = game.cells();
    const cells = new Uint32Array(memory.buffer, cellsPtr, GRID_SIZE * GRID_SIZE);
    window.cells = cells;

    ctx.beginPath();
    ctx.fillStyle = theme.CELL_COLOR;

    for (let i = 0; i < GRID_SIZE; i++) {
        for (let j = 0; j < GRID_SIZE; j++) {
            const idx = getIndex(j, i);
            if (cells[idx] === 0) continue;
            const rectx = TOTAL_CELL_SIZE * i + CELL_PADDING;
            const recty = TOTAL_CELL_SIZE * j + CELL_PADDING;
            ctx.fillRect(rectx, recty, CELL_SIZE, CELL_SIZE);
            ctx.fillStyle = theme.TEXT_COLOR;
            ctx.fillText(`${cells[idx]}`, rectx + CELL_SIZE / 2, recty + CELL_SIZE / 2);
        }
    }
    ctx.stroke();
}

const renderLoop = () => {
    drawGrid();
    fillGrid();

    requestAnimationFrame(renderLoop);
}

renderLoop();