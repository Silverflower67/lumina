use serde::{ser,Serialize};
use super::error::{Error, Result};

pub struct Serializer {
    output: Vec<u8>,
}
