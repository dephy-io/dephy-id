//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use crate::generated::types::KeyType;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct ActivateDevice {
    /// The system program
    pub system_program: solana_program::pubkey::Pubkey,
    /// The SPL Token 2022 program
    pub token_program2022: solana_program::pubkey::Pubkey,
    /// The associated token program
    pub ata_program: solana_program::pubkey::Pubkey,
    /// The Instructions sysvar
    pub instructions: solana_program::pubkey::Pubkey,
    /// The account paying for the storage fees
    pub payer: solana_program::pubkey::Pubkey,
    /// The Device pubkey
    pub device: solana_program::pubkey::Pubkey,
    /// Vendor of the Device
    pub vendor: solana_program::pubkey::Pubkey,
    /// Product of the Device
    pub product_mint: solana_program::pubkey::Pubkey,
    /// The Product atoken for Device
    pub product_atoken: solana_program::pubkey::Pubkey,
    /// The Device Owner pubkey
    pub user: solana_program::pubkey::Pubkey,
    /// The NFT mint account
    pub did_mint: solana_program::pubkey::Pubkey,
    /// The NFT atoken account
    pub did_atoken: solana_program::pubkey::Pubkey,
}

impl ActivateDevice {
    pub fn instruction(
        &self,
        args: ActivateDeviceInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: ActivateDeviceInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(12 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program2022,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.ata_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.instructions,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.payer, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.device,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.vendor,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.product_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.product_atoken,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.user, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.did_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.did_atoken,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&ActivateDeviceInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&args).unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::DEPHY_ID_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ActivateDeviceInstructionData {
    discriminator: u8,
}

impl ActivateDeviceInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 4 }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ActivateDeviceInstructionArgs {
    pub bump: u8,
    pub key_type: KeyType,
}

/// Instruction builder for `ActivateDevice`.
///
/// ### Accounts:
///
///   0. `[optional]` system_program (default to `11111111111111111111111111111111`)
///   1. `[]` token_program2022
///   2. `[optional]` ata_program (default to `ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL`)
///   3. `[]` instructions
///   4. `[writable, signer]` payer
///   5. `[]` device
///   6. `[]` vendor
///   7. `[]` product_mint
///   8. `[]` product_atoken
///   9. `[]` user
///   10. `[writable]` did_mint
///   11. `[writable]` did_atoken
#[derive(Clone, Debug, Default)]
pub struct ActivateDeviceBuilder {
    system_program: Option<solana_program::pubkey::Pubkey>,
    token_program2022: Option<solana_program::pubkey::Pubkey>,
    ata_program: Option<solana_program::pubkey::Pubkey>,
    instructions: Option<solana_program::pubkey::Pubkey>,
    payer: Option<solana_program::pubkey::Pubkey>,
    device: Option<solana_program::pubkey::Pubkey>,
    vendor: Option<solana_program::pubkey::Pubkey>,
    product_mint: Option<solana_program::pubkey::Pubkey>,
    product_atoken: Option<solana_program::pubkey::Pubkey>,
    user: Option<solana_program::pubkey::Pubkey>,
    did_mint: Option<solana_program::pubkey::Pubkey>,
    did_atoken: Option<solana_program::pubkey::Pubkey>,
    bump: Option<u8>,
    key_type: Option<KeyType>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl ActivateDeviceBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    /// The system program
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    /// The SPL Token 2022 program
    #[inline(always)]
    pub fn token_program2022(
        &mut self,
        token_program2022: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.token_program2022 = Some(token_program2022);
        self
    }
    /// `[optional account, default to 'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL']`
    /// The associated token program
    #[inline(always)]
    pub fn ata_program(&mut self, ata_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.ata_program = Some(ata_program);
        self
    }
    /// The Instructions sysvar
    #[inline(always)]
    pub fn instructions(&mut self, instructions: solana_program::pubkey::Pubkey) -> &mut Self {
        self.instructions = Some(instructions);
        self
    }
    /// The account paying for the storage fees
    #[inline(always)]
    pub fn payer(&mut self, payer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.payer = Some(payer);
        self
    }
    /// The Device pubkey
    #[inline(always)]
    pub fn device(&mut self, device: solana_program::pubkey::Pubkey) -> &mut Self {
        self.device = Some(device);
        self
    }
    /// Vendor of the Device
    #[inline(always)]
    pub fn vendor(&mut self, vendor: solana_program::pubkey::Pubkey) -> &mut Self {
        self.vendor = Some(vendor);
        self
    }
    /// Product of the Device
    #[inline(always)]
    pub fn product_mint(&mut self, product_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.product_mint = Some(product_mint);
        self
    }
    /// The Product atoken for Device
    #[inline(always)]
    pub fn product_atoken(&mut self, product_atoken: solana_program::pubkey::Pubkey) -> &mut Self {
        self.product_atoken = Some(product_atoken);
        self
    }
    /// The Device Owner pubkey
    #[inline(always)]
    pub fn user(&mut self, user: solana_program::pubkey::Pubkey) -> &mut Self {
        self.user = Some(user);
        self
    }
    /// The NFT mint account
    #[inline(always)]
    pub fn did_mint(&mut self, did_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.did_mint = Some(did_mint);
        self
    }
    /// The NFT atoken account
    #[inline(always)]
    pub fn did_atoken(&mut self, did_atoken: solana_program::pubkey::Pubkey) -> &mut Self {
        self.did_atoken = Some(did_atoken);
        self
    }
    #[inline(always)]
    pub fn bump(&mut self, bump: u8) -> &mut Self {
        self.bump = Some(bump);
        self
    }
    #[inline(always)]
    pub fn key_type(&mut self, key_type: KeyType) -> &mut Self {
        self.key_type = Some(key_type);
        self
    }
    /// Add an aditional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = ActivateDevice {
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
            token_program2022: self
                .token_program2022
                .expect("token_program2022 is not set"),
            ata_program: self.ata_program.unwrap_or(solana_program::pubkey!(
                "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
            )),
            instructions: self.instructions.expect("instructions is not set"),
            payer: self.payer.expect("payer is not set"),
            device: self.device.expect("device is not set"),
            vendor: self.vendor.expect("vendor is not set"),
            product_mint: self.product_mint.expect("product_mint is not set"),
            product_atoken: self.product_atoken.expect("product_atoken is not set"),
            user: self.user.expect("user is not set"),
            did_mint: self.did_mint.expect("did_mint is not set"),
            did_atoken: self.did_atoken.expect("did_atoken is not set"),
        };
        let args = ActivateDeviceInstructionArgs {
            bump: self.bump.clone().expect("bump is not set"),
            key_type: self.key_type.clone().expect("key_type is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `activate_device` CPI accounts.
pub struct ActivateDeviceCpiAccounts<'a, 'b> {
    /// The system program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The SPL Token 2022 program
    pub token_program2022: &'b solana_program::account_info::AccountInfo<'a>,
    /// The associated token program
    pub ata_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The Instructions sysvar
    pub instructions: &'b solana_program::account_info::AccountInfo<'a>,
    /// The account paying for the storage fees
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// The Device pubkey
    pub device: &'b solana_program::account_info::AccountInfo<'a>,
    /// Vendor of the Device
    pub vendor: &'b solana_program::account_info::AccountInfo<'a>,
    /// Product of the Device
    pub product_mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// The Product atoken for Device
    pub product_atoken: &'b solana_program::account_info::AccountInfo<'a>,
    /// The Device Owner pubkey
    pub user: &'b solana_program::account_info::AccountInfo<'a>,
    /// The NFT mint account
    pub did_mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// The NFT atoken account
    pub did_atoken: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `activate_device` CPI instruction.
pub struct ActivateDeviceCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The system program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The SPL Token 2022 program
    pub token_program2022: &'b solana_program::account_info::AccountInfo<'a>,
    /// The associated token program
    pub ata_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The Instructions sysvar
    pub instructions: &'b solana_program::account_info::AccountInfo<'a>,
    /// The account paying for the storage fees
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// The Device pubkey
    pub device: &'b solana_program::account_info::AccountInfo<'a>,
    /// Vendor of the Device
    pub vendor: &'b solana_program::account_info::AccountInfo<'a>,
    /// Product of the Device
    pub product_mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// The Product atoken for Device
    pub product_atoken: &'b solana_program::account_info::AccountInfo<'a>,
    /// The Device Owner pubkey
    pub user: &'b solana_program::account_info::AccountInfo<'a>,
    /// The NFT mint account
    pub did_mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// The NFT atoken account
    pub did_atoken: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: ActivateDeviceInstructionArgs,
}

impl<'a, 'b> ActivateDeviceCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: ActivateDeviceCpiAccounts<'a, 'b>,
        args: ActivateDeviceInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            system_program: accounts.system_program,
            token_program2022: accounts.token_program2022,
            ata_program: accounts.ata_program,
            instructions: accounts.instructions,
            payer: accounts.payer,
            device: accounts.device,
            vendor: accounts.vendor,
            product_mint: accounts.product_mint,
            product_atoken: accounts.product_atoken,
            user: accounts.user,
            did_mint: accounts.did_mint,
            did_atoken: accounts.did_atoken,
            __args: args,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(12 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program2022.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.ata_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.instructions.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.payer.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.device.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.vendor.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.product_mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.product_atoken.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.user.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.did_mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.did_atoken.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = borsh::to_vec(&ActivateDeviceInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::DEPHY_ID_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(12 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.system_program.clone());
        account_infos.push(self.token_program2022.clone());
        account_infos.push(self.ata_program.clone());
        account_infos.push(self.instructions.clone());
        account_infos.push(self.payer.clone());
        account_infos.push(self.device.clone());
        account_infos.push(self.vendor.clone());
        account_infos.push(self.product_mint.clone());
        account_infos.push(self.product_atoken.clone());
        account_infos.push(self.user.clone());
        account_infos.push(self.did_mint.clone());
        account_infos.push(self.did_atoken.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `ActivateDevice` via CPI.
///
/// ### Accounts:
///
///   0. `[]` system_program
///   1. `[]` token_program2022
///   2. `[]` ata_program
///   3. `[]` instructions
///   4. `[writable, signer]` payer
///   5. `[]` device
///   6. `[]` vendor
///   7. `[]` product_mint
///   8. `[]` product_atoken
///   9. `[]` user
///   10. `[writable]` did_mint
///   11. `[writable]` did_atoken
#[derive(Clone, Debug)]
pub struct ActivateDeviceCpiBuilder<'a, 'b> {
    instruction: Box<ActivateDeviceCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> ActivateDeviceCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(ActivateDeviceCpiBuilderInstruction {
            __program: program,
            system_program: None,
            token_program2022: None,
            ata_program: None,
            instructions: None,
            payer: None,
            device: None,
            vendor: None,
            product_mint: None,
            product_atoken: None,
            user: None,
            did_mint: None,
            did_atoken: None,
            bump: None,
            key_type: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// The system program
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }
    /// The SPL Token 2022 program
    #[inline(always)]
    pub fn token_program2022(
        &mut self,
        token_program2022: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_program2022 = Some(token_program2022);
        self
    }
    /// The associated token program
    #[inline(always)]
    pub fn ata_program(
        &mut self,
        ata_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.ata_program = Some(ata_program);
        self
    }
    /// The Instructions sysvar
    #[inline(always)]
    pub fn instructions(
        &mut self,
        instructions: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.instructions = Some(instructions);
        self
    }
    /// The account paying for the storage fees
    #[inline(always)]
    pub fn payer(&mut self, payer: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.payer = Some(payer);
        self
    }
    /// The Device pubkey
    #[inline(always)]
    pub fn device(
        &mut self,
        device: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.device = Some(device);
        self
    }
    /// Vendor of the Device
    #[inline(always)]
    pub fn vendor(
        &mut self,
        vendor: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.vendor = Some(vendor);
        self
    }
    /// Product of the Device
    #[inline(always)]
    pub fn product_mint(
        &mut self,
        product_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.product_mint = Some(product_mint);
        self
    }
    /// The Product atoken for Device
    #[inline(always)]
    pub fn product_atoken(
        &mut self,
        product_atoken: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.product_atoken = Some(product_atoken);
        self
    }
    /// The Device Owner pubkey
    #[inline(always)]
    pub fn user(&mut self, user: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.user = Some(user);
        self
    }
    /// The NFT mint account
    #[inline(always)]
    pub fn did_mint(
        &mut self,
        did_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.did_mint = Some(did_mint);
        self
    }
    /// The NFT atoken account
    #[inline(always)]
    pub fn did_atoken(
        &mut self,
        did_atoken: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.did_atoken = Some(did_atoken);
        self
    }
    #[inline(always)]
    pub fn bump(&mut self, bump: u8) -> &mut Self {
        self.instruction.bump = Some(bump);
        self
    }
    #[inline(always)]
    pub fn key_type(&mut self, key_type: KeyType) -> &mut Self {
        self.instruction.key_type = Some(key_type);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let args = ActivateDeviceInstructionArgs {
            bump: self.instruction.bump.clone().expect("bump is not set"),
            key_type: self
                .instruction
                .key_type
                .clone()
                .expect("key_type is not set"),
        };
        let instruction = ActivateDeviceCpi {
            __program: self.instruction.__program,

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),

            token_program2022: self
                .instruction
                .token_program2022
                .expect("token_program2022 is not set"),

            ata_program: self
                .instruction
                .ata_program
                .expect("ata_program is not set"),

            instructions: self
                .instruction
                .instructions
                .expect("instructions is not set"),

            payer: self.instruction.payer.expect("payer is not set"),

            device: self.instruction.device.expect("device is not set"),

            vendor: self.instruction.vendor.expect("vendor is not set"),

            product_mint: self
                .instruction
                .product_mint
                .expect("product_mint is not set"),

            product_atoken: self
                .instruction
                .product_atoken
                .expect("product_atoken is not set"),

            user: self.instruction.user.expect("user is not set"),

            did_mint: self.instruction.did_mint.expect("did_mint is not set"),

            did_atoken: self.instruction.did_atoken.expect("did_atoken is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct ActivateDeviceCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program2022: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ata_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    instructions: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    device: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    vendor: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    product_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    product_atoken: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    user: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    did_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    did_atoken: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    bump: Option<u8>,
    key_type: Option<KeyType>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
