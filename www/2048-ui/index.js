
import { Game } from "2048-rs";
import { STORAGE_KEYS } from "./constants";
import { addEventListeners } from "./game/controller";
import { GameRenderer } from "./game/render-game";
import { getFromLocalStorage, setLocalStorage } from "./utils";

const canvas = document.getElementById("game-canvas");
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

addEventListeners(canvas, game, updateScore);

const renderLoop = () => {
    renderer.drawGrid();
    renderer.fillGrid();

    requestAnimationFrame(renderLoop);
}

renderLoop();