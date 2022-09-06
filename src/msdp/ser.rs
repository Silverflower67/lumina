use super::error::{Error, Result};
use bytes::{BufMut, BytesMut};
use serde::{ser, Serialize};

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
        let b = if v { &b"true" } else { &b"false" };
        self.output.put(b);
        Ok(())
    }
    fn serialize_i64(self, v: i64) -> Result<()> {
        self.output.put(&v.to_string());
        Ok(())
    }
    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }
    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }
    fn serialize_i32(self, v: i32) -> Result<(), Self::Error> {
        self.serialize_i64(i64::from(v))
    }
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(f64::from(v))
    }
    fn serialize_f64(self, v: f64) -> Result<()> {
        self.output.put(&v.to_string());
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.output.put(&v.to_string());
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
        self.output.put(v);
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(&v.to_string())
    }
}

impl<'a> ser::SerializeSeq for Serializer {
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

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.output.put_u8(2);
        value.serialize(&mut **self)
    }
    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.output.put_u8(6);
        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
       where
               T: Serialize {
           self.output.put_u8(2);
           value.serialize(&mut **self)
       }
    fn end(self) -> Result<()> {
           self.output.put(&[4,6]);
           Ok(())
       }
}
