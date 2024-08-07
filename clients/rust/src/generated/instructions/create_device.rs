//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>
//!

use crate::generated::types::DeviceSigningAlgorithm;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct CreateDevice {
    /// The system program
    pub system_program: solana_program::pubkey::Pubkey,
    /// The SPL Token 2022 program
    pub token2022_program: solana_program::pubkey::Pubkey,
    /// The associated token program
    pub ata_program: solana_program::pubkey::Pubkey,
    /// The account paying for the storage fees
    pub payer: solana_program::pubkey::Pubkey,
    /// The vendor
    pub vendor: solana_program::pubkey::Pubkey,
    /// The mint account of the product
    pub product_mint: solana_program::pubkey::Pubkey,
    /// The associated token account of the product
    pub product_associated_token: solana_program::pubkey::Pubkey,
    /// The device
    pub device: solana_program::pubkey::Pubkey,
    /// The mint account of the device
    pub device_mint: solana_program::pubkey::Pubkey,
}

impl CreateDevice {
    pub fn instruction(
        &self,
        args: CreateDeviceInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: CreateDeviceInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(9 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token2022_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.ata_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.payer, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.vendor,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.product_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.product_associated_token,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.device,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.device_mint,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&CreateDeviceInstructionData::new()).unwrap();
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
pub struct CreateDeviceInstructionData {
    discriminator: u8,
}

impl CreateDeviceInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 2 }
    }
}

impl Default for CreateDeviceInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateDeviceInstructionArgs {
    pub name: String,
    pub uri: String,
    pub additional_metadata: Vec<(String, String)>,
    pub signing_alg: DeviceSigningAlgorithm,
}

/// Instruction builder for `CreateDevice`.
///
/// ### Accounts:
///
///   0. `[optional]` system_program (default to `11111111111111111111111111111111`)
///   1. `[optional]` token2022_program (default to `TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb`)
///   2. `[optional]` ata_program (default to `ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL`)
///   3. `[writable, signer]` payer
///   4. `[signer]` vendor
///   5. `[writable]` product_mint
///   6. `[writable]` product_associated_token
///   7. `[]` device
///   8. `[writable]` device_mint
#[derive(Clone, Debug, Default)]
pub struct CreateDeviceBuilder {
    system_program: Option<solana_program::pubkey::Pubkey>,
    token2022_program: Option<solana_program::pubkey::Pubkey>,
    ata_program: Option<solana_program::pubkey::Pubkey>,
    payer: Option<solana_program::pubkey::Pubkey>,
    vendor: Option<solana_program::pubkey::Pubkey>,
    product_mint: Option<solana_program::pubkey::Pubkey>,
    product_associated_token: Option<solana_program::pubkey::Pubkey>,
    device: Option<solana_program::pubkey::Pubkey>,
    device_mint: Option<solana_program::pubkey::Pubkey>,
    name: Option<String>,
    uri: Option<String>,
    additional_metadata: Option<Vec<(String, String)>>,
    signing_alg: Option<DeviceSigningAlgorithm>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl CreateDeviceBuilder {
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
    /// `[optional account, default to 'TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb']`
    /// The SPL Token 2022 program
    #[inline(always)]
    pub fn token2022_program(
        &mut self,
        token2022_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.token2022_program = Some(token2022_program);
        self
    }
    /// `[optional account, default to 'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL']`
    /// The associated token program
    #[inline(always)]
    pub fn ata_program(&mut self, ata_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.ata_program = Some(ata_program);
        self
    }
    /// The account paying for the storage fees
    #[inline(always)]
    pub fn payer(&mut self, payer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.payer = Some(payer);
        self
    }
    /// The vendor
    #[inline(always)]
    pub fn vendor(&mut self, vendor: solana_program::pubkey::Pubkey) -> &mut Self {
        self.vendor = Some(vendor);
        self
    }
    /// The mint account of the product
    #[inline(always)]
    pub fn product_mint(&mut self, product_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.product_mint = Some(product_mint);
        self
    }
    /// The associated token account of the product
    #[inline(always)]
    pub fn product_associated_token(
        &mut self,
        product_associated_token: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.product_associated_token = Some(product_associated_token);
        self
    }
    /// The device
    #[inline(always)]
    pub fn device(&mut self, device: solana_program::pubkey::Pubkey) -> &mut Self {
        self.device = Some(device);
        self
    }
    /// The mint account of the device
    #[inline(always)]
    pub fn device_mint(&mut self, device_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.device_mint = Some(device_mint);
        self
    }
    #[inline(always)]
    pub fn name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }
    #[inline(always)]
    pub fn uri(&mut self, uri: String) -> &mut Self {
        self.uri = Some(uri);
        self
    }
    #[inline(always)]
    pub fn additional_metadata(&mut self, additional_metadata: Vec<(String, String)>) -> &mut Self {
        self.additional_metadata = Some(additional_metadata);
        self
    }
    #[inline(always)]
    pub fn signing_alg(&mut self, signing_alg: DeviceSigningAlgorithm) -> &mut Self {
        self.signing_alg = Some(signing_alg);
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
        let accounts = CreateDevice {
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
            token2022_program: self.token2022_program.unwrap_or(solana_program::pubkey!(
                "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
            )),
            ata_program: self.ata_program.unwrap_or(solana_program::pubkey!(
                "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
            )),
            payer: self.payer.expect("payer is not set"),
            vendor: self.vendor.expect("vendor is not set"),
            product_mint: self.product_mint.expect("product_mint is not set"),
            product_associated_token: self
                .product_associated_token
                .expect("product_associated_token is not set"),
            device: self.device.expect("device is not set"),
            device_mint: self.device_mint.expect("device_mint is not set"),
        };
        let args = CreateDeviceInstructionArgs {
            name: self.name.clone().expect("name is not set"),
            uri: self.uri.clone().expect("uri is not set"),
            additional_metadata: self
                .additional_metadata
                .clone()
                .expect("additional_metadata is not set"),
            signing_alg: self.signing_alg.clone().expect("signing_alg is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `create_device` CPI accounts.
pub struct CreateDeviceCpiAccounts<'a, 'b> {
    /// The system program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The SPL Token 2022 program
    pub token2022_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The associated token program
    pub ata_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The account paying for the storage fees
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// The vendor
    pub vendor: &'b solana_program::account_info::AccountInfo<'a>,
    /// The mint account of the product
    pub product_mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// The associated token account of the product
    pub product_associated_token: &'b solana_program::account_info::AccountInfo<'a>,
    /// The device
    pub device: &'b solana_program::account_info::AccountInfo<'a>,
    /// The mint account of the device
    pub device_mint: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `create_device` CPI instruction.
pub struct CreateDeviceCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The system program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The SPL Token 2022 program
    pub token2022_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The associated token program
    pub ata_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The account paying for the storage fees
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// The vendor
    pub vendor: &'b solana_program::account_info::AccountInfo<'a>,
    /// The mint account of the product
    pub product_mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// The associated token account of the product
    pub product_associated_token: &'b solana_program::account_info::AccountInfo<'a>,
    /// The device
    pub device: &'b solana_program::account_info::AccountInfo<'a>,
    /// The mint account of the device
    pub device_mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: CreateDeviceInstructionArgs,
}

impl<'a, 'b> CreateDeviceCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: CreateDeviceCpiAccounts<'a, 'b>,
        args: CreateDeviceInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            system_program: accounts.system_program,
            token2022_program: accounts.token2022_program,
            ata_program: accounts.ata_program,
            payer: accounts.payer,
            vendor: accounts.vendor,
            product_mint: accounts.product_mint,
            product_associated_token: accounts.product_associated_token,
            device: accounts.device,
            device_mint: accounts.device_mint,
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
        let mut accounts = Vec::with_capacity(9 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token2022_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.ata_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.payer.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.vendor.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.product_mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.product_associated_token.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.device.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.device_mint.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = borsh::to_vec(&CreateDeviceInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::DEPHY_ID_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(9 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.system_program.clone());
        account_infos.push(self.token2022_program.clone());
        account_infos.push(self.ata_program.clone());
        account_infos.push(self.payer.clone());
        account_infos.push(self.vendor.clone());
        account_infos.push(self.product_mint.clone());
        account_infos.push(self.product_associated_token.clone());
        account_infos.push(self.device.clone());
        account_infos.push(self.device_mint.clone());
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

/// Instruction builder for `CreateDevice` via CPI.
///
/// ### Accounts:
///
///   0. `[]` system_program
///   1. `[]` token2022_program
///   2. `[]` ata_program
///   3. `[writable, signer]` payer
///   4. `[signer]` vendor
///   5. `[writable]` product_mint
///   6. `[writable]` product_associated_token
///   7. `[]` device
///   8. `[writable]` device_mint
#[derive(Clone, Debug)]
pub struct CreateDeviceCpiBuilder<'a, 'b> {
    instruction: Box<CreateDeviceCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CreateDeviceCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(CreateDeviceCpiBuilderInstruction {
            __program: program,
            system_program: None,
            token2022_program: None,
            ata_program: None,
            payer: None,
            vendor: None,
            product_mint: None,
            product_associated_token: None,
            device: None,
            device_mint: None,
            name: None,
            uri: None,
            additional_metadata: None,
            signing_alg: None,
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
    pub fn token2022_program(
        &mut self,
        token2022_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token2022_program = Some(token2022_program);
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
    /// The account paying for the storage fees
    #[inline(always)]
    pub fn payer(&mut self, payer: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.payer = Some(payer);
        self
    }
    /// The vendor
    #[inline(always)]
    pub fn vendor(
        &mut self,
        vendor: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.vendor = Some(vendor);
        self
    }
    /// The mint account of the product
    #[inline(always)]
    pub fn product_mint(
        &mut self,
        product_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.product_mint = Some(product_mint);
        self
    }
    /// The associated token account of the product
    #[inline(always)]
    pub fn product_associated_token(
        &mut self,
        product_associated_token: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.product_associated_token = Some(product_associated_token);
        self
    }
    /// The device
    #[inline(always)]
    pub fn device(
        &mut self,
        device: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.device = Some(device);
        self
    }
    /// The mint account of the device
    #[inline(always)]
    pub fn device_mint(
        &mut self,
        device_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.device_mint = Some(device_mint);
        self
    }
    #[inline(always)]
    pub fn name(&mut self, name: String) -> &mut Self {
        self.instruction.name = Some(name);
        self
    }
    #[inline(always)]
    pub fn uri(&mut self, uri: String) -> &mut Self {
        self.instruction.uri = Some(uri);
        self
    }
    #[inline(always)]
    pub fn additional_metadata(&mut self, additional_metadata: Vec<(String, String)>) -> &mut Self {
        self.instruction.additional_metadata = Some(additional_metadata);
        self
    }
    #[inline(always)]
    pub fn signing_alg(&mut self, signing_alg: DeviceSigningAlgorithm) -> &mut Self {
        self.instruction.signing_alg = Some(signing_alg);
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
        let args = CreateDeviceInstructionArgs {
            name: self.instruction.name.clone().expect("name is not set"),
            uri: self.instruction.uri.clone().expect("uri is not set"),
            additional_metadata: self
                .instruction
                .additional_metadata
                .clone()
                .expect("additional_metadata is not set"),
            signing_alg: self
                .instruction
                .signing_alg
                .clone()
                .expect("signing_alg is not set"),
        };
        let instruction = CreateDeviceCpi {
            __program: self.instruction.__program,

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),

            token2022_program: self
                .instruction
                .token2022_program
                .expect("token2022_program is not set"),

            ata_program: self
                .instruction
                .ata_program
                .expect("ata_program is not set"),

            payer: self.instruction.payer.expect("payer is not set"),

            vendor: self.instruction.vendor.expect("vendor is not set"),

            product_mint: self
                .instruction
                .product_mint
                .expect("product_mint is not set"),

            product_associated_token: self
                .instruction
                .product_associated_token
                .expect("product_associated_token is not set"),

            device: self.instruction.device.expect("device is not set"),

            device_mint: self
                .instruction
                .device_mint
                .expect("device_mint is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct CreateDeviceCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token2022_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ata_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    vendor: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    product_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    product_associated_token: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    device: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    device_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    name: Option<String>,
    uri: Option<String>,
    additional_metadata: Option<Vec<(String, String)>>,
    signing_alg: Option<DeviceSigningAlgorithm>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
