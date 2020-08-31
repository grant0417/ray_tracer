import * as wasm from "ray_tracer/ray_tracer";

wasm.init_panic_hook();

const width_input = document.getElementById('width');
const height_input = document.getElementById('height');
const samples_input = document.getElementById('samples');
const render_button = document.getElementById('render-button');
const scene_select = document.getElementById('scene');
const progress_bar = document.getElementById('progress');

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

function render_image() {
    const array = new Uint8ClampedArray(wasm.render_image_array(scene_select.value, canvas.width, canvas.height, samples_input.value));
    const image = new ImageData(array, canvas.width);
    ctx.putImageData(image, 0, 0);
}

render_button.addEventListener("click", () => {
    render_button.disabled = true;
    render_image();
    render_button.disabled = false;
});

const progress_ctx = progress_bar.getContext('2d');
progress_ctx.fillStyle = '#007bff';
progress_ctx.fillRect(0, 0, 1000, 1)