use std::io::Error;

use anchor_lang::AnchorDeserialize;
use solana_sdk::pubkey::Pubkey;

/**
 * Name Record Header: home of the name accounts.
 */
#[derive(Clone, Debug, AnchorDeserialize, Eq, PartialEq)]
pub struct NameRecordHeader {
    // Names are hierarchical.  `parent_name` contains the account address of the parent
    // name, or `Pubkey::default()` if no parent exists.
    pub parent_name: Pubkey,

    // The owner of this name
    pub owner: Pubkey,

    // The class of data this account represents
    // If `Pubkey::default()` the data is unspecified.
    // 'nclass' is a reserved word that cannot be used here.
    pub nclass: Pubkey,

    // time is in unix timestamp in UTC.
    // programs must respect the expiry_ts. after by which a rent is void.
    // the data is invalid unless extended by the owner or
    // new owner comes with new data replacing the old one.
    // defaults to 0
    pub expires_at: u64,
    // data sits here owner/parent name owner can add as many data as they please.
    pub is_valid: bool,
}

impl<'a> NameRecordHeader {
    pub const HASH_PREFIX: &'a str = "ALT Name Service";
    pub const LEN: usize = 8 + std::mem::size_of::<NameRecordHeader>() + 88_usize;

    pub fn deserialize_name_record(src: &[u8]) -> Result<NameRecordHeader, Error> {
        let mut p = &src[8..];
        let name_record_header = NameRecordHeader::deserialize(&mut p)?;
        Ok(name_record_header)
    }

    pub fn deserialize_data_string(src: &[u8]) -> String {
        let p = &src[Self::LEN..];
        let len = u32::from_le_bytes(p[0..4].try_into().unwrap()) as usize;

        let domain_data = String::from(std::str::from_utf8(&p[4..4 + len]).unwrap());
        domain_data
    }
}
