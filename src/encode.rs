// // functionality for encoding bencode trees
// #![allow(dead_code)]

// use super::Item;

// use std::{collections::BTreeMap, str::from_utf8};

// fn encode_int(int: u64) -> Vec<u8> {
//     format!("i{}e", int).as_bytes().to_vec()
// }

// fn encode_str<'a>(str: &'a [u8]) -> Vec<u8> {
//     format!("{}:{}", str.len(), from_utf8(str).unwrap())
//         .as_bytes()
//         .to_vec()
// }

// fn encode_dict<'a>(dict: BTreeMap<&'a [u8], Item<'a>>) -> Vec<u8> {
//     let mut encdict: Vec<u8> = vec![b'd'];
//     for (key, val) in dict {
//         encdict.extend_from_slice(&encode_str(&key));
//         match val {
//             Item::Int(int) => encdict.extend_from_slice(&encode_int(int)),
//             Item::String(str) => encdict.extend_from_slice(&encode_str(&str)),
//             Item::List(list) => encdict.extend_from_slice(&encode_list(list)),
//             Item::Dict(dict) => encdict.extend_from_slice(&encode_dict(dict)),
//         }
//     }
//     encdict.push(b'e');
//     encdict
// }

// fn encode_list<'a>(list: Vec<Item<'a>>) -> Vec<u8> {
//     let mut enclist: Vec<u8> = vec![b'l'];
//     for item in list {
//         match item {
//             Item::Int(int) => enclist.extend_from_slice(&encode_int(int)),
//             Item::String(str) => enclist.extend_from_slice(&encode_str(&str)),
//             Item::List(list) => enclist.extend_from_slice(&encode_list(list)),
//             Item::Dict(dict) => enclist.extend_from_slice(&encode_dict(dict)),
//         }
//     }
//     enclist.push(b'e');
//     enclist
// }

// pub fn encode<'a>(tree: Vec<Item<'a>>) -> Vec<u8> {
//     let mut enctree: Vec<u8> = vec![];
//     for item in tree {
//         match item {
//             Item::Int(int) => enctree.extend_from_slice(&encode_int(int)),
//             Item::String(str) => enctree.extend_from_slice(&encode_str(&str)),
//             Item::List(list) => enctree.extend_from_slice(&encode_list(list)),
//             Item::Dict(dict) => enctree.extend_from_slice(&encode_dict(dict)),
//         }
//     }
//     enctree
// }
