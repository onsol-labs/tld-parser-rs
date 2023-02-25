use anchor_lang::AnchorDeserialize;
use solana_sdk::pubkey::Pubkey;
use std::io::Error;

/**
 * ANS Main domain
 */
#[derive(Clone, Debug, AnchorDeserialize, Eq, PartialEq)]
pub struct MainDomain {
    /// name account of the main domain
    pub name_account: Pubkey,
    /// tld of the domain of interest
    pub tld: String,
    /// domain of the main domain
    pub domain: String,
}

impl MainDomain {
    pub const MAIN_DOMAIN_SIZE: usize = 8 + 32 + 10 + 10 + 100;

    /// deserializes the main domain if it exists.
    /// will throw an error due to deserialization error.
    pub fn deserialize_main_domain(src: &[u8]) -> Result<MainDomain, Error> {
        let mut p = &src[8..];
        let name_record_header = MainDomain::deserialize(&mut p)?;
        Ok(name_record_header)
    }
}
