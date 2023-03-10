import { memory } from "2048-rs/twentyfortyeight_bg.wasm";

const GRID_SIZE = 4; // number of rows = cols
const CELL_SIZE = 80; // cell size in px
const CELL_PADDING = 4; // cell padding on both sides in px
const FONT_FAMILY = 'Arial';

const TOTAL_CELL_SIZE = CELL_SIZE + 2 * CELL_PADDING;

const MOVE_ANIMATE_TIME = 75; // in ms

const lightTheme = {
    LINE_COLOR: 'grey',
    CELL_COLOR(value) {
        let value_str = `${value}`;
        if (value_str.length == 1) {
            return 'burlywood';
        } else if (value_str.length == 2) {
            return '#87ded0';
        } else {
            return '#c37aef';
        }
    },
    TEXT_COLOR: 'white',
    BG_COLOR: 'white',
};
const theme = lightTheme;

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
        this.ctx.fillStyle = theme.BG_COLOR;
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

    fillGrid(update, dt, onUpdateConsumed) {
        const cellsPtr = this.game.cells();
        const cells = new Uint32Array(memory.buffer, cellsPtr, GRID_SIZE * GRID_SIZE);

        if (update) {
            update.dt += dt;
            if (update.dt > MOVE_ANIMATE_TIME) {
                update.dt = 0;
                onUpdateConsumed();
            }
        }

        this.ctx.beginPath();

        for (let i = 0; i < GRID_SIZE; i++) {
            for (let j = 0; j < GRID_SIZE; j++) {
                const idx = getIndex(j, i);
                let x = j, y = i, value = cells[idx];
                if (update && update.dt > 0) {
                    let m = update.moves.find(move => move.before.x === x && move.before.y === y);
                    if (m) {
                        if (!m.done) {
                            m.done = 0;
                        }
                        m.done += dt / MOVE_ANIMATE_TIME;
                        if (m.done < 1) {
                            x += (m.after.x - m.before.x) * m.done;
                            y += (m.after.y - m.before.y) * m.done;
                            value = m.value;
                        } else {
                            update.moves = update.moves.filter(move => !(move.before.x === x && move.before.y === y));
                        }
                    } else if (update.new_piece.x === x && update.new_piece.y === y) {
                        value = 0;
                    } else {
                        m = update.moves.find(move => move.after.x == x && move.after.y === y);
                        if (m) {
                            value = 0;
                        }
                    }
                }
                if (value === 0) continue;
                this.renderCell(value, x, y);
            }
        }
        this.ctx.stroke();
    }

    renderCell(value, x, y) {

        // render rect
        this.ctx.fillStyle = theme.CELL_COLOR(value);
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
