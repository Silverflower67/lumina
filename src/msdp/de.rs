use std::ops::{AddAssign,MulAssign,Neg};
use serde::Deserialize;
use bytes::{BytesMut, BufMut};
use serde::de::{
    self,DeserializeSeed,EnumAccess,IntoDeserializer,MapAccess,SeqAccess,VariantAccess,Visitor
};
use super::error::{Error,Result};

pub struct Deserializer<'de> {
    input: &'de Vec<u8>,
}

impl<'de> Deserializer<'de> {
    pub fn from_vec(input: &'de Vec<u8>) -> Self {
        Deserializer { input }
    }
}

pub fn from_vec<'a,T>(v: &'a Vec<u8>) -> Result<T> where T: Deserialize<'a> {
    let mut deserializer = Deserializer::from_vec(v);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(Error::TrailingBytes)
    }
}

impl<'de> Deserializer<'de> {
    fn peek_byte(&mut self) -> Result<u8> {
        self.input.iter().next().ok_or(Error::Eof)
    }
    fn next_byte(&mut self) -> Result<u8> {
        let b = self.peek_byte()?;
        self.input.remove(0);
        Ok(b)
    }
    fn parse_bool(&mut self) -> Result<bool> {
    }
}
