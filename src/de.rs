use crate::{
    Flags,
    XpProtocol
};
use std::convert::TryInto;

pub fn from_bytes<'a>(v: &'a [u8]) -> XpProtocol {
    XpProtocol::new(u16::from_be_bytes(v[0..2].try_into().unwrap()), Flags(u16::from_be_bytes(v[2..4].try_into().unwrap())), &v[8..])
}
