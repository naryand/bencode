use serde::{ser, Serialize};

use crate::Error;

// functionality for encoding bencode trees

pub struct Serializer {
    output: Vec<u8>,
}

pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>, Error>
where
    T: Serialize,
{
    let mut serializer = Serializer { output: Vec::new() };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.output.push(b'i');
        self.output.extend_from_slice(v.to_string().as_bytes());
        self.output.push(b'e');
        Ok(())
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.serialize_bytes(v.as_bytes())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.output
            .extend_from_slice(v.len().to_string().as_bytes());
        self.output.push(b':');
        self.output.extend_from_slice(v);
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
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
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        self.output.extend_from_slice(b"d");
        variant.serialize(&mut *self)?;
        self.output.extend_from_slice(b"e");
        value.serialize(&mut *self)?;
        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.output.extend_from_slice(b"l");
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.output.extend_from_slice(b"d");
        variant.serialize(&mut *self)?;
        self.output.extend_from_slice(b"l");
        Ok(self)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.output.extend_from_slice(b"d");
        Ok(self)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.output.extend_from_slice(b"d");
        variant.serialize(&mut *self)?;
        self.output.extend_from_slice(b"d");
        Ok(self)
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.output.extend_from_slice(b"e");
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.output.extend_from_slice(b"e");
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
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.output.extend_from_slice(b"e");
        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.output.extend_from_slice(b"ee");
        Ok(())
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        key.serialize(&mut **self)
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.output.extend_from_slice(b"e");
        Ok(())
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.output.extend_from_slice(b"e");
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.output.extend_from_slice(b"ee");
        Ok(())
    }
}

#[test]
fn test1() {
    #[derive(serde::Serialize, Debug, PartialEq, Eq)]
    struct C {
        i: u64,
        j: u64,
    }
    #[derive(serde::Serialize, Debug, PartialEq, Eq)]
    struct X {
        num: u64,
        b: C,
    }

    let data = b"d3:numi1e1:bd1:ii32e1:ji64eee";
    let y = X {
        num: 1,
        b: C { i: 32, j: 64 },
    };

    let x = to_bytes::<X>(&y).unwrap();
    assert_eq!(x, data)
}

#[test]
fn test2() {
    #[derive(serde::Serialize, Debug, PartialEq, Eq)]
    struct C<'a> {
        i: u64,
        #[serde(borrow)]
        #[serde(with = "serde_bytes")]
        j: &'a [u8],
    }
    #[derive(serde::Serialize, Debug, PartialEq, Eq)]
    struct X<'a> {
        num: u64,
        #[serde(borrow)]
        b: C<'a>,
    }

    let data = b"d3:numi1e1:bd1:ii32e1:j4:weedee";
    let y = X {
        num: 1,
        b: C { i: 32, j: b"weed" },
    };

    let x = to_bytes::<X>(&y).unwrap();
    assert_eq!(x, data)
}

#[test]
fn test3() {
    #[derive(serde::Serialize, Debug, PartialEq, Eq)]
    #[serde(untagged)]
    enum R<'a> {
        A {
            #[serde(borrow)]
            #[serde(with = "serde_bytes")]
            id: &'a [u8],
        },
    }
    #[derive(serde::Serialize, Debug, PartialEq, Eq)]
    #[serde(untagged)]
    enum X<'a> {
        #[allow(dead_code)]
        A {
            #[serde(borrow)]
            r: R<'a>,
            #[serde(borrow)]
            #[serde(with = "serde_bytes")]
            t: &'a [u8],
            #[serde(borrow)]
            #[serde(with = "serde_bytes")]
            y: &'a [u8],
        },
        B {
            #[serde(borrow)]
            a: R<'a>,
            #[serde(borrow)]
            #[serde(with = "serde_bytes")]
            q: &'a [u8],
            #[serde(borrow)]
            #[serde(with = "serde_bytes")]
            t: &'a [u8],
            #[serde(borrow)]
            #[serde(with = "serde_bytes")]
            y: &'a [u8],
        },
    }

    let data = b"d1:ad2:id20:abcdefghij0123456789e1:q4:ping1:t2:aa1:y1:qe";
    let y = X::B {
        a: R::A {
            id: b"abcdefghij0123456789",
        },
        q: b"ping",
        t: b"aa",
        y: b"q",
    };

    let x = to_bytes::<X>(&y).unwrap();
    assert_eq!(x, data)
}
