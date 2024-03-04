//! A Tld Parser for parsing AllDomains ANS domains in the Solana blockchain.
//!

use std::str::FromStr;

use {
    solana_account_decoder::UiAccountEncoding,
    solana_client::{
        nonblocking::rpc_client::RpcClient,
        rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
        rpc_filter::Memcmp,
        rpc_filter::RpcFilterType,
    },
    solana_sdk::pubkey::Pubkey,
    spl_token_2022::{extension::StateWithExtensions, state::Account},
    std::{
        error::Error,
        sync::Arc,
        time::{SystemTime, UNIX_EPOCH},
    },
};
pub mod constants;
pub mod name_record_handler;
pub mod pda;
pub mod state;
pub mod types;
pub mod utils;
pub use {constants::*, pda::*, state::*, types::*, utils::*};

/**
 * Tld Parser in for ANS Protocol in Solana blockchain.
 */
pub struct TldParser {
    pub rpc_client: Arc<RpcClient>,
}

impl TldParser {
    /// Returns ANS Main Domain from user pubkey
    /// # Example
    ///
    /// ```
    /// use std::{
    ///    error::Error,
    ///    sync::Arc,
    ///    str::FromStr,
    /// };
    /// use solana_sdk::{pubkey, pubkey::Pubkey};
    /// use solana_client::{
    ///     nonblocking::rpc_client::RpcClient,
    ///     client_error::ClientError,
    /// };
    /// use tldparser::TldParser;
    ///
    /// const API_ENDPOINT: &str = "";
    /// #[tokio::main]
    /// async fn main () -> Result<(), Box<dyn Error>> {
    ///   let rpc_client = RpcClient::new(API_ENDPOINT.to_string());
    ///   let parser = TldParser {
    ///     rpc_client: Arc::new(rpc_client),
    ///   };
    ///   let owner = Pubkey::from_str("2EGGxj2qbNAJNgLCPKca8sxZYetyTjnoRspTPjzN2D67").unwrap();
    ///   let main_domain = parser.get_main_domain(&owner).await?;
    ///   Ok(())
    /// }
    /// ```
    pub async fn get_main_domain(
        &self,
        user_address: &Pubkey,
    ) -> Result<MainDomain, Box<dyn Error>> {
        let (main_domain_key, _) = find_main_domain(user_address);
        let main_domain_data = self.rpc_client.get_account_data(&main_domain_key).await?;
        let main_domain = MainDomain::deserialize_main_domain(main_domain_data.as_slice())?;
        Ok(main_domain)
    }
    /// Returns All Users ANS Domains Pubkeys from user pubkey
    /// # Example
    ///
    /// ```
    /// use std::{
    ///    error::Error,
    ///    sync::Arc,
    ///    str::FromStr,
    /// };
    /// use solana_sdk::{pubkey, pubkey::Pubkey};
    /// use solana_client::{
    ///     nonblocking::rpc_client::RpcClient,
    ///     client_error::ClientError,
    /// };
    /// use tldparser::TldParser;
    ///
    /// const API_ENDPOINT: &str = "";
    /// #[tokio::main]
    /// async fn main () -> Result<(), Box<dyn Error>> {
    ///   let rpc_client = RpcClient::new(API_ENDPOINT.to_string());
    ///   let parser = TldParser {
    ///     rpc_client: Arc::new(rpc_client),
    ///   };
    ///   let owner = Pubkey::from_str("2EGGxj2qbNAJNgLCPKca8sxZYetyTjnoRspTPjzN2D67").unwrap();
    ///   let all_domains = parser.get_all_user_domains(&owner).await?;
    ///   Ok(())
    /// }
    /// ```
    pub async fn get_all_user_domains(
        &self,
        user_address: &Pubkey,
    ) -> Result<Vec<Pubkey>, Box<dyn Error>> {
        let memcmp = RpcFilterType::Memcmp(Memcmp::new_base58_encoded(40, user_address.as_ref()));
        let rpc_config = RpcAccountInfoConfig {
            encoding: Some(UiAccountEncoding::Base64),
            data_slice: None,
            commitment: None,
            min_context_slot: None,
        };
        let config = RpcProgramAccountsConfig {
            filters: Some(vec![memcmp]),
            account_config: rpc_config,
            with_context: None,
        };
        let all_accounts = self
            .rpc_client
            .get_program_accounts_with_config(&ANS_PROGRAM_ID, config)
            .await?;
        let name_account_keys = all_accounts.into_iter().map(|(pubkey, _)| pubkey).collect();
        Ok(name_account_keys)
    }
    /// Returns All Users Domains Pubkeys for a specific tld from user pubkey
    /// # Example
    ///
    /// ```
    /// use std::{
    ///    error::Error,
    ///    sync::Arc,
    ///    str::FromStr,
    /// };
    /// use solana_sdk::{pubkey, pubkey::Pubkey};
    /// use solana_client::{
    ///     nonblocking::rpc_client::RpcClient,
    ///     client_error::ClientError,
    /// };
    /// use tldparser::TldParser;
    ///
    /// const API_ENDPOINT: &str = "";
    /// #[tokio::main]
    /// async fn main () -> Result<(), Box<dyn Error>> {
    ///   let rpc_client = RpcClient::new(API_ENDPOINT.to_string());
    ///   let parser = TldParser {
    ///     rpc_client: Arc::new(rpc_client),
    ///   };
    ///   let owner = Pubkey::from_str("2EGGxj2qbNAJNgLCPKca8sxZYetyTjnoRspTPjzN2D67").unwrap();
    ///   let all_domains_from_abc = parser.get_all_user_domains_from_tld(&owner, &".abc".to_string()).await?;
    ///   Ok(())
    /// }
    /// ```
    pub async fn get_all_user_domains_from_tld(
        &self,
        user_address: &Pubkey,
        tld: &String,
    ) -> Result<Vec<Pubkey>, Box<dyn Error>> {
        let parent_name_account = get_name_parent_from_tld(tld);
        let memcmp_parent =
            RpcFilterType::Memcmp(Memcmp::new_base58_encoded(8, parent_name_account.as_ref()));
        let memcmp_user =
            RpcFilterType::Memcmp(Memcmp::new_base58_encoded(40, user_address.as_ref()));
        let rpc_config = RpcAccountInfoConfig {
            encoding: Some(UiAccountEncoding::Base64),
            data_slice: None,
            commitment: None,
            min_context_slot: None,
        };
        let config = RpcProgramAccountsConfig {
            filters: Some(vec![memcmp_parent, memcmp_user]),
            account_config: rpc_config,
            with_context: None,
        };
        let all_tld_accounts = self
            .rpc_client
            .get_program_accounts_with_config(&ANS_PROGRAM_ID, config)
            .await?;
        let name_account_keys = all_tld_accounts
            .into_iter()
            .map(|(pubkey, _)| pubkey)
            .collect();
        Ok(name_account_keys)
    }

    /// Returns the owner pubkey from domain name e.g. "miester.abc"
    /// # Example
    ///
    /// ```
    /// use std::{
    ///    error::Error,
    ///    sync::Arc,
    ///    str::FromStr,
    /// };
    /// use solana_sdk::{pubkey, pubkey::Pubkey};
    /// use solana_client::{
    ///     nonblocking::rpc_client::RpcClient,
    ///     client_error::ClientError,
    /// };
    /// use tldparser::TldParser;
    ///
    /// const API_ENDPOINT: &str = "";
    /// #[tokio::main]
    /// async fn main () -> Result<(), Box<dyn Error>> {
    ///   let rpc_client = RpcClient::new(API_ENDPOINT.to_string());
    ///   let parser = TldParser {
    ///     rpc_client: Arc::new(rpc_client),
    ///   };
    ///   let owner_of_domain = Pubkey::from_str("2EGGxj2qbNAJNgLCPKca8sxZYetyTjnoRspTPjzN2D67").unwrap();
    ///   let owner = parser.get_owner_from_domain_tld(&"miester.abc".to_string()).await?;
    ///   assert_eq!(owner, owner_of_domain);
    ///   Ok(())
    /// }
    /// ```
    pub async fn get_owner_from_domain_tld(
        &self,
        domain_tld: &String,
    ) -> Result<Pubkey, Box<dyn Error>> {
        let domain_tld_split: Vec<&str> = domain_tld.split('.').collect();
        let domain = domain_tld_split[0];
        let dot = ".".to_owned();
        let tld = dot + domain_tld_split[1];
        let parent_name_account = get_name_parent_from_tld(&tld);
        let (name_account_key, _) =
            find_name_account_from_name(&domain.to_string(), None, Some(&parent_name_account));
        let name_account_data = self.rpc_client.get_account_data(&name_account_key).await?;
        let mut name_account =
            NameRecordHeader::deserialize_name_record(name_account_data.as_slice())?;
        if name_account.expires_at > 0 {
            let time_now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            // grace period  = 45 days * 24 hours * 60 minutes * 60 seconds = 3_888_000 seconds
            let grace_period = 45 * 24 * 60 * 60;
            // added grace period = 45 days in unix_timestamp (seconds)
            if time_now + grace_period > name_account.expires_at {
                name_account.is_valid = false;
                name_account.owner = Pubkey::default();
            } else {
                name_account.is_valid = true;
            }
        }
        let mut owner = name_account.owner;
        let (tld_house_key, _) = find_tld_house(&tld);
        let (name_house_key, _) = find_name_house(&tld_house_key);
        // check whether domain is wrapped.
        let nft_record_key = find_nft_record(&name_account_key, &name_house_key).0;
        if owner == nft_record_key {
            let nft_record_data_vec = self.rpc_client.get_account_data(&nft_record_key).await?;
            let nft_record = NftRecord::from_account_info(&nft_record_data_vec)?;
            let response =
                get_token_largest_accounts(&self.rpc_client, &nft_record.nft_mint_account).await?;
            let associated_token_account =
                Pubkey::from_str(&response.value.first().unwrap().address).unwrap();
            let associated_token_account_data = self
                .rpc_client
                .get_account_data(&associated_token_account)
                .await?;

            let ata_data = &associated_token_account_data;
            if let Ok(associated_token_account_data_account) =
                StateWithExtensions::<Account>::unpack(&ata_data)
            {
                owner = associated_token_account_data_account.base.owner;
            }
        }
        Ok(owner)
    }

    /// Returns the name_record_header from domain name e.g. "miester.abc"
    /// # Example
    ///
    /// ```
    /// use std::{
    ///    error::Error,
    ///    sync::Arc,
    ///    str::FromStr,
    /// };
    /// use solana_sdk::{pubkey, pubkey::Pubkey};
    /// use solana_client::{
    ///     nonblocking::rpc_client::RpcClient,
    ///     client_error::ClientError,
    /// };
    /// use tldparser::TldParser;
    ///
    /// const API_ENDPOINT: &str = "";
    /// #[tokio::main]
    /// async fn main () -> Result<(), Box<dyn Error>> {
    ///   let rpc_client = RpcClient::new(API_ENDPOINT.to_string());
    ///   let parser = TldParser {
    ///     rpc_client: Arc::new(rpc_client),
    ///   };
    ///   let owner_of_domain = Pubkey::from_str("2EGGxj2qbNAJNgLCPKca8sxZYetyTjnoRspTPjzN2D67").unwrap();
    ///   let name_record_header = parser.get_name_record_from_domain_tld(&"miester.abc".to_string()).await?;
    ///   assert_eq!(name_record_header.owner, owner_of_domain);
    ///   Ok(())
    /// }
    /// ```
    pub async fn get_name_record_from_domain_tld(
        &self,
        domain_tld: &String,
    ) -> Result<NameRecordHeader, Box<dyn Error>> {
        let domain_tld_split: Vec<&str> = domain_tld.split('.').collect();
        let domain = domain_tld_split[0];
        let dot = ".".to_owned();
        let tld = dot + domain_tld_split[1];
        let parent_name_account = get_name_parent_from_tld(&tld);
        let (name_account_key, _) =
            find_name_account_from_name(&domain.to_string(), None, Some(&parent_name_account));
        let name_account_data = self.rpc_client.get_account_data(&name_account_key).await?;
        let mut name_account =
            NameRecordHeader::deserialize_name_record(name_account_data.as_slice())?;
        if name_account.expires_at > 0 {
            let time_now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            // grace period  = 45 days * 24 hours * 60 minutes * 60 seconds = 3_888_000 seconds
            let grace_period = 45 * 24 * 60 * 60;
            if time_now + grace_period > name_account.expires_at {
                name_account.is_valid = true
            }
        }
        Ok(name_account)
    }

    /// Returns the name_record_header from name_account
    /// # Example
    ///
    /// ```
    /// use std::{
    ///    error::Error,
    ///    sync::Arc,
    ///    str::FromStr,
    /// };
    /// use solana_sdk::{pubkey, pubkey::Pubkey};
    /// use solana_client::{
    ///     nonblocking::rpc_client::RpcClient,
    ///     client_error::ClientError,
    /// };
    /// use tldparser::TldParser;
    ///
    /// const API_ENDPOINT: &str = "";
    /// #[tokio::main]
    /// async fn main () -> Result<(), Box<dyn Error>> {
    ///   let rpc_client = RpcClient::new(API_ENDPOINT.to_string());
    ///   let parser = TldParser {
    ///     rpc_client: Arc::new(rpc_client),
    ///   };
    ///   let owner_of_domain = Pubkey::from_str("2EGGxj2qbNAJNgLCPKca8sxZYetyTjnoRspTPjzN2D67").unwrap();
    ///   let name_account: Pubkey = pubkey!("9YzfCEHb62bQ47snUyjkxhC9Eb6y7CSodK3m8CKWstjV");
    ///   let name_record_header = parser.get_name_record_from_name_account(&name_account).await?;
    ///   assert_eq!(name_record_header.owner, owner_of_domain);
    ///   Ok(())
    /// }
    /// ```
    pub async fn get_name_record_from_name_account(
        &self,
        name_account: &Pubkey,
    ) -> Result<NameRecordHeader, Box<dyn Error>> {
        let name_account_data = self.rpc_client.get_account_data(name_account).await?;
        let mut name_account =
            NameRecordHeader::deserialize_name_record(name_account_data.as_slice())?;
        if name_account.expires_at > 0 {
            let time_now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            
            // grace period  = 45 days * 24 hours * 60 minutes * 60 seconds = 3_888_000 seconds
            let grace_period = 45 * 24 * 60 * 60;

            if time_now + grace_period > name_account.expires_at {
                name_account.is_valid = true
            }
        }
        Ok(name_account)
    }
    /// Returns the tld from parent_name
    /// # Example
    ///
    /// ```
    /// use std::{
    ///    error::Error,
    ///    sync::Arc,
    ///    str::FromStr,
    /// };
    /// use solana_sdk::{pubkey, pubkey::Pubkey};
    /// use solana_client::{
    ///     nonblocking::rpc_client::RpcClient,
    ///     client_error::ClientError,
    /// };
    /// use tldparser::TldParser;
    ///
    /// const API_ENDPOINT: &str = "";
    /// #[tokio::main]
    /// async fn main () -> Result<(), Box<dyn Error>> {
    ///   let rpc_client = RpcClient::new(API_ENDPOINT.to_string());
    ///   let parser = TldParser {
    ///     rpc_client: Arc::new(rpc_client),
    ///   };
    ///   let parent_name: Pubkey = pubkey!("3pSeaEVTcKLkXPCpZHDpHUMWAogYFZgKSiVtyvqcgo8a");
    ///   let tld = parser.get_tld_from_parent_account(&parent_name).await?;
    ///   assert_eq!(tld, ".abc".to_string());
    ///   Ok(())
    /// }
    /// ```
    pub async fn get_tld_from_parent_account(
        &self,
        parent_account: &Pubkey,
    ) -> Result<String, Box<dyn Error>> {
        let name_parent_data = self.rpc_client.get_account_data(parent_account).await?;
        let name_parent = NameRecordHeader::deserialize_name_record(name_parent_data.as_slice())?;
        let tld_house_data = self.rpc_client.get_account_data(&name_parent.owner).await?;
        // let tld = tld_house_data[];
        let tld_len_start = 8 + 32 + 32 + 32;
        let tld_len_end = 8 + 32 + 32 + 32 + 4;
        let tld_len = u32::from_le_bytes(
            tld_house_data[tld_len_start..tld_len_end]
                .try_into()
                .unwrap(),
        ) as usize;

        Ok(String::from(
            std::str::from_utf8(&tld_house_data[tld_len_end..tld_len_end + tld_len])
                .unwrap()
                .trim_matches(char::from(0)),
        ))
    }
    /// Returns the domain from a known name class or tld_house
    /// # Example
    ///
    /// ```
    /// use std::{
    ///    error::Error,
    ///    sync::Arc,
    ///    str::FromStr,
    /// };
    /// use solana_sdk::{pubkey, pubkey::Pubkey};
    /// use solana_client::{
    ///     nonblocking::rpc_client::RpcClient,
    ///     client_error::ClientError,
    /// };
    /// use tldparser::{pda::*, TldParser};
    ///
    /// const API_ENDPOINT: &str = "";
    /// #[tokio::main]
    /// async fn main () -> Result<(), Box<dyn Error>> {
    ///   let rpc_client = RpcClient::new(API_ENDPOINT.to_string());
    ///   let parser = TldParser {
    ///     rpc_client: Arc::new(rpc_client),
    ///   };
    ///   let name_account: Pubkey = pubkey!("9YzfCEHb62bQ47snUyjkxhC9Eb6y7CSodK3m8CKWstjV");
    ///   let (tld_house, _) = find_tld_house(&".abc".to_string());
    ///   let domain = parser.reverse_lookup_name_account_with_known_name_class(&name_account, &tld_house).await?;
    ///   assert_eq!(domain, "miester".to_string());
    ///   Ok(())
    /// }
    /// ```
    pub async fn reverse_lookup_name_account_with_known_name_class(
        &self,
        name_account: &Pubkey,
        parent_account_owner: &Pubkey,
    ) -> Result<String, Box<dyn Error>> {
        let reverse_lookup_hash = get_hashed_name(&name_account.to_string());
        let (reverse_lookup_key, _) = find_name_account_from_hashed_name(
            &reverse_lookup_hash,
            Some(parent_account_owner),
            None,
        );
        let reverse_lookup_data = self
            .rpc_client
            .get_account_data(&reverse_lookup_key)
            .await?;

        let domain_name =
            NameRecordHeader::deserialize_reverse_lookup_domain_name(&reverse_lookup_data).unwrap();
        Ok(domain_name)
    }

    /// Returns the domain name from name_account
    /// slower than having to know the name class from before
    /// because it does 2 more rpc calls than with known name_class
    ///
    /// # Example
    ///
    /// ```
    /// use std::{
    ///    error::Error,
    ///    sync::Arc,
    ///    str::FromStr,
    /// };
    /// use solana_sdk::{pubkey, pubkey::Pubkey};
    /// use solana_client::{
    ///     nonblocking::rpc_client::RpcClient,
    ///     client_error::ClientError,
    /// };
    /// use tldparser::{pda::*, TldParser};
    ///
    /// const API_ENDPOINT: &str = "";
    /// #[tokio::main]
    /// async fn main () -> Result<(), Box<dyn Error>> {
    ///   let rpc_client = RpcClient::new(API_ENDPOINT.to_string());
    ///   let parser = TldParser {
    ///     rpc_client: Arc::new(rpc_client),
    ///   };
    ///   let name_account: Pubkey = pubkey!("9YzfCEHb62bQ47snUyjkxhC9Eb6y7CSodK3m8CKWstjV");
    ///   let domain = parser.reverse_lookup_name_account(&name_account).await?;
    ///   assert_eq!(domain, "miester".to_string());
    ///   Ok(())
    /// }
    /// ```
    pub async fn reverse_lookup_name_account(
        &self,
        name_account: &Pubkey,
    ) -> Result<String, Box<dyn Error>> {
        let name_record_header = self.get_name_record_from_name_account(name_account).await?;
        let tld = self
            .get_tld_from_parent_account(&name_record_header.parent_name)
            .await?;
        // name_class
        let (tld_house, _) = find_tld_house(&tld);
        let reverse_lookup_hash = get_hashed_name(&name_account.to_string());
        let (reverse_lookup_key, _) =
            find_name_account_from_hashed_name(&reverse_lookup_hash, Some(&tld_house), None);
        let reverse_lookup_data = self
            .rpc_client
            .get_account_data(&reverse_lookup_key)
            .await?;
        let domain_len_start = 200;
        let domain_len_end = reverse_lookup_data.len();

        let domain_name = String::from(
            std::str::from_utf8(&reverse_lookup_data[domain_len_start..domain_len_end]).unwrap(),
        );
        Ok(domain_name)
    }
}
