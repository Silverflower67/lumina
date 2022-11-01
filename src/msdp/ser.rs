use super::error::{Error, Result};
use bytes::{BufMut, BytesMut};
use serde::{ser, Serialize};


/// Convert a value of type `T` to a MSDP-ready [`Vec<u8>`] (doesn't include IACs)
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let mut serializer = Serializer {
        output: BytesMut::new(),
    };
    value.serialize(&mut serializer)?;
    Ok(serializer.output.to_vec())
}

/// An [MSDP](https://mudhalla.net/tintin/protocols/msdp/) `serde` serializer.<br/>
/// It's not recommended to use this struct directly; use [`to_vec`] instead
pub struct Serializer {
    output: BytesMut,
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeMap = Self;
    type SerializeSeq = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        let b = if v { &b"TRUE"[..] } else { &b"FALSE"[..] };
        self.output.put(b);
        Ok(())
    }
    fn serialize_i64(self, v: i64) -> Result<()> {
        self.output.put((&v.to_string()).as_bytes());
        Ok(())
    }
    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }
    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }
    fn serialize_i32(self, v: i32) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }
    fn serialize_f32(self, v: f32) -> Result<()> {
        self.serialize_f64(f64::from(v))
    }
    fn serialize_f64(self, v: f64) -> Result<()> {
        self.output.put((&v.to_string()).as_bytes());
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.output.put((&v.to_string()).as_bytes());
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.output.put(v.as_bytes());
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        use ser::SerializeSeq;
        let mut seq = self.serialize_seq(Some(v.len()))?;
        for byte in v {
            seq.serialize_element(byte)?;
        }
        seq.end()
    }

    fn serialize_none(self) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(self)
    }
    fn serialize_unit(self) -> Result<()> {
        self.output.put(&b"NULL"[..]);
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.serialize_str(variant)
    }
    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(self)
    }
    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: Serialize,
    {
        self.output.put(&[3, 1][..]);
        variant.serialize(&mut *self)?;
        self.output.put_u8(2);
        value.serialize(&mut *self)?;
        self.output.put_u8(4);
        Ok(())
    }
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.output.put_u8(5);
        Ok(self)
    }
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.output.put(&[3, 1][..]);
        variant.serialize(&mut *self)?;
        self.output.put(&[2, 5][..]);
        Ok(self)
    }
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        self.output.put_u8(3);
        Ok(self)
    }
    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }
    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.output.put(&[3, 1][..]);
        variant.serialize(&mut *self)?;
        self.output.put(&[2, 4][..]);
        Ok(self)
    }
}
impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.output.put_u8(2);
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output.put_u8(6);
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.output.put_u8(2);
        value.serialize(&mut **self)
    }
    fn end(self) -> Result<()> {
        self.output.put_u8(6);
        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.output.put_u8(2);
        value.serialize(&mut **self)
    }
    fn end(self) -> Result<()> {
        self.output.put_u8(6);
        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.output.put_u8(2);
        value.serialize(&mut **self)
    }
    fn end(self) -> Result<()> {
        self.output.put(&[4, 6][..]);
        Ok(())
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.output.put_u8(1);
        let mut temp_ser = Serializer {output: BytesMut::new()};
        key.serialize(&mut temp_ser)?;
        let up = temp_ser.output.to_ascii_uppercase();
        self.output.put(&up[..]);
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.output.put_u8(2);
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output.put_u8(4);
        Ok(())
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.output.put_u8(1);
        let mut temp_ser = Serializer {output: BytesMut::new()};
        key.serialize(&mut temp_ser)?;
        let up = temp_ser.output.to_ascii_uppercase();
        self.output.put(&up[..]);
        self.output.put_u8(2);
        value.serialize(&mut **self)
    }
    fn end(self) -> Result<()> {
        self.output.put_u8(4);
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        let mut temp_ser = Serializer {output: BytesMut::new()};
        key.serialize(&mut temp_ser)?;
        let up = temp_ser.output.to_ascii_uppercase();
        self.output.put(&up[..]);
        value.serialize(&mut **self)
    }
    fn end(self) -> Result<()> {
        self.output.put(&[4, 4][..]);
        Ok(())
    }
}
