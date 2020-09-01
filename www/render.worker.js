onmessage = async function(e) {
    const wasm = await import('ray_tracer/ray_tracer')
    const array = new Uint8ClampedArray(wasm.render_image_array(e.data[0], e.data[1], e.data[2], e.data[3]));
    const image = new ImageData(array, e.data[1]);
    postMessage(image);
}
