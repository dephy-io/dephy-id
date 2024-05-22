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
  IAccountMeta,
  IAccountSignerMeta,
  IInstruction,
  IInstructionWithAccounts,
  IInstructionWithData,
  ReadonlyAccount,
  ReadonlySignerAccount,
  TransactionSigner,
  WritableAccount,
  WritableSignerAccount,
  addDecoderSizePrefix,
  addEncoderSizePrefix,
  combineCodec,
  getArrayDecoder,
  getArrayEncoder,
  getStructDecoder,
  getStructEncoder,
  getTupleDecoder,
  getTupleEncoder,
  getU32Decoder,
  getU32Encoder,
  getU8Decoder,
  getU8Encoder,
  getUtf8Decoder,
  getUtf8Encoder,
  transformEncoder,
} from '@solana/web3.js';
import { DEPHY_ID_PROGRAM_ADDRESS } from '../programs';
import { ResolvedAccount, getAccountMetaFactory } from '../shared';

export type CreateVendorInstruction<
  TProgram extends string = typeof DEPHY_ID_PROGRAM_ADDRESS,
  TAccountSystemProgram extends
    | string
    | IAccountMeta<string> = '11111111111111111111111111111111',
  TAccountToken2022Program extends string | IAccountMeta<string> = string,
  TAccountAtaProgram extends
    | string
    | IAccountMeta<string> = 'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL',
  TAccountPayer extends string | IAccountMeta<string> = string,
  TAccountVendor extends string | IAccountMeta<string> = string,
  TAccountVendorMint extends string | IAccountMeta<string> = string,
  TAccountVendorAssociatedToken extends string | IAccountMeta<string> = string,
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountSystemProgram extends string
        ? ReadonlyAccount<TAccountSystemProgram>
        : TAccountSystemProgram,
      TAccountToken2022Program extends string
        ? ReadonlyAccount<TAccountToken2022Program>
        : TAccountToken2022Program,
      TAccountAtaProgram extends string
        ? ReadonlyAccount<TAccountAtaProgram>
        : TAccountAtaProgram,
      TAccountPayer extends string
        ? WritableSignerAccount<TAccountPayer> &
            IAccountSignerMeta<TAccountPayer>
        : TAccountPayer,
      TAccountVendor extends string
        ? ReadonlySignerAccount<TAccountVendor> &
            IAccountSignerMeta<TAccountVendor>
        : TAccountVendor,
      TAccountVendorMint extends string
        ? WritableAccount<TAccountVendorMint>
        : TAccountVendorMint,
      TAccountVendorAssociatedToken extends string
        ? WritableAccount<TAccountVendorAssociatedToken>
        : TAccountVendorAssociatedToken,
      ...TRemainingAccounts,
    ]
  >;

export type CreateVendorInstructionData = {
  discriminator: number;
  bump: number;
  name: string;
  symbol: string;
  uri: string;
  additionalMetadata: Array<readonly [string, string]>;
};

export type CreateVendorInstructionDataArgs = {
  bump: number;
  name: string;
  symbol: string;
  uri: string;
  additionalMetadata: Array<readonly [string, string]>;
};

export function getCreateVendorInstructionDataEncoder(): Encoder<CreateVendorInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([
      ['discriminator', getU8Encoder()],
      ['bump', getU8Encoder()],
      ['name', addEncoderSizePrefix(getUtf8Encoder(), getU32Encoder())],
      ['symbol', addEncoderSizePrefix(getUtf8Encoder(), getU32Encoder())],
      ['uri', addEncoderSizePrefix(getUtf8Encoder(), getU32Encoder())],
      [
        'additionalMetadata',
        getArrayEncoder(
          getTupleEncoder([
            addEncoderSizePrefix(getUtf8Encoder(), getU32Encoder()),
            addEncoderSizePrefix(getUtf8Encoder(), getU32Encoder()),
          ])
        ),
      ],
    ]),
    (value) => ({ ...value, discriminator: 1 })
  );
}

export function getCreateVendorInstructionDataDecoder(): Decoder<CreateVendorInstructionData> {
  return getStructDecoder([
    ['discriminator', getU8Decoder()],
    ['bump', getU8Decoder()],
    ['name', addDecoderSizePrefix(getUtf8Decoder(), getU32Decoder())],
    ['symbol', addDecoderSizePrefix(getUtf8Decoder(), getU32Decoder())],
    ['uri', addDecoderSizePrefix(getUtf8Decoder(), getU32Decoder())],
    [
      'additionalMetadata',
      getArrayDecoder(
        getTupleDecoder([
          addDecoderSizePrefix(getUtf8Decoder(), getU32Decoder()),
          addDecoderSizePrefix(getUtf8Decoder(), getU32Decoder()),
        ])
      ),
    ],
  ]);
}

export function getCreateVendorInstructionDataCodec(): Codec<
  CreateVendorInstructionDataArgs,
  CreateVendorInstructionData
> {
  return combineCodec(
    getCreateVendorInstructionDataEncoder(),
    getCreateVendorInstructionDataDecoder()
  );
}

export type CreateVendorInput<
  TAccountSystemProgram extends string = string,
  TAccountToken2022Program extends string = string,
  TAccountAtaProgram extends string = string,
  TAccountPayer extends string = string,
  TAccountVendor extends string = string,
  TAccountVendorMint extends string = string,
  TAccountVendorAssociatedToken extends string = string,
> = {
  /** The system program */
  systemProgram?: Address<TAccountSystemProgram>;
  /** The token 2022 program */
  token2022Program: Address<TAccountToken2022Program>;
  /** The associated token program */
  ataProgram?: Address<TAccountAtaProgram>;
  /** The account paying for the storage fees */
  payer: TransactionSigner<TAccountPayer>;
  /** The vendor */
  vendor: TransactionSigner<TAccountVendor>;
  /** The mint account of the vendor */
  vendorMint: Address<TAccountVendorMint>;
  /** The associated token account of the vendor */
  vendorAssociatedToken: Address<TAccountVendorAssociatedToken>;
  bump: CreateVendorInstructionDataArgs['bump'];
  name: CreateVendorInstructionDataArgs['name'];
  symbol: CreateVendorInstructionDataArgs['symbol'];
  uri: CreateVendorInstructionDataArgs['uri'];
  additionalMetadata: CreateVendorInstructionDataArgs['additionalMetadata'];
};

export function getCreateVendorInstruction<
  TAccountSystemProgram extends string,
  TAccountToken2022Program extends string,
  TAccountAtaProgram extends string,
  TAccountPayer extends string,
  TAccountVendor extends string,
  TAccountVendorMint extends string,
  TAccountVendorAssociatedToken extends string,
>(
  input: CreateVendorInput<
    TAccountSystemProgram,
    TAccountToken2022Program,
    TAccountAtaProgram,
    TAccountPayer,
    TAccountVendor,
    TAccountVendorMint,
    TAccountVendorAssociatedToken
  >
): CreateVendorInstruction<
  typeof DEPHY_ID_PROGRAM_ADDRESS,
  TAccountSystemProgram,
  TAccountToken2022Program,
  TAccountAtaProgram,
  TAccountPayer,
  TAccountVendor,
  TAccountVendorMint,
  TAccountVendorAssociatedToken
> {
  // Program address.
  const programAddress = DEPHY_ID_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    systemProgram: { value: input.systemProgram ?? null, isWritable: false },
    token2022Program: {
      value: input.token2022Program ?? null,
      isWritable: false,
    },
    ataProgram: { value: input.ataProgram ?? null, isWritable: false },
    payer: { value: input.payer ?? null, isWritable: true },
    vendor: { value: input.vendor ?? null, isWritable: false },
    vendorMint: { value: input.vendorMint ?? null, isWritable: true },
    vendorAssociatedToken: {
      value: input.vendorAssociatedToken ?? null,
      isWritable: true,
    },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  // Original args.
  const args = { ...input };

  // Resolve default values.
  if (!accounts.systemProgram.value) {
    accounts.systemProgram.value =
      '11111111111111111111111111111111' as Address<'11111111111111111111111111111111'>;
  }
  if (!accounts.ataProgram.value) {
    accounts.ataProgram.value =
      'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL' as Address<'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL'>;
  }

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.systemProgram),
      getAccountMeta(accounts.token2022Program),
      getAccountMeta(accounts.ataProgram),
      getAccountMeta(accounts.payer),
      getAccountMeta(accounts.vendor),
      getAccountMeta(accounts.vendorMint),
      getAccountMeta(accounts.vendorAssociatedToken),
    ],
    programAddress,
    data: getCreateVendorInstructionDataEncoder().encode(
      args as CreateVendorInstructionDataArgs
    ),
  } as CreateVendorInstruction<
    typeof DEPHY_ID_PROGRAM_ADDRESS,
    TAccountSystemProgram,
    TAccountToken2022Program,
    TAccountAtaProgram,
    TAccountPayer,
    TAccountVendor,
    TAccountVendorMint,
    TAccountVendorAssociatedToken
  >;

  return instruction;
}

export type ParsedCreateVendorInstruction<
  TProgram extends string = typeof DEPHY_ID_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    /** The system program */
    systemProgram: TAccountMetas[0];
    /** The token 2022 program */
    token2022Program: TAccountMetas[1];
    /** The associated token program */
    ataProgram: TAccountMetas[2];
    /** The account paying for the storage fees */
    payer: TAccountMetas[3];
    /** The vendor */
    vendor: TAccountMetas[4];
    /** The mint account of the vendor */
    vendorMint: TAccountMetas[5];
    /** The associated token account of the vendor */
    vendorAssociatedToken: TAccountMetas[6];
  };
  data: CreateVendorInstructionData;
};

export function parseCreateVendorInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedCreateVendorInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 7) {
    // TODO: Coded error.
    throw new Error('Not enough accounts');
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts![accountIndex]!;
    accountIndex += 1;
    return accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      systemProgram: getNextAccount(),
      token2022Program: getNextAccount(),
      ataProgram: getNextAccount(),
      payer: getNextAccount(),
      vendor: getNextAccount(),
      vendorMint: getNextAccount(),
      vendorAssociatedToken: getNextAccount(),
    },
    data: getCreateVendorInstructionDataDecoder().decode(instruction.data),
  };
}
