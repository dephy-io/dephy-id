//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>
//!

use crate::generated::types::Key;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProgramAccount {
    pub key: Key,
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
    )]
    pub authority: Pubkey,
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
    )]
    pub product_mint: Pubkey,
}

impl ProgramAccount {
    pub const LEN: usize = 65;

    /// Prefix values used to generate a PDA for this account.
    ///
    /// Values are positional and appear in the following order:
    ///
    ///   0. `ProgramAccount::PREFIX`
    ///   1. authority (`Pubkey`)
    ///   2. product_mint (`Pubkey`)
    pub const PREFIX: &'static [u8] = "Program".as_bytes();

    pub fn create_pda(
        authority: Pubkey,
        product_mint: Pubkey,
        bump: u8,
    ) -> Result<solana_program::pubkey::Pubkey, solana_program::pubkey::PubkeyError> {
        solana_program::pubkey::Pubkey::create_program_address(
            &[
                "Program".as_bytes(),
                authority.as_ref(),
                product_mint.as_ref(),
                &[bump],
            ],
            &crate::PRODUCT_PROGRAM_ID,
        )
    }

    pub fn find_pda(
        authority: &Pubkey,
        product_mint: &Pubkey,
    ) -> (solana_program::pubkey::Pubkey, u8) {
        solana_program::pubkey::Pubkey::find_program_address(
            &[
                "Program".as_bytes(),
                authority.as_ref(),
                product_mint.as_ref(),
            ],
            &crate::PRODUCT_PROGRAM_ID,
        )
    }

    #[inline(always)]
    pub fn from_bytes(data: &[u8]) -> Result<Self, std::io::Error> {
        let mut data = data;
        Self::deserialize(&mut data)
    }
}

impl<'a> TryFrom<&solana_program::account_info::AccountInfo<'a>> for ProgramAccount {
    type Error = std::io::Error;

    fn try_from(
        account_info: &solana_program::account_info::AccountInfo<'a>,
    ) -> Result<Self, Self::Error> {
        let mut data: &[u8] = &(*account_info.data).borrow();
        Self::deserialize(&mut data)
    }
}

#[cfg(feature = "anchor")]
impl anchor_lang::AccountDeserialize for ProgramAccount {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        Ok(Self::deserialize(buf)?)
    }
}

#[cfg(feature = "anchor")]
impl anchor_lang::AccountSerialize for ProgramAccount {}

#[cfg(feature = "anchor")]
impl anchor_lang::Owner for ProgramAccount {
    fn owner() -> Pubkey {
        crate::PRODUCT_PROGRAM_ID
    }
}

#[cfg(feature = "anchor-idl-build")]
impl anchor_lang::IdlBuild for ProgramAccount {}

#[cfg(feature = "anchor-idl-build")]
impl anchor_lang::Discriminator for ProgramAccount {
    const DISCRIMINATOR: [u8; 8] = [0; 8];
}
