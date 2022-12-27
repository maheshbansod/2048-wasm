
import { Game } from "2048-rs";
import { GameRenderer } from "./game/render-game";

const canvas = document.getElementById("game-canvas");
let game = Game.new();

let renderer = new GameRenderer(canvas, game);

const renderLoop = () => {
    renderer.drawGrid();
    renderer.fillGrid();

    requestAnimationFrame(renderLoop);
}

renderLoop();