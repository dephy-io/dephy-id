use borsh::{BorshDeserialize, BorshSerialize};
use shank::{ShankContext, ShankInstruction};

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, ShankContext, ShankInstruction)]
#[rustfmt::skip]
pub enum KwilExampleInstruction {
    #[account(0, name="system_program", desc = "The system program")]
    #[account(1, name="dephy_program", desc = "The DePHY program")]
    #[account(2, writable, signer, name="payer", desc = "The account paying for the storage fees")]
    #[account(3, signer, name="owner", desc = "The DID owner")]
    #[account(4, writable, name="publisher", desc = "The Publisher account")]
    #[account(5, name="vendor", desc = "Vendor of the Device")]
    #[account(6, name="product_mint", desc = "Product of the Device")]
    #[account(7, name="device", desc = "The Device pubkey")]
    #[account(8, name="product_atoken", desc = "The Product atoken for Device")]
    #[account(9, name="user", desc = "The Device Owner pubkey")]
    #[account(10, name="did_mint", desc = "The NFT mint account")]
    #[account(11, name="did_atoken", desc = "The NFT atoken account")]
    Publish(PublishArgs),
    // Unpublish

    #[account(0, name="system_program", desc = "The system program")]
    #[account(1, writable, signer, name="payer", desc = "The account paying for the storage fees")]
    #[account(2, signer, name="user", desc = "The Registered user")]
    #[account(3, writable, name="linked", desc = "The Linked account")]
    Link(LinkArgs),
    // Unlink

    #[account(0, name="system_program", desc = "The system program")]
    #[account(1, writable, signer, name="payer", desc = "The account paying for the storage fees")]
    #[account(2, signer, name="user", desc = "The Registered user")]
    #[account(3, name="publisher", desc = "The Publisher account")]
    #[account(4, name="linked", desc = "The Linked account")]
    #[account(5, writable, name="subscriber", desc = "The Subscriber account")]
    Subscribe(SubscribeArgs),
    // Unsubscribe
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct PublishArgs {
    pub bump: u8,
    pub eth_address: [u8; 20],
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct LinkArgs {
    pub bump: u8,
    pub eth_address: [u8; 20],
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct SubscribeArgs {
    pub bump: u8,
}
