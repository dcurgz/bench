mod wire {
    include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
}

use rand::rngs::OsRng;
use ed25519_dalek::{
    PUBLIC_KEY_LENGTH,
    SECRET_KEY_LENGTH,
    KEYPAIR_LENGTH,
    SIGNATURE_LENGTH,
    SigningKey,
    Signature,
};

enum SigningKeyType {
    KEY_TYPE_UNSPECIFIED,
    KEY_TYPE_ED25519,
}



pub fn create_identity(
    key_type: SigningKeyType,
    initial_username: &str
) {
    let mut csprng = OsRng;
    let signing_key: SigningKey = SigningKey::generate(&mut csprng);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1 + 2 == 3, true);
    }
}
