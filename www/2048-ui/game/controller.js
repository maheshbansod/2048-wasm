import { Direction } from "2048-rs";

const UP_KEY = 'ArrowUp';
const DOWN_KEY = 'ArrowDown';
const LEFT_KEY = 'ArrowLeft';
const RIGHT_KEY = 'ArrowRight';

const ARROW_KEY_DIRECTION_MAPPING = {
    [UP_KEY]: Direction.Up,
    [DOWN_KEY]: Direction.Down,
    [LEFT_KEY]: Direction.Left,
    [RIGHT_KEY]: Direction.Right,
};

export function addEventListeners(canvas, game, callbacks) {

    const doMove = (direction) => {
        try {
            const update = game.play_swipe(direction);
            if (callbacks && callbacks.onMoveSuccess)
                callbacks.onMoveSuccess({ ...update, dt: 0 });
        } catch (e) {
            if (e == "Illegal move") {
                // ignore error
            }
        }
        if (callbacks && callbacks.afterMove) {
            callbacks.afterMove();
        }
    }

    canvas.addEventListener("touchstart", startTouch, false);
    canvas.addEventListener("touchmove", (e) => {
        let d = moveTouch(e);
        if (d !== undefined) {
            doMove(d);
        }
    }, false);

    document.addEventListener('keyup', function (e) {
        const code = e.code;
        if (Object.hasOwn(ARROW_KEY_DIRECTION_MAPPING, code)) {
            doMove(ARROW_KEY_DIRECTION_MAPPING[code]);
        }
    });
};

// Swipe Up / Down / Left / Right
var initialX = null;
var initialY = null;

function startTouch(e) {
    initialX = e.touches[0].clientX;
    initialY = e.touches[0].clientY;
};

function moveTouch(e) {
    if (initialX === null) {
        return;
    }

    if (initialY === null) {
        return;
    }
    let direction;

    var currentX = e.touches[0].clientX;
    var currentY = e.touches[0].clientY;

    var diffX = initialX - currentX;
    var diffY = initialY - currentY;

    if (Math.abs(diffX) > Math.abs(diffY)) {
        // sliding horizontally
        if (diffX > 0) {
            // swiped left
            direction = Direction.Left;
        } else {
            // swiped right
            direction = Direction.Right;
        }
    } else {
        // sliding vertically
        if (diffY > 0) {
            // swiped up
            direction = Direction.Up;
        } else {
            // swiped down
            direction = Direction.Down;
        }
    }

    initialX = null;
    initialY = null;

    e.preventDefault();

    return direction;
};