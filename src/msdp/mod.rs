//! This module provides utils for serializing and deserializing data in the [MSDP](https://mudhalla.net/tintin/protocols/msdp/) protocol
mod error;
mod ser;
mod de;
#[cfg(test)]
mod tests;

pub use ser::{Serializer,to_vec};
pub use de::{Deserializer,from_slice};
pub use error::{Error,Result};