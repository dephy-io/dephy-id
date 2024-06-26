/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import {
  Address,
  Codec,
  Decoder,
  Encoder,
  combineCodec,
  getAddressDecoder,
  getAddressEncoder,
  getStructDecoder,
  getStructEncoder,
  getU8Decoder,
  getU8Encoder,
} from '@solana/web3.js';

export type SubscriberData = {
  bump: number;
  publisher: Address;
  linked: Address;
};

export type SubscriberDataArgs = SubscriberData;

export function getSubscriberDataEncoder(): Encoder<SubscriberDataArgs> {
  return getStructEncoder([
    ['bump', getU8Encoder()],
    ['publisher', getAddressEncoder()],
    ['linked', getAddressEncoder()],
  ]);
}

export function getSubscriberDataDecoder(): Decoder<SubscriberData> {
  return getStructDecoder([
    ['bump', getU8Decoder()],
    ['publisher', getAddressDecoder()],
    ['linked', getAddressDecoder()],
  ]);
}

export function getSubscriberDataCodec(): Codec<
  SubscriberDataArgs,
  SubscriberData
> {
  return combineCodec(getSubscriberDataEncoder(), getSubscriberDataDecoder());
}
