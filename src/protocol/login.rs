use crate::protocol::types;
use crate::protocol::types::Property;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug)]
pub enum LoginState {
    Start([u8; 4]), // before C -> S: Encryption Response
    Verified,       // before C -> S:
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MojangAuthenticateResult {
    pub id: Uuid,
    pub name: String,
    pub properties: Vec<MojangProfileProperty>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MojangProfileProperty {
    name: String,
    value: String,
    signature: String,
}

impl From<&MojangProfileProperty> for types::Property {
    fn from(value: &MojangProfileProperty) -> Self {
        Property {
            name: types::String(value.name.clone()),
            value: types::String(value.value.clone()),
            signature: types::PrefixedOptional(Some(types::String(value.signature.clone()))),
        }
    }
}
