use crate::{
    XpProtocol,
};
use bincode::{DefaultOptions, Options, Result};

pub fn from_bytes<'a>(v: &'a [u8]) -> Result<XpProtocol> {
    DefaultOptions::new()
        .with_big_endian()
        .with_fixint_encoding()
        .deserialize(v)
}
