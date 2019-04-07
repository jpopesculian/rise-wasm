const fs = require("fs");
const bip39 = require("bip39");
const { bip32, crypto } = require("bitcoinjs-lib");
const epochTime = new Date("2016-05-24T17:00:00.000Z");

console.log("-> Started!");

const mnemonics = [
  "protect surprise middle action clarify behind pistol bounce riot enough hero theme",
  "whale small thumb cage nice minor dune actual radio injury item property",
  "suit cruise furnace attract machine stand that village when spy upon between",
  "love bundle rival charge myself maple obvious purpose another tide december whale",
  "wait topple patch season poem task cruel border liberty just sudden ginger"
];

const wallets = mnemonics
  .map(bip39.mnemonicToSeed)
  .map(seed => bip32.fromSeed(seed));

const signatures = wallets
  .slice(0, 3)
  .map(wallet => wallet.sign(crypto.hash256(wallet.publicKey)));

// mess up signature
// signatures[0][0] = 0xff;

const wasmBin = fs.readFileSync(path.join(__dirname, "multisig.wasm"));

const { verify } = require("../../pkg/rise_wasm");

try {
  verify(wasmBin, {
    time: Math.trunc((new Date().getTime() - epochTime.getTime()) / 1000),
    args: signatures
  });
  console.log("-> Success!");
} catch (err) {
  console.error("-> Failed to verify script!");
}
