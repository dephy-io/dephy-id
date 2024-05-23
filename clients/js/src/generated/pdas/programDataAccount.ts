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
  getProgramDerivedAddress,
  getUtf8Encoder,
} from '@solana/web3.js';

export async function findProgramDataAccountPda(
  config: { programAddress?: Address | undefined } = {}
): Promise<ProgramDerivedAddress> {
  const {
    programAddress = 'DiSU1nme5VJvMdry5FYhfw6LLFb3HUFLkCLZDe53x3PV' as Address<'DiSU1nme5VJvMdry5FYhfw6LLFb3HUFLkCLZDe53x3PV'>,
  } = config;
  return await getProgramDerivedAddress({
    programAddress,
    seeds: [getUtf8Encoder().encode('DePHY_ID')],
  });
}
