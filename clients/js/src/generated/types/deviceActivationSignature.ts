/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import {
  combineCodec,
  fixDecoderSize,
  fixEncoderSize,
  getBytesDecoder,
  getBytesEncoder,
  getDiscriminatedUnionDecoder,
  getDiscriminatedUnionEncoder,
  getStructDecoder,
  getStructEncoder,
  getTupleDecoder,
  getTupleEncoder,
  getU8Decoder,
  getU8Encoder,
  type Codec,
  type Decoder,
  type Encoder,
  type GetDiscriminatedUnionVariant,
  type GetDiscriminatedUnionVariantContent,
  type ReadonlyUint8Array,
} from '@solana/web3.js';

export type DeviceActivationSignature =
  | { __kind: 'Ed25519'; fields: readonly [ReadonlyUint8Array] }
  | { __kind: 'Secp256k1'; fields: readonly [ReadonlyUint8Array, number] }
  | { __kind: 'EthSecp256k1'; fields: readonly [ReadonlyUint8Array, number] };

export type DeviceActivationSignatureArgs = DeviceActivationSignature;

export function getDeviceActivationSignatureEncoder(): Encoder<DeviceActivationSignatureArgs> {
  return getDiscriminatedUnionEncoder([
    [
      'Ed25519',
      getStructEncoder([
        ['fields', getTupleEncoder([fixEncoderSize(getBytesEncoder(), 64)])],
      ]),
    ],
    [
      'Secp256k1',
      getStructEncoder([
        [
          'fields',
          getTupleEncoder([
            fixEncoderSize(getBytesEncoder(), 64),
            getU8Encoder(),
          ]),
        ],
      ]),
    ],
    [
      'EthSecp256k1',
      getStructEncoder([
        [
          'fields',
          getTupleEncoder([
            fixEncoderSize(getBytesEncoder(), 64),
            getU8Encoder(),
          ]),
        ],
      ]),
    ],
  ]);
}

export function getDeviceActivationSignatureDecoder(): Decoder<DeviceActivationSignature> {
  return getDiscriminatedUnionDecoder([
    [
      'Ed25519',
      getStructDecoder([
        ['fields', getTupleDecoder([fixDecoderSize(getBytesDecoder(), 64)])],
      ]),
    ],
    [
      'Secp256k1',
      getStructDecoder([
        [
          'fields',
          getTupleDecoder([
            fixDecoderSize(getBytesDecoder(), 64),
            getU8Decoder(),
          ]),
        ],
      ]),
    ],
    [
      'EthSecp256k1',
      getStructDecoder([
        [
          'fields',
          getTupleDecoder([
            fixDecoderSize(getBytesDecoder(), 64),
            getU8Decoder(),
          ]),
        ],
      ]),
    ],
  ]);
}

export function getDeviceActivationSignatureCodec(): Codec<
  DeviceActivationSignatureArgs,
  DeviceActivationSignature
> {
  return combineCodec(
    getDeviceActivationSignatureEncoder(),
    getDeviceActivationSignatureDecoder()
  );
}

// Data Enum Helpers.
export function deviceActivationSignature(
  kind: 'Ed25519',
  data: GetDiscriminatedUnionVariantContent<
    DeviceActivationSignatureArgs,
    '__kind',
    'Ed25519'
  >['fields']
): GetDiscriminatedUnionVariant<
  DeviceActivationSignatureArgs,
  '__kind',
  'Ed25519'
>;
export function deviceActivationSignature(
  kind: 'Secp256k1',
  data: GetDiscriminatedUnionVariantContent<
    DeviceActivationSignatureArgs,
    '__kind',
    'Secp256k1'
  >['fields']
): GetDiscriminatedUnionVariant<
  DeviceActivationSignatureArgs,
  '__kind',
  'Secp256k1'
>;
export function deviceActivationSignature(
  kind: 'EthSecp256k1',
  data: GetDiscriminatedUnionVariantContent<
    DeviceActivationSignatureArgs,
    '__kind',
    'EthSecp256k1'
  >['fields']
): GetDiscriminatedUnionVariant<
  DeviceActivationSignatureArgs,
  '__kind',
  'EthSecp256k1'
>;
export function deviceActivationSignature<
  K extends DeviceActivationSignatureArgs['__kind'],
  Data,
>(kind: K, data?: Data) {
  return Array.isArray(data)
    ? { __kind: kind, fields: data }
    : { __kind: kind, ...(data ?? {}) };
}

export function isDeviceActivationSignature<
  K extends DeviceActivationSignature['__kind'],
>(
  kind: K,
  value: DeviceActivationSignature
): value is DeviceActivationSignature & { __kind: K } {
  return value.__kind === kind;
}
