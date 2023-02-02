use dotenv::var;
use lazy_static::lazy_static;
use dryoc::dryocsecretbox::*;
use serde::{Deserialize, Serialize};
use serde::de::{DeserializeOwned};

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedData {
    pub(crate) data: DryocSecretBox<Mac, Vec<u8>>,
    pub(crate) nonce: Nonce,
}

lazy_static! {
    static ref SECRET_STRING : String = var("secret").expect("secret not found");
}

fn get_secret() -> Vec<u8> {
    let mut secret = SECRET_STRING.as_bytes().to_vec();
    secret.resize(32, 0);
    secret
}

pub fn encrypt<T: Serialize>(data: &T) -> EncryptedData {
    let nonce = Nonce::gen();
    let secret = get_secret();
    let data = DryocSecretBox::encrypt_to_vecbox(&serde_json::to_string(&data).unwrap().as_bytes(), &nonce, &secret);
    EncryptedData { data, nonce }
}

pub fn decrypt<T: DeserializeOwned>(data: &EncryptedData) -> T {
    let secret = get_secret();
    let data = DryocSecretBox::decrypt_to_vec(&data.data, &data.nonce, &secret).unwrap();
    serde_json::from_str(&String::from_utf8(data).unwrap()).unwrap()
}

pub fn from_reader<T: DeserializeOwned, R: std::io::Read>(reader: R) -> Result<T, serde_json::Error> {
    let data: EncryptedData = serde_json::from_reader(reader)?;
    Ok(decrypt(&data))
}

pub fn to_writer<W: std::io::Write, T: Serialize>(writer: W, data: &T) -> Result<(), serde_json::Error> {
    let data = encrypt(data);
    serde_json::to_writer(writer, &data)
}
