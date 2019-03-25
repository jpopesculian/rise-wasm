console.log("-> Started!");

const wasmBin = fs.readFileSync(path.join(__dirname, "loop.wasm"));
const { verify } = require("../../pkg/rise_wasm");

const LOOP_NUM = 20; // gas threshold around 50

try {
  verify(wasmBin, {
    name: "main",
    args: [Buffer.from([LOOP_NUM])]
  });
  console.log("-> Success!");
} catch (err) {
  console.error("-> Failed to verify script!");
}
