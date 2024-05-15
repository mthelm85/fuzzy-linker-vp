const assert = require('assert');
const wasm = require('../pkg/fuzzy_linker_vp.js');

async function run() {
    const wasmModule = await wasm; // Await the wasm module

    const vec1 = ["bla", "blub", "asdf", ":assd", "ast", "baube"];
    const vec2 = ["barb", "ass"];
    const radius = 2;

    const result = wasmModule.fuzzy_join(vec1, vec2, radius);
    console.log('Result:', result);
    const expectedResult = [{ id: 0, connected_nodes: [5,1] }, { id: 1, connected_nodes: [3,2,4] }];
    console.log('Expected Result:', expectedResult);
    assert.deepStrictEqual(result, expectedResult, 'The result does not match the expected result');
}

run();