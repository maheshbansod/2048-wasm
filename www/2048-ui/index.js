
import { Game, GameState } from "2048-rs";
import { STORAGE_KEYS } from "./constants";
import { addEventListeners } from "./game/controller";
import { GameRenderer } from "./game/render-game";
import { getFromLocalStorage, setLocalStorage } from "./utils";

const canvas = document.getElementById("game-canvas");
const gameStateElem = document.getElementById('game-over');
const currentScoreElem = document.getElementById("score");
const highScoreElem = document.getElementById('high-score');
let highScore = getFromLocalStorage(STORAGE_KEYS.HIGH_SCORE);
if (!highScore) {
    highScore = 0;
}


let game = Game.new();

let renderer = new GameRenderer(canvas, game);

const updateHighScore = () => {
    highScoreElem.textContent = highScore;
}

const updateScore = () => {
    const score = game.score();
    currentScoreElem.textContent = score;

    if (score > highScore) {
        highScore = score;
        setLocalStorage(STORAGE_KEYS.HIGH_SCORE, score);
        updateHighScore();
    }
};

updateHighScore();
updateScore();

const updateGameState = () => {
    const state = game.state();
    if (state == GameState.Over) {
        gameStateElem.classList.remove('hidden');
    }
}

const onMoveSuccess = (update) => {
    updateScore();
    animationStart(update);
}

const restartGame = () => {
    game.reset();
    gameStateElem.classList.add('hidden');
    updateHighScore();
};

document.getElementById('try-again-btn').addEventListener('click', restartGame);

addEventListeners(canvas, game, {
    onMoveSuccess,
    afterMove: updateGameState
});

let animatingUpdate = null;

const animationStart = (update) => {
    animatingUpdate = update;
};

let lastTime = 0;
const renderLoop = () => {
    const dt = performance.now() - lastTime;
    renderer.drawGrid();
    renderer.fillGrid(animatingUpdate, dt, () => { animatingUpdate = null; });

    lastTime = performance.now();
    requestAnimationFrame(renderLoop);
}

renderLoop();