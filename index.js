import init, { get_texture, trace, init_settings } from './pkg/vef2_2025_e.js';

let settings;

let WIDTH = 1280;
let HEIGHT = 720;

/* 
* LAYOUT:
* 0 Texture Width
* 1 Texture Height
* 2 Samples Per Pixel
* 3 Max Bounces
*/

let gl;
let wasmMemory;
let texturePointer;
let textureData;
let i32View;

async function runWasm() {
    wasmMemory = (await init()).memory;
    settings = (await init_settings()) / 4;
    i32View = new Int32Array(wasmMemory.buffer);
    i32View[settings + 0] = WIDTH;
    i32View[settings + 1] = HEIGHT;
    i32View[settings + 2] = 1;
    i32View[settings + 3] = 4;
    trace();
    texturePointer = await get_texture();
    webglSetup();
}

runWasm();

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