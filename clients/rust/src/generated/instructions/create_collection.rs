//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use crate::generated::types::Plugin;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct CreateCollection {
    /// The address of the new asset
    pub collection_address: solana_program::pubkey::Pubkey,
    /// The authority of the new asset
    pub update_authority: Option<solana_program::pubkey::Pubkey>,
    /// The account paying for the storage fees
    pub payer: solana_program::pubkey::Pubkey,
    /// The owner of the new asset. Defaults to the authority if not present.
    pub owner: Option<solana_program::pubkey::Pubkey>,
    /// The system program
    pub system_program: solana_program::pubkey::Pubkey,
}

impl CreateCollection {
    pub fn instruction(
        &self,
        args: CreateCollectionInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: CreateCollectionInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(5 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.collection_address,
            true,
        ));
        if let Some(update_authority) = self.update_authority {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                update_authority,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_CORE_PROGRAM_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.payer, true,
        ));
        if let Some(owner) = self.owner {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                owner, false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_CORE_PROGRAM_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = CreateCollectionInstructionData::new().try_to_vec().unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::MPL_CORE_PROGRAM_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
struct CreateCollectionInstructionData {
    discriminator: u8,
}

impl CreateCollectionInstructionData {
    fn new() -> Self {
        Self { discriminator: 1 }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateCollectionInstructionArgs {
    pub name: String,
    pub uri: String,
    pub plugins: Vec<Plugin>,
}

/// Instruction builder.
#[derive(Default)]
pub struct CreateCollectionBuilder {
    collection_address: Option<solana_program::pubkey::Pubkey>,
    update_authority: Option<solana_program::pubkey::Pubkey>,
    payer: Option<solana_program::pubkey::Pubkey>,
    owner: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    name: Option<String>,
    uri: Option<String>,
    plugins: Option<Vec<Plugin>>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl CreateCollectionBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// The address of the new asset
    #[inline(always)]
    pub fn collection_address(
        &mut self,
        collection_address: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.collection_address = Some(collection_address);
        self
    }
    /// `[optional account]`
    /// The authority of the new asset
    #[inline(always)]
    pub fn update_authority(
        &mut self,
        update_authority: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.update_authority = update_authority;
        self
    }
    /// The account paying for the storage fees
    #[inline(always)]
    pub fn payer(&mut self, payer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.payer = Some(payer);
        self
    }
    /// `[optional account]`
    /// The owner of the new asset. Defaults to the authority if not present.
    #[inline(always)]
    pub fn owner(&mut self, owner: Option<solana_program::pubkey::Pubkey>) -> &mut Self {
        self.owner = owner;
        self
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    /// The system program
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
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
    pub fn plugins(&mut self, plugins: Vec<Plugin>) -> &mut Self {
        self.plugins = Some(plugins);
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
        let accounts = CreateCollection {
            collection_address: self
                .collection_address
                .expect("collection_address is not set"),
            update_authority: self.update_authority,
            payer: self.payer.expect("payer is not set"),
            owner: self.owner,
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
        };
        let args = CreateCollectionInstructionArgs {
            name: self.name.clone().expect("name is not set"),
            uri: self.uri.clone().expect("uri is not set"),
            plugins: self.plugins.clone().expect("plugins is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `create_collection` CPI accounts.
pub struct CreateCollectionCpiAccounts<'a, 'b> {
    /// The address of the new asset
    pub collection_address: &'b solana_program::account_info::AccountInfo<'a>,
    /// The authority of the new asset
    pub update_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The account paying for the storage fees
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// The owner of the new asset. Defaults to the authority if not present.
    pub owner: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The system program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `create_collection` CPI instruction.
pub struct CreateCollectionCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The address of the new asset
    pub collection_address: &'b solana_program::account_info::AccountInfo<'a>,
    /// The authority of the new asset
    pub update_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The account paying for the storage fees
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// The owner of the new asset. Defaults to the authority if not present.
    pub owner: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The system program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: CreateCollectionInstructionArgs,
}

impl<'a, 'b> CreateCollectionCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: CreateCollectionCpiAccounts<'a, 'b>,
        args: CreateCollectionInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            collection_address: accounts.collection_address,
            update_authority: accounts.update_authority,
            payer: accounts.payer,
            owner: accounts.owner,
            system_program: accounts.system_program,
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
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.collection_address.key,
            true,
        ));
        if let Some(update_authority) = self.update_authority {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                *update_authority.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_CORE_PROGRAM_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.payer.key,
            true,
        ));
        if let Some(owner) = self.owner {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                *owner.key, false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_CORE_PROGRAM_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = CreateCollectionInstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::MPL_CORE_PROGRAM_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(5 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.collection_address.clone());
        if let Some(update_authority) = self.update_authority {
            account_infos.push(update_authority.clone());
        }
        account_infos.push(self.payer.clone());
        if let Some(owner) = self.owner {
            account_infos.push(owner.clone());
        }
        account_infos.push(self.system_program.clone());
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

/// `create_collection` CPI instruction builder.
pub struct CreateCollectionCpiBuilder<'a, 'b> {
    instruction: Box<CreateCollectionCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CreateCollectionCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(CreateCollectionCpiBuilderInstruction {
            __program: program,
            collection_address: None,
            update_authority: None,
            payer: None,
            owner: None,
            system_program: None,
            name: None,
            uri: None,
            plugins: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// The address of the new asset
    #[inline(always)]
    pub fn collection_address(
        &mut self,
        collection_address: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.collection_address = Some(collection_address);
        self
    }
    /// `[optional account]`
    /// The authority of the new asset
    #[inline(always)]
    pub fn update_authority(
        &mut self,
        update_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.update_authority = update_authority;
        self
    }
    /// The account paying for the storage fees
    #[inline(always)]
    pub fn payer(&mut self, payer: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.payer = Some(payer);
        self
    }
    /// `[optional account]`
    /// The owner of the new asset. Defaults to the authority if not present.
    #[inline(always)]
    pub fn owner(
        &mut self,
        owner: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.owner = owner;
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
    pub fn plugins(&mut self, plugins: Vec<Plugin>) -> &mut Self {
        self.instruction.plugins = Some(plugins);
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
        let args = CreateCollectionInstructionArgs {
            name: self.instruction.name.clone().expect("name is not set"),
            uri: self.instruction.uri.clone().expect("uri is not set"),
            plugins: self
                .instruction
                .plugins
                .clone()
                .expect("plugins is not set"),
        };
        let instruction = CreateCollectionCpi {
            __program: self.instruction.__program,

            collection_address: self
                .instruction
                .collection_address
                .expect("collection_address is not set"),

            update_authority: self.instruction.update_authority,

            payer: self.instruction.payer.expect("payer is not set"),

            owner: self.instruction.owner,

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct CreateCollectionCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    collection_address: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    update_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    owner: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    name: Option<String>,
    uri: Option<String>,
    plugins: Option<Vec<Plugin>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
