/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import {
  Address,
  ProgramDerivedAddress,
  getAddressEncoder,
  getProgramDerivedAddress,
  getUtf8Encoder,
} from '@solana/web3.js';

export type SubscriberAccountSeeds = {
  /** The publisher account */
  publisher: Address;
  /** The linked account */
  linked: Address;
};

export async function findSubscriberAccountPda(
  seeds: SubscriberAccountSeeds,
  config: { programAddress?: Address | undefined } = {}
): Promise<ProgramDerivedAddress> {
  const {
    programAddress = '7TPUtQM5UNdMSEPEgGEY5HpLJZhK9tbK4SG4wmTcSkKn' as Address<'7TPUtQM5UNdMSEPEgGEY5HpLJZhK9tbK4SG4wmTcSkKn'>,
  } = config;
  return await getProgramDerivedAddress({
    programAddress,
    seeds: [
      getUtf8Encoder().encode('SUBSCRIBER'),
      getAddressEncoder().encode(seeds.publisher),
      getAddressEncoder().encode(seeds.linked),
    ],
  });
}