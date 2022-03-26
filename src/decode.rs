/// functionality for decoding bencoded byte strings
use nom::{
    bytes::complete::{tag, take_until1},
    character::complete::{digit1, u64},
    combinator::map_parser,
    multi::length_data,
    sequence::{delimited,terminated},
    IResult,
};
use serde::{
    de::{self, EnumAccess, IntoDeserializer, VariantAccess},
    Deserialize,
};

use crate::Error;

//
// ------------------------------- NOM -------------------------------
//

fn int<'a>(i: &'a [u8]) -> IResult<&'a [u8], u64> {
    map_parser(delimited(tag("i"), take_until1("e"), tag("e")), u64)(i)
}

fn str<'a>(i: &'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    length_data(map_parser(terminated(digit1, tag(":")), u64))(i)
}

//
// ------------------------------- SERDE -------------------------------
//

pub struct Deserializer<'de> {
    input: &'de [u8],
    pos: usize,
}

impl<'de> Deserializer<'de> {
    pub fn from_bytes(input: &'de [u8]) -> Self {
        Deserializer { input, pos: 0 }
    }
}

pub fn from_bytes<'a, T: Deserialize<'a>>(i: &'a [u8]) -> Result<T, Error> {
    let mut deserializer = Deserializer::from_bytes(i);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input.len() <= deserializer.pos {
        Ok(t)
    } else {
        Err(Error::Message("trailing bytes".to_string()))
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.input[self.pos] {
            b'i' => self.deserialize_u64(visitor),
            b'0'..=b'9' => self.deserialize_bytes(visitor),
            b'l' => self.deserialize_seq(visitor),
            b'd' => self.deserialize_map(visitor),
            _ => Err(Error::Message("no match on any".to_string())),
        }
    }

    fn deserialize_bool<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::Unimplemented)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_u64(visitor)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_u64(visitor)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_u64(visitor)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_u64(visitor)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_u64(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_u64(visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_u64(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let len = self.input.len();
        let res = int(&self.input[self.pos..]).map_err(|e| Error::Message(e.to_string()))?;
        self.pos = len - res.0.len();
        visitor.visit_u64(res.1)
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::Unimplemented)
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::Unimplemented)
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::Unimplemented)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let len = self.input.len();
        let res = str(&self.input[self.pos..]).map_err(|e| Error::Message(e.to_string()))?;
        self.pos = len - res.0.len();
        let str = std::str::from_utf8(res.1).map_err(|e| Error::Message(e.to_string()))?;

        visitor.visit_borrowed_str(str)
    }

    fn deserialize_string<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::Unimplemented)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let len = self.input.len();
        let res = str(&self.input[self.pos..]).map_err(|e| Error::Message(e.to_string()))?;
        self.pos = len - res.0.len();
        visitor.visit_borrowed_bytes(res.1)
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::Unimplemented)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_some(self)   
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::Unimplemented)
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::Unimplemented)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.input[self.pos] {
            b'l' => {
                self.pos += 1;
                let seq = visitor.visit_seq(SeqMap::new(&mut self))?;
                match self.input[self.pos] {
                    b'e' => {
                        self.pos += 1;
                        Ok(seq)
                    }
                    _ => Err(Error::Message("seq end de error".to_string())),
                }
            }
            _ => Err(Error::Message("seq start de error".to_string())),
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.input[self.pos] {
            b'd' => {
                self.pos += 1;
                let map = visitor.visit_map(SeqMap::new(&mut self))?;
                match self.input[self.pos] {
                    b'e' => {
                        self.pos += 1;
                        Ok(map)
                    }
                    _ => Err(Error::Message("map end de error".to_string())),
                }
            }
            _ => Err(Error::Message("map start de error".to_string())),
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.input[self.pos] {
            b'0'..=b'9' => {
                let len = self.input.len();
                let res =
                    str(&self.input[self.pos..]).map_err(|e| Error::Message(e.to_string()))?;
                self.pos = len - res.0.len();
                visitor.visit_enum(std::str::from_utf8(res.1).unwrap().into_deserializer())
            }
            b'd' => {
                self.pos += 1;
                let value = visitor.visit_enum(Enum::new(self))?;
                match self.input[self.pos] {
                    b'e' => {
                        self.pos += 1;
                        Ok(value)
                    }
                    _ => Err(Error::Message("enum end de error".to_string())),
                }
            }
            _ => Err(Error::Message("enum start de error".to_string())),
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

struct SeqMap<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> SeqMap<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        Self { de }
    }
}

impl<'de, 'a> de::SeqAccess<'de> for SeqMap<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        match self.de.input[self.de.pos] {
            b'e' => Ok(None),
            _ => seed.deserialize(&mut *self.de).map(Some),
        }
    }
}

impl<'de, 'a> de::MapAccess<'de> for SeqMap<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        match self.de.input[self.de.pos] {
            b'e' => Ok(None),
            _ => seed.deserialize(&mut *self.de).map(Some),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }
}

struct Enum<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> Enum<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        Self { de }
    }
}

impl<'de, 'a> EnumAccess<'de> for Enum<'a, 'de> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        Ok((seed.deserialize(&mut *self.de)?, self))
    }
}

impl<'de, 'a> VariantAccess<'de> for Enum<'a, 'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Err(Error::Message("unit variant".to_string()))
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        de::Deserializer::deserialize_seq(&mut *self.de, visitor)
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        de::Deserializer::deserialize_map(&mut *self.de, visitor)
    }
}

//
// ------------------------------- TESTS -------------------------------
//

#[test]
fn test1() {
    #[derive(serde::Deserialize, Debug, PartialEq, Eq)]
    struct C {
        i: u64,
        j: u64,
    }
    #[derive(serde::Deserialize, Debug, PartialEq, Eq)]
    struct X {
        num: u64,
        b: C,
    }
    let data = b"d3:numi1e1:bd1:ii32e1:ji64eee";

    let x = from_bytes::<X>(data).unwrap();
    let y = X {
        num: 1,
        b: C { i: 32, j: 64 },
    };

    assert_eq!(x, y)
}

#[test]
fn test2() {
    #[derive(serde::Deserialize, Debug, PartialEq, Eq)]
    struct C<'a> {
        i: u64,
        j: &'a [u8],
    }
    #[derive(serde::Deserialize, Debug, PartialEq, Eq)]
    struct X<'a> {
        num: u64,
        #[serde(borrow)]
        b: C<'a>,
    }
    let data = b"d3:numi1e1:bd1:ii32e1:j4:weeeee";
    let x = from_bytes::<X>(data).unwrap();
    let y = X {
        num: 1,
        b: C { i: 32, j: b"weee" },
    };

    assert_eq!(x, y)
}

#[test]
fn test3() {
    #[derive(serde::Deserialize, Debug, PartialEq, Eq)]
    #[serde(untagged)]
    enum R<'a> {
        A { id: &'a [u8] },
    }
    #[derive(serde::Deserialize, Debug, PartialEq, Eq)]
    #[serde(untagged)]
    enum X<'a> {
        A {
            #[serde(borrow)]
            r: R<'a>,
            t: &'a [u8],
            y: &'a [u8],
        },
        B {
            t: &'a [u8],
            y: &'a [u8],
            q: &'a [u8],
            #[serde(borrow)]
            a: R<'a>,
        },
    }

    let data = b"d1:ad2:id20:abcdefghij0123456789e1:q4:ping1:t2:aa1:y1:qe";

    let x = from_bytes::<X>(data).unwrap();
    let y = X::B {
        t: b"aa",
        y: b"q",
        q: b"ping",
        a: R::A {
            id: b"abcdefghij0123456789",
        },
    };

    assert_eq!(x, y)
}
