/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */
import { type Address, type Codec, type Decoder, type Encoder, type IAccountMeta, type IAccountSignerMeta, type IInstruction, type IInstructionWithAccounts, type IInstructionWithData, type ReadonlyAccount, type ReadonlySignerAccount, type TransactionSigner, type WritableAccount, type WritableSignerAccount } from '@solana/web3.js';
import { DEPHY_ID_PROGRAM_ADDRESS } from '../programs';
export type InitializeInstruction<TProgram extends string = typeof DEPHY_ID_PROGRAM_ADDRESS, TAccountSystemProgram extends string | IAccountMeta<string> = '11111111111111111111111111111111', TAccountPayer extends string | IAccountMeta<string> = string, TAccountProgramData extends string | IAccountMeta<string> = string, TAccountAuthority extends string | IAccountMeta<string> = string, TRemainingAccounts extends readonly IAccountMeta<string>[] = []> = IInstruction<TProgram> & IInstructionWithData<Uint8Array> & IInstructionWithAccounts<[
    TAccountSystemProgram extends string ? ReadonlyAccount<TAccountSystemProgram> : TAccountSystemProgram,
    TAccountPayer extends string ? WritableSignerAccount<TAccountPayer> & IAccountSignerMeta<TAccountPayer> : TAccountPayer,
    TAccountProgramData extends string ? WritableAccount<TAccountProgramData> : TAccountProgramData,
    TAccountAuthority extends string ? ReadonlySignerAccount<TAccountAuthority> & IAccountSignerMeta<TAccountAuthority> : TAccountAuthority,
    ...TRemainingAccounts
]>;
export type InitializeInstructionData = {
    discriminator: number;
    bump: number;
};
export type InitializeInstructionDataArgs = {
    bump: number;
};
export declare function getInitializeInstructionDataEncoder(): Encoder<InitializeInstructionDataArgs>;
export declare function getInitializeInstructionDataDecoder(): Decoder<InitializeInstructionData>;
export declare function getInitializeInstructionDataCodec(): Codec<InitializeInstructionDataArgs, InitializeInstructionData>;
export type InitializeInput<TAccountSystemProgram extends string = string, TAccountPayer extends string = string, TAccountProgramData extends string = string, TAccountAuthority extends string = string> = {
    /** The system program */
    systemProgram?: Address<TAccountSystemProgram>;
    /** The account paying for the storage fees */
    payer: TransactionSigner<TAccountPayer>;
    /** The program data account for the program */
    programData: Address<TAccountProgramData>;
    /** The authority account of the program */
    authority: TransactionSigner<TAccountAuthority>;
    bump: InitializeInstructionDataArgs['bump'];
};
export declare function getInitializeInstruction<TAccountSystemProgram extends string, TAccountPayer extends string, TAccountProgramData extends string, TAccountAuthority extends string>(input: InitializeInput<TAccountSystemProgram, TAccountPayer, TAccountProgramData, TAccountAuthority>): InitializeInstruction<typeof DEPHY_ID_PROGRAM_ADDRESS, TAccountSystemProgram, TAccountPayer, TAccountProgramData, TAccountAuthority>;
export type ParsedInitializeInstruction<TProgram extends string = typeof DEPHY_ID_PROGRAM_ADDRESS, TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[]> = {
    programAddress: Address<TProgram>;
    accounts: {
        /** The system program */
        systemProgram: TAccountMetas[0];
        /** The account paying for the storage fees */
        payer: TAccountMetas[1];
        /** The program data account for the program */
        programData: TAccountMetas[2];
        /** The authority account of the program */
        authority: TAccountMetas[3];
    };
    data: InitializeInstructionData;
};
export declare function parseInitializeInstruction<TProgram extends string, TAccountMetas extends readonly IAccountMeta[]>(instruction: IInstruction<TProgram> & IInstructionWithAccounts<TAccountMetas> & IInstructionWithData<Uint8Array>): ParsedInitializeInstruction<TProgram, TAccountMetas>;
//# sourceMappingURL=initialize.d.ts.map