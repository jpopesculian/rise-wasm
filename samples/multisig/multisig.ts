import "../../assembly/allocators/buddy";
import {
  hexDecode,
  loadUint8Array,
  verifyMultiSig
} from "../../assembly/helpers";

function checkMultiSig(): void {
  let signatures: Array<Uint8Array> = [
    loadUint8Array(0),
    loadUint8Array(1),
    loadUint8Array(2)
  ];
  let publicKeys: Array<Uint8Array> = [
    hexDecode(
      "02c8175e806940e8551c89295679cc117b4e7dab3f23052543340cf414434bb1c9"
    ),
    hexDecode(
      "02a801eab82a8c4f156edbad9c3aa05b5b7414d0c87f6b98a6f0b26ac14dd9295f"
    ),
    hexDecode(
      "031356d552257c30f81ca6c6582e0407818f72dd8883de91d8123b26d7bb0c8240"
    ),
    hexDecode(
      "03aa1c24af9d2ec2d91985c4f0171afbbe7af36c252c8d05d4f66c81ac5dd09ef7"
    ),
    hexDecode(
      "03f167c5817c656ee2fb9c1ca738afff97fe5916e1850f1de3ec731da4e8e8674c"
    )
  ];

  if (!verifyMultiSig(signatures, publicKeys)) {
    throw new Error("Signature doesn't match");
  }
}

checkMultiSig();
