(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "../pkg sync recursive":
/*!*******************!*\
  !*** ../pkg sync ***!
  \*******************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("function webpackEmptyContext(req) {\n\tvar e = new Error(\"Cannot find module '\" + req + \"'\");\n\te.code = 'MODULE_NOT_FOUND';\n\tthrow e;\n}\nwebpackEmptyContext.keys = function() { return []; };\nwebpackEmptyContext.resolve = webpackEmptyContext;\nmodule.exports = webpackEmptyContext;\nwebpackEmptyContext.id = \"../pkg sync recursive\";\n\n//# sourceURL=webpack:///../pkg_sync?");

/***/ }),

/***/ "../pkg/ray_tracer.js":
/*!****************************!*\
  !*** ../pkg/ray_tracer.js ***!
  \****************************/
/*! exports provided: render_image_array, get_scenes, init_panic_hook, __wbindgen_string_new, __wbg_new_59cb74e423758ede, __wbg_stack_558ba5917b466edd, __wbg_error_4bb6c2a97407129a, __wbindgen_object_drop_ref, __wbg_getRandomValues_f5e14ab7ac8e995d, __wbg_randomFillSync_d5bd2d655fdf256a, __wbg_self_1b7a39e3a92c949c, __wbg_require_604837428532a733, __wbg_crypto_968f1772287e2df0, __wbindgen_is_undefined, __wbg_getRandomValues_a3d34b4fee3c2869 */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _ray_tracer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./ray_tracer_bg.wasm */ \"../pkg/ray_tracer_bg.wasm\");\n/* harmony import */ var _ray_tracer_bg_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./ray_tracer_bg.js */ \"../pkg/ray_tracer_bg.js\");\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"render_image_array\", function() { return _ray_tracer_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"render_image_array\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"get_scenes\", function() { return _ray_tracer_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"get_scenes\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"init_panic_hook\", function() { return _ray_tracer_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"init_panic_hook\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_string_new\", function() { return _ray_tracer_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_string_new\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_new_59cb74e423758ede\", function() { return _ray_tracer_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_new_59cb74e423758ede\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_stack_558ba5917b466edd\", function() { return _ray_tracer_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_stack_558ba5917b466edd\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_error_4bb6c2a97407129a\", function() { return _ray_tracer_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_error_4bb6c2a97407129a\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_drop_ref\", function() { return _ray_tracer_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_object_drop_ref\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_getRandomValues_f5e14ab7ac8e995d\", function() { return _ray_tracer_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_getRandomValues_f5e14ab7ac8e995d\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_randomFillSync_d5bd2d655fdf256a\", function() { return _ray_tracer_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_randomFillSync_d5bd2d655fdf256a\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_self_1b7a39e3a92c949c\", function() { return _ray_tracer_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_self_1b7a39e3a92c949c\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_require_604837428532a733\", function() { return _ray_tracer_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_require_604837428532a733\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_crypto_968f1772287e2df0\", function() { return _ray_tracer_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_crypto_968f1772287e2df0\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_is_undefined\", function() { return _ray_tracer_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_is_undefined\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_getRandomValues_a3d34b4fee3c2869\", function() { return _ray_tracer_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_getRandomValues_a3d34b4fee3c2869\"]; });\n\n\n\n\n//# sourceURL=webpack:///../pkg/ray_tracer.js?");

/***/ }),

/***/ "../pkg/ray_tracer_bg.js":
/*!*******************************!*\
  !*** ../pkg/ray_tracer_bg.js ***!
  \*******************************/
/*! exports provided: render_image_array, get_scenes, init_panic_hook, __wbindgen_string_new, __wbg_new_59cb74e423758ede, __wbg_stack_558ba5917b466edd, __wbg_error_4bb6c2a97407129a, __wbindgen_object_drop_ref, __wbg_getRandomValues_f5e14ab7ac8e995d, __wbg_randomFillSync_d5bd2d655fdf256a, __wbg_self_1b7a39e3a92c949c, __wbg_require_604837428532a733, __wbg_crypto_968f1772287e2df0, __wbindgen_is_undefined, __wbg_getRandomValues_a3d34b4fee3c2869 */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* WEBPACK VAR INJECTION */(function(module) {/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"render_image_array\", function() { return render_image_array; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"get_scenes\", function() { return get_scenes; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"init_panic_hook\", function() { return init_panic_hook; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_string_new\", function() { return __wbindgen_string_new; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_new_59cb74e423758ede\", function() { return __wbg_new_59cb74e423758ede; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_stack_558ba5917b466edd\", function() { return __wbg_stack_558ba5917b466edd; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_error_4bb6c2a97407129a\", function() { return __wbg_error_4bb6c2a97407129a; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_drop_ref\", function() { return __wbindgen_object_drop_ref; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_getRandomValues_f5e14ab7ac8e995d\", function() { return __wbg_getRandomValues_f5e14ab7ac8e995d; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_randomFillSync_d5bd2d655fdf256a\", function() { return __wbg_randomFillSync_d5bd2d655fdf256a; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_self_1b7a39e3a92c949c\", function() { return __wbg_self_1b7a39e3a92c949c; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_require_604837428532a733\", function() { return __wbg_require_604837428532a733; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_crypto_968f1772287e2df0\", function() { return __wbg_crypto_968f1772287e2df0; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_is_undefined\", function() { return __wbindgen_is_undefined; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_getRandomValues_a3d34b4fee3c2869\", function() { return __wbg_getRandomValues_a3d34b4fee3c2869; });\n/* harmony import */ var _ray_tracer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./ray_tracer_bg.wasm */ \"../pkg/ray_tracer_bg.wasm\");\n\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nlet cachegetUint8Memory0 = null;\nfunction getUint8Memory0() {\n    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _ray_tracer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory0 = new Uint8Array(_ray_tracer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory0;\n}\n\nfunction getStringFromWasm0(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n\nconst heap = new Array(32).fill(undefined);\n\nheap.push(undefined, null, true, false);\n\nlet heap_next = heap.length;\n\nfunction addHeapObject(obj) {\n    if (heap_next === heap.length) heap.push(heap.length + 1);\n    const idx = heap_next;\n    heap_next = heap[idx];\n\n    heap[idx] = obj;\n    return idx;\n}\n\nfunction getObject(idx) { return heap[idx]; }\n\nfunction dropObject(idx) {\n    if (idx < 36) return;\n    heap[idx] = heap_next;\n    heap_next = idx;\n}\n\nfunction takeObject(idx) {\n    const ret = getObject(idx);\n    dropObject(idx);\n    return ret;\n}\n\nlet WASM_VECTOR_LEN = 0;\n\nconst lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;\n\nlet cachedTextEncoder = new lTextEncoder('utf-8');\n\nconst encodeString = (typeof cachedTextEncoder.encodeInto === 'function'\n    ? function (arg, view) {\n    return cachedTextEncoder.encodeInto(arg, view);\n}\n    : function (arg, view) {\n    const buf = cachedTextEncoder.encode(arg);\n    view.set(buf);\n    return {\n        read: arg.length,\n        written: buf.length\n    };\n});\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n\n    if (realloc === undefined) {\n        const buf = cachedTextEncoder.encode(arg);\n        const ptr = malloc(buf.length);\n        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);\n        WASM_VECTOR_LEN = buf.length;\n        return ptr;\n    }\n\n    let len = arg.length;\n    let ptr = malloc(len);\n\n    const mem = getUint8Memory0();\n\n    let offset = 0;\n\n    for (; offset < len; offset++) {\n        const code = arg.charCodeAt(offset);\n        if (code > 0x7F) break;\n        mem[ptr + offset] = code;\n    }\n\n    if (offset !== len) {\n        if (offset !== 0) {\n            arg = arg.slice(offset);\n        }\n        ptr = realloc(ptr, len, len = offset + arg.length * 3);\n        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);\n        const ret = encodeString(arg, view);\n\n        offset += ret.written;\n    }\n\n    WASM_VECTOR_LEN = offset;\n    return ptr;\n}\n\nlet cachegetInt32Memory0 = null;\nfunction getInt32Memory0() {\n    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== _ray_tracer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetInt32Memory0 = new Int32Array(_ray_tracer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetInt32Memory0;\n}\n\nfunction getArrayU8FromWasm0(ptr, len) {\n    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);\n}\n/**\n* @param {string} scene\n* @param {number} width\n* @param {number} height\n* @param {number} samples\n* @returns {Uint8Array}\n*/\nfunction render_image_array(scene, width, height, samples) {\n    try {\n        const retptr = _ray_tracer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_0\"].value - 16;\n        _ray_tracer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_0\"].value = retptr;\n        var ptr0 = passStringToWasm0(scene, _ray_tracer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _ray_tracer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n        var len0 = WASM_VECTOR_LEN;\n        _ray_tracer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"render_image_array\"](retptr, ptr0, len0, width, height, samples);\n        var r0 = getInt32Memory0()[retptr / 4 + 0];\n        var r1 = getInt32Memory0()[retptr / 4 + 1];\n        var v1 = getArrayU8FromWasm0(r0, r1).slice();\n        _ray_tracer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](r0, r1 * 1);\n        return v1;\n    } finally {\n        _ray_tracer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_0\"].value += 16;\n    }\n}\n\nlet cachegetUint32Memory0 = null;\nfunction getUint32Memory0() {\n    if (cachegetUint32Memory0 === null || cachegetUint32Memory0.buffer !== _ray_tracer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint32Memory0 = new Uint32Array(_ray_tracer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint32Memory0;\n}\n\nfunction getArrayJsValueFromWasm0(ptr, len) {\n    const mem = getUint32Memory0();\n    const slice = mem.subarray(ptr / 4, ptr / 4 + len);\n    const result = [];\n    for (let i = 0; i < slice.length; i++) {\n        result.push(takeObject(slice[i]));\n    }\n    return result;\n}\n/**\n* @returns {any[]}\n*/\nfunction get_scenes() {\n    try {\n        const retptr = _ray_tracer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_0\"].value - 16;\n        _ray_tracer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_0\"].value = retptr;\n        _ray_tracer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"get_scenes\"](retptr);\n        var r0 = getInt32Memory0()[retptr / 4 + 0];\n        var r1 = getInt32Memory0()[retptr / 4 + 1];\n        var v0 = getArrayJsValueFromWasm0(r0, r1).slice();\n        _ray_tracer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](r0, r1 * 4);\n        return v0;\n    } finally {\n        _ray_tracer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_0\"].value += 16;\n    }\n}\n\n/**\n*/\nfunction init_panic_hook() {\n    _ray_tracer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"init_panic_hook\"]();\n}\n\nfunction handleError(f) {\n    return function () {\n        try {\n            return f.apply(this, arguments);\n\n        } catch (e) {\n            _ray_tracer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_exn_store\"](addHeapObject(e));\n        }\n    };\n}\n\nconst __wbindgen_string_new = function(arg0, arg1) {\n    var ret = getStringFromWasm0(arg0, arg1);\n    return addHeapObject(ret);\n};\n\nconst __wbg_new_59cb74e423758ede = function() {\n    var ret = new Error();\n    return addHeapObject(ret);\n};\n\nconst __wbg_stack_558ba5917b466edd = function(arg0, arg1) {\n    var ret = getObject(arg1).stack;\n    var ptr0 = passStringToWasm0(ret, _ray_tracer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _ray_tracer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n    var len0 = WASM_VECTOR_LEN;\n    getInt32Memory0()[arg0 / 4 + 1] = len0;\n    getInt32Memory0()[arg0 / 4 + 0] = ptr0;\n};\n\nconst __wbg_error_4bb6c2a97407129a = function(arg0, arg1) {\n    try {\n        console.error(getStringFromWasm0(arg0, arg1));\n    } finally {\n        _ray_tracer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](arg0, arg1);\n    }\n};\n\nconst __wbindgen_object_drop_ref = function(arg0) {\n    takeObject(arg0);\n};\n\nconst __wbg_getRandomValues_f5e14ab7ac8e995d = function(arg0, arg1, arg2) {\n    getObject(arg0).getRandomValues(getArrayU8FromWasm0(arg1, arg2));\n};\n\nconst __wbg_randomFillSync_d5bd2d655fdf256a = function(arg0, arg1, arg2) {\n    getObject(arg0).randomFillSync(getArrayU8FromWasm0(arg1, arg2));\n};\n\nconst __wbg_self_1b7a39e3a92c949c = handleError(function() {\n    var ret = self.self;\n    return addHeapObject(ret);\n});\n\nconst __wbg_require_604837428532a733 = function(arg0, arg1) {\n    var ret = __webpack_require__(\"../pkg sync recursive\")(getStringFromWasm0(arg0, arg1));\n    return addHeapObject(ret);\n};\n\nconst __wbg_crypto_968f1772287e2df0 = function(arg0) {\n    var ret = getObject(arg0).crypto;\n    return addHeapObject(ret);\n};\n\nconst __wbindgen_is_undefined = function(arg0) {\n    var ret = getObject(arg0) === undefined;\n    return ret;\n};\n\nconst __wbg_getRandomValues_a3d34b4fee3c2869 = function(arg0) {\n    var ret = getObject(arg0).getRandomValues;\n    return addHeapObject(ret);\n};\n\n\n/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(/*! ./../www/node_modules/webpack/buildin/harmony-module.js */ \"./node_modules/webpack/buildin/harmony-module.js\")(module)))\n\n//# sourceURL=webpack:///../pkg/ray_tracer_bg.js?");

/***/ }),

/***/ "../pkg/ray_tracer_bg.wasm":
/*!*********************************!*\
  !*** ../pkg/ray_tracer_bg.wasm ***!
  \*********************************/
/*! exports provided: memory, render_image_array, get_scenes, init_panic_hook, __wbindgen_export_0, __wbindgen_malloc, __wbindgen_realloc, __wbindgen_free, __wbindgen_exn_store */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./ray_tracer_bg.js */ \"../pkg/ray_tracer_bg.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../pkg/ray_tracer_bg.wasm?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var ray_tracer_ray_tracer__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ray_tracer/ray_tracer */ \"../pkg/ray_tracer.js\");\n/* harmony import */ var _render_worker__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./render.worker */ \"./render.worker.js\");\n\n\n\nray_tracer_ray_tracer__WEBPACK_IMPORTED_MODULE_0__[\"init_panic_hook\"]();\n\nconst width_input = document.getElementById('width');\nconst height_input = document.getElementById('height');\nconst samples_input = document.getElementById('samples');\nconst render_button = document.getElementById('render-button');\nconst scene_select = document.getElementById('scene');\nconst progress_bar = document.getElementById('progress');\nconst loader = document.getElementById('loader');\n\nconst scenes = ray_tracer_ray_tracer__WEBPACK_IMPORTED_MODULE_0__[\"get_scenes\"]();\n\nfor (let i = 0; i < scenes.length; i++) {\n    const option = document.createElement(\"option\");\n    option.text = scenes[i];\n    scene_select.add(option, scene_select[i])\n}\n\nscene_select.value = \"Cornell Box with Cubes\"\n\nconst canvas = document.getElementById(\"render-canvas\");\ncanvas.height = height_input.value;\ncanvas.width = width_input.value;\n\nwidth_input.addEventListener(\"input\", event => {\n    canvas.width = event.target.value;\n    ctx.fillRect(0, 0, canvas.width, canvas.height);\n});\n\nheight_input.addEventListener(\"input\", event => {\n    canvas.height = event.target.value;\n    ctx.fillRect(0, 0, canvas.width, canvas.height);\n});\n\nconst ctx = canvas.getContext('2d');\n\nctx.fillStyle = 'black';\nctx.fillRect(0, 0, canvas.width, canvas.height);\n\nfunction render_image(scene, width, height, samples) {\n    const array = new Uint8ClampedArray(ray_tracer_ray_tracer__WEBPACK_IMPORTED_MODULE_0__[\"render_image_array\"](scene, width, height, samples));\n    return new ImageData(array, canvas.width);\n}\n\nfunction rendering_mode() {\n    loader.classList.add(\"loader\");\n    width_input.disabled = true;\n    height_input.disabled = true;\n    samples_input.disabled = true;\n    scene_select.disabled = true;\n}\n\nfunction normal_mode() {\n    loader.classList.remove(\"loader\");\n    width_input.disabled = false;\n    height_input.disabled = false;\n    samples_input.disabled = false;\n    scene_select.disabled = false;\n}\n\nlet worker = new _render_worker__WEBPACK_IMPORTED_MODULE_1__[\"default\"](\"render.worker.js\");\n\nrender_button.addEventListener(\"click\", () => {\n    if (render_button.value === \"Render\") {\n        rendering_mode()\n        if (window.Worker) {\n            //render_button.value = \"Stop\";\n            //render_button.classList.add(\"bg-danger\")\n            render_button.disabled = true;\n            worker.postMessage([scene_select.value, canvas.width, canvas.height, samples_input.value]);\n        } else {\n            render_button.disabled = true;\n            ctx.putImageData(render_image(scene_select.value, canvas.width, canvas.height, samples_input.value), 0, 0);\n            render_button.disabled = false;\n            normal_mode()\n        }\n    } else {\n        worker.terminate();\n        worker = new _render_worker__WEBPACK_IMPORTED_MODULE_1__[\"default\"](\"render.worker.js\");\n        render_button.value = \"Render\";\n        render_button.classList.remove(\"bg-danger\")\n        render_button.disabled = false;\n        normal_mode()\n    }\n});\n\nworker.onmessage = (e) => {\n    ctx.putImageData(e.data, 0, 0)\n    render_button.value = \"Render\";\n    render_button.classList.remove(\"bg-danger\")\n    render_button.disabled = false;\n    normal_mode()\n}\n\nconst progress_ctx = progress_bar.getContext('2d');\nprogress_ctx.fillStyle = '#007bff';\nprogress_ctx.fillRect(0, 0, 1000, 1)\n\n//# sourceURL=webpack:///./index.js?");

/***/ }),

/***/ "./node_modules/webpack/buildin/harmony-module.js":
/*!*******************************************!*\
  !*** (webpack)/buildin/harmony-module.js ***!
  \*******************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = function(originalModule) {\n\tif (!originalModule.webpackPolyfill) {\n\t\tvar module = Object.create(originalModule);\n\t\t// module.parent = undefined by default\n\t\tif (!module.children) module.children = [];\n\t\tObject.defineProperty(module, \"loaded\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.l;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"id\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.i;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"exports\", {\n\t\t\tenumerable: true\n\t\t});\n\t\tmodule.webpackPolyfill = 1;\n\t}\n\treturn module;\n};\n\n\n//# sourceURL=webpack:///(webpack)/buildin/harmony-module.js?");

/***/ }),

/***/ "./render.worker.js":
/*!**************************!*\
  !*** ./render.worker.js ***!
  \**************************/
/*! exports provided: default */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony default export */ __webpack_exports__[\"default\"] = (function() {\n  return new Worker(__webpack_require__.p + \"bootstrap.worker.js\");\n});\n\n\n//# sourceURL=webpack:///./render.worker.js?");

/***/ })

}]);