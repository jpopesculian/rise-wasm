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
