use crypto::digest::Digest;
use crypto::sha2::Sha512;
use sawtooth_sdk::signing::{Context, PrivateKey, PublicKey};

pub fn generate_key_pair(context: &Box<dyn Context>) -> (Box<dyn PrivateKey>, Box<dyn PublicKey>){
    let private_key = context.new_random_private_key().unwrap();
    let public_key = context.get_public_key(&*private_key).unwrap();

    println!("DEBUG: private_key: {:?}\npublic_key: {:?}", &*private_key.as_hex(), &*public_key.as_hex());

    (private_key, public_key)
}

pub fn hashed_value(value: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.input_str(value);
    let hex = hasher.result_str();
    hex
}
