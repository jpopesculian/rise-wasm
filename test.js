const fs = require("fs");
const bip39 = require("bip39");
const { bip32, crypto } = require("bitcoinjs-lib");

const seed = bip39.mnemonicToSeed(
  "protect surprise middle action clarify behind pistol bounce riot enough hero theme" /* RIGHT */
  // "suit cruise furnace attract machine stand that village when spy upon between" /* WRONG */
);
const wallet = bip32.fromSeed(seed);
const pubKeyHash = crypto.hash256(wallet.publicKey);
const sig = wallet.sign(pubKeyHash);

/* modify signature */
// sig[0] = 0xff;

const wasmBin = fs.readFileSync("./test.wasm");
const args = Buffer.from(
  sig.toString("hex") + wallet.publicKey.toString("hex"),
  "utf8"
);

const { verify } = require("./pkg/rise_wasm");

// try {
verify(wasmBin, {
  args: [sig.toString("hex"), wallet.publicKey.toString("hex")]
});
// console.log("Success!");
// } catch (err) {
// console.error("Failed to verify script");
// }
