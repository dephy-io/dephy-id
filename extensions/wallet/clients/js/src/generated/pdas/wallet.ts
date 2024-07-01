/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import {
  getAddressEncoder,
  getProgramDerivedAddress,
  getUtf8Encoder,
  type Address,
  type ProgramDerivedAddress,
} from '@solana/web3.js';

export type WalletSeeds = {
  /** The device of the wallet account */
  device: Address;
  /** The authority of the wallet account */
  authority: Address;
};

export async function findWalletPda(
  seeds: WalletSeeds,
  config: { programAddress?: Address | undefined } = {}
): Promise<ProgramDerivedAddress> {
  const {
    programAddress = '5ZYZAwmhpkVUbXQWmzEnBCBRxiWdEcYYykwyApWJdEUv' as Address<'5ZYZAwmhpkVUbXQWmzEnBCBRxiWdEcYYykwyApWJdEUv'>,
  } = config;
  return await getProgramDerivedAddress({
    programAddress,
    seeds: [
      getUtf8Encoder().encode('WALLET'),
      getAddressEncoder().encode(seeds.device),
      getAddressEncoder().encode(seeds.authority),
    ],
  });
}