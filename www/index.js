import * as wasm from "ray_tracer/ray_tracer";
import Worker from './render.worker'

wasm.init_panic_hook();

const width_input = document.getElementById('width');
const height_input = document.getElementById('height');
const samples_input = document.getElementById('samples');
const render_button = document.getElementById('render-button');
const scene_select = document.getElementById('scene');
const progress_bar = document.getElementById('progress');
const loader = document.getElementById('loader');

const scenes = wasm.get_scenes();

for (let i = 0; i < scenes.length; i++) {
    const option = document.createElement("option");
    option.text = scenes[i];
    scene_select.add(option, scene_select[i])
}

scene_select.value = "Cornell Box with Cubes"

const canvas = document.getElementById("render-canvas");
canvas.height = height_input.value;
canvas.width = width_input.value;

width_input.addEventListener("input", event => {
    canvas.width = event.target.value;
    ctx.fillRect(0, 0, canvas.width, canvas.height);
});

height_input.addEventListener("input", event => {
    canvas.height = event.target.value;
    ctx.fillRect(0, 0, canvas.width, canvas.height);
});

const ctx = canvas.getContext('2d');

ctx.fillStyle = 'black';
ctx.fillRect(0, 0, canvas.width, canvas.height);

function render_image(scene, width, height, samples) {
    const array = new Uint8ClampedArray(wasm.render_image_array(scene, width, height, samples));
    return new ImageData(array, canvas.width);
}

function rendering_mode() {
    loader.classList.add("loader");
    width_input.disabled = true;
    height_input.disabled = true;
    samples_input.disabled = true;
    scene_select.disabled = true;
}

function normal_mode() {
    loader.classList.remove("loader");
    width_input.disabled = false;
    height_input.disabled = false;
    samples_input.disabled = false;
    scene_select.disabled = false;
}

let worker = new Worker("render.worker.js");

render_button.addEventListener("click", () => {
    if (render_button.value === "Render") {
        rendering_mode()
        if (window.Worker) {
            //render_button.value = "Stop";
            //render_button.classList.add("bg-danger")
            render_button.disabled = true;
            worker.postMessage([scene_select.value, canvas.width, canvas.height, samples_input.value]);
        } else {
            render_button.disabled = true;
            ctx.putImageData(render_image(scene_select.value, canvas.width, canvas.height, samples_input.value), 0, 0);
            render_button.disabled = false;
            normal_mode()
        }
    } else {
        worker.terminate();
        worker = new Worker("render.worker.js");
        render_button.value = "Render";
        render_button.classList.remove("bg-danger")
        render_button.disabled = false;
        normal_mode()
    }
});

worker.onmessage = (e) => {
    ctx.putImageData(e.data, 0, 0)
    render_button.value = "Render";
    render_button.classList.remove("bg-danger")
    render_button.disabled = false;
    normal_mode()
}

const progress_ctx = progress_bar.getContext('2d');
progress_ctx.fillStyle = '#007bff';
progress_ctx.fillRect(0, 0, 1000, 1)