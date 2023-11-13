/**
 * List of ANS Records
 */
pub enum Record {
    IPFS,
    ARWV,
    SOL,
    ETH,
    BTC,
    LATTICA,
    LTC,
    DOGE,
    Email,
    Url,
    Discord,
    Github,
    Reddit,
    Twitter,
    Telegram,
    Pic,
    SHDW,
    POINT,
}
/**
 * Retrieve the string version of the enum of ANS Records
 */
pub fn get_record_string(record: Record) -> String {
    match record {
        Record::IPFS => "IPFS".to_string(),
        Record::ARWV => "ARWV".to_string(),
        Record::SOL => "SOL".to_string(),
        Record::ETH => "ETH".to_string(),
        Record::BTC => "BTC".to_string(),
        Record::LATTICA => "Lattica".to_string(),
        Record::LTC => "LTC".to_string(),
        Record::DOGE => "DOGE".to_string(),
        Record::Email => "email".to_string(),
        Record::Url => "url".to_string(),
        Record::Discord => "discord".to_string(),
        Record::Github => "github".to_string(),
        Record::Reddit => "reddit".to_string(),
        Record::Twitter => "twitter".to_string(),
        Record::Telegram => "telegram".to_string(),
        Record::Pic => "pic".to_string(),
        Record::SHDW => "SHDW".to_string(),
        Record::POINT => "POINT".to_string(),
    }
}
