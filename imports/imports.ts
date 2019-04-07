import { crypto, ECPair } from "bitcoinjs-lib";

const epochTime = new Date("2016-05-24T17:00:00.000Z");

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

export const verify_multi_sig = (
  sigs: Array<Uint8Array>,
  publicKeys: Array<Uint8Array>
) => {
  console.log(
    `-> verifying: sigs<${sigs.map(sig =>
      Buffer.from(sig).toString("hex")
    )}> pubKeys<${publicKeys.map(publicKey =>
      Buffer.from(publicKey).toString("hex")
    )}>`
  );
  for (const sig of sigs) {
    let verified = false;
    for (const publicKey of publicKeys) {
      verified = verified || verify_sig(sig, publicKey);
      if (verified) {
        break;
      }
    }
    if (!verified) {
      return false;
    }
  }
  return true;
};

export const epoch_time = (
  year: number,
  month: number,
  day: number,
  hours: number,
  minutes: number,
  seconds: number
): BigInt => {
  return BigInt(
    Math.trunc(
      (Date.UTC(year, month, day, hours, minutes, seconds, 0) -
        epochTime.getTime()) /
        1000
    )
  );
};
