use crate::{constants::*, name_record_handler::*, state::*, types::*};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{hash::hashv, pubkey::Pubkey};
use std::error::Error;

pub fn get_name_parent_from_tld(tld: &String) -> Pubkey {
    let parent_hashed_name = get_hashed_name(tld);
    let [parent_hash_seed, parent_name_class_seed, parent_name_parent_seed] =
        get_name_service_seeds_from_hashed_name(parent_hashed_name, None, Some(&ORIGIN_TLD_KEY));
    let parent_seeds: &[&[u8]] = &[
        &parent_hash_seed[..],
        &parent_name_class_seed[..],
        &parent_name_parent_seed[..],
    ];
    let (name_parent, _) = Pubkey::find_program_address(parent_seeds, &ANS_PROGRAM_ID);
    name_parent
}

pub fn get_name_service_seeds_from_hashed_name(
    hashed_name: Vec<u8>,
    name_class_opt: Option<&Pubkey>,
    name_parent_opt: Option<&Pubkey>,
) -> [Vec<u8>; 3] {
    let seeds_vec = hashed_name;
    let name_class = name_class_opt.cloned().unwrap_or_default();
    let name_parent = name_parent_opt.cloned().unwrap_or_default();

    [
        seeds_vec,
        name_class.to_bytes().to_vec(),
        name_parent.to_bytes().to_vec(),
    ]
}

pub fn get_hashed_name(name: &String) -> Vec<u8> {
    hashv(&[(NameRecordHeader::HASH_PREFIX.to_owned() + name).as_bytes()])
        .as_ref()
        .to_vec()
}

pub fn get_name_service_seeds_from_name(
    name: &String,
    name_class_opt: Option<&Pubkey>,
    name_parent_opt: Option<&Pubkey>,
) -> [Vec<u8>; 3] {
    let hashed_name: Vec<u8> = get_hashed_name(&name);
    let name_class = name_class_opt.cloned().unwrap_or_default();
    let name_parent = name_parent_opt.cloned().unwrap_or_default();

    [
        hashed_name,
        name_class.to_bytes().to_vec(),
        name_parent.to_bytes().to_vec(),
    ]
}

pub fn get_program_address(seeds_with_bump: &[&[u8]], program_id: &Pubkey) -> Pubkey {
    Pubkey::create_program_address(seeds_with_bump, program_id).unwrap()
}

// not implemented yet
pub async fn find_domain_name_records(
    client: &RpcClient,
    domain_tld: &str,
) -> Result<Option<NameRecordHeader>, Box<dyn Error>> {
    let multi_record_pubkeys = [
        (get_domain_key(
            &format!("{}.{}", get_record_string(Record::Url), domain_tld),
            true,
        ))?
        .pubkey,
        (get_domain_key(
            &format!("{}.{}", get_record_string(Record::IPFS), domain_tld),
            true,
        ))?
        .pubkey,
        (get_domain_key(
            &format!("{}.{}", get_record_string(Record::ARWV), domain_tld),
            true,
        ))?
        .pubkey,
        (get_domain_key(
            &format!("{}.{}", get_record_string(Record::SHDW), domain_tld),
            true,
        ))?
        .pubkey,
    ];

    let name_record_account_infos = client
        .get_multiple_accounts(multi_record_pubkeys.as_ref())
        .await?;

    for value in name_record_account_infos.into_iter().flatten() {
        if let Ok(name_record_data) =
            NameRecordHeader::deserialize_name_record(value.data.as_slice())
        {
            return Ok(Some(name_record_data));
        }
    }

    Ok(None)
}

pub async fn get_record(
    client: &RpcClient,
    domain_tld: &str,
    record: Record,
) -> Result<Option<String>, Box<dyn Error>> {
    let pubkey = (get_domain_key(
        &format!("{}.{}", get_record_string(record), domain_tld),
        true,
    ))?
    .pubkey;
    let name_record = client.get_account_data(&pubkey).await?;
    let record_data = NameRecordHeader::deserialize_data_string(&name_record);
    Ok(Some(record_data))
}
