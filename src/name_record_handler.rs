use std::io::Error;

use crate::{constants::*, pda::*, utils::*};
use solana_sdk::pubkey::Pubkey;

pub fn get_domain_key(domain_tld: &str, record: bool) -> Result<DomainKeyResult, Error> {
    let domain_tld_split: Vec<&str> = domain_tld.split('.').collect();
    if domain_tld_split.len() == 3 {
        // handles subdomains
        let tld = format!(".{}", domain_tld_split[2]);
        let domain = domain_tld_split[1];
        let sub_domain = domain_tld_split[0];
        // parent key
        let parent_key = _get_name_account(&tld, None).0;
        // domain key
        let domain_key = _get_name_account(&domain.to_string(), Some(&parent_key)).0;
        // Sub domain
        let prefix = if record { "1" } else { "0" };
        let sub = format!("{}{}", prefix, sub_domain);
        let (pubkey, hashed) = _get_name_account(&sub, Some(&domain_key));
        return Ok(DomainKeyResult {
            pubkey,
            hashed,
            is_sub: true,
            parent: Some(domain_key),
            is_sub_record: false,
        });
    } else if domain_tld_split.len() == 4 && record {
        // handles four-level subdomain
        let tld = format!(".{}", domain_tld_split[3]);
        let domain = domain_tld_split[2];
        let sub_domain = domain_tld_split[1];
        let multi_level_sub_domain = domain_tld_split[0];
        // parent key
        let parent_key = _get_name_account(&tld, None).0;
        // domain key
        let domain_key = _get_name_account(&domain.to_string(), Some(&parent_key)).0;
        // Sub domain has to be added when we create subdomains for users which are not records
        let sub_key = _get_name_account(&format!("\0{}", sub_domain), Some(&domain_key)).0;
        // Sub record
        let record_prefix = "1";
        let (pubkey, hashed) = _get_name_account(
            &format!("{}{}", record_prefix, multi_level_sub_domain),
            Some(&sub_key),
        );
        return Ok(DomainKeyResult {
            pubkey,
            hashed,
            is_sub: true,
            parent: Some(domain_key),
            is_sub_record: true,
        });
    } else if domain_tld_split.len() > 4 {
        panic!("Invalid derivation input, found more than 4 level subdomain");
    }
    // just a regular domain_tld
    let tld_name = format!(".{}", domain_tld_split[1]);
    let parent_key_domain_account = _get_name_account(&tld_name, None).0;
    let domain = domain_tld_split[0];
    let (pubkey, hashed) = _get_name_account(&domain.to_string(), Some(&parent_key_domain_account));
    Ok(DomainKeyResult {
        pubkey,
        hashed,
        is_sub: false,
        parent: None,
        is_sub_record: false,
    })
}

#[derive(Debug)]
pub struct DomainKeyResult {
    pub pubkey: Pubkey,
    pub hashed: Vec<u8>,
    pub is_sub: bool,
    pub parent: Option<Pubkey>,
    pub is_sub_record: bool,
}

fn _get_name_account(name: &String, parent: Option<&Pubkey>) -> (Pubkey, Vec<u8>) {
    let name_account;
    if parent.is_none() {
        let hashed_parentless = get_hashed_name(name);
        name_account =
            find_name_account_from_hashed_name(&hashed_parentless, None, Some(&ORIGIN_TLD_KEY)).0;
        return (name_account, hashed_parentless);
    }
    let hashed = get_hashed_name(name);
    name_account = find_name_account_from_hashed_name(&hashed, None, parent).0;
    (name_account, hashed)
}
