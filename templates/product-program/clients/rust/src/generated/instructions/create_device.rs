//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct CreateDevice {
    /// The program derived address of the program account to create (seeds: ['Program'])
    pub program_pda: solana_program::pubkey::Pubkey,
    /// The account paying for the storage fees
    pub payer: solana_program::pubkey::Pubkey,
    /// The system program
    pub system_program: solana_program::pubkey::Pubkey,
    /// The SPL Token 2022 program
    pub token2022_program: solana_program::pubkey::Pubkey,
    /// The associated token program
    pub ata_program: solana_program::pubkey::Pubkey,
    /// DePHY ID program id
    pub dephy_id_program: solana_program::pubkey::Pubkey,
    /// PDA as product vendor (seeds: ['VENDOR'])
    pub vendor: solana_program::pubkey::Pubkey,
    /// PDA of the product mint account (program: dephy_id, seeds: ['DePHY_ID-PRODUCT', vendor, PRODUCT_NAME])
    pub product_mint: solana_program::pubkey::Pubkey,
    /// The device's owner
    pub owner: solana_program::pubkey::Pubkey,
    /// PDA of the virtual device (seeds: ['DEVICE', owner])
    pub device: solana_program::pubkey::Pubkey,
    /// The associated token account of the product
    pub product_atoken: solana_program::pubkey::Pubkey,
    /// The mint account of the device
    pub device_mint: solana_program::pubkey::Pubkey,
    /// The associated token account for the device
    pub device_atoken: solana_program::pubkey::Pubkey,
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
        let mut accounts = Vec::with_capacity(13 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.program_pda,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.payer, true,
        ));
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
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.dephy_id_program,
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
            self.owner, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.device,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.product_atoken,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.device_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.device_atoken,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&CreateDeviceInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&args).unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::PRODUCT_PROGRAM_ID,
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
        Self { discriminator: 1 }
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
    pub challenge: u8,
}

/// Instruction builder for `CreateDevice`.
///
/// ### Accounts:
///
///   0. `[writable]` program_pda
///   1. `[writable, signer]` payer
///   2. `[optional]` system_program (default to `11111111111111111111111111111111`)
///   3. `[]` token2022_program
///   4. `[optional]` ata_program (default to `ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL`)
///   5. `[]` dephy_id_program
///   6. `[]` vendor
///   7. `[]` product_mint
///   8. `[]` owner
///   9. `[]` device
///   10. `[writable]` product_atoken
///   11. `[writable]` device_mint
///   12. `[writable]` device_atoken
#[derive(Clone, Debug, Default)]
pub struct CreateDeviceBuilder {
    program_pda: Option<solana_program::pubkey::Pubkey>,
    payer: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    token2022_program: Option<solana_program::pubkey::Pubkey>,
    ata_program: Option<solana_program::pubkey::Pubkey>,
    dephy_id_program: Option<solana_program::pubkey::Pubkey>,
    vendor: Option<solana_program::pubkey::Pubkey>,
    product_mint: Option<solana_program::pubkey::Pubkey>,
    owner: Option<solana_program::pubkey::Pubkey>,
    device: Option<solana_program::pubkey::Pubkey>,
    product_atoken: Option<solana_program::pubkey::Pubkey>,
    device_mint: Option<solana_program::pubkey::Pubkey>,
    device_atoken: Option<solana_program::pubkey::Pubkey>,
    challenge: Option<u8>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl CreateDeviceBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// The program derived address of the program account to create (seeds: ['Program'])
    #[inline(always)]
    pub fn program_pda(&mut self, program_pda: solana_program::pubkey::Pubkey) -> &mut Self {
        self.program_pda = Some(program_pda);
        self
    }
    /// The account paying for the storage fees
    #[inline(always)]
    pub fn payer(&mut self, payer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.payer = Some(payer);
        self
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
    /// DePHY ID program id
    #[inline(always)]
    pub fn dephy_id_program(
        &mut self,
        dephy_id_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.dephy_id_program = Some(dephy_id_program);
        self
    }
    /// PDA as product vendor (seeds: ['VENDOR'])
    #[inline(always)]
    pub fn vendor(&mut self, vendor: solana_program::pubkey::Pubkey) -> &mut Self {
        self.vendor = Some(vendor);
        self
    }
    /// PDA of the product mint account (program: dephy_id, seeds: ['DePHY_ID-PRODUCT', vendor, PRODUCT_NAME])
    #[inline(always)]
    pub fn product_mint(&mut self, product_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.product_mint = Some(product_mint);
        self
    }
    /// The device's owner
    #[inline(always)]
    pub fn owner(&mut self, owner: solana_program::pubkey::Pubkey) -> &mut Self {
        self.owner = Some(owner);
        self
    }
    /// PDA of the virtual device (seeds: ['DEVICE', owner])
    #[inline(always)]
    pub fn device(&mut self, device: solana_program::pubkey::Pubkey) -> &mut Self {
        self.device = Some(device);
        self
    }
    /// The associated token account of the product
    #[inline(always)]
    pub fn product_atoken(&mut self, product_atoken: solana_program::pubkey::Pubkey) -> &mut Self {
        self.product_atoken = Some(product_atoken);
        self
    }
    /// The mint account of the device
    #[inline(always)]
    pub fn device_mint(&mut self, device_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.device_mint = Some(device_mint);
        self
    }
    /// The associated token account for the device
    #[inline(always)]
    pub fn device_atoken(&mut self, device_atoken: solana_program::pubkey::Pubkey) -> &mut Self {
        self.device_atoken = Some(device_atoken);
        self
    }
    #[inline(always)]
    pub fn challenge(&mut self, challenge: u8) -> &mut Self {
        self.challenge = Some(challenge);
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
            program_pda: self.program_pda.expect("program_pda is not set"),
            payer: self.payer.expect("payer is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
            token2022_program: self
                .token2022_program
                .expect("token2022_program is not set"),
            ata_program: self.ata_program.unwrap_or(solana_program::pubkey!(
                "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
            )),
            dephy_id_program: self.dephy_id_program.expect("dephy_id_program is not set"),
            vendor: self.vendor.expect("vendor is not set"),
            product_mint: self.product_mint.expect("product_mint is not set"),
            owner: self.owner.expect("owner is not set"),
            device: self.device.expect("device is not set"),
            product_atoken: self.product_atoken.expect("product_atoken is not set"),
            device_mint: self.device_mint.expect("device_mint is not set"),
            device_atoken: self.device_atoken.expect("device_atoken is not set"),
        };
        let args = CreateDeviceInstructionArgs {
            challenge: self.challenge.clone().expect("challenge is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `create_device` CPI accounts.
pub struct CreateDeviceCpiAccounts<'a, 'b> {
    /// The program derived address of the program account to create (seeds: ['Program'])
    pub program_pda: &'b solana_program::account_info::AccountInfo<'a>,
    /// The account paying for the storage fees
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// The system program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The SPL Token 2022 program
    pub token2022_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The associated token program
    pub ata_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// DePHY ID program id
    pub dephy_id_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// PDA as product vendor (seeds: ['VENDOR'])
    pub vendor: &'b solana_program::account_info::AccountInfo<'a>,
    /// PDA of the product mint account (program: dephy_id, seeds: ['DePHY_ID-PRODUCT', vendor, PRODUCT_NAME])
    pub product_mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// The device's owner
    pub owner: &'b solana_program::account_info::AccountInfo<'a>,
    /// PDA of the virtual device (seeds: ['DEVICE', owner])
    pub device: &'b solana_program::account_info::AccountInfo<'a>,
    /// The associated token account of the product
    pub product_atoken: &'b solana_program::account_info::AccountInfo<'a>,
    /// The mint account of the device
    pub device_mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// The associated token account for the device
    pub device_atoken: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `create_device` CPI instruction.
pub struct CreateDeviceCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The program derived address of the program account to create (seeds: ['Program'])
    pub program_pda: &'b solana_program::account_info::AccountInfo<'a>,
    /// The account paying for the storage fees
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// The system program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The SPL Token 2022 program
    pub token2022_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The associated token program
    pub ata_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// DePHY ID program id
    pub dephy_id_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// PDA as product vendor (seeds: ['VENDOR'])
    pub vendor: &'b solana_program::account_info::AccountInfo<'a>,
    /// PDA of the product mint account (program: dephy_id, seeds: ['DePHY_ID-PRODUCT', vendor, PRODUCT_NAME])
    pub product_mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// The device's owner
    pub owner: &'b solana_program::account_info::AccountInfo<'a>,
    /// PDA of the virtual device (seeds: ['DEVICE', owner])
    pub device: &'b solana_program::account_info::AccountInfo<'a>,
    /// The associated token account of the product
    pub product_atoken: &'b solana_program::account_info::AccountInfo<'a>,
    /// The mint account of the device
    pub device_mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// The associated token account for the device
    pub device_atoken: &'b solana_program::account_info::AccountInfo<'a>,
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
            program_pda: accounts.program_pda,
            payer: accounts.payer,
            system_program: accounts.system_program,
            token2022_program: accounts.token2022_program,
            ata_program: accounts.ata_program,
            dephy_id_program: accounts.dephy_id_program,
            vendor: accounts.vendor,
            product_mint: accounts.product_mint,
            owner: accounts.owner,
            device: accounts.device,
            product_atoken: accounts.product_atoken,
            device_mint: accounts.device_mint,
            device_atoken: accounts.device_atoken,
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
        let mut accounts = Vec::with_capacity(13 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.program_pda.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.payer.key,
            true,
        ));
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
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.dephy_id_program.key,
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
            *self.owner.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.device.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.product_atoken.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.device_mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.device_atoken.key,
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
            program_id: crate::PRODUCT_PROGRAM_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(13 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.program_pda.clone());
        account_infos.push(self.payer.clone());
        account_infos.push(self.system_program.clone());
        account_infos.push(self.token2022_program.clone());
        account_infos.push(self.ata_program.clone());
        account_infos.push(self.dephy_id_program.clone());
        account_infos.push(self.vendor.clone());
        account_infos.push(self.product_mint.clone());
        account_infos.push(self.owner.clone());
        account_infos.push(self.device.clone());
        account_infos.push(self.product_atoken.clone());
        account_infos.push(self.device_mint.clone());
        account_infos.push(self.device_atoken.clone());
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
///   0. `[writable]` program_pda
///   1. `[writable, signer]` payer
///   2. `[]` system_program
///   3. `[]` token2022_program
///   4. `[]` ata_program
///   5. `[]` dephy_id_program
///   6. `[]` vendor
///   7. `[]` product_mint
///   8. `[]` owner
///   9. `[]` device
///   10. `[writable]` product_atoken
///   11. `[writable]` device_mint
///   12. `[writable]` device_atoken
#[derive(Clone, Debug)]
pub struct CreateDeviceCpiBuilder<'a, 'b> {
    instruction: Box<CreateDeviceCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CreateDeviceCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(CreateDeviceCpiBuilderInstruction {
            __program: program,
            program_pda: None,
            payer: None,
            system_program: None,
            token2022_program: None,
            ata_program: None,
            dephy_id_program: None,
            vendor: None,
            product_mint: None,
            owner: None,
            device: None,
            product_atoken: None,
            device_mint: None,
            device_atoken: None,
            challenge: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// The program derived address of the program account to create (seeds: ['Program'])
    #[inline(always)]
    pub fn program_pda(
        &mut self,
        program_pda: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.program_pda = Some(program_pda);
        self
    }
    /// The account paying for the storage fees
    #[inline(always)]
    pub fn payer(&mut self, payer: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.payer = Some(payer);
        self
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
    /// DePHY ID program id
    #[inline(always)]
    pub fn dephy_id_program(
        &mut self,
        dephy_id_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.dephy_id_program = Some(dephy_id_program);
        self
    }
    /// PDA as product vendor (seeds: ['VENDOR'])
    #[inline(always)]
    pub fn vendor(
        &mut self,
        vendor: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.vendor = Some(vendor);
        self
    }
    /// PDA of the product mint account (program: dephy_id, seeds: ['DePHY_ID-PRODUCT', vendor, PRODUCT_NAME])
    #[inline(always)]
    pub fn product_mint(
        &mut self,
        product_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.product_mint = Some(product_mint);
        self
    }
    /// The device's owner
    #[inline(always)]
    pub fn owner(&mut self, owner: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.owner = Some(owner);
        self
    }
    /// PDA of the virtual device (seeds: ['DEVICE', owner])
    #[inline(always)]
    pub fn device(
        &mut self,
        device: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.device = Some(device);
        self
    }
    /// The associated token account of the product
    #[inline(always)]
    pub fn product_atoken(
        &mut self,
        product_atoken: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.product_atoken = Some(product_atoken);
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
    /// The associated token account for the device
    #[inline(always)]
    pub fn device_atoken(
        &mut self,
        device_atoken: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.device_atoken = Some(device_atoken);
        self
    }
    #[inline(always)]
    pub fn challenge(&mut self, challenge: u8) -> &mut Self {
        self.instruction.challenge = Some(challenge);
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
            challenge: self
                .instruction
                .challenge
                .clone()
                .expect("challenge is not set"),
        };
        let instruction = CreateDeviceCpi {
            __program: self.instruction.__program,

            program_pda: self
                .instruction
                .program_pda
                .expect("program_pda is not set"),

            payer: self.instruction.payer.expect("payer is not set"),

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

            dephy_id_program: self
                .instruction
                .dephy_id_program
                .expect("dephy_id_program is not set"),

            vendor: self.instruction.vendor.expect("vendor is not set"),

            product_mint: self
                .instruction
                .product_mint
                .expect("product_mint is not set"),

            owner: self.instruction.owner.expect("owner is not set"),

            device: self.instruction.device.expect("device is not set"),

            product_atoken: self
                .instruction
                .product_atoken
                .expect("product_atoken is not set"),

            device_mint: self
                .instruction
                .device_mint
                .expect("device_mint is not set"),

            device_atoken: self
                .instruction
                .device_atoken
                .expect("device_atoken is not set"),
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
    program_pda: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token2022_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ata_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    dephy_id_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    vendor: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    product_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    owner: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    device: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    product_atoken: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    device_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    device_atoken: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    challenge: Option<u8>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}