const epochTime = new Date("2016-05-24T17:00:00.000Z");

console.log("-> Started!");

const wasmBin = fs.readFileSync(path.join(__dirname, "loop.wasm"));
const { verify } = require("../../pkg/rise_wasm");

const LOOP_NUM = 20; // gas threshold around 50

try {
  verify(wasmBin, {
    time: Math.trunc((new Date().getTime() - epochTime.getTime()) / 1000),
    args: [Buffer.from([LOOP_NUM])]
  });
  console.log("-> Success!");
} catch (err) {
  console.error("-> Failed to verify script!");
}
