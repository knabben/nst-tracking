use crypto::digest::Digest;
use crypto::sha2::Sha512;

pub fn hashed_value(value: &str) -> String {
  let mut hasher = Sha512::new();
  hasher.input_str(value);
  let hex = hasher.result_str();
  hex
}
