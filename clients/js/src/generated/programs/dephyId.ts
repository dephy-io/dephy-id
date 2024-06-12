/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import { Address, containsBytes, getU8Encoder } from '@solana/web3.js';
import {
  ParsedActivateDeviceInstruction,
  ParsedCreateActivatedDeviceInstruction,
  ParsedCreateDeviceInstruction,
  ParsedCreateProductInstruction,
  ParsedInitializeInstruction,
} from '../instructions';
import { Key, getKeyEncoder } from '../types';

export const DEPHY_ID_PROGRAM_ADDRESS =
  'hdMghjD73uASxgJXi6e1mGPsXqnADMsrqB1bveqABP1' as Address<'hdMghjD73uASxgJXi6e1mGPsXqnADMsrqB1bveqABP1'>;

export enum DephyIdAccount {
  ProgramDataAccount,
}

export function identifyDephyIdAccount(
  account: { data: Uint8Array } | Uint8Array
): DephyIdAccount {
  const data = account instanceof Uint8Array ? account : account.data;
  if (containsBytes(data, getKeyEncoder().encode(Key.ProgramDataAccount), 0)) {
    return DephyIdAccount.ProgramDataAccount;
  }
  throw new Error(
    'The provided account could not be identified as a dephyId account.'
  );
}

export enum DephyIdInstruction {
  Initialize,
  CreateProduct,
  CreateDevice,
  ActivateDevice,
  CreateActivatedDevice,
}

export function identifyDephyIdInstruction(
  instruction: { data: Uint8Array } | Uint8Array
): DephyIdInstruction {
  const data =
    instruction instanceof Uint8Array ? instruction : instruction.data;
  if (containsBytes(data, getU8Encoder().encode(0), 0)) {
    return DephyIdInstruction.Initialize;
  }
  if (containsBytes(data, getU8Encoder().encode(1), 0)) {
    return DephyIdInstruction.CreateProduct;
  }
  if (containsBytes(data, getU8Encoder().encode(2), 0)) {
    return DephyIdInstruction.CreateDevice;
  }
  if (containsBytes(data, getU8Encoder().encode(3), 0)) {
    return DephyIdInstruction.ActivateDevice;
  }
  if (containsBytes(data, getU8Encoder().encode(4), 0)) {
    return DephyIdInstruction.CreateActivatedDevice;
  }
  throw new Error(
    'The provided instruction could not be identified as a dephyId instruction.'
  );
}

export type ParsedDephyIdInstruction<
  TProgram extends string = 'hdMghjD73uASxgJXi6e1mGPsXqnADMsrqB1bveqABP1',
> =
  | ({
      instructionType: DephyIdInstruction.Initialize;
    } & ParsedInitializeInstruction<TProgram>)
  | ({
      instructionType: DephyIdInstruction.CreateProduct;
    } & ParsedCreateProductInstruction<TProgram>)
  | ({
      instructionType: DephyIdInstruction.CreateDevice;
    } & ParsedCreateDeviceInstruction<TProgram>)
  | ({
      instructionType: DephyIdInstruction.ActivateDevice;
    } & ParsedActivateDeviceInstruction<TProgram>)
  | ({
      instructionType: DephyIdInstruction.CreateActivatedDevice;
    } & ParsedCreateActivatedDeviceInstruction<TProgram>);
