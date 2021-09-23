#![allow(dead_code)]
// bencode subfolder and item enum implemenation

pub mod decode;
pub mod encode;

use std::collections::BTreeMap;

#[derive(Debug)]
pub enum Item<'a> {
    Int(u64),
    String(&'a [u8]),
    List(Vec<Item<'a>>),
    Dict(BTreeMap<&'a [u8], Item<'a>>),
}

impl<'a> Item<'a> {
    pub fn get_int(&'a self) -> u64 {
        match self {
            Item::Int(int) => *int,
            _ => unreachable!(),
        }
    }
    pub fn get_str(&'a self) -> &'a [u8] {
        match self {
            Item::String(str) => str,
            _ => unreachable!(),
        }
    }
    pub fn get_list(&'a self) -> &'a Vec<Item<'a>> {
        match self {
            Item::List(list) => list,
            _ => unreachable!(),
        }
    }
    pub fn get_dict(&'a self) -> &'a BTreeMap<&'a [u8], Item<'a>> {
        match self {
            Item::Dict(dict) => dict,
            _ => unreachable!(),
        }
    }
}