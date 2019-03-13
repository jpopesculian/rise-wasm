const { crypto, ECPair } = require("bitcoinjs-lib");
const { decode, encode } = require("./utils");

module.exports = {
  add: (x, y) => {
    return x + y;
  },
  minus: (x, y) => {
    return x - y;
  },
  look: bytes => {
    console.log(decode(bytes));
  },
  hash160: bytes => {
    console.log(`hashing: <${decode(bytes).toString("hex")}>`);
    return crypto.hash160(decode(bytes)).toString("hex");
  },
  compare: (bytes1, bytes2) => {
    console.log(
      `comparing: <${decode(bytes1).toString("hex")}> <${decode(
        bytes2
      ).toString("hex")}>`
    );
    for (let i = 0; i < bytes1.length; i++) {
      if (bytes1[i] !== bytes2[i]) {
        return 0;
      }
      return 1;
    }
  },
  verify_sig: (sig, publicKey) => {
    console.log(
      `verifying: sig<${decode(sig).toString("hex")}> pubKey<${decode(
        publicKey
      ).toString("hex")}>`
    );
    const pubKey = decode(publicKey);
    const hash = crypto.hash256(pubKey);
    const result = ECPair.fromPublicKey(pubKey).verify(hash, decode(sig));
    return result ? 1 : 0;
  }
};
