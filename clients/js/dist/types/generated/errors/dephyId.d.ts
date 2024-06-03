/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */
/** DeserializationError: Error deserializing an account */
export declare const DEPHY_ID_ERROR__DESERIALIZATION_ERROR = 0;
/** SerializationError: Error serializing an account */
export declare const DEPHY_ID_ERROR__SERIALIZATION_ERROR = 1;
/** InvalidProgramOwner: Invalid program owner. This likely mean the provided account does not exist */
export declare const DEPHY_ID_ERROR__INVALID_PROGRAM_OWNER = 2;
/** InvalidPda: Invalid PDA derivation */
export declare const DEPHY_ID_ERROR__INVALID_PDA = 3;
/** ExpectedEmptyAccount: Expected empty account */
export declare const DEPHY_ID_ERROR__EXPECTED_EMPTY_ACCOUNT = 4;
/** ExpectedNonEmptyAccount: Expected non empty account */
export declare const DEPHY_ID_ERROR__EXPECTED_NON_EMPTY_ACCOUNT = 5;
/** ExpectedSignerAccount: Expected signer account */
export declare const DEPHY_ID_ERROR__EXPECTED_SIGNER_ACCOUNT = 6;
/** ExpectedWritableAccount: Expected writable account */
export declare const DEPHY_ID_ERROR__EXPECTED_WRITABLE_ACCOUNT = 7;
/** AccountMismatch: Account mismatch */
export declare const DEPHY_ID_ERROR__ACCOUNT_MISMATCH = 8;
/** InvalidAccountKey: Invalid account key */
export declare const DEPHY_ID_ERROR__INVALID_ACCOUNT_KEY = 9;
/** NumericalOverflow: Numerical overflow */
export declare const DEPHY_ID_ERROR__NUMERICAL_OVERFLOW = 10;
/** MissingInstruction: Missing instruction */
export declare const DEPHY_ID_ERROR__MISSING_INSTRUCTION = 11;
export type DephyIdError = typeof DEPHY_ID_ERROR__ACCOUNT_MISMATCH | typeof DEPHY_ID_ERROR__DESERIALIZATION_ERROR | typeof DEPHY_ID_ERROR__EXPECTED_EMPTY_ACCOUNT | typeof DEPHY_ID_ERROR__EXPECTED_NON_EMPTY_ACCOUNT | typeof DEPHY_ID_ERROR__EXPECTED_SIGNER_ACCOUNT | typeof DEPHY_ID_ERROR__EXPECTED_WRITABLE_ACCOUNT | typeof DEPHY_ID_ERROR__INVALID_ACCOUNT_KEY | typeof DEPHY_ID_ERROR__INVALID_PDA | typeof DEPHY_ID_ERROR__INVALID_PROGRAM_OWNER | typeof DEPHY_ID_ERROR__MISSING_INSTRUCTION | typeof DEPHY_ID_ERROR__NUMERICAL_OVERFLOW | typeof DEPHY_ID_ERROR__SERIALIZATION_ERROR;
export declare function getDephyIdErrorMessage(code: DephyIdError): string;
//# sourceMappingURL=dephyId.d.ts.map