use tldparser::*;
use {
    solana_client::nonblocking::rpc_client::RpcClient,
    solana_sdk::{pubkey, pubkey::Pubkey},
    std::{error::Error, sync::Arc},
};

#[tokio::test]
async fn all_lib_functions() -> Result<(), Box<dyn Error>> {
    /// intializations and constants
    const API_ENDPOINT: &str = "";

    let rpc_client = RpcClient::new(API_ENDPOINT.to_string());
    let parser = TldParser {
        rpc_client: Arc::new(rpc_client),
    };
    let owner: Pubkey = pubkey!("2EGGxj2qbNAJNgLCPKca8sxZYetyTjnoRspTPjzN2D67");
    let parent_account: Pubkey = pubkey!("3pSeaEVTcKLkXPCpZHDpHUMWAogYFZgKSiVtyvqcgo8a");
    let name_account: Pubkey = pubkey!("9YzfCEHb62bQ47snUyjkxhC9Eb6y7CSodK3m8CKWstjV");
    let abc = ".abc".to_string();

    let result_all_user_domains = &parser.get_all_user_domains(&owner).await?;
    // println!(
    //     "result_all_user_domains: {:?} ",
    //     result_all_user_domains.len()
    // );
    assert_eq!(result_all_user_domains.len(), 18);

    let result_all_user_domains_from_tld =
        parser.get_all_user_domains_from_tld(&owner, &abc).await?;
    // println!(
    //     "result_all_user_domains_from_tld: {:?} ",
    //     result_all_user_domains_from_tld.len()
    // );
    assert_eq!(result_all_user_domains_from_tld.len(), 3);

    let result_owner_from_domain_tld = parser
        .get_owner_from_domain_tld(&"cicu.abc".to_string())
        .await?;
    // println!(
    //     "result_owner_from_domain_tld: {:?} ",
    //     result_owner_from_domain_tld
    // );
    assert_eq!(result_owner_from_domain_tld, owner);

    let result_name_record_from_domain_tld = parser
        .get_name_record_from_domain_tld(&"miester.abc".to_string())
        .await?;
    // println!(
    //     "result_name_record_from_domain_tld: {:?} ",
    //     result_name_record_from_domain_tld
    // );
    assert!(result_name_record_from_domain_tld.is_valid);

    let result_tld_from_parent_account =
        parser.get_tld_from_parent_account(&parent_account).await?;
    // println!(
    //     "result_tld_from_parent_account: {:?} ",
    //     result_tld_from_parent_account
    // );
    assert_eq!(result_tld_from_parent_account, abc);

    // name_class or tld_house
    let (tld_house, _) = find_tld_house(&abc);
    let result_reverse_lookup_domain_name_with_known_name_class = parser
        .reverse_lookup_name_account_with_known_name_class(&name_account, &tld_house)
        .await?;
    // println!(
    //     "result_reverse_lookup_domain_name: {:?} ",
    //     result_reverse_lookup_domain_name_with_known_name_class
    // );
    assert_eq!(
        result_reverse_lookup_domain_name_with_known_name_class,
        "miester".to_string()
    );

    let result_main_domain = parser.get_main_domain(&owner).await?;
    // println!("result_main_domain: {:?} ", result_main_domain);
    assert_eq!(result_main_domain.tld, abc);
    assert_eq!(result_main_domain.domain, "miester".to_string());

    let result_reverse_lookup_domain_name =
        parser.reverse_lookup_name_account(&name_account).await?;
    // println!("domain name: {:?} ", result_reverse_lookup_domain_name);
    assert_eq!(result_reverse_lookup_domain_name, "miester".to_string());
    // assert!(false);
    Ok(())
}
