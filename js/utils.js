module.exports = {
  decode: bytes => Buffer.from(Buffer.from(bytes).toString("utf8"), "hex"),
  encode: bytes => Buffer.from(Buffer.from(bytes).toString("hex"), "utf8")
};
