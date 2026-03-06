pub mod wire {
    include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));

    pub use crate::wire::identity::Identity as Identity;
    pub use crate::wire::identity::ChainAction as ChainAction;
    pub use crate::wire::identity::SignedChainAction as SignedChainAction;
    pub use crate::wire::identity::RotateKeyAction as RotateKeyAction;
    pub use crate::wire::identity::DeviceAction as DeviceAction;
    pub use crate::wire::identity::UsernameAction as UsernameAction;
    pub use crate::wire::identity::UserMetadata as UserMetadata;
    pub use crate::wire::identity::SignedUserMetadata as SignedUserMetadata;
    pub use crate::wire::identity::UserIdentity as UserIdentity;
}

use std::str::FromStr;

use wire::identity::identity::KeyType as SigningKeyType;
use wire::identity::user_metadata::Version as UserMetadataVersion;
use wire::identity::chain_action::Action as ChainActionType;

// anyhow: Error context and helpers.
extern crate anyhow;
use anyhow::{Result};
// err_derive: Error type convenience definitions.
extern crate err_derive;
use err_derive::Error;

// protobuf: Wire protocol.
extern crate protobuf;
use protobuf::{EnumOrUnknown, Message, MessageField};

extern crate rand;
//use rand::rngs::SysRng;

extern crate sha256;
use sha256::Sha256Digest;

extern crate ed25519_dalek;
use ed25519_dalek::{
    //KEYPAIR_LENGTH,
    //SECRET_KEY_LENGTH,
    PUBLIC_KEY_LENGTH,
    SIGNATURE_LENGTH,
    Signature,
    SigningKey,
    Verifier,
    VerifyingKey,
    ed25519::signature::Signer
};

use crate::wire::SignedChainAction;

fn fingerprint(key: &[u8; PUBLIC_KEY_LENGTH]) -> String {
    let hash = key.digest();
    String::from_str(&hash[0..32]).unwrap()
}

#[derive(Debug, Error)]
pub enum ParserError {
    #[error(display = "a required field ({:?}) is not present", _0)]
    MissingRequiredField(&'static str),
    #[error(display = "a required array field ({:?}) has the wrong size. expected {:?}, got {:?}.", _0, _1, _2)]
    WronglySizedField {
        reason: &'static str,
        expected: usize,
        present: usize,
    },
    #[error(display = "a signature field is the wrong size. expected {:?}, got {:?}", SIGNATURE_LENGTH, _0)]
    WronglySizedSignature(usize),
    #[error(display = "an identity {:?} cannot be parsed", _0)]
    BadIdentity(String),
    #[error(display = "an signature from {:?} cannot be verified", _0)]
    BadSignature(String),
    #[error(display = "cannot write object to binary format")]
    BadObject(),
    #[error(display = "a binary payload cannot be parsed")]
    BinaryFormatError(),
}

struct Identity {
    public_key: [u8; PUBLIC_KEY_LENGTH],
    public_key_type: SigningKeyType,
    __src: Option<wire::Identity>,
}

trait Sign {
    fn sign(&self, bytes: &[u8]) -> [u8; SIGNATURE_LENGTH];
}

trait Verify {
    fn verify(&self, bytes: &[u8], signature: &[u8; SIGNATURE_LENGTH]) -> Result<(), ParserError>;
}

struct ChainKey {
    identity: Identity,
    private_key: SigningKey,
}

impl Sign for ChainKey {
    fn sign(&self, bytes: &[u8]) -> [u8; SIGNATURE_LENGTH] {
        let signature = self.private_key.sign(bytes);
        signature.to_bytes()
    }
}

impl Verify for Identity {
    fn verify(&self, bytes: &[u8], signature: &[u8; SIGNATURE_LENGTH]) -> Result<(), ParserError> {
        let fingerprint = fingerprint(&self.public_key);
        let signature = Signature::try_from(signature)
            .map_err(|_| ParserError::WronglySizedSignature(bytes.len()))?;
        let verifier = VerifyingKey::from_bytes(&self.public_key)
            .map_err(|_| ParserError::BadIdentity(fingerprint.clone()))?;
        verifier.verify(bytes, &signature)
            .map_err(|_| ParserError::BadSignature(fingerprint.clone()))?;
        Ok(())
    }
}

struct ChainAction {
    counter: u32,
    created_at: u64,
    device_origin: Identity,
    action_type: ChainActionType,
    prev_signature: Vec<u8>,
    //TODO payload
    __src: Option<wire::SignedChainAction>,
}

struct UserMetadata {
    version: UserMetadataVersion,
    updated_at: u64,
    display_name: String,
    avatar_hash: Vec<u8>,
    __src: Option<wire::SignedUserMetadata>,
}

struct UserIdentity {
    chain_key: Identity,
    action_chain: Vec<ChainAction>,
    metadata: UserMetadata,
    __src: Option<wire::UserIdentity>,
}

struct Parser {}

trait Encoder<W>
where W: Message {
    fn encode(&self) -> Result<W, ParserError>;
}

trait SignerEncoder<W, K>
where W: Message, K: Sign {
    fn encode(&self, _: &K) -> Result<W, ParserError>;
}

trait Decoder<T>
where Self: Message {
    fn decode(&self) -> Result<T, ParserError>;
}

trait VerifierDecoder<T>
where Self: Message {
    fn decode(&self, _: Identity) -> Result<T, ParserError>;
}

impl Encoder<wire::Identity> for Identity {
    fn encode(&self) -> Result<wire::Identity, ParserError> {
        let mut w = wire::Identity::new();
        w.public_key = Some(self.public_key.to_vec());
        w.public_key_type = Some(EnumOrUnknown::new(self.public_key_type));
        Ok(w)
    }
}

impl Decoder<Identity> for wire::Identity {
    fn decode(&self) -> Result<Identity, ParserError> {
        // check field presence: public_key.
        let public_key: Vec<u8> = self.public_key.clone()
            .ok_or(ParserError::MissingRequiredField("public_key"))?;
        // check field size: public_key.
        let public_key: &[u8; PUBLIC_KEY_LENGTH] = public_key.as_array::<PUBLIC_KEY_LENGTH>()
            .ok_or(ParserError::WronglySizedField {
                reason: "public_key",
                expected: PUBLIC_KEY_LENGTH,
                present: public_key.len(),
            })?;
        // check field presence: public_key_type.
        let public_key_type = self.public_key_type
            .and_then(|k| k.enum_value().ok())
            .ok_or(ParserError::MissingRequiredField("public_key_type"))?;
        let result = Identity {
            public_key: *public_key,
            public_key_type,
            __src: Some(self.clone()),
        };
        Ok(result)
    }
}

impl Encoder<wire::ChainAction> for ChainAction {
    fn encode(&self) -> Result<wire::ChainAction, ParserError> {
        let device_origin: wire::Identity = self.device_origin
            .encode()
            .map_err(|_| ParserError::BadObject())?;

        let mut unsigned = wire::ChainAction::new();
        unsigned.counter = Some(self.counter);
        unsigned.created_at = Some(self.created_at);
        unsigned.device_origin = Some(device_origin).into();
        unsigned.action_type = Some(EnumOrUnknown::new(self.action_type));
        unsigned.prev_signature = Some(self.prev_signature.to_vec());
        //TODO payload
        Ok(unsigned)
    }
}

impl Decoder<ChainAction> for wire::ChainAction {
    fn decode(&self) -> Result<ChainAction, ParserError> {
        //TODO: this will be done in UserIdentity decoding as a VerifierDecoder
        //let bytes = signed.chain_action_bytes.as_ref()
        //    .ok_or(ParserError::MissingRequiredField("chain_action_bytes"))?;
        //let action = wire::ChainAction::parse_from_bytes(&bytes).ok()
        //    .ok_or(ParserError::BinaryFormatError())?;
        // get signature
        //let signature = signed.chain_action_signature.as_ref()
        //    .ok_or(ParserError::MissingRequiredField("chain_action_signature"))?;
        //let signature = signature
        //    .as_array::<SIGNATURE_LENGTH>()
        //    .ok_or(ParserError::WronglySizedSignature(signature.len()))?;
        // verify signature
        //self.chain_key.identity.verify(&bytes, &signature)?;
        //
        let counter = self.counter
            .ok_or(ParserError::MissingRequiredField("counter"))?;
        let created_at = self.created_at
            .ok_or(ParserError::MissingRequiredField("created_at"))?;
        let device_origin = self.device_origin.as_ref()
            .ok_or(ParserError::MissingRequiredField("device_origin"))?;
        let device_origin = device_origin
            .decode()
            .ok()
            .ok_or(ParserError::BinaryFormatError())?;
        let action_type = self.action_type
            .and_then(|a| a.enum_value().ok())
            .ok_or(ParserError::MissingRequiredField("self_type"))?;
        let prev_signature = self.prev_signature.clone()
            .ok_or(ParserError::MissingRequiredField("prev_signature"))?;
        //TODO payload
        let result = ChainAction {
            counter,
            created_at,
            device_origin,
            action_type,
            prev_signature,
            __src: None,
        };
        Ok(result)
    }
}

impl SignerEncoder<wire::SignedChainAction, ChainKey> for ChainAction {
    fn encode(&self, chain_key: &ChainKey) -> Result<wire::SignedChainAction, ParserError> {
        let unsigned: wire::ChainAction = Encoder::encode(self)
            .map_err(|_| ParserError::BadObject())?;
        let mut signed = wire::SignedChainAction::new();
        let bytes = unsigned.write_to_bytes().unwrap();
        signed.chain_action_signature = Some(chain_key.sign(&bytes).to_vec());
        signed.chain_action_bytes     = Some(bytes); 
        Ok(signed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1 + 2 == 3, true);
    }
}
