/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

/** DeserializationError: Error deserializing an account */
export const KWIL_ERROR__DESERIALIZATION_ERROR = 0x0; // 0
/** SerializationError: Error serializing an account */
export const KWIL_ERROR__SERIALIZATION_ERROR = 0x1; // 1
/** InvalidProgramOwner: Invalid program owner. This likely mean the provided account does not exist */
export const KWIL_ERROR__INVALID_PROGRAM_OWNER = 0x2; // 2
/** InvalidPda: Invalid PDA derivation */
export const KWIL_ERROR__INVALID_PDA = 0x3; // 3
/** ExpectedEmptyAccount: Expected empty account */
export const KWIL_ERROR__EXPECTED_EMPTY_ACCOUNT = 0x4; // 4
/** ExpectedNonEmptyAccount: Expected non empty account */
export const KWIL_ERROR__EXPECTED_NON_EMPTY_ACCOUNT = 0x5; // 5
/** ExpectedSignerAccount: Expected signer account */
export const KWIL_ERROR__EXPECTED_SIGNER_ACCOUNT = 0x6; // 6
/** ExpectedWritableAccount: Expected writable account */
export const KWIL_ERROR__EXPECTED_WRITABLE_ACCOUNT = 0x7; // 7
/** AccountMismatch: Account mismatch */
export const KWIL_ERROR__ACCOUNT_MISMATCH = 0x8; // 8
/** InvalidAccountKey: Invalid account key */
export const KWIL_ERROR__INVALID_ACCOUNT_KEY = 0x9; // 9
/** NumericalOverflow: Numerical overflow */
export const KWIL_ERROR__NUMERICAL_OVERFLOW = 0xa; // 10

export type KwilError =
  | typeof KWIL_ERROR__ACCOUNT_MISMATCH
  | typeof KWIL_ERROR__DESERIALIZATION_ERROR
  | typeof KWIL_ERROR__EXPECTED_EMPTY_ACCOUNT
  | typeof KWIL_ERROR__EXPECTED_NON_EMPTY_ACCOUNT
  | typeof KWIL_ERROR__EXPECTED_SIGNER_ACCOUNT
  | typeof KWIL_ERROR__EXPECTED_WRITABLE_ACCOUNT
  | typeof KWIL_ERROR__INVALID_ACCOUNT_KEY
  | typeof KWIL_ERROR__INVALID_PDA
  | typeof KWIL_ERROR__INVALID_PROGRAM_OWNER
  | typeof KWIL_ERROR__NUMERICAL_OVERFLOW
  | typeof KWIL_ERROR__SERIALIZATION_ERROR;

let kwilErrorMessages: Record<KwilError, string> | undefined;
if (__DEV__) {
  kwilErrorMessages = {
    [KWIL_ERROR__ACCOUNT_MISMATCH]: `Account mismatch`,
    [KWIL_ERROR__DESERIALIZATION_ERROR]: `Error deserializing an account`,
    [KWIL_ERROR__EXPECTED_EMPTY_ACCOUNT]: `Expected empty account`,
    [KWIL_ERROR__EXPECTED_NON_EMPTY_ACCOUNT]: `Expected non empty account`,
    [KWIL_ERROR__EXPECTED_SIGNER_ACCOUNT]: `Expected signer account`,
    [KWIL_ERROR__EXPECTED_WRITABLE_ACCOUNT]: `Expected writable account`,
    [KWIL_ERROR__INVALID_ACCOUNT_KEY]: `Invalid account key`,
    [KWIL_ERROR__INVALID_PDA]: `Invalid PDA derivation`,
    [KWIL_ERROR__INVALID_PROGRAM_OWNER]: `Invalid program owner. This likely mean the provided account does not exist`,
    [KWIL_ERROR__NUMERICAL_OVERFLOW]: `Numerical overflow`,
    [KWIL_ERROR__SERIALIZATION_ERROR]: `Error serializing an account`,
  };
}

export function getKwilErrorMessage(code: KwilError): string {
  if (__DEV__) {
    return (kwilErrorMessages as Record<KwilError, string>)[code];
  }

  return 'Error message not available in production bundles. Compile with `__DEV__` set to true to see more information.';
}
