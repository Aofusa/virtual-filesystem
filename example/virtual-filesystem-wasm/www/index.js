import { Universe } from "virtual-filesystem-wasm";

const pre = document.getElementById("virtual-filesystem-wasm-canvas");
const universe = Universe.new();

const renderLoop = () => {
    pre.textContent = universe.render();
    universe.tick();

    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);

