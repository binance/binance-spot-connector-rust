use crate::http::Signature;
use base64::encode;
use ed25519_dalek::pkcs8::DecodePrivateKey;
use ed25519_dalek::SigningKey;
use ed25519_dalek::{Signature as Ed25519Signature, Signer};
use hmac::{Hmac, Mac};
use sha2::{digest::InvalidLength, Sha256};

pub fn sign(payload: &str, signature: &Signature) -> Result<String, InvalidLength> {
    match signature {
        Signature::Hmac(signature) => sign_hmac(payload, &signature.api_secret),
        Signature::Ed25519(signature) => sign_ed25519(payload, &signature.key),
    }
}

fn sign_hmac(payload: &str, key: &str) -> Result<String, InvalidLength> {
    let mut mac = Hmac::<Sha256>::new_from_slice(key.to_string().as_bytes())?;

    mac.update(payload.to_string().as_bytes());
    let result = mac.finalize();
    Ok(format!("{:x}", result.into_bytes()))
}

fn sign_ed25519(payload: &str, key: &str) -> Result<String, InvalidLength> {
    let private = SigningKey::from_pkcs8_pem(key);

    let signing_key: SigningKey = SigningKey::from_bytes(&private.unwrap().to_bytes());
    let signature: Ed25519Signature = signing_key.sign(&payload.to_string().into_bytes());
    Ok(encode(signature.to_bytes()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn sign_payload_with_hmac_test() {
        let payload = "symbol=LTCBTC&side=BUY&type=LIMIT&timeInForce=GTC&quantity=1&price=0.1&recvWindow=5000&timestamp=1499827319559";
        let key = "NhqPtmdSJYdKjVHjA7PZj4Mge3R5YNiP1e3UZjInClVN65XAbvqqM6A7H5fATj0j";

        let signature = super::sign_hmac(payload, key).unwrap();

        assert_eq!(
            signature,
            "c8db56825ae71d6d79447849e617115f4a920fa2acdcab2b053c4b2838bd6b71".to_owned()
        );
    }

    #[test]
    fn sign_payload_with_ed25519_test() {
        let payload = "type=SPOT&timestamp=1685686334211";
        let key = "-----BEGIN PRIVATE KEY-----
MC4CAQAwBQYDK2VwBCIEIE4rJ0goma1nbu1d8T1dp//0pe40jnf8tghwRhsSY4Bk
-----END PRIVATE KEY-----";
        let signature = super::sign_ed25519(payload, key).unwrap();
        assert_eq!(
            signature,
            "E4nWIl3yUJgJFL6LoWImsrEwNegMiN9SN1FWKw+P3xXkJ2T/MtSq3Cg7fVnOGFWxTBX6vrTJJNoZnVtAgs1CAQ==".to_owned()
        );
    }
}
