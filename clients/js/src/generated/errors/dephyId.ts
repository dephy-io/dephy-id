/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

/** DeserializationError: Error deserializing an account */
export const DEPHY_ID_ERROR__DESERIALIZATION_ERROR = 0x0; // 0
/** SerializationError: Error serializing an account */
export const DEPHY_ID_ERROR__SERIALIZATION_ERROR = 0x1; // 1
/** InvalidProgramOwner: Invalid program owner. This likely mean the provided account does not exist */
export const DEPHY_ID_ERROR__INVALID_PROGRAM_OWNER = 0x2; // 2
/** InvalidPda: Invalid PDA derivation */
export const DEPHY_ID_ERROR__INVALID_PDA = 0x3; // 3
/** ExpectedEmptyAccount: Expected empty account */
export const DEPHY_ID_ERROR__EXPECTED_EMPTY_ACCOUNT = 0x4; // 4
/** ExpectedNonEmptyAccount: Expected non empty account */
export const DEPHY_ID_ERROR__EXPECTED_NON_EMPTY_ACCOUNT = 0x5; // 5
/** ExpectedSignerAccount: Expected signer account */
export const DEPHY_ID_ERROR__EXPECTED_SIGNER_ACCOUNT = 0x6; // 6
/** ExpectedWritableAccount: Expected writable account */
export const DEPHY_ID_ERROR__EXPECTED_WRITABLE_ACCOUNT = 0x7; // 7
/** AccountMismatch: Account mismatch */
export const DEPHY_ID_ERROR__ACCOUNT_MISMATCH = 0x8; // 8
/** InvalidAccountKey: Invalid account key */
export const DEPHY_ID_ERROR__INVALID_ACCOUNT_KEY = 0x9; // 9
/** NumericalOverflow: Numerical overflow */
export const DEPHY_ID_ERROR__NUMERICAL_OVERFLOW = 0xa; // 10
/** MissingInstruction: Missing instruction */
export const DEPHY_ID_ERROR__MISSING_INSTRUCTION = 0xb; // 11
/** SignatureMismatch: Signature mismatch */
export const DEPHY_ID_ERROR__SIGNATURE_MISMATCH = 0xc; // 12

export type DephyIdError =
  | typeof DEPHY_ID_ERROR__ACCOUNT_MISMATCH
  | typeof DEPHY_ID_ERROR__DESERIALIZATION_ERROR
  | typeof DEPHY_ID_ERROR__EXPECTED_EMPTY_ACCOUNT
  | typeof DEPHY_ID_ERROR__EXPECTED_NON_EMPTY_ACCOUNT
  | typeof DEPHY_ID_ERROR__EXPECTED_SIGNER_ACCOUNT
  | typeof DEPHY_ID_ERROR__EXPECTED_WRITABLE_ACCOUNT
  | typeof DEPHY_ID_ERROR__INVALID_ACCOUNT_KEY
  | typeof DEPHY_ID_ERROR__INVALID_PDA
  | typeof DEPHY_ID_ERROR__INVALID_PROGRAM_OWNER
  | typeof DEPHY_ID_ERROR__MISSING_INSTRUCTION
  | typeof DEPHY_ID_ERROR__NUMERICAL_OVERFLOW
  | typeof DEPHY_ID_ERROR__SERIALIZATION_ERROR
  | typeof DEPHY_ID_ERROR__SIGNATURE_MISMATCH;

let dephyIdErrorMessages: Record<DephyIdError, string> | undefined;
if (__DEV__) {
  dephyIdErrorMessages = {
    [DEPHY_ID_ERROR__ACCOUNT_MISMATCH]: `Account mismatch`,
    [DEPHY_ID_ERROR__DESERIALIZATION_ERROR]: `Error deserializing an account`,
    [DEPHY_ID_ERROR__EXPECTED_EMPTY_ACCOUNT]: `Expected empty account`,
    [DEPHY_ID_ERROR__EXPECTED_NON_EMPTY_ACCOUNT]: `Expected non empty account`,
    [DEPHY_ID_ERROR__EXPECTED_SIGNER_ACCOUNT]: `Expected signer account`,
    [DEPHY_ID_ERROR__EXPECTED_WRITABLE_ACCOUNT]: `Expected writable account`,
    [DEPHY_ID_ERROR__INVALID_ACCOUNT_KEY]: `Invalid account key`,
    [DEPHY_ID_ERROR__INVALID_PDA]: `Invalid PDA derivation`,
    [DEPHY_ID_ERROR__INVALID_PROGRAM_OWNER]: `Invalid program owner. This likely mean the provided account does not exist`,
    [DEPHY_ID_ERROR__MISSING_INSTRUCTION]: `Missing instruction`,
    [DEPHY_ID_ERROR__NUMERICAL_OVERFLOW]: `Numerical overflow`,
    [DEPHY_ID_ERROR__SERIALIZATION_ERROR]: `Error serializing an account`,
    [DEPHY_ID_ERROR__SIGNATURE_MISMATCH]: `Signature mismatch`,
  };
}

export function getDephyIdErrorMessage(code: DephyIdError): string {
  if (__DEV__) {
    return (dephyIdErrorMessages as Record<DephyIdError, string>)[code];
  }

  return 'Error message not available in production bundles. Compile with `__DEV__` set to true to see more information.';
}
