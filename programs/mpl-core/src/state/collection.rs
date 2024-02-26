use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug, ShankAccount)]
pub struct CollectionData {
    pub key: Key,                 //1
    pub update_authority: Pubkey, //32
    pub owner: Pubkey,            //32
    pub name: String,             //4
    pub uri: String,              //4
    pub num_minted: u64,          //8
    pub num_migrated: u64,        //8
    pub current_size: u64,        //8
}