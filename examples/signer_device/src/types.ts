import * as secp256k1 from '@noble/secp256k1';
import * as ed25519 from '@noble/ed25519';
import { keccak256 } from "ethereum-cryptography/keccak.js";
import { u8aToU8a } from "@polkadot/util";

import { sha512 } from '@noble/hashes/sha512';
ed25519.etc.sha512Sync = (...m) => sha512(ed25519.etc.concatBytes(...m));

type HexLike = Uint8Array | `0x${string}`;

export type KeySelection = "did" | "ed25519" | "secp256k1";
export type KeyType = "ed25519" | "secp256k1";
export type KeyField = {
    keyType: KeyType,
    privateKey: HexLike,
    publicKey: string,
    bs58PublicKey?: string,
}
export type DeviceExtensions = {
    vendor: string,
    product: string,
}
export type Wallet = {
    keys: {
        did: KeyField,
        secp256k1: KeyField,
        ed25519: KeyField,
    },
    extensions: {
        vendor: string,
        product: string,
    }
};

export class Device {
    keys: {
        did: KeyField,
        secp256k1: KeyField,
        ed25519: KeyField,
    };
    extensions: DeviceExtensions;

    constructor(wallet: Wallet) {
        this.keys = wallet.keys
        this.extensions = wallet.extensions;
    }

    sign(message: HexLike, key: KeySelection): Uint8Array {
        const keyPair = this.keys[key];
        switch (keyPair.keyType) {
            case "ed25519":
                return ed25519.sign(u8aToU8a(message), u8aToU8a(keyPair.privateKey));
            case "secp256k1":
                const hashedMessage = keccak256(u8aToU8a(message));
                return secp256k1.sign(
                    hashedMessage, u8aToU8a(keyPair.privateKey),
                    { extraEntropy: true }
                ).toCompactRawBytes();
        }
    }

    verify(signature: HexLike, message: HexLike, key: KeySelection): boolean {
        const keyPair = this.keys[key];
        switch (keyPair.keyType) {
            case "ed25519":
                return ed25519.verify(u8aToU8a(signature), u8aToU8a(message), u8aToU8a(keyPair.publicKey));
            case "secp256k1":
                const hashedMessage = keccak256(u8aToU8a(message));
                return secp256k1.verify(u8aToU8a(signature), hashedMessage, u8aToU8a(keyPair.publicKey));
        }
    }
}
