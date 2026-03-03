mod wire {
    include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
}

use crate::wire::identity::identity::KeyType as SigningKeyType;
use crate::wire::identity::user_metadata::Version as UserMetadataVersion;
use crate::wire::identity::chain_action::Action as ChainActionType;

// anyhow: Error context and helpers.
extern crate anyhow;
use anyhow::{Context, Result};
// err_derive: Error type convenience definitions.
extern crate err_derive;
use err_derive::Error;

// protobuf: Wire protocol.
extern crate protobuf;
use protobuf::EnumOrUnknown;

extern crate rand;
use rand::rngs::SysRng;

use std::fmt;

extern crate ed25519_dalek;
use ed25519_dalek::{
    PUBLIC_KEY_LENGTH,
    SECRET_KEY_LENGTH,
    KEYPAIR_LENGTH,
    SIGNATURE_LENGTH,
    SigningKey,
    Signature,
};

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
}

pub trait Wire {
    type WireType;

    fn to_wire(&self) -> Self::WireType;
    fn from_wire(t: &Self::WireType) -> Result<Self, ParserError> where Self: Sized;
}

struct Identity {
    public_key: [u8; PUBLIC_KEY_LENGTH],
    public_key_type: SigningKeyType,
}

impl Wire for Identity {
    type WireType = crate::wire::identity::Identity;

    fn to_wire(&self) -> Self::WireType {
        let mut w = Self::WireType::new();
        w.public_key = Some(self.public_key.to_vec());
        w.public_key_type = Some(EnumOrUnknown::new(self.public_key_type));
        w
    }

    fn from_wire(t: &Self::WireType) -> Result<Self, ParserError> {
        // check field presence: public_key.
        let public_key: Vec<u8> = t.public_key.clone()
            .ok_or(ParserError::MissingRequiredField("public_key"))?;
        // check field size: public_key.
        let public_key: &[u8; PUBLIC_KEY_LENGTH] = public_key.as_array::<PUBLIC_KEY_LENGTH>()
            .ok_or(ParserError::WronglySizedField {
                reason: "public_key",
                expected: PUBLIC_KEY_LENGTH,
                present: public_key.len(),
            })?;
        // check field presence: public_key_type.
        let public_key_type = t.public_key_type
            .and_then(|k| k.enum_value().ok())
            .ok_or(ParserError::MissingRequiredField("public_key_type"))?;
        let result = Self {
            public_key: *public_key,
            public_key_type,
        };
        Ok(result)
    }
}

struct ChainAction {
    counter: u32,
    created_at: u64,
    device_origin: Identity,
    action_type: ChainActionType,
    prev_signature: Vec<u8>,
    // stored by the parser to enable verification
    __bytes: Option<Vec<u8>>,
}

impl Wire for ChainAction {
    type WireType = crate::wire::identity::ChainAction;
 
    fn to_wire(&self) -> Self::WireType {
        let w = Self::WireType::new();
        w.counter = Some(&self.counter);
        w.created_at = Some(&self.created_at);
        w.device_origin = Some(&self.device_origin);
        w.action_type = Some(EnumOrUnknown::new(&self.action_type));
        w.prev_signature = Some(&self.prev_signature.to_vec());
        w
    }

    fn from_wire(t: &Self::WireType) -> Result<Self, ParserError> {
        //TODO
        let bytes = t.write_to_bytes().unwrap();
        let result = Self {
            counter,
            created_at,
            device_origin,
            action_type,
            prev_signature,
            __bytes: bytes,
        };
        Ok(result)
    }
}

struct UserMetadata {
    version: UserMetadataVersion,
    updated_at: u64,
    display_name: String,
    avatar_hash: Vec<u8>,
    __bytes: Option<Vec<u8>>,
}

struct UserIdentity {
    chain_key: Identity,
    action_chain: Vec<ChainAction>,
    metadata: UserMetadata,
}

//pub fn create_identity(
//    key_type: SigningKeyType,
//    initial_username: &str,
//) -> UserIdentity {
//    let mut csprng = OsRng;
//    let signing_key: SigningKey = SigningKey::generate(&mut csprng);
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1 + 2 == 3, true);
    }
}
