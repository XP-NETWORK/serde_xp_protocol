use crate::{
    XpProtocol,
};
use bincode::{Options, DefaultOptions, Result};

pub fn to_bytes<'a>(v: &XpProtocol<'a>) -> Result<Vec<u8>> {
    DefaultOptions::new()
        .with_big_endian()
        .with_fixint_encoding()
        .serialize(v)
}
