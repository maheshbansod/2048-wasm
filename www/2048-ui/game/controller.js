import { Direction } from "2048-rs";

const UP_KEY = 'ArrowUp';
const DOWN_KEY = 'ArrowDown';
const LEFT_KEY = 'ArrowLeft';
const RIGHT_KEY = 'ArrowRight';

export function addEventListeners(game) {
    document.addEventListener('keyup', function (e) {
        const code = e.code;
        try {
            switch (code) {
                case UP_KEY: {
                    game.play_swipe(Direction.Up);
                    break;
                }
                case LEFT_KEY: {
                    game.play_swipe(Direction.Left);
                    break;
                }
                case RIGHT_KEY: {
                    game.play_swipe(Direction.Right);
                    break;
                }
                case DOWN_KEY: {
                    game.play_swipe(Direction.Down);
                    break;
                }
            }
        } catch (e) {
            if (e === "Illegal move") {
                // illegal move
            } else {
                console.error(e);
            }
        }
    });
}