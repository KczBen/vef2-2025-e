import init, { get_texture, trace, init_settings, add_sphere } from './pkg/vef2_2025_e.js';

const fpsDisplay = document.getElementById('fpsDisplay');

let settings;

let WIDTH = 160;
let HEIGHT = 90;
let MAX_SAMPLES = 512;
let MAX_DEPTH = 8;

/* 
* LAYOUT:
* 0 Texture Width
* 1 Texture Height
* 2 Samples Per Pixel
* 3 Max Bounces
* 4 Texture changed
*/

let gl;
let wasmMemory;
let texturePointer;
let textureData;
let i32View;
let timeStart;

async function initWasm() {
    wasmMemory = (await init()).memory;
    settings = (await init_settings()) / 4;
    i32View = new Int32Array(wasmMemory.buffer);
    i32View[settings + 0] = WIDTH;
    i32View[settings + 1] = HEIGHT;
    i32View[settings + 2] = MAX_SAMPLES;
    i32View[settings + 3] = MAX_DEPTH;
    i32View[settings + 4] = 0;
}

WIDTH = window.innerWidth / 2;
HEIGHT = window.innerHeight / 2;

await initWasm();

setupScene();
runTracer();

async function runTracer() {
    timeStart = performance.now();
    trace();
    console.log("Began tracing");

    let samples = 0;

    while (samples < MAX_SAMPLES) {
        if (i32View[settings + 4] === 1) {
            i32View[settings + 4] = 0;
            texturePointer = await get_texture();
            webglSetup();
            samples += 1;
            fpsDisplay.innerHTML = `Samples per second: ${1000 / ((performance.now() - timeStart) / samples)}`;
        }

        else {
            await sleep(1);
        }
    }
}

function setupScene() {
    // Ground
    add_sphere(0, -1000, 0, 1000, 0, 0.5, 0.5, 0.2, 0.0);
    // Glass
    add_sphere(0, 0.5, 0, 1, 2, 0.7, 0.1, 0.1, 1.6);
    add_sphere(0, 0.5, 0, 0.5, 2, 1.0, 1.0, 1.0, 1.0);
    // Metal inside glass
    add_sphere(0, 0.5, 0, 0.25, 1, 0.7, 0.1, 0.1, 0.0);

    // Blue metal (left)
    add_sphere(-1.5, 0.25, -0.5, 0.5, 1, 0.1, 0.3, 0.8, 0.15);

    // Green metal (right)
    add_sphere(0.0, 0.3, 1.7, 0.6, 1, 0.1, 0.5, 0.2, 0.0);

    // Yellow (Lambertian)
    add_sphere(-12.0, 2.0, 0.0, 4.0, 0, 0.4, 0.4, 0.1, 0.0);
}

function sleep(time) {
    return new Promise(resolve => setTimeout(resolve, time));
}

function resizeCanvas() {
    const canvas = document.getElementById("gl-canvas");
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    gl.viewport(0, 0, canvas.width, canvas.height);
}

window.addEventListener('resize', resizeCanvas);

// setup webgl on load
function webglSetup() {
    const canvas = document.getElementById("gl-canvas");

    gl = canvas.getContext('webgl2');
    if (!gl) { alert("WebGL isn't available"); }

    const vShaderCode =
    /* glsl */ `#version 300 es
    precision mediump float;

    out vec2 texcoords;

    void main() {
        vec2 vertices[3] = vec2[3](
            vec2(-1.0, -1.0),
            vec2(3.0, -1.0),
            vec2(-1.0, 3.0)
        );
        gl_Position = vec4(vertices[gl_VertexID], 0.0, 1.0);
        texcoords = 0.5 * gl_Position.xy + vec2(0.5);
    }`

    const fShaderCode =
    /* glsl */ `#version 300 es
    precision mediump float;

    in vec2 texcoords;
    out vec4 fragColor;
    uniform sampler2D uSampler;

    void main() {
        fragColor = vec4(texture(uSampler, texcoords).rgb, 1.0);
    }`

    const vShader = gl.createShader(gl.VERTEX_SHADER);
    const fShader = gl.createShader(gl.FRAGMENT_SHADER);
    const program = gl.createProgram();

    gl.shaderSource(vShader, vShaderCode);
    gl.shaderSource(fShader, fShaderCode);

    gl.compileShader(vShader);
    gl.compileShader(fShader);
    gl.attachShader(program, vShader);
    gl.attachShader(program, fShader);
    gl.linkProgram(program);

    gl.useProgram(program);
    gl.viewport(0, 0, canvas.width, canvas.height);
    gl.clearColor(1.0, 1.0, 1.0, 1.0);
    
    gl.activeTexture(gl.TEXTURE0);
    gl.uniform1i(gl.getUniformLocation(program, 'uSampler'), 0);

    textureData = new Uint8Array(wasmMemory.buffer, texturePointer, WIDTH * HEIGHT * 3);

    const texture = gl.createTexture();
    gl.bindTexture(gl.TEXTURE_2D, texture);
    gl.pixelStorei(gl.UNPACK_ALIGNMENT, 1);
    gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGB, WIDTH, HEIGHT, 0, gl.RGB, gl.UNSIGNED_BYTE, textureData);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);

    resizeCanvas();
    
    render();
}

function render() {
    gl.clear(gl.COLOR_BUFFER_BIT);
    gl.drawArrays(gl.TRIANGLES, 0, 3);

    requestAnimationFrame(render);
}