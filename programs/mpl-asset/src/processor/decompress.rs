use borsh::{BorshDeserialize, BorshSerialize};
use mpl_utils::assert_signer;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke,
    program_memory::sol_memcpy, rent::Rent, system_instruction, system_program, sysvar::Sysvar,
};

use crate::{
    error::MplAssetError,
    instruction::accounts::CreateAccounts,
    state::{Asset, Compressible, DataState, HashedAsset, Key},
};

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct DecompressArgs {}

pub(crate) fn decompress<'a>(
    accounts: &'a [AccountInfo<'a>],
    args: DecompressArgs,
) -> ProgramResult {
    Ok(())
}