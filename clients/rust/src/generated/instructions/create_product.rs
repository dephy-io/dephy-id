//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct CreateProduct {
    /// The system program
    pub system_program: solana_program::pubkey::Pubkey,
    /// The SPL Token 2022 program
    pub token_program2022: solana_program::pubkey::Pubkey,
    /// The account paying for the storage fees
    pub payer: solana_program::pubkey::Pubkey,
    /// Vendor account
    pub vendor: solana_program::pubkey::Pubkey,
    /// The product mint account
    pub product_mint: solana_program::pubkey::Pubkey,
}

impl CreateProduct {
    pub fn instruction(
        &self,
        args: CreateProductInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: CreateProductInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(5 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program2022,
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
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&CreateProductInstructionData::new()).unwrap();
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
pub struct CreateProductInstructionData {
    discriminator: u8,
}

impl CreateProductInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 2 }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateProductInstructionArgs {
    pub seed: [u8; 32],
    pub bump: u8,
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub additional_metadata: Vec<(String, String)>,
}

/// Instruction builder for `CreateProduct`.
///
/// ### Accounts:
///
///   0. `[optional]` system_program (default to `11111111111111111111111111111111`)
///   1. `[]` token_program2022
///   2. `[writable, signer]` payer
///   3. `[signer]` vendor
///   4. `[writable]` product_mint
#[derive(Clone, Debug, Default)]
pub struct CreateProductBuilder {
    system_program: Option<solana_program::pubkey::Pubkey>,
    token_program2022: Option<solana_program::pubkey::Pubkey>,
    payer: Option<solana_program::pubkey::Pubkey>,
    vendor: Option<solana_program::pubkey::Pubkey>,
    product_mint: Option<solana_program::pubkey::Pubkey>,
    seed: Option<[u8; 32]>,
    bump: Option<u8>,
    name: Option<String>,
    symbol: Option<String>,
    uri: Option<String>,
    additional_metadata: Option<Vec<(String, String)>>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl CreateProductBuilder {
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
    /// The account paying for the storage fees
    #[inline(always)]
    pub fn payer(&mut self, payer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.payer = Some(payer);
        self
    }
    /// Vendor account
    #[inline(always)]
    pub fn vendor(&mut self, vendor: solana_program::pubkey::Pubkey) -> &mut Self {
        self.vendor = Some(vendor);
        self
    }
    /// The product mint account
    #[inline(always)]
    pub fn product_mint(&mut self, product_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.product_mint = Some(product_mint);
        self
    }
    #[inline(always)]
    pub fn seed(&mut self, seed: [u8; 32]) -> &mut Self {
        self.seed = Some(seed);
        self
    }
    #[inline(always)]
    pub fn bump(&mut self, bump: u8) -> &mut Self {
        self.bump = Some(bump);
        self
    }
    #[inline(always)]
    pub fn name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }
    #[inline(always)]
    pub fn symbol(&mut self, symbol: String) -> &mut Self {
        self.symbol = Some(symbol);
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
        let accounts = CreateProduct {
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
            token_program2022: self
                .token_program2022
                .expect("token_program2022 is not set"),
            payer: self.payer.expect("payer is not set"),
            vendor: self.vendor.expect("vendor is not set"),
            product_mint: self.product_mint.expect("product_mint is not set"),
        };
        let args = CreateProductInstructionArgs {
            seed: self.seed.clone().expect("seed is not set"),
            bump: self.bump.clone().expect("bump is not set"),
            name: self.name.clone().expect("name is not set"),
            symbol: self.symbol.clone().expect("symbol is not set"),
            uri: self.uri.clone().expect("uri is not set"),
            additional_metadata: self
                .additional_metadata
                .clone()
                .expect("additional_metadata is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `create_product` CPI accounts.
pub struct CreateProductCpiAccounts<'a, 'b> {
    /// The system program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The SPL Token 2022 program
    pub token_program2022: &'b solana_program::account_info::AccountInfo<'a>,
    /// The account paying for the storage fees
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// Vendor account
    pub vendor: &'b solana_program::account_info::AccountInfo<'a>,
    /// The product mint account
    pub product_mint: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `create_product` CPI instruction.
pub struct CreateProductCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The system program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The SPL Token 2022 program
    pub token_program2022: &'b solana_program::account_info::AccountInfo<'a>,
    /// The account paying for the storage fees
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// Vendor account
    pub vendor: &'b solana_program::account_info::AccountInfo<'a>,
    /// The product mint account
    pub product_mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: CreateProductInstructionArgs,
}

impl<'a, 'b> CreateProductCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: CreateProductCpiAccounts<'a, 'b>,
        args: CreateProductInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            system_program: accounts.system_program,
            token_program2022: accounts.token_program2022,
            payer: accounts.payer,
            vendor: accounts.vendor,
            product_mint: accounts.product_mint,
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
        let mut accounts = Vec::with_capacity(5 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program2022.key,
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
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = borsh::to_vec(&CreateProductInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::DEPHY_ID_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(5 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.system_program.clone());
        account_infos.push(self.token_program2022.clone());
        account_infos.push(self.payer.clone());
        account_infos.push(self.vendor.clone());
        account_infos.push(self.product_mint.clone());
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

/// Instruction builder for `CreateProduct` via CPI.
///
/// ### Accounts:
///
///   0. `[]` system_program
///   1. `[]` token_program2022
///   2. `[writable, signer]` payer
///   3. `[signer]` vendor
///   4. `[writable]` product_mint
#[derive(Clone, Debug)]
pub struct CreateProductCpiBuilder<'a, 'b> {
    instruction: Box<CreateProductCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CreateProductCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(CreateProductCpiBuilderInstruction {
            __program: program,
            system_program: None,
            token_program2022: None,
            payer: None,
            vendor: None,
            product_mint: None,
            seed: None,
            bump: None,
            name: None,
            symbol: None,
            uri: None,
            additional_metadata: None,
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
    /// The account paying for the storage fees
    #[inline(always)]
    pub fn payer(&mut self, payer: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.payer = Some(payer);
        self
    }
    /// Vendor account
    #[inline(always)]
    pub fn vendor(
        &mut self,
        vendor: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.vendor = Some(vendor);
        self
    }
    /// The product mint account
    #[inline(always)]
    pub fn product_mint(
        &mut self,
        product_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.product_mint = Some(product_mint);
        self
    }
    #[inline(always)]
    pub fn seed(&mut self, seed: [u8; 32]) -> &mut Self {
        self.instruction.seed = Some(seed);
        self
    }
    #[inline(always)]
    pub fn bump(&mut self, bump: u8) -> &mut Self {
        self.instruction.bump = Some(bump);
        self
    }
    #[inline(always)]
    pub fn name(&mut self, name: String) -> &mut Self {
        self.instruction.name = Some(name);
        self
    }
    #[inline(always)]
    pub fn symbol(&mut self, symbol: String) -> &mut Self {
        self.instruction.symbol = Some(symbol);
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
        let args = CreateProductInstructionArgs {
            seed: self.instruction.seed.clone().expect("seed is not set"),
            bump: self.instruction.bump.clone().expect("bump is not set"),
            name: self.instruction.name.clone().expect("name is not set"),
            symbol: self.instruction.symbol.clone().expect("symbol is not set"),
            uri: self.instruction.uri.clone().expect("uri is not set"),
            additional_metadata: self
                .instruction
                .additional_metadata
                .clone()
                .expect("additional_metadata is not set"),
        };
        let instruction = CreateProductCpi {
            __program: self.instruction.__program,

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),

            token_program2022: self
                .instruction
                .token_program2022
                .expect("token_program2022 is not set"),

            payer: self.instruction.payer.expect("payer is not set"),

            vendor: self.instruction.vendor.expect("vendor is not set"),

            product_mint: self
                .instruction
                .product_mint
                .expect("product_mint is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct CreateProductCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program2022: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    vendor: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    product_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    seed: Option<[u8; 32]>,
    bump: Option<u8>,
    name: Option<String>,
    symbol: Option<String>,
    uri: Option<String>,
    additional_metadata: Option<Vec<(String, String)>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
