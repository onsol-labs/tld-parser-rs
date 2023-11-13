use std::io::Error;

use anchor_lang::AnchorDeserialize;
use solana_sdk::pubkey::Pubkey;

#[derive(AnchorDeserialize, Clone, PartialEq, Eq, Default, Debug)]
pub enum Tag {
    Uninitialized,
    #[default]
    ActiveRecord,
    InactiveRecord,
}

#[derive(Clone, Debug, AnchorDeserialize, Eq, PartialEq)]
pub struct NftRecord {
    /// Tag
    pub tag: Tag,

    /// bump
    pub bump: u8,

    /// Name account of the record
    pub name_account: Pubkey,

    /// Record owner
    pub owner: Pubkey,

    /// NFT mint
    pub nft_mint_account: Pubkey,

    /// tld house
    pub tld_house: Pubkey,
}

impl<'a> NftRecord {
    pub const PREFIX: &'a [u8; 10] = b"nft_record";
    pub const LEN: usize = 8 + 1 + 1 + 32 + 32 + 32 + 32 + 64;

    pub fn from_account_info(data_vec: &Vec<u8>) -> Result<NftRecord, Error> {
        let data = &data_vec as &[u8];
        let result = NftRecord::deserialize(&mut &data[8..])?;
        Ok(result)
    }

    pub fn is_active(&self) -> bool {
        self.tag == Tag::ActiveRecord
    }
}
