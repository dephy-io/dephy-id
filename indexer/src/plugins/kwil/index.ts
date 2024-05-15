import { Address, Base58EncodedBytes, getBase16Decoder, getBase58Encoder } from "@solana/web3.js";
import { KWIL_PROGRAM_ADDRESS, KwilInstruction, identifyKwilInstruction, parseCreateKwilInstruction, parseUpdateAclInstruction } from "./generated";
import { keccak_256 } from "@noble/hashes/sha3";
import { WebKwil, KwilSigner, Utils as KwilUtils } from '@kwilteam/kwil-js';
import { Wallet } from "ethers";

type PartiallyDecodedTransactionInstruction = {
    accounts: readonly Address[];
    data: Base58EncodedBytes;
    programId: Address;
}

type ParsedTransactionInstruction = {
    parsed: {
        info?: object;
        type: string;
    };
    program: string;
    programId: Address;
}

interface IxMeta {
    tx: string
    index: number
}

const dbOwnerAddress = "0x5BaD959d0AA3DFA44131F61E39192E475c3fBF29"
const dbName = "playground"
const dbid = KwilUtils.generateDBID(dbOwnerAddress, dbName);
let dbOwnerKwilSigner: KwilSigner;
let kwil: WebKwil;

function to_checksum_address(address_bytes: Uint8Array) {
    const decoder = getBase16Decoder()
    const address = decoder.decode(address_bytes)
    const hash = decoder.decode(keccak_256(address))
    let result = '0x'
    for (var i = 0; i < address.length; i++) {
        if (parseInt(hash[i], 16) >= 8) {
            result += address[i].toUpperCase()
        } else {
            result += address[i]
        }
    }
    return result
}

// TODO: persist
let address_mapping: {[key in Address]: string} = {}

async function save_eth_address(account_pubkey: Address, eth_address: Uint8Array) {
    address_mapping[account_pubkey] = to_checksum_address(eth_address)
}

async function load_eth_address(account_pubkey: Address): Promise<string> {
    return address_mapping[account_pubkey]
}

async function createACL(kwil_account: Address, subject: Uint8Array, target: string, read_level: number, write_level: number) {
    // dbOwnerKwilSigner.identifier == await load_eth_address(kwil_account)
    return await kwil.execute({
        dbid,
        action: "create_acl",
        inputs: [
            {
                $subject: to_checksum_address(subject),
                $target: target,
                $read_level: read_level,
                $write_level: write_level,
            }
        ]
    }, dbOwnerKwilSigner);
}

export async function initialize() {
    const dbOwnerEthPrivateKey = "c71d41fa79464fa467aee3f56436b366baa2e738d07808b6cbf1219f43152a61";
    const dbOwnerEthSigner = new Wallet(dbOwnerEthPrivateKey);
    const dbOwnerEthAddress = await dbOwnerEthSigner.getAddress();
    dbOwnerKwilSigner = new KwilSigner(dbOwnerEthSigner, dbOwnerEthAddress);

    kwil = new WebKwil({
        kwilProvider: "http://localhost:8080",
        chainId: "kwil-chain-tmp",
    });
}

export function matchIx(ix: PartiallyDecodedTransactionInstruction | ParsedTransactionInstruction) {
    return ix.programId == KWIL_PROGRAM_ADDRESS
}

export async function processIx(ix: PartiallyDecodedTransactionInstruction, meta: IxMeta) {
    const kwil_ix = {
        accounts: ix.accounts.map(address => ({address, role: 0})),
        data: Uint8Array.from(getBase58Encoder().encode(ix.data)),
        programAddress: ix.programId,
    }

    switch (identifyKwilInstruction(kwil_ix)) {
        case KwilInstruction.CreateKwil:
            const create_kwil = parseCreateKwilInstruction(kwil_ix)
            await save_eth_address(create_kwil.accounts.kwilAccount.address, Uint8Array.from(create_kwil.data.kwilSigner))
            break;

        case KwilInstruction.UpdateAcl:
            const update_acl = parseUpdateAclInstruction(kwil_ix)
            const subject = update_acl.data.subject
            await createACL(
                update_acl.accounts.kwilAccount.address,
                Uint8Array.from(subject),
                update_acl.data.target,
                update_acl.data.readLevel,
                update_acl.data.writeLevel
            )
            break;

        default:
            break;
    }
}

