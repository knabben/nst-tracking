use crypto::digest::Digest;
use crypto::sha2::Sha512;

const FAMILY_NAME: &str = "trade";
const AGENT: &str = "ae";

pub fn get_supply_chain_prefix() -> String {
    let mut sha = Sha512::new();
    sha.input_str(&FAMILY_NAME);
    sha.result_str()[..6].to_string()
}