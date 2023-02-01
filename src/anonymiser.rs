use sha2::Digest;
use sha2::Sha256;

pub fn anonymise(s: &String) -> String {
    let hash = Sha256::digest(s);
    format!("{:X}", hash)
}
