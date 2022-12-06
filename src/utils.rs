use crate::http::Signature;
use base64::encode;
use hmac::{Hmac, Mac};
use rsa::pkcs1v15::SigningKey;
use rsa::{pkcs8::DecodePrivateKey, RsaPrivateKey};
use sha2::{digest::InvalidLength, Sha256};
use signature::RandomizedSigner;

pub fn sign(payload: &str, signature: &Signature) -> Result<String, InvalidLength> {
    match signature {
        Signature::Hmac(signature) => sign_hmac(payload, &signature.api_secret),
        Signature::Rsa(signature) => {
            sign_rsa(payload, &signature.key, signature.password.as_deref())
        }
    }
}

fn sign_hmac(payload: &str, key: &str) -> Result<String, InvalidLength> {
    let mut mac = Hmac::<Sha256>::new_from_slice(key.to_string().as_bytes())?;

    mac.update(payload.to_string().as_bytes());
    let result = mac.finalize();
    Ok(format!("{:x}", result.into_bytes()))
}

fn sign_rsa(payload: &str, key: &str, password: Option<&str>) -> Result<String, InvalidLength> {
    let mut rng = rand::thread_rng();
    let private_key = match password {
        Some(password) => RsaPrivateKey::from_pkcs8_encrypted_pem(key, password),
        None => RsaPrivateKey::from_pkcs8_pem(key),
    }
    .map_err(|e| format!("{}", e))
    .expect("failed to generate a key");
    let signing_key = SigningKey::<Sha256>::new_with_prefix(private_key);

    let signature = signing_key.sign_with_rng(&mut rng, &payload.to_string().into_bytes());
    Ok(encode(signature))
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
    fn sign_payload_with_unprotected_rsa_test() {
        let payload = "symbol=LTCBTC&side=BUY&type=LIMIT&timeInForce=GTC&quantity=1&price=0.1&recvWindow=5000&timestamp=1499827319559";
        let key = "-----BEGIN PRIVATE KEY-----
MIIEvAIBADANBgkqhkiG9w0BAQEFAASCBKYwggSiAgEAAoIBAQDiYtnbq3oSo2XG
etLbpN0YwV/f4XFFQ4A1tYn4sikzj12NRR+G1eyEpoRIY8QylIhpMGQ4uXzDIF+f
Ma3xoaSDqVBFSKpZDTprNWsT7aaiNrdwlfH5jO/qbU35qdWg4Q/sXJv97KUGCX2y
xzSyOcppD5vLzSDX5QO320qbqsrr8Dcvt4CE3PHIcbC/4ogQ1J2MI2XMD5xq1Lo9
XIFXpB1eJjNheHxRYMgBdK1VsukcUT65M+FRJXKytob0honax/wODrZVs4n00VlK
jsbxoOimipst+L+ksQJOjUGakFjUY2c7dmras1dhYfzi1tF0Fv3hgcASGQo0ELwi
5L8RTJerAgMBAAECggEAGVO909mnWpZ1OHNdS71yDubzdvSd24XlvhyFRxGHkoEj
kj59fHRKdby7Z+3hcVc0u6/yRnZaAuqISCMitrnC0ggLNvUeOcBFHOFpDV21GsjM
VQgxfh3KaYu5AYmCoPEM+wCRYyMqgzZkna+zsYzNnFtmrrJQTlcMx3D9M9Uzxplv
c7vpseEapCF6VBXY74E/GIguV3P4BmVxB1nF+kbw672bR2bTzOZyWsBfWzMZFqZF
MpYWS3NOxIb8AfoCUcMBEfc4AvPDvjY/RgiMMJBEZ9tx+zg5LRY/NDci5+LA0cJU
qU3IjJNypl3Ej7FybEPnvD9PeLDrBCS9vEhYfBweYQKBgQD0dXojvdtV9jrYuGnT
h0V/NMoVYyKQDIjV0zodcJRG3nMtPYkFEMj/oU/3jjbLrEgQ/nyYoXIOBPRUTne/
raxGPAh7clr2z0JSaxs2xy5KUzsQkcZwUz3KsuD3j1NlQx0pxrtFdbrqRR++5V8T
X6V0X85EWPBdONkuTShGNxc+PwKBgQDtEvJ0LY9Y4UsmC6e8T6mz/xufeKCIKm5E
5KjWkCkBSAaTcV/IIECo6OqtRloHB97d0BvzBR4oNgHda+NYOF/n7NRNP2QSlsFA
0pvZB8uwVmNy6g0BntfDdpS3kqbuGk/uj1IyBXEe+eMrgPWA/OYnmTKV1whsj2gX
OfDRhY1jlQKBgDVIygeyYXW6RxdoEwEdciOJESdaRXValhTthrRWDqvn3vkUeaJW
B0nZEoImSIFamxbBMc1fG2o84DjkJavtIijo+vxVb8huYsluOS9WeRuQSsaQz806
B7UACpNLdWUUHO43OCiqwcRAruvAOVoSthDItUAmlrXrJ0O1Y7ryVSmnAoGADGD2
4/YVDBEHeX8WTRDjzZA+gv8rcrC0fUhrQHjai0SVg1FiLPLyLV+sEUwhG0jf7DkX
nILh5jubsVIW1t5qs5N8KmRq90WD5byZgqWTjyuTmcZ602DV51DjxwoTSU96aPx2
0EHUfEeUAwuCxi9uXRb0uzo1KGX8i2ntFyOgFFkCgYB8d9736biLl+gfa9QQtheE
aT4vocxAvIMk2+nvQQYRWHwLhfcp8crrR0HsYMTpLrObNCx9gjMnw7sGxeLwNZKP
tuq8jWUGRSW0eRDpE/vm/Fc3m2zx1i7el/Y68uLHiiDUaOrSXyqa3rRSABxbl6mt
lPRDw/3ItYIPZMBrCoiEqA==
-----END PRIVATE KEY-----";

        let signature = super::sign_rsa(payload, key, None).unwrap();

        assert_eq!(
            signature,
            "BfXmFj/XbqhD63gvR3oyb5D+rKQEJTF/DiqjUwph+omj56hMo5wjRevWGfiy/3hYPrjI9TVn8pqVp5zaO6VVukz5FSApDCbciZM1YZNS7ooNF9RWbeYmZoZTQMosJL6oMS+ax8077r6BWaNKiCnUa5mTNPiHEvT9AQHXvQww73d7TqPzZwTGF5KlwWX8MH0KyTYbxNuWHZbYpYoYXGk/ZTCtehPYntOP7QS/8/hgQC3G8UZDM6x5RYwrlVMoANKWQE0g/ir+UzXU9rbrzD9aH/1NrvEZBgdKyK6WEpRRY5YhZt1N1Rlo289Z1/wcYPX0BIzXAnDKveJFRE6DWy2dtQ==".to_owned()
        );
    }

    #[test]
    fn sign_payload_with_protected_rsa_test() {
        let payload = "symbol=LTCBTC&side=BUY&type=LIMIT&timeInForce=GTC&quantity=1&price=0.1&recvWindow=5000&timestamp=1499827319559";
        let key = "-----BEGIN ENCRYPTED PRIVATE KEY-----
MIIFLTBXBgkqhkiG9w0BBQ0wSjApBgkqhkiG9w0BBQwwHAQIXS3wC2Alj+8CAggA
MAwGCCqGSIb3DQIJBQAwHQYJYIZIAWUDBAEqBBDJFDZqxZArHRyklGfgXRKJBIIE
0OINEOzodwbD0wbRgQqnS5Ccn35dD4xrjP0rLAWT0ZyeCnkMaktcxLHCtMGEDFby
WCjQbY3k82HJCOjd4POvnxErm3zlCY72YOoIJLuLYh1yq/7VZxgOq2Wg9PzljdGz
Y87w+eAdFMh+a/1NFqtwAkZb3CcPvFBFjWxwck5+VNxvUD4eRyStHAsbkQl5riGw
AoeSF9Z9S1qKnanZq7oVHfGJ9+D7yhPYKVJjx3VI3813aBPh8DlnilVdet4VJSpz
Z1icl+T+0IALyryW/ADA8tMHnxz0RBZGTwakvjt7rkNuoA/SP+jfcLZtQYpW7wB/
ub0GCJr4J76NlcyTXICasbM0V+6axnHyq65lVyAQgB4OVWJR7s0hfC5SqVDhKzAe
1v2ELwiTaC+jBS+Hf1rMbCMSIX8Q+fqiEmErFp1NJkIkqEp5qFN84cjZPSP9qJ51
76PHGXcib/G5bX/Y7TLmLUlqGOSWBy+5NWDL7ULVUtrwIvboMGE0KWrsfXOMXkmM
UNaMXIhBxnVmfW3E8KGpgj8bjlRlPFNLyZSGKYgA4HqiIagbcULUJ1MWqCtcrRHF
ATv9jD66BNxGhgfljzBDiIaj5Fg/bxDNp7J8M5ayj6ZcmYgp0z+x6lQGyXlJsQUO
CjE9x0H4JDvtnRBZsiPafDVQaeUi7kH/cB7Ec4/WwyWPt3NV8wkurvqxuzmiUpez
Z+Z14yESq1btKkC7oCPtsPeBMjcm/erwb7IM38CKaYvoT8fkritAsJEpwZ0F0c4d
LK3P+icDWtpBPXkfjwuqE1WeDI3AIQhbdQcMtZ/81lBVfu2Aib6/XoD9RG335PTB
RlbzOXtWcPQ6aA0NrT29Whq26vXhzA2dpAP1C7loJMO6O4QwlrmeTdZUr2++khQ1
vyg86gB3FT2cJDmtjFSd1YJN16//mDq5AW32F7Izc+P0rn66YzjF3evmBoMfsEGN
WuYWOSfnRiu+V186WyB0kGWiiflxfPPBI+5bsKRrjjLatvgWioNsDlr7xyeQKxtk
IWOl0s3XjYGMoTCg1/gqaSDPJ+dcmam9U97Ki9l+dos5eDzaGOYIVwTn+MpwwVmq
eNIaKfkwQACmMDAPk5qftOLDwXwm+vDUqLwpyDn44Bb6njgBdzg8jLNPcv3eAQXs
xw18+eYk6z7Jx9qH+RbrfB4TiZYngr7nS+zdGbnCFQu5L/vaS8+PpPLYmCnmcFnN
x09BgR3UPL/kOuyZQdgKDl3DxSvbFZpp6HuzzLqx5fZFhCI2/D/SJXTQTGFU3u+A
cbuZ4k2NEeI39RhRk+/j4gohjVYVGtOORgS26s9h4uth9tgrkS0NLb7aTdkmJqoq
/LNsWCyJD7bw45lndLlesumpDd61VfbS/uoG4zxEvF2qvH8Z+xosM3DxadwIddI2
whK0vUx8EY9iFzuJQooCi7BbmRcJLQ0vnfCkqZx+OsFoeb8p/x/ZNEEkh+KB2dra
HiYfPVQk8pNMGzz7qBP+mkMYp17o6LLr0Qr9JWmGh9jHOPocYKfdaO+cV4U5xNTX
0VRPncv56t8YNcSSX++rkH90NoFYmYUHpI88ynwHQfmI1aJAbHkpKcuqB9Ec8dxy
LoKyC9IAZpLT6GVprBYIZo4xVBC8fAjxGGGnDPpNnyrP
-----END ENCRYPTED PRIVATE KEY-----";
        let password = "password";

        let signature = super::sign_rsa(payload, key, Some(password)).unwrap();

        assert_eq!(
            signature,
            "BfXmFj/XbqhD63gvR3oyb5D+rKQEJTF/DiqjUwph+omj56hMo5wjRevWGfiy/3hYPrjI9TVn8pqVp5zaO6VVukz5FSApDCbciZM1YZNS7ooNF9RWbeYmZoZTQMosJL6oMS+ax8077r6BWaNKiCnUa5mTNPiHEvT9AQHXvQww73d7TqPzZwTGF5KlwWX8MH0KyTYbxNuWHZbYpYoYXGk/ZTCtehPYntOP7QS/8/hgQC3G8UZDM6x5RYwrlVMoANKWQE0g/ir+UzXU9rbrzD9aH/1NrvEZBgdKyK6WEpRRY5YhZt1N1Rlo289Z1/wcYPX0BIzXAnDKveJFRE6DWy2dtQ==".to_owned()
        );
    }
}
