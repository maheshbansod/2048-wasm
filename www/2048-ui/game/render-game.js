import { memory } from "2048-rs/twentyfortyeight_bg.wasm";


const GRID_SIZE = 4; // number of rows = cols
const CELL_SIZE = 80; // cell size in px
const CELL_PADDING = 4; // cell padding on both sides in px
const FONT_FAMILY = 'Arial';

const TOTAL_CELL_SIZE = CELL_SIZE + 2 * CELL_PADDING;

const theme = {
    LINE_COLOR: 'grey',
    CELL_COLOR: 'green',
    TEXT_COLOR: 'white',
    BG_COLOR: 'white',
}

const getIndex = (x, y) => y * GRID_SIZE + x;

const ctxFont = (size_ratio, font_family = FONT_FAMILY) => `${size_ratio * CELL_SIZE}px ${font_family}`

export class GameRenderer {
    // game;
    // canvas;
    // ctx;
    constructor(canvas, game) {
        const ctx = canvas.getContext('2d');
        canvas.width = TOTAL_CELL_SIZE * GRID_SIZE;
        canvas.height = TOTAL_CELL_SIZE * GRID_SIZE;
        ctx.textBaseline = 'middle';
        ctx.textAlign = 'center';
        ctx.font = ctxFont(0.75);

        this.canvas = canvas;
        this.ctx = ctx;
        this.game = game;
    }

    drawGrid() {
        this.ctx.fillStyle = 'black';
        this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);
        this.ctx.beginPath();
        this.ctx.strokeStyle = theme.LINE_COLOR;

        // vertical lines
        for (let i = 0; i <= GRID_SIZE; i++) {
            this.ctx.moveTo(i * TOTAL_CELL_SIZE, 0);
            this.ctx.lineTo(i * TOTAL_CELL_SIZE, TOTAL_CELL_SIZE * GRID_SIZE);
        }

        // horizontal lines
        for (let i = 0; i <= GRID_SIZE; i++) {
            this.ctx.moveTo(0, i * TOTAL_CELL_SIZE);
            this.ctx.lineTo(TOTAL_CELL_SIZE * GRID_SIZE, i * TOTAL_CELL_SIZE);
        }

        this.ctx.stroke();
    }

    fillGrid() {
        const cellsPtr = this.game.cells();
        const cells = new Uint32Array(memory.buffer, cellsPtr, GRID_SIZE * GRID_SIZE);

        this.ctx.beginPath();

        for (let i = 0; i < GRID_SIZE; i++) {
            for (let j = 0; j < GRID_SIZE; j++) {
                const idx = getIndex(j, i);
                if (cells[idx] === 0) continue;
                this.renderCell(cells[idx], j, i);
            }
        }
        this.ctx.stroke();
    }

    renderCell(value, x, y) {

        // render rect
        this.ctx.fillStyle = theme.CELL_COLOR;
        const rectx = TOTAL_CELL_SIZE * x + CELL_PADDING;
        const recty = TOTAL_CELL_SIZE * y + CELL_PADDING;
        this.ctx.fillRect(rectx, recty, CELL_SIZE, CELL_SIZE);

        // render value
        this.ctx.fillStyle = theme.TEXT_COLOR;
        let value_str = `${value}`;
        if (value_str.length > 1) {
            const font_size = -0.14 * value_str.length + 0.9; // calculate font-size
            this.ctx.font = ctxFont(font_size);
        } else {
            this.ctx.font = ctxFont(0.75);
        }
        this.ctx.fillText(value_str, rectx + CELL_SIZE / 2, recty + CELL_SIZE / 2);
    }
}
