const fs = require("fs");
const bip39 = require("bip39");
const { bip32, crypto } = require("bitcoinjs-lib");
const epochTime = new Date("2016-05-24T17:00:00.000Z");

console.log("-> Started!");

const seed = bip39.mnemonicToSeed(
  "protect surprise middle action clarify behind pistol bounce riot enough hero theme" /* RIGHT */
  // "suit cruise furnace attract machine stand that village when spy upon between" /* WRONG */
);
const wallet = bip32.fromSeed(seed);
const pubKeyHash = crypto.hash256(wallet.publicKey);
const sig = wallet.sign(pubKeyHash);

/* modify signature */
// sig[0] = 0xff;

const wasmBin = fs.readFileSync(path.join(__dirname, "p2pkh.wasm"));

const { verify } = require("../../pkg/rise_wasm");

try {
  verify(wasmBin, {
    time: Math.trunc((new Date().getTime() - epochTime.getTime()) / 1000),
    args: [sig, wallet.publicKey]
  });
  console.log("-> Success!");
} catch (err) {
  console.error("-> Failed to verify script!");
}
