import * as wasm from "wasm-tetris";

const DOWN_KEY_THROTTLE_TIME = 200;
const FALL_TIME = 1000;

let downInputCallback = null;

const renderLoop = (target) => {
  target.textContent = wasm.render_frame();
  requestAnimationFrame(() => renderLoop(target));
}

const initialize = () => {
  const tetris = document.getElementById("tetris");
  
  //display initial game state
  tetris.textContent = wasm.render_frame();

  //start running the game
  requestAnimationFrame(() => renderLoop(tetris));
  setInterval(() => {
    if (!downInputCallback) { wasm.update_state(); }
  }, FALL_TIME);
}

//controls
document.addEventListener("keydown", (e) => {
  switch (e.key) {
  case "ArrowDown":
    wasm.update_state();
    if (!downInputCallback) {
      downInputCallback = setInterval(() => {
	wasm.update_state();
      }, DOWN_KEY_THROTTLE_TIME);
    }
    break;
  case "ArrowLeft":
    wasm.left_input();
    break;
  case "ArrowRight":
    wasm.right_input();
    break;
  case "z":
    wasm.left_rotate_input();
    break;
  case "x":
    wasm.right_rotate_input();
    break;
  }
});

document.addEventListener("keyup", (e) => {
  if (e.key === "ArrowDown" && downInputCallback) {
    clearInterval(downInputCallback);
    downInputCallback = null;
  }
});

// initial load
if (document.readyState === 'complete') {
  initialize();
} else {
  document.addEventListener("DOMContentLoaded", initialize);
}
