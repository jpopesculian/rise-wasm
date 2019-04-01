import { crypto, ECPair } from "bitcoinjs-lib";

export const hash160 = (bytes: Uint8Array) => {
  console.log(`-> hashing: <${Buffer.from(bytes).toString("hex")}>`);
  return crypto.hash160(Buffer.from(bytes));
};

export const compare = (left: Uint8Array, right: Uint8Array) => {
  console.log(
    `-> comparing: <${Buffer.from(left).toString("hex")}> <${Buffer.from(
      right
    ).toString("hex")}>`
  );
  return Buffer.from(left).equals(Buffer.from(right));
};

export const verify_sig = (sig: Uint8Array, publicKey: Uint8Array) => {
  console.log(
    `-> verifying: sig<${Buffer.from(sig).toString(
      "hex"
    )}> pubKey<${Buffer.from(publicKey).toString("hex")}>`
  );
  const pubKey = Buffer.from(publicKey);
  const hash = crypto.hash256(pubKey);
  return ECPair.fromPublicKey(pubKey).verify(hash, Buffer.from(sig));
};
