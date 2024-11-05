use anyhow::{anyhow, Result};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct JwtClaims {
    pub iss: String,
    pub aud: String,
    pub iat: i32,
    pub exp: i32,
    pub lxm: String,
}

#[derive(Deserialize)]
pub struct JwtHeader {
    pub typ: String,
    pub alg: String,
}

pub(crate) fn validate(multibase_key: &str, signature: &[u8], content: &str) -> Result<()> {
    let (_, decoded_multibase_key) = multibase::decode(multibase_key)?;
    match &decoded_multibase_key[..2] {
        // secp256k1
        [0xe7, 0x01] => {
            let signature = ecdsa::Signature::from_slice(signature)?;
            let verifying_key =
                k256::ecdsa::VerifyingKey::from_sec1_bytes(&decoded_multibase_key[2..])?;
            ecdsa::signature::Verifier::verify(&verifying_key, content.as_bytes(), &signature)?;
            Ok(())
        }
        // p256
        [0x80, 0x24] => {
            let signature = ecdsa::Signature::from_slice(signature)?;
            let verifying_key =
                p256::ecdsa::VerifyingKey::from_sec1_bytes(&decoded_multibase_key[2..])?;
            ecdsa::signature::Verifier::verify(&verifying_key, content.as_bytes(), &signature)?;
            Ok(())
        }
        _ => Err(anyhow!(
            "invalid multibase: {:?}",
            &decoded_multibase_key[..2]
        )),
    }
}
