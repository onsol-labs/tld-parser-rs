# TLD Parser (rust)

library to parse tld house domains via alternative name service (ANS) on the Solana Blockchain. 

- TLD Parser is in active development. 
- So all APIs are subject to change.

## Examples
current functions and how to use them. 

the library only works in mainnet. 

the devnet values are in constants.ts file

the example below is a replica of the tests in `tests` folder


## States
current state is the NameRecordHeader, it is the data retrieved from any ANS account.

the account structure:
- `parent_name: PublicKey;`

parent is a name account that can have many children (name accounts)
- `owner: PublicKey;`

name account owner can be default if the name account has expired
- `nclass: PublicKey;`

name class is an account that holds an account state (Main domain, DNS, Subdomains) or can be Publickey.default
- `expires_at: Date;`

the date by which the name account will expire. would be 0 if non expirable domains
- `is_valid: boolean;`

only valid for expirable domains