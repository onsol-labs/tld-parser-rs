use solana_sdk::pubkey::Pubkey;

use crate::{constants::*, utils::*};

pub fn find_tld_state() -> (Pubkey, u8) {
    let tld_house_seeds = &[PDA_SEED.as_bytes()];
    Pubkey::find_program_address(tld_house_seeds, &TLD_HOUSE_PROGRAM_ID)
}

pub fn find_tld_house(tld: &String) -> (Pubkey, u8) {
    let tld_house_seeds = &[PREFIX.as_bytes(), tld.as_bytes()];
    Pubkey::find_program_address(tld_house_seeds, &TLD_HOUSE_PROGRAM_ID)
}

pub fn find_tld_house_treasury(tld: &String) -> (Pubkey, u8) {
    let tld_treasury_seeds = &[PREFIX.as_bytes(), tld.as_bytes(), TREASURY.as_bytes()];
    Pubkey::find_program_address(tld_treasury_seeds, &TLD_HOUSE_PROGRAM_ID)
}

pub fn find_main_domain(user: &Pubkey) -> (Pubkey, u8) {
    let main_domain_seeds = &[MAIN_DOMAIN_PREFIX.as_bytes(), user.as_ref()];
    Pubkey::find_program_address(main_domain_seeds, &TLD_HOUSE_PROGRAM_ID)
}

pub fn find_claimable_domain(tld_house: &Pubkey, domain_account: &Pubkey) -> (Pubkey, u8) {
    let claimable_domain_seeds = &[
        CLAIMABLE_DOMAIN_PREFIX.as_bytes(),
        tld_house.as_ref(),
        domain_account.as_ref(),
    ];
    Pubkey::find_program_address(claimable_domain_seeds, &TLD_HOUSE_PROGRAM_ID)
}
pub fn find_name_house(tld_house: &Pubkey) -> (Pubkey, u8) {
    let tld_house_bytes = tld_house.to_bytes();
    let name_house_seeds = &[NAME_HOUSE_PREFIX.as_bytes(), tld_house_bytes.as_ref()];
    Pubkey::find_program_address(name_house_seeds, &NAME_HOUSE_PROGRAM_ID)
}

pub fn find_nft_record(name_account: &Pubkey, name_house_account: &Pubkey) -> (Pubkey, u8) {
    let name_house_account_bytes = name_house_account.to_bytes();
    let name_account_bytes = name_account.to_bytes();
    let nft_record_seeds = &[
        NFT_RECORD_PREFIX.as_bytes(),
        name_house_account_bytes.as_ref(),
        name_account_bytes.as_ref(),
    ];
    Pubkey::find_program_address(nft_record_seeds, &NAME_HOUSE_PROGRAM_ID)
}

pub fn find_mint_address(name_account: &Pubkey, name_house_account: &Pubkey) -> (Pubkey, u8) {
    let name_house_account_bytes = name_house_account.to_bytes();
    let name_account_bytes = name_account.to_bytes();
    let mint_address_seeds = &[
        NAME_HOUSE_PREFIX.as_bytes(),
        name_house_account_bytes.as_ref(),
        name_account_bytes.as_ref(),
    ];
    Pubkey::find_program_address(mint_address_seeds, &NAME_HOUSE_PROGRAM_ID)
}

pub fn find_collection_mint_address(tld_house: &Pubkey) -> (Pubkey, u8) {
    let tld_house_bytes = tld_house.to_bytes();
    let collection_mint_address_seeds = &[COLLECTION_PREFIX.as_bytes(), tld_house_bytes.as_ref()];
    Pubkey::find_program_address(collection_mint_address_seeds, &NAME_HOUSE_PROGRAM_ID)
}

pub fn find_name_account_from_hashed_name(
    hashed_name: &Vec<u8>,
    name_class_opt: Option<&Pubkey>,
    name_parent_opt: Option<&Pubkey>,
) -> (Pubkey, u8) {
    let [hash_seeds, name_class_seed, name_parent_seed] = get_name_service_seeds_from_hashed_name(
        hashed_name.to_vec(),
        name_class_opt,
        name_parent_opt,
    );
    let checked_account_seeds: &[&[u8]] =
        &[&hash_seeds[..], &name_class_seed[..], &name_parent_seed[..]];

    Pubkey::find_program_address(checked_account_seeds, &ANS_PROGRAM_ID)
}

pub fn find_name_account_from_name(
    name: &String,
    name_class_opt: Option<&Pubkey>,
    name_parent_opt: Option<&Pubkey>,
) -> (Pubkey, u8) {
    let [hash_seeds, name_class_seed, name_parent_seed] =
        get_name_service_seeds_from_name(name, name_class_opt, name_parent_opt);
    let checked_account_seeds: &[&[u8]] =
        &[&hash_seeds[..], &name_class_seed[..], &name_parent_seed[..]];

    Pubkey::find_program_address(checked_account_seeds, &ANS_PROGRAM_ID)
}
