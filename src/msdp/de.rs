use std::ops::{AddAssign,MulAssign,Neg};
use serde::Deserialize;
use bytes::{BytesMut, BufMut};
use serde::de::{
    self,DeserializeSeed,EnumAccess,IntoDeserializer,MapAccess,SeqAccess,VariantAccess,Visitor
};
use super::error::{Error,Result};

pub struct Deserializer<'de> {
    input: &'de BytesMut,
}

impl<'de> Deserializer<'de> {
    pub fn from_vec(input: &'de Vec<u8>) -> Self {
        let mut b = BytesMut::new();
        b.copy_from_slice(&input[..]);
        Deserializer { input: }
    }
}