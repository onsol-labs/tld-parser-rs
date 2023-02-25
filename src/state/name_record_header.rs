use std::io::Error;

use anchor_lang::AnchorDeserialize;
use solana_sdk::pubkey::Pubkey;

/**
 * Name Record Header: home of the name accounts.
 */
#[derive(Clone, Debug, AnchorDeserialize, Eq, PartialEq)]
pub struct NameRecordHeader {
    /// Names are hierarchical.  `parent_name` contains the account address of the parent
    /// name, or `Pubkey::default()` if no parent exists.
    pub parent_name: Pubkey,

    /// The owner of this name, could be nft_record if the domain has been wrapped.
    pub owner: Pubkey,

    /// The class of data this account represents
    /// If `Pubkey::default()` the data is unspecified.
    /// 'class' is a reserved word that cannot be used here.
    pub nclass: Pubkey,

    /// time is in unix timestamp
    /// programs must respect the expires_at.
    /// the data is invalid unless renewed by the owner or
    /// new owner comes with new data replacing the old one.
    /// defaults to 0
    pub expires_at: u64,
    /// is the name account data valid (not expired)
    pub is_valid: bool,
    // data sits here owner/parent name owner can add as many data as they please.
}

impl<'a> NameRecordHeader {
    pub const HASH_PREFIX: &'a str = "ALT Name Service";
    pub const LEN: usize = 8 + std::mem::size_of::<NameRecordHeader>() + 80_usize;

    /// deserializes the name record header if it exists.
    /// will throw an error due to deserialization error.
    pub fn deserialize_name_record(src: &[u8]) -> Result<NameRecordHeader, Error> {
        let mut p = &src[8..];
        let name_record_header = NameRecordHeader::deserialize(&mut p)?;
        Ok(name_record_header)
    }

    /// deserialized data if it exists.
    /// will throw an error due to len not being found.
    pub fn deserialize_data_string(src: &[u8]) -> Result<String, Error> {
        let p = &src[Self::LEN..];
        let len = u32::from_le_bytes(p[0..4].try_into().unwrap()) as usize;

        let domain_data = String::from(std::str::from_utf8(&p[4..4 + len]).unwrap());
        Ok(domain_data)
    }

    /// deserialized reverse lookup domain name if it exists.
    /// will throw an error due to len not being found.
    pub fn deserialize_reverse_lookup_domain_name(src: &[u8]) -> Result<String, Error> {
        let p = &src[Self::LEN..src.len()];
        let domain_data = String::from(std::str::from_utf8(&p).unwrap());
        Ok(domain_data)
    }
}
