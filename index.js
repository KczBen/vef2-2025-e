import init, { get_texture, trace, init_settings, add_sphere } from './pkg/vef2_2025_e.js';

const fpsDisplay = document.getElementById('fpsDisplay');

let settings;

// Default
let WIDTH = 160;
let HEIGHT = 90;
let MAX_SAMPLES = 256;
let MAX_DEPTH = 8;

// Should be this if we can get the screen size
WIDTH = window.innerWidth / 2;
HEIGHT = window.innerHeight / 2;
const MOVEWIDTH = WIDTH / 4;
const MOVEHEIGHT = HEIGHT / 4;

/* 
* LAYOUT:
* 0 Texture Width
* 1 Texture Height
* 2 Samples Per Pixel
* 3 Max Bounces
* 4 Origin X
* 5 Origin Y
* 6 Origin Z
* 7 LookAt X
* 8 LookAt Y
* 9 LookAt Z
* 10 Texture changed
* 11 User input, reset rendering
* 12 Busy
*/

let gl;
let wasmMemory;
let texturePointer;
let textureData;
let i32View;
let f32View;
let timeStart;

let originX = -2.0;
let originY = 2.0;
let originZ = 1.0;

let lookAtX = 0.0;
let lookAtY = 0.0;
let lookAtZ = 0.0;

let orbit = false;
let pan = false;
let prevMouseX = 0;
let prevMouseY = 0;

async function initWasm() {
    wasmMemory = (await init()).memory;
    settings = (await init_settings()) / 4;
    let length = 13;
    i32View = new Int32Array(wasmMemory.buffer);
    f32View = new Float32Array(wasmMemory.buffer);
    i32View[settings + 0] = WIDTH;
    i32View[settings + 1] = HEIGHT;
    i32View[settings + 2] = MAX_SAMPLES;
    i32View[settings + 3] = MAX_DEPTH;
    i32View[settings + 10] = 0;
    i32View[settings + 11] = 0;
    i32View[settings + 12] = 0;

    f32View[settings + 4] = originX;
    f32View[settings + 5] = originY;
    f32View[settings + 6] = originZ;
    f32View[settings + 7] = lookAtX;
    f32View[settings + 8] = lookAtY;
    f32View[settings + 9] = lookAtZ;
}

await initWasm();
setupScene();
runTracer();

async function runTracer() {
    // wait for path tracer to stop
    while (i32View[settings + 12] === 1) {
        await sleep(1);
    }

    
    i32View[settings + 11] = 0;
    timeStart = performance.now();
    trace();
    
    texturePointer = await get_texture();
    if (orbit) {
        webglSetup(MOVEWIDTH, MOVEHEIGHT, 1);
    }
    else {
        webglSetup(WIDTH, HEIGHT, 0);
    }

    let samples = 0;

    while (samples < MAX_SAMPLES) {
        if (i32View[settings + 10] === 1) {
            i32View[settings + 10] = 0;
            texturePointer = await get_texture();
            if (orbit | pan) {
                webglSetup(MOVEWIDTH, MOVEHEIGHT, 1);
            }
            else {
                webglSetup(WIDTH, HEIGHT, 0);
            }
            samples += 1;
            fpsDisplay.innerHTML = `Samples per second: ${1000 / ((performance.now() - timeStart) / samples)}`;
        }

        else {
            await sleep(1);
        }
    }

    i32View[settings + 12] = 0;
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

window.addEventListener('wheel', function(event) {
    let delta = event.deltaY;
    
    let vecX = lookAtX - originX;
    let vecY = lookAtY - originY;
    let vecZ = lookAtZ - originZ;
    
    let length = Math.sqrt(vecX*vecX + vecY*vecY + vecZ*vecZ);
    
    if (length === 0) return;
    
    let normX = vecX / length;
    let normY = vecY / length;
    let normZ = vecZ / length;
    
    let step = 0.001 * delta;
    originX -= normX * step;
    originY -= normY * step;
    originZ -= normZ * step;

    f32View[settings + 4] = originX;
    f32View[settings + 5] = originY;
    f32View[settings + 6] = originZ;

    i32View[settings + 11] = 1;

    runTracer();
});

document.addEventListener('contextmenu', (e) => e.preventDefault());

document.addEventListener('click', (e) => e.preventDefault());

document.addEventListener('mousedown', (e) => {
    prevMouseX = e.clientX;
    prevMouseY = e.clientY;
    i32View[settings + 0] = MOVEWIDTH;
    i32View[settings + 1] = MOVEHEIGHT;
    i32View[settings + 2] = 1;

    // Pan
    if (e.button === 0) {
        pan = true;
    }
    
    // Orbit
    if (e.button === 2) {
        orbit = true;
    }
});

document.addEventListener('mouseup', (e) => {
    if (e.button === 0) {
        pan = false;
    }
    
    if (e.button === 2) {
        orbit = false;
    }

    i32View[settings + 0] = WIDTH;
    i32View[settings + 1] = HEIGHT;
    i32View[settings + 2] = MAX_SAMPLES;
    i32View[settings + 11] = 1;

    runTracer();
});
  
document.addEventListener('mousemove', (e) => {
    if (orbit) {
        orbitCamera(e);
    }

    else if (pan) {
        panCamera(e)
    }
    
});

function orbitCamera(e) {
    const deltaX = e.clientX - prevMouseX;
    const deltaY = e.clientY - prevMouseY;
    prevMouseX = e.clientX;
    prevMouseY = e.clientY;

    const offsetX = originX - lookAtX;
    const offsetY = originY - lookAtY;
    const offsetZ = originZ - lookAtZ;

    const radius = Math.sqrt(offsetX * offsetX + offsetY * offsetY + offsetZ * offsetZ);
    let theta = Math.atan2(offsetZ, offsetX);
    let phi = Math.acos(offsetY / radius);

    const sensitivity = 0.005;

    theta += deltaX * sensitivity;
    phi -= deltaY * sensitivity;

    const epsilon = 0.1;
    phi = Math.max(epsilon, Math.min(Math.PI - epsilon, phi));

    originX = lookAtX + radius * Math.sin(phi) * Math.cos(theta);
    originY = lookAtY + radius * Math.cos(phi);
    originZ = lookAtZ + radius * Math.sin(phi) * Math.sin(theta);

    f32View[settings + 4] = originX;
    f32View[settings + 5] = originY;
    f32View[settings + 6] = originZ;

    i32View[settings + 11] = 1;

    runTracer();
}

function panCamera(e) {
    const deltaX = e.clientX - prevMouseX;
    const deltaY = e.clientY - prevMouseY;
    prevMouseX = e.clientX;
    prevMouseY = e.clientY;

    const panSpeed = 0.01;

    const viewDir = {
        x: lookAtX - originX,
        y: lookAtY - originY,
        z: lookAtZ - originZ
    };

    let right = cross(viewDir, { x: 0, y: 1, z: 0 });
    right = normalize(right);

    let up = cross(right, viewDir);
    up = normalize(up);

    const offset = {
        x: right.x * (-deltaX * panSpeed) + up.x * (deltaY * panSpeed),
        y: right.y * (-deltaX * panSpeed) + up.y * (deltaY * panSpeed),
        z: right.z * (-deltaX * panSpeed) + up.z * (deltaY * panSpeed)
    };

    originX += offset.x;
    originY += offset.y;
    originZ += offset.z;

    lookAtX += offset.x;
    lookAtY += offset.y;
    lookAtZ += offset.z;

    f32View[settings + 4] = originX;
    f32View[settings + 5] = originY;
    f32View[settings + 6] = originZ;
    f32View[settings + 7] = lookAtX;
    f32View[settings + 8] = lookAtY;
    f32View[settings + 9] = lookAtZ;

    i32View[settings + 11] = 1;

    runTracer();
}

function cross(v1, v2) {
    return {
        x: v1.y * v2.z - v1.z * v2.y,
        y: v1.z * v2.x - v1.x * v2.z,
        z: v1.x * v2.y - v1.y * v2.x
    };
}

function normalize(v) {
    const len = Math.hypot(v.x, v.y, v.z);
    return len > 0 ? { x: v.x / len, y: v.y / len, z: v.z / len } : { x: 0, y: 0, z: 0 };
}

window.addEventListener('resize', resizeCanvas);

// setup webgl on load
function webglSetup(width, height, mode) {
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

    textureData = new Uint8Array(wasmMemory.buffer, texturePointer, width * height * 3);

    const texture = gl.createTexture();
    gl.bindTexture(gl.TEXTURE_2D, texture);
    gl.pixelStorei(gl.UNPACK_ALIGNMENT, 1);
    gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGB, width, height, 0, gl.RGB, gl.UNSIGNED_BYTE, textureData);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
    if (mode === 0) {
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
    }

    else {
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);
    }

    resizeCanvas();
    
    render();
}

function render() {
    gl.clear(gl.COLOR_BUFFER_BIT);
    gl.drawArrays(gl.TRIANGLES, 0, 3);

    requestAnimationFrame(render);
}