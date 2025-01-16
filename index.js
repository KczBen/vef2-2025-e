const wasmModule = require('./pkg/vef2_2025_e.js');

async function run() {
    //await wasmModule.default();
    const message = wasmModule.greet("Node.js");
    console.log(message);
}

run().catch(console.error);