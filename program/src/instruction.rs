use borsh::{BorshDeserialize, BorshSerialize};
use shank::{ShankContext, ShankInstruction};
use solana_program::{pubkey::Pubkey, entrypoint::ProgramResult};


#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, ShankContext, ShankInstruction)]
#[rustfmt::skip]
pub enum Instruction {
    /// Initialize the program.
    #[account(0, name="system_program", desc="The system program")]
    #[account(1, writable, signer, name="payer", desc="The account paying for the storage fees")]
    #[account(2, writable, name="program_data", desc="The program data account for the program")]
    #[account(3, signer, name="authority", desc="The authority account of the program")]
    Initialize(InitializeArgs),

    /// Vendor register a Product
    #[account(0, name="system_program", desc="The system program")]
    #[account(1, name="token_2022_program", desc="The SPL Token 2022 program")]
    #[account(2, writable, signer, name="payer", desc="The account paying for the storage fees")]
    #[account(3, signer, name="vendor", desc="The vendor")]
    #[account(4, writable, name="product_mint", desc="The mint account of the product")]
    CreateProduct(CreateProductArgs),

    /// Vendor register a Device
    #[account(0, name="system_program", desc="The system program")]
    #[account(1, name="token_2022_program", desc="The SPL Token 2022 program")]
    #[account(2, name="ata_program", desc="The associated token program")]
    #[account(3, writable, signer, name="payer", desc="The account paying for the storage fees")]
    #[account(4, signer, name="vendor", desc="The vendor")]
    #[account(5, writable, name="product_mint", desc="The mint account of the product")]
    #[account(6, writable, name="product_associated_token", desc="The associated token account of the product")]
    #[account(7, name="device", desc="The device")]
    #[account(8, writable, name="device_mint", desc="The mint account of the device")]
    CreateDevice(CreateDeviceArgs),

    /// User activate a Device
    #[account(0, name="system_program", desc="The system program")]
    #[account(1, name="token_2022_program", desc="The SPL Token 2022 program")]
    #[account(2, name="ata_program", desc="The associated token program")]
    #[account(3, name="instructions", desc="The instructions sysvar")]
    #[account(4, writable, signer, name="payer", desc="The account paying for the storage fees")]
    #[account(5, name="vendor", desc="The vendor")]
    #[account(6, name="product_mint", desc="The mint account for the product")]
    #[account(7, name="product_associated_token", desc="The associated token account for the product")]
    #[account(8, name="device", desc="The device")]
    #[account(9, writable, name="device_mint", desc="The mint account for the device")]
    #[account(10, writable, name="device_associated_token", desc="The associated token account for the device")]
    #[account(11, name="owner", desc="The device's owner")]
    ActivateDevice(ActivateDeviceArgs),
}


#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct InitializeArgs {
    pub bump: u8,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct CreateProductArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub additional_metadata: Vec<(String, String)>,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum DeviceSigningAlgorithm {
    Ed25519,
    Secp256k1,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct CreateDeviceArgs {
    pub name: String,
    pub uri: String,
    pub additional_metadata: Vec<(String, String)>,
    pub signing_alg: DeviceSigningAlgorithm,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum DeviceActivationSignature {
    Ed25519([u8; 64]),
    Secp256k1([u8; 64], u8),
    EthSecp256k1([u8; 64], u8),
}

impl DeviceActivationSignature {
    pub fn verify(&self, pubkey: &Pubkey, message: &[u8]) -> ProgramResult {
        match self {
            DeviceActivationSignature::Ed25519(signature) => {
                crate::ed25519::verify_signature(pubkey, signature, message)
            },
            DeviceActivationSignature::Secp256k1(signature, recovery_id) => {
                crate::secp256k1::verify_signature(pubkey, signature, *recovery_id, message)
            },
            DeviceActivationSignature::EthSecp256k1(signature, recovery_id) => {
                let eth_message = [b"\x19Ethereum Signed Message:\n", message.len().to_string().as_bytes(), message].concat();
                crate::secp256k1::verify_signature(pubkey, signature, *recovery_id, eth_message.as_ref())
            },
        }
    }
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct ActivateDeviceArgs {
    pub signature: DeviceActivationSignature,
    pub message_slot: u64,
}

