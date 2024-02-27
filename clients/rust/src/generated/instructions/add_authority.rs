//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use crate::generated::types::Authority;
use crate::generated::types::PluginType;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct AddAuthority {
    /// The address of the asset
    pub asset_address: solana_program::pubkey::Pubkey,
    /// The collection to which the asset belongs
    pub collection: Option<solana_program::pubkey::Pubkey>,
    /// The owner or delegate of the asset
    pub authority: solana_program::pubkey::Pubkey,
    /// The account paying for the storage fees
    pub payer: Option<solana_program::pubkey::Pubkey>,
    /// The system program
    pub system_program: solana_program::pubkey::Pubkey,
    /// The SPL Noop Program
    pub log_wrapper: Option<solana_program::pubkey::Pubkey>,
}

impl AddAuthority {
    pub fn instruction(
        &self,
        args: AddAuthorityInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: AddAuthorityInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(6 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.asset_address,
            false,
        ));
        if let Some(collection) = self.collection {
            accounts.push(solana_program::instruction::AccountMeta::new(
                collection, false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_CORE_PROGRAM_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        if let Some(payer) = self.payer {
            accounts.push(solana_program::instruction::AccountMeta::new(payer, true));
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
        if let Some(log_wrapper) = self.log_wrapper {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                log_wrapper,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_CORE_PROGRAM_ID,
                false,
            ));
        }
        accounts.extend_from_slice(remaining_accounts);
        let mut data = AddAuthorityInstructionData::new().try_to_vec().unwrap();
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
struct AddAuthorityInstructionData {
    discriminator: u8,
}

impl AddAuthorityInstructionData {
    fn new() -> Self {
        Self { discriminator: 5 }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AddAuthorityInstructionArgs {
    pub plugin_type: PluginType,
    pub new_authority: Authority,
}

/// Instruction builder.
#[derive(Default)]
pub struct AddAuthorityBuilder {
    asset_address: Option<solana_program::pubkey::Pubkey>,
    collection: Option<solana_program::pubkey::Pubkey>,
    authority: Option<solana_program::pubkey::Pubkey>,
    payer: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    log_wrapper: Option<solana_program::pubkey::Pubkey>,
    plugin_type: Option<PluginType>,
    new_authority: Option<Authority>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl AddAuthorityBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// The address of the asset
    #[inline(always)]
    pub fn asset_address(&mut self, asset_address: solana_program::pubkey::Pubkey) -> &mut Self {
        self.asset_address = Some(asset_address);
        self
    }
    /// `[optional account]`
    /// The collection to which the asset belongs
    #[inline(always)]
    pub fn collection(&mut self, collection: Option<solana_program::pubkey::Pubkey>) -> &mut Self {
        self.collection = collection;
        self
    }
    /// The owner or delegate of the asset
    #[inline(always)]
    pub fn authority(&mut self, authority: solana_program::pubkey::Pubkey) -> &mut Self {
        self.authority = Some(authority);
        self
    }
    /// `[optional account]`
    /// The account paying for the storage fees
    #[inline(always)]
    pub fn payer(&mut self, payer: Option<solana_program::pubkey::Pubkey>) -> &mut Self {
        self.payer = payer;
        self
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    /// The system program
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    /// `[optional account]`
    /// The SPL Noop Program
    #[inline(always)]
    pub fn log_wrapper(
        &mut self,
        log_wrapper: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.log_wrapper = log_wrapper;
        self
    }
    #[inline(always)]
    pub fn plugin_type(&mut self, plugin_type: PluginType) -> &mut Self {
        self.plugin_type = Some(plugin_type);
        self
    }
    #[inline(always)]
    pub fn new_authority(&mut self, new_authority: Authority) -> &mut Self {
        self.new_authority = Some(new_authority);
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
        let accounts = AddAuthority {
            asset_address: self.asset_address.expect("asset_address is not set"),
            collection: self.collection,
            authority: self.authority.expect("authority is not set"),
            payer: self.payer,
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
            log_wrapper: self.log_wrapper,
        };
        let args = AddAuthorityInstructionArgs {
            plugin_type: self.plugin_type.clone().expect("plugin_type is not set"),
            new_authority: self
                .new_authority
                .clone()
                .expect("new_authority is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `add_authority` CPI accounts.
pub struct AddAuthorityCpiAccounts<'a, 'b> {
    /// The address of the asset
    pub asset_address: &'b solana_program::account_info::AccountInfo<'a>,
    /// The collection to which the asset belongs
    pub collection: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The owner or delegate of the asset
    pub authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// The account paying for the storage fees
    pub payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The system program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The SPL Noop Program
    pub log_wrapper: Option<&'b solana_program::account_info::AccountInfo<'a>>,
}

/// `add_authority` CPI instruction.
pub struct AddAuthorityCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The address of the asset
    pub asset_address: &'b solana_program::account_info::AccountInfo<'a>,
    /// The collection to which the asset belongs
    pub collection: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The owner or delegate of the asset
    pub authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// The account paying for the storage fees
    pub payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The system program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The SPL Noop Program
    pub log_wrapper: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The arguments for the instruction.
    pub __args: AddAuthorityInstructionArgs,
}

impl<'a, 'b> AddAuthorityCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: AddAuthorityCpiAccounts<'a, 'b>,
        args: AddAuthorityInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            asset_address: accounts.asset_address,
            collection: accounts.collection,
            authority: accounts.authority,
            payer: accounts.payer,
            system_program: accounts.system_program,
            log_wrapper: accounts.log_wrapper,
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
        let mut accounts = Vec::with_capacity(6 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.asset_address.key,
            false,
        ));
        if let Some(collection) = self.collection {
            accounts.push(solana_program::instruction::AccountMeta::new(
                *collection.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_CORE_PROGRAM_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.authority.key,
            true,
        ));
        if let Some(payer) = self.payer {
            accounts.push(solana_program::instruction::AccountMeta::new(
                *payer.key, true,
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
        if let Some(log_wrapper) = self.log_wrapper {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                *log_wrapper.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_CORE_PROGRAM_ID,
                false,
            ));
        }
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = AddAuthorityInstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::MPL_CORE_PROGRAM_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(6 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.asset_address.clone());
        if let Some(collection) = self.collection {
            account_infos.push(collection.clone());
        }
        account_infos.push(self.authority.clone());
        if let Some(payer) = self.payer {
            account_infos.push(payer.clone());
        }
        account_infos.push(self.system_program.clone());
        if let Some(log_wrapper) = self.log_wrapper {
            account_infos.push(log_wrapper.clone());
        }
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

/// `add_authority` CPI instruction builder.
pub struct AddAuthorityCpiBuilder<'a, 'b> {
    instruction: Box<AddAuthorityCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> AddAuthorityCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(AddAuthorityCpiBuilderInstruction {
            __program: program,
            asset_address: None,
            collection: None,
            authority: None,
            payer: None,
            system_program: None,
            log_wrapper: None,
            plugin_type: None,
            new_authority: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// The address of the asset
    #[inline(always)]
    pub fn asset_address(
        &mut self,
        asset_address: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.asset_address = Some(asset_address);
        self
    }
    /// `[optional account]`
    /// The collection to which the asset belongs
    #[inline(always)]
    pub fn collection(
        &mut self,
        collection: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.collection = collection;
        self
    }
    /// The owner or delegate of the asset
    #[inline(always)]
    pub fn authority(
        &mut self,
        authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.authority = Some(authority);
        self
    }
    /// `[optional account]`
    /// The account paying for the storage fees
    #[inline(always)]
    pub fn payer(
        &mut self,
        payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.payer = payer;
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
    /// `[optional account]`
    /// The SPL Noop Program
    #[inline(always)]
    pub fn log_wrapper(
        &mut self,
        log_wrapper: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.log_wrapper = log_wrapper;
        self
    }
    #[inline(always)]
    pub fn plugin_type(&mut self, plugin_type: PluginType) -> &mut Self {
        self.instruction.plugin_type = Some(plugin_type);
        self
    }
    #[inline(always)]
    pub fn new_authority(&mut self, new_authority: Authority) -> &mut Self {
        self.instruction.new_authority = Some(new_authority);
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
        let args = AddAuthorityInstructionArgs {
            plugin_type: self
                .instruction
                .plugin_type
                .clone()
                .expect("plugin_type is not set"),
            new_authority: self
                .instruction
                .new_authority
                .clone()
                .expect("new_authority is not set"),
        };
        let instruction = AddAuthorityCpi {
            __program: self.instruction.__program,

            asset_address: self
                .instruction
                .asset_address
                .expect("asset_address is not set"),

            collection: self.instruction.collection,

            authority: self.instruction.authority.expect("authority is not set"),

            payer: self.instruction.payer,

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),

            log_wrapper: self.instruction.log_wrapper,
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct AddAuthorityCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    asset_address: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    collection: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    log_wrapper: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    plugin_type: Option<PluginType>,
    new_authority: Option<Authority>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
