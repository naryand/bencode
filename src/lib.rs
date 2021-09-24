#![allow(dead_code)]
// bencode subfolder and item enum implemenation

pub mod decode;
pub mod encode;

#[derive(Debug)]
pub enum Item<'a> {
    Int(u64),
    String(&'a [u8]),
    List(()),
    Dict(()),
}
