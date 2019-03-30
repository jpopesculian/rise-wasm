import * as crypto from "crypto";
import { As } from "type-tagger";
import { RiseV2 } from "dpos-offline";
import * as bech32 from "bech32-buffer";

const INT_32_SIZE = 4;
const MAX_BYTE = 255;
const BYTE_SIZE = 8;

function getInt32BytesLE(x: number): Uint8Array {
  var bytes = new Uint8Array(INT_32_SIZE);
  for (let i = 0; i < INT_32_SIZE; i++) {
    bytes[i] = x & MAX_BYTE;
    x = x >> BYTE_SIZE;
  }
  return bytes;
}

export function jsStringBytesLE(str: string): Uint8Array {
  const length = str.length;
  const lengthDescriptor = Buffer.from(getInt32BytesLE(length));
  const lengthDescriptorSize = lengthDescriptor.length;
  const totalSize = length * 2 + lengthDescriptorSize;
  const bytes = Buffer.from(str, "utf16le");

  const result = Buffer.alloc(totalSize);
  lengthDescriptor.copy(result, 0);
  bytes.copy(result, lengthDescriptorSize);

  return Uint8Array.from(result);
}

export function bytesToMemString(bytes: Uint8Array): string {
  return `\\${Array.from(bytes)
    .map(b => {
      let hexRep = b.toString(16);
      if (b < 16) {
        hexRep = "0" + hexRep;
      }
      return hexRep;
    })
    .join("\\")}`;
}

function hash256(bytes: Buffer): Buffer & As<"publicKey"> {
  return crypto
    .createHash("sha256")
    .update(bytes)
    .digest() as Buffer & As<"publicKey">;
}

export function scriptToAddress(bytes: Buffer): string {
  return RiseV2.calcAddress(hash256(bytes));
}

export function validateAddressScript(address: string, bytes: Buffer): boolean {
  const pubKey = Buffer.from(bech32.decode(address).data);
  const hash = Buffer.concat([Buffer.from([1]), hash256(bytes)]);
  return hash.equals(pubKey);
}
