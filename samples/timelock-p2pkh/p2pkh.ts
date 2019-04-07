import "../../assembly/allocators/buddy";
import {
  compare,
  NOW,
  epochTime,
  hash160,
  hexDecode,
  loadUint8Array,
  verifySig
} from "../../assembly/helpers";

const localHash = "2ef1eacc8cad29a27a54312731d6f3624e013e46";

function checkSig(): void {
  let signature = loadUint8Array(0);
  let pubKey = loadUint8Array(1);
  let pubHash = hash160(pubKey);
  let localHashArr = hexDecode(localHash);

  if (!compare(localHashArr, pubHash)) {
    throw new Error("Public keys don't match");
  }
  if (!verifySig(signature, pubKey)) {
    throw new Error("Signature doesn't match");
  }
}

function start(): void {
  if (NOW > epochTime(2019, 3, 1, 0, 0, 0)) {
    checkSig();
  } else {
    throw new Error("Funds available after April 1, 2019");
  }
}

start();
