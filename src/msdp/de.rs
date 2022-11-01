use super::error::{Error, Result};

use nom::bytes::complete::{is_a, is_not, take, take_while};
use nom::character::complete::{alphanumeric1, digit1};
use nom::combinator::{map, map_parser, not, opt, eof, peek};
use nom::error::Error as NomError;
use nom::number::complete::be_u8;
use nom::sequence::{delimited, pair, preceded, terminated};
use nom::Err as NomErr;
use nom::{branch::alt, bytes::complete::tag, error::ErrorKind, Finish, IResult};
use serde::{
    de::{
        self, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess, VariantAccess,
        Visitor,
    },
    Deserialize,
};
use std::num::NonZeroI128;
use std::ops::{AddAssign, Mul, MulAssign, Neg};

pub struct Deserializer<'de> {
    input: &'de [u8],
}

impl<'de> Deserializer<'de> {
    pub fn from_slice(input: &'de [u8]) -> Self {
        Deserializer { input }
    }
    fn peek_byte(&mut self) -> Result<u8> {
        self.input.iter().next().cloned().ok_or(Error::Eof)
    }
    fn next_byte(&mut self) -> Result<u8> {
        self.input.into_iter().next().cloned().ok_or(Error::Eof)
    }
}

pub fn from_slice<'a, T>(input: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_slice(input);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(Error::TrailingBytes)
    }
}

fn parse_bool(i: &[u8]) -> IResult<&[u8], bool,Error> {
    let mut bool_tags = alt((tag(b"TRUE"), tag(b"FALSE")));
    let (i, output) = bool_tags(i)?;
    let parsed: bool;
    if output == &b"TRUE"[..] {
        parsed = true
    } else if output == &b"FALSE"[..] {
        parsed = false
    } else {
        return Err(NomErr::Error(Error::Parse("Expected boolean")));
    }
    Ok((i, parsed))
}

fn parse_unsigned<T>(i: &[u8]) -> IResult<&[u8], T>
where
    T: AddAssign<T> + MulAssign<T> + From<u8>,
{
    let (i, int) = nom::character::complete::u8(i)?;
    Ok((i, T::from(int)))
}

fn parse_signed<T>(i: &[u8]) -> IResult<&[u8], T>
where
    T: Neg<Output = T> + AddAssign<T> + MulAssign<T> + From<i8>,
{
    let (i, minus) = opt(tag("-"))(i)?;
    let is_neg = minus.is_some();
    let (i, int): (_, i16) = parse_unsigned(i)?;
    let parsed = if is_neg {
        T::from(-int as i8)
    } else {
        T::from(int as i8)
    };
    Ok((i, parsed))
}


fn dbytes(i: &[u8]) -> IResult<&[u8], &[u8],Error> {
    is_a(&b"\x01\x02\x03\x04\x05\x06"[..])(i)
}
fn not_dbytes(i: &[u8]) -> IResult<&[u8], &[u8],Error> {
    is_not(&b"\x01\x02\x03\x04\x05\x06"[..])(i)
}
fn parse_string(i: &[u8]) -> IResult<&[u8], String,Error> {
    let (i, data) = not_dbytes(i)?;
    Ok((i, String::from_utf8(data.to_owned()).unwrap()))
}

macro_rules! de_clos {
    ($e:expr) => {
        |_: Error| $e
    };
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.peek_byte()? {
            b'N' => self.deserialize_unit(visitor),
            b'T' | b'F' => self.deserialize_bool(visitor),
            b'0'..=b'9' => self.deserialize_u64(visitor),
            b'-' => self.deserialize_i64(visitor),
            5 => self.deserialize_seq(visitor),
            3 => self.deserialize_map(visitor),
            _ => self.deserialize_str(visitor),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (i, parsed) = parse_bool(self.input).map_err(|_| Error::Parse("Expected boolean"))?;
        self.input = i;
        visitor.visit_bool(parsed)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (i, parsed) = parse_signed(self.input).map_err(|_| Error::Parse("Expected integer"))?;
        self.input = i;
        visitor.visit_i8(parsed)
    }
    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (i, parsed) = parse_signed(self.input).map_err(|_| Error::Parse("Expected integer"))?;
        self.input = i;
        visitor.visit_i16(parsed)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (i, parsed) = parse_signed(self.input).map_err(|_| Error::Parse("Expected integer"))?;
        self.input = i;
        visitor.visit_i32(parsed)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (i, parsed) = parse_signed(self.input).map_err(|_| Error::Parse("Expected integer"))?;
        self.input = i;
        visitor.visit_i64(parsed)
    }
    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (i, parsed) =
            parse_unsigned(self.input).map_err(|_| Error::Parse("Expected integer"))?;
        self.input = i;
        visitor.visit_u8(parsed)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (i, parsed) =
            parse_unsigned(self.input).map_err(|_| Error::Parse("Expected integer"))?;
        self.input = i;
        visitor.visit_u16(parsed)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (i, parsed) =
            parse_unsigned(self.input).map_err(|_| Error::Parse("Expected integer"))?;
        self.input = i;
        visitor.visit_u32(parsed)
    }
    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (i, parsed) =
            parse_unsigned(self.input).map_err(|_| Error::Parse("Expected integer"))?;
            self.input = i;
        visitor.visit_u64(parsed)
    }
    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (i, v) = parse_string(self.input).map_err(|_| Error::Parse("Error parsing string"))?;
        self.input = i;
        visitor.visit_string(v)
    }
    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (i, null) = opt::<_,_,Error,_>(tag("NULL"))(self.input).unwrap();
        self.input = i;
        if null.is_some() {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (i, _) = tag(b"NULL")(self.input).map_err(|_: NomErr<Error>| Error::Parse("Expected null"))?;
        self.input = i;
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        fn start(i: &[u8]) -> IResult<&[u8],&[u8],Error> {
            tag(b"\x05")(i)
        }
        let (i,_) = start(self.input).map_err(|_| Error::ExpectedArrayStart)?;
        let value = visitor.visit_seq(DByteSeparator::new(self))?;
        fn end(i: &[u8]) -> IResult<&[u8],&[u8], Error> {
            tag(b"\x06")(i)
        }
        let (i,_) = end(i).map_err(|_| Error::ExpectedArrayEnd)?;
        self.input = i;
        Ok(value)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de> {
        fn start(i: &[u8]) -> IResult<&[u8],&[u8],Error> {
            tag(b"\x03")(i)
        }
        fn end(i: &[u8]) -> IResult<&[u8],&[u8],Error> {
            tag(b"\x04")(i)
        }
        (self.input,_) = start(self.input).map_err(|_| Error::ExpectedMapStart)?;
        let value = visitor.visit_map(DByteSeparator::new(self))?;
        (self.input,_) = end(self.input).map_err(|_| Error::ExpectedMapEnd)?;
        Ok(value)

    }
    fn deserialize_struct<V>(
            self,
            _name: &'static str,
            _fields: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value>
        where
            V: Visitor<'de> {
        self.deserialize_map(visitor)
    }
    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de> {
        self.deserialize_seq(visitor)
    }
    fn deserialize_tuple_struct<V>(
            self,
            _name: &'static str,
            _len: usize,
            visitor: V,
        ) -> Result<V::Value>
        where
            V: Visitor<'de> {
        self.deserialize_seq(visitor)
    }
    fn deserialize_enum<V>(
            self,
            _name: &'static str,
            _variants: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value>
        where
            V: Visitor<'de> {
            if peek(not_dbytes)(self.input).is_ok() {
                visitor.visit_enum(parse_string(self.input).finish()?.1.into_deserializer())
            } else if self.next_byte()? == 3{
                let value = visitor.visit_enum(Enum::new(self))?;
                if self.next_byte()? == 4 {
                    Ok(value)
                } else {
                    Err(Error::ExpectedMapEnd)
                }
            } else {
                Err(Error::Parse("Expected enum"))
            }
    }
    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de> {
        self.deserialize_str(visitor)
    }
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de> {
        self.deserialize_any(visitor)
    }
}

struct DByteSeparator<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

struct Enum<'a,'de: 'a> {
    de: &'a mut Deserializer<'de>
}

impl<'a,'de> Enum<'a,'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        Enum { de }
    }
}

impl<'de,'a> EnumAccess<'de> for Enum<'a,'de>  {
    type Error = Error;
    type Variant = Self;
    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
        where
            V: DeserializeSeed<'de> {
        let (i,key) = var(self.de.input).map_err(|_| Error::ExpectedVar)?;
        self.de.input = i;
        let string_de = key.to_ascii_lowercase().into_deserializer();
        let val = seed.deserialize(string_de)?;
        if self.de.next_byte()? == 2 {
            Ok((val,self))
        } else {
            Err(Error::ExpectedVal)
        }
    }
}

impl<'de,'a> VariantAccess<'de> for Enum<'a,'de> {
    type Error = Error;
    fn unit_variant(self) -> Result<()> {
        Err(Error::Parse("Expected string"))
    }
    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
        where
            T: DeserializeSeed<'de> {
        seed.deserialize(self.de)
    }
    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de> {
        de::Deserializer::deserialize_seq(self.de, visitor)
    }
    fn struct_variant<V>(
            self,
            _fields: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value>
        where
            V: Visitor<'de> {
        de::Deserializer::deserialize_map(self.de, visitor)
    }
}

impl<'a, 'de> DByteSeparator<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        DByteSeparator { de }
    }
}

fn val(i: &[u8]) -> IResult<&[u8], &[u8],Error> {
   tag(b"\x02")(i)
}
fn var(i: &[u8]) -> IResult<&[u8], &[u8],Error> {
    preceded(tag(b"\x01"), not_dbytes)(i)
}

impl<'de, 'a> SeqAccess<'de> for DByteSeparator<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        let (_,eof) = opt::<_,_,NomError<_>,_>(peek(tag(&b"\x06"[..])))(self.de.input).unwrap();
        if eof.is_some() {
            return Ok(None);
        }
        let (i, _) = val(self.de.input).map_err(|_| Error::ExpectedVal)?;
        self.de.input = i;
        seed.deserialize(&mut *self.de).map(Some)
    }
}

impl<'de, 'a> MapAccess<'de> for DByteSeparator<'a, 'de> {
    type Error = Error;
    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        let (_,end) = opt::<_,_,Error,_>(peek(tag(b"\x04")))(self.de.input).unwrap();
        if end.is_some() {
            return Ok(None);
        }
        let (i,key) = var(self.de.input).map_err(|_| Error::ExpectedVar)?;
        self.de.input = i;
        let (_,skey) = parse_string(key).finish()?;
        seed.deserialize(skey.to_ascii_lowercase().into_deserializer()).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
        where
            V: DeserializeSeed<'de> {
        let (i,_) = val(self.de.input).map_err(|_|Error::ExpectedVal)?;
        self.de.input = i;
        seed.deserialize(&mut *self.de)
    }
}