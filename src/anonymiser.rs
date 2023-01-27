use sha2::{Sha256};
use sha2::Digest;

pub fn anonymise(s: &String) -> String {
    let hash = Sha256::digest(s);
    format!("{:X}", hash)
}


