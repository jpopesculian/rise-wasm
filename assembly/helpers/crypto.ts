@external("env", "hash160")
export declare function hash160(input: Uint8Array): Uint8Array;

@external("env", "verify_sig")
export declare function verifySig(sig: Uint8Array, pubKey: Uint8Array): bool;

@external("env", "verify_multi_sig")
export declare function verifyMultiSig(sigs: Array<Uint8Array>, pubKeys: Array<Uint8Array>): bool;
