use crate::{
    Flags,
    XpProtocol
};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

pub fn from_bytes<'a, T: Deserialize<'a> + Serialize, E>(v: &'a [u8], de: fn(&'a [u8]) -> Result<T, E>) -> Result<XpProtocol<'a, T>, E>
where
    E: serde::de::Error + serde::ser::Error
{
    XpProtocol::new(u16::from_be_bytes(v[0..2].try_into().unwrap()), Flags(&v[2..3]), &v[8..], de)
}
