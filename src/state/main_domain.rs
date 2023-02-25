use anchor_lang::AnchorDeserialize;
use solana_sdk::pubkey::Pubkey;
use std::io::Error;

/**
 * ANS Main domain
 */
#[derive(Clone, Debug, AnchorDeserialize, Eq, PartialEq)]
pub struct MainDomain {
    pub name_account: Pubkey,
    pub tld: String,
    pub domain: String,
}

impl MainDomain {
    pub const MAIN_DOMAIN_SIZE: usize = 8 + 32 + 10 + 10 + 100;
    pub fn deserialize_main_domain(src: &[u8]) -> Result<MainDomain, Error> {
        let mut p = &src[8..];
        let name_record_header = MainDomain::deserialize(&mut p)?;
        Ok(name_record_header)
    }
}
