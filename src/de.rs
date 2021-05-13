use crate::{
    Flags,
    XpProtocol
};
use serde::{de::DeserializeOwned, Serialize};
use std::convert::TryInto;

pub fn from_bytes<'a, T: DeserializeOwned + Serialize, E>(v: &'a [u8], de: fn(&'a [u8]) -> Result<T, E>) -> Result<XpProtocol<T>, E>
where
    E: serde::de::Error + serde::ser::Error
{
    XpProtocol::new(u16::from_be_bytes(v[0..2].try_into().unwrap()), Flags(u16::from_be_bytes(v[2..4].try_into().unwrap())), &v[8..], de)
}
